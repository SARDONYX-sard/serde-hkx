//! Provides a method that can be used to sort bytes and XML to serialize.

use crate::errors::ser::UnexpectedCyclicSortSnafu;

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
    fn sort_for_bytes(&mut self) -> Result<(), Self::Error>;

    /// Sort by dependent class for XML serialization.
    ///
    /// # Errors
    /// Error when using circular references. (Behavior is a state machine, so if it is correct, it should not be circular.)
    fn sort_for_xml(&mut self) -> Result<(), Self::Error>;
}

impl<V> HavokSort for indexmap::IndexMap<usize, V>
where
    V: havok_serde::HavokClass,
{
    type Error = crate::errors::ser::Error;

    #[inline]
    fn sort_for_bytes(&mut self) -> Result<(), Self::Error> {
        let min_key = self.keys().min();
        let is_official_index_rule = min_key.is_some() && min_key != Some(&1);

        if is_official_index_rule {
            crate::tri!(self.sort_for_xml());
            // Now they are in order of dependency, but with binary data, it is necessary to serialize
            // from the root of the dependency. Therefore, we reverse it.
            self.reverse();
        }
        Ok(())
    }

    fn sort_for_xml(&mut self) -> Result<(), Self::Error> {
        if self.is_empty() {
            return Ok(());
        }

        let mut topo_sort = topo_sort::TopoSort::with_capacity(self.len());
        for (&index, class) in self.iter() {
            let deps_indexes = class.deps_indexes();
            tracing::debug!("index = {index}, deps_indexes = {deps_indexes:?}");
            topo_sort.insert(index, deps_indexes);
        }

        let mut sorted_classes = indexmap::IndexMap::with_capacity(self.len());
        for result in &topo_sort {
            match result {
                Ok((index, _)) => {
                    if let Some(class) = self.swap_remove(index) {
                        sorted_classes.insert(*index, class);
                    }
                }
                Err(_) => return UnexpectedCyclicSortSnafu.fail(),
            }
        }

        *self = sorted_classes;
        Ok(())
    }
}
