//! Provides a method that can be used to sort bytes and XML to serialize.

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

impl<V> HavokSort for indexmap::IndexMap<usize, V>
where
    V: havok_serde::HavokClass,
{
    type Error = crate::errors::ser::Error;

    fn sort_for_bytes(&mut self) {
        if self.is_empty() {
            return;
        }

        let mut sorted_keys = Vec::new();
        if let Some((&root_key, root_class)) =
            self.keys().min().and_then(|min| self.get_key_value(min))
        {
            collect_deps(self, root_key, root_class, &mut sorted_keys);
        }

        #[cfg(feature = "tracing")]
        tracing::debug!("sorted_keys = {sorted_keys:?}");
        let mut sorted_classes = indexmap::IndexMap::with_capacity(self.len());
        for index in &sorted_keys {
            if let Some(class) = self.swap_remove(index) {
                sorted_classes.insert(*index, class);
            }
        }

        *self = sorted_classes;
    }

    fn sort_for_xml(&mut self) -> Result<usize, Self::Error> {
        if self.is_empty() {
            Ok(0)
        } else if let Some((first_key, first_value)) = self.shift_remove_index(0) {
            self.insert(first_key, first_value);
            Ok(first_key)
        } else {
            Err(crate::errors::ser::Error::Message {
                msg: "Missing top pointer.".to_owned(),
            })
        }
    }
}

/// Create an acyclic directed graph from the order of fields in the root to the tail branch (class of dependencies).
fn collect_deps<V>(
    classes: &indexmap::IndexMap<usize, V>,
    key: usize,
    class: &V,
    sorted: &mut Vec<usize>,
) where
    V: havok_serde::HavokClass,
{
    if sorted.contains(&key) {
        return;
    }

    sorted.push(key);
    let deps = class.deps_indexes();
    #[cfg(feature = "tracing")]
    tracing::debug!("index = {key}, deps_indexes = {deps:?}");

    for dep_key in deps {
        if let Some(dep_class) = classes.get(&dep_key) {
            collect_deps(classes, dep_key, dep_class, sorted);
        }
    }
}
