//! Provides a method that can be used to sort bytes and XML to serialize.
use crate::{errors::ser::Error as SerError, ClassMapKey, GenericClassMap};
use havok_serde::HavokClass;
use havok_types::Pointer;
use indexmap::IndexMap;
use std::borrow::Cow;

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
    fn sort_for_bytes(&mut self);

    /// Sort by dependent class for XML serialization.
    ///
    /// # Return
    /// top ptr
    ///
    /// # Errors
    /// Missing top pointer.
    fn sort_for_xml(&mut self) -> Result<Pointer<'static>, Self::Error>;
}

impl<V> HavokSort for GenericClassMap<'_, V>
where
    V: HavokClass,
{
    type Error = SerError;

    #[allow(clippy::ptr_arg)]
    fn sort_for_bytes(&mut self) {
        fn collect_deps<'bytes, V>(
            classes: &GenericClassMap<'bytes, V>,
            key: &Cow<'bytes, str>,
            sorted_keys: &mut Vec<ClassMapKey<'bytes>>,
        ) where
            V: HavokClass,
        {
            if sorted_keys.contains(key) {
                return;
            }

            sorted_keys.push(key.clone());

            if let Some(class) = classes.get(key) {
                let deps = class.deps_indexes();
                #[cfg(feature = "tracing")]
                tracing::trace!("index = {key}, deps_indexes = {deps:?}");

                for dep_key in deps {
                    let dep_key_str = Cow::Owned(dep_key.to_string());
                    collect_deps(classes, &dep_key_str, sorted_keys);
                }
            }
        }

        if self.is_empty() {
            return;
        }

        let root_key = match self.keys().min() {
            Some(min) => min.clone(),
            None => return,
        };

        let mut sorted_keys = Vec::new();
        collect_deps(self, &root_key, &mut sorted_keys);

        let mut sorted_classes = Self::with_capacity(self.len());
        for key in sorted_keys {
            if let Some(class) = self.swap_remove(&key) {
                sorted_classes.insert(key, class);
            }
        }

        *self = sorted_classes;

        #[cfg(feature = "tracing")]
        tracing::trace!("sorted_keys = {:?}", self.keys().collect::<Vec<_>>());
    }

    fn sort_for_xml(&mut self) -> Result<Pointer<'static>, Self::Error> {
        /// Create an acyclic directed graph from the order of fields in the root to the tail branch (class of dependencies).
        fn collect_deps<'a, 'xml: 'a, V>(
            classes: &'a IndexMap<ClassMapKey<'xml>, V>,
            key: &ClassMapKey<'xml>,
            class: &'a V,
            sorted: &mut Vec<ClassMapKey<'xml>>, // change 'static to 'xml here
        ) where
            V: HavokClass,
        {
            if sorted.contains(key) {
                return;
            }

            let deps = class.deps_indexes();
            #[cfg(feature = "tracing")]
            tracing::trace!("index = {key}, deps_indexes = {deps:?}");

            for dep_key in deps {
                if let Some(dep_class) = classes.get(dep_key.get()) {
                    let dep_key_str = Cow::Owned(dep_key.to_string());
                    collect_deps(classes, &dep_key_str, dep_class, sorted);
                }
            }

            sorted.push(key.clone());
        }

        let (root_key, sorted_keys) = {
            let mut sorted_keys = Vec::new();
            let (root_key, root_class) = self
                .keys()
                .min()
                .and_then(|min| self.get_key_value(min))
                .ok_or_else(|| SerError::Message {
                    msg: "Missing top pointer.".to_owned(),
                })?;

            collect_deps(self, root_key, root_class, &mut sorted_keys);

            (root_key.to_string().into(), sorted_keys)
        };

        #[cfg(feature = "tracing")]
        tracing::trace!("sorted_keys = {sorted_keys:?}");

        // Create a new IndexMap sorted by the collected dependency order
        let mut sorted_classes = Self::with_capacity(self.len());
        for key in sorted_keys {
            if let Some(class) = self.swap_remove(&key) {
                sorted_classes.insert(key, class);
            }
        }

        // Replace the original IndexMap with the sorted one
        *self = sorted_classes;

        Ok(Pointer::new(root_key))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::mocks::new_defaultmale;

    #[test]
    fn test_sort() {
        use super::*;

        let mut classes = new_defaultmale();
        classes.sort_for_bytes();

        assert_eq!(classes, new_defaultmale());
    }
}
