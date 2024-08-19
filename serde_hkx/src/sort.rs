//! Provides a method that can be used to sort bytes and XML to serialize.
use crate::errors::ser::Error as SerError;
use havok_serde::HavokClass;
use indexmap::IndexMap;

/// Trait that provides a method that can be used to sort bytes and XML to serialize
pub trait HavokSort {
    type Error;

    /// Sort by dependent class from root for serialization of bytes.
    ///
    /// There is a rule that binary data serializes class dependency pointers in order from the root, which is `hkRootLevelContainer`.
    /// This is because behavior is written by state transitions, which are state machines.
    ///
    /// # Current implementation
    /// - If deserialize a binary with `serde-hkx` -> already sorted for bytes
    /// - If deserialize xml of official SDK -> sort is required
    ///
    /// # Errors
    /// Error when using circular references. (Behavior is a state machine, so if it is correct, it should not be circular.)
    fn sort_for_bytes(&mut self);

    /// Sort by dependent class for XML serialization.
    ///
    /// # Return
    /// top ptr
    ///
    /// # Errors
    /// Error when using circular references. (Behavior is a state machine, so if it is correct, it should not be circular.)
    fn sort_for_xml(&mut self) -> Result<usize, Self::Error>;
}

impl<V> HavokSort for IndexMap<usize, V>
where
    V: HavokClass,
{
    type Error = SerError;

    fn sort_for_bytes(&mut self) {
        fn collect_deps<V>(
            classes: &mut IndexMap<usize, V>,
            key: usize,
            sorted_keys: &mut Vec<usize>,
        ) where
            V: HavokClass,
        {
            if sorted_keys.contains(&key) {
                return;
            }

            let current_index = match classes.get_index_of(&key) {
                Some(index) => index,
                None => return, // Return if the key doesn't exist.
            };

            sorted_keys.push(key);
            classes.swap_indices(sorted_keys.len().saturating_sub(1), current_index);

            if let Some(class) = classes.get(&key) {
                let deps = class.deps_indexes();

                #[cfg(feature = "tracing")]
                tracing::debug!("index = {key}, deps_indexes = {deps:?}");

                for dep_key in deps {
                    collect_deps(classes, dep_key, sorted_keys);
                }
            }
        }

        if self.is_empty() {
            return;
        }

        let &root_key = match self.keys().min() {
            Some(min) => min,
            None => return,
        };

        let mut sorted_keys = Vec::with_capacity(self.len());
        collect_deps(self, root_key, &mut sorted_keys);

        #[cfg(feature = "tracing")]
        tracing::debug!("sorted_keys = {:?}", sorted_keys);
    }

    fn sort_for_xml(&mut self) -> Result<usize, Self::Error> {
        /// Create an acyclic directed graph from the order of fields in the root to the tail branch (class of dependencies).
        fn collect_deps<V>(
            classes: &IndexMap<usize, V>,
            key: usize,
            class: &V,
            sorted: &mut Vec<usize>,
        ) where
            V: havok_serde::HavokClass,
        {
            if sorted.contains(&key) {
                return;
            }

            let deps = class.deps_indexes();
            #[cfg(feature = "tracing")]
            tracing::debug!("index = {key}, deps_indexes = {deps:?}");

            for dep_key in deps {
                if let Some(dep_class) = classes.get(&dep_key) {
                    collect_deps(classes, dep_key, dep_class, sorted);
                }
            }

            sorted.push(key);
        }

        let mut sorted_keys = Vec::new();
        let (&root_key, root_class) = self
            .keys()
            .min()
            .and_then(|min| self.get_key_value(min))
            .ok_or(SerError::Message {
                msg: "Missing top pointer.".to_owned(),
            })?;
        collect_deps(self, root_key, root_class, &mut sorted_keys);

        #[cfg(feature = "tracing")]
        tracing::debug!("sorted_keys = {sorted_keys:?}");
        let mut sorted_classes = IndexMap::with_capacity(self.len());
        for index in &sorted_keys {
            if let Some(class) = self.swap_remove(index) {
                sorted_classes.insert(*index, class);
            }
        }
        *self = sorted_classes;
        Ok(root_key)
    }
}
