//! Provides a method that can be used to sort bytes and XML to serialize.
use crate::errors::ser::Error as SerError;
use havok_serde::HavokClass;
use indexmap::IndexMap;
use std::collections::HashMap;

/// Trait that provides methods to sort classes for Havok binary/XML serialization.
pub trait HavokSort {
    type Error;

    /// Sort classes by dependency order for binary serialization.
    ///
    /// Binary HKX serialization requires class dependency pointers to be ordered
    /// from the root class, which is `hkRootLevelContainer`.
    ///
    /// # Implementation details
    /// This method assumes that the dependency graph is already valid and does
    /// not perform cycle detection for performance reasons.
    ///
    /// - Data deserialized from binary HKX by `serde-hkx` is already sorted.
    /// - XML data from the official Havok SDK requires sorting before binary
    ///   serialization.
    ///
    /// For a validated version, see [`HavokSort::checked_sort_for_bytes`].
    ///
    /// # Errors
    /// Returns an error if the top pointer is missing.
    fn sort_for_bytes(&mut self) -> Result<(), Self::Error>;

    /// Sort classes by dependency order for XML serialization.
    ///
    /// # Returns
    /// The root pointer.
    ///
    /// # Errors
    /// Returns an error if the top pointer is missing.
    ///
    /// # Implementation details
    /// This method assumes that the dependency graph is already valid and does
    /// not perform cycle detection for performance reasons.
    ///
    /// For a validated version, see [`HavokSort::checked_sort_for_xml`].
    fn sort_for_xml(&mut self) -> Result<usize, Self::Error>;

    /// Sort classes for binary serialization after validating the dependency graph.
    ///
    /// This is equivalent to [`HavokSort::sort_for_bytes`], but performs cycle
    /// detection before sorting.
    ///
    /// Use this method when the input source is not guaranteed to have a valid
    /// HKX dependency graph.
    ///
    /// # Errors
    /// Returns [`Error::CycleDetected`] if a dependency cycle is detected.
    ///
    /// A cyclic dependency graph cannot be deterministically serialized.
    fn checked_sort_for_bytes(&mut self) -> Result<(), Self::Error>;

    /// Sort classes for XML serialization after validating the dependency graph.
    ///
    /// This is equivalent to [`HavokSort::sort_for_xml`], but performs cycle
    /// detection before sorting.
    ///
    /// Use this method when the input source is not guaranteed to have a valid
    /// HKX dependency graph.
    ///
    /// # Returns
    /// The root pointer.
    ///
    /// # Errors
    /// - [`Error::CycleDetected`] if a dependency cycle is detected.
    /// - Any error returned by [`HavokSort::sort_for_xml`].
    ///
    /// A cyclic dependency graph cannot be deterministically serialized.
    fn checked_sort_for_xml(&mut self) -> Result<usize, Self::Error>;
}

impl<V> HavokSort for IndexMap<usize, V>
where
    V: HavokClass,
{
    type Error = SerError;

    fn sort_for_bytes(&mut self) -> Result<(), Self::Error> {
        let (root_key, _) = find_root_ptr(self)?;
        sort_for_bytes_with_root(self, root_key);
        Ok(())
    }

    fn checked_sort_for_bytes(&mut self) -> Result<(), Self::Error> {
        if self.is_empty() {
            return Ok(());
        }

        let (root_key, _) = find_root_ptr(self)?;

        let mut states = HashMap::new();
        let mut path = Vec::new();
        check_cycle(self, root_key, &mut states, &mut path)?;

        sort_for_bytes_with_root(self, root_key);
        Ok(())
    }

    fn sort_for_xml(&mut self) -> Result<usize, Self::Error> {
        let (root_key, _) = find_root_ptr(self)?;
        sort_for_xml_with_root(self, root_key);

        Ok(root_key)
    }

    fn checked_sort_for_xml(&mut self) -> Result<usize, Self::Error> {
        let (root_key, _) = find_root_ptr(self)?;

        let mut states = HashMap::new();
        let mut path = Vec::new();
        check_cycle(self, root_key, &mut states, &mut path)?;

        sort_for_xml_with_root(self, root_key);

        Ok(root_key)
    }
}

fn sort_for_bytes_with_root<V>(classes: &mut IndexMap<usize, V>, root_key: usize)
where
    V: HavokClass,
{
    fn collect_deps<V>(classes: &mut IndexMap<usize, V>, key: usize, sorted_keys: &mut Vec<usize>)
    where
        V: HavokClass,
    {
        if sorted_keys.contains(&key) {
            return;
        }

        let current_index = match classes.get_index_of(&key) {
            Some(index) => index,
            None => return,
        };

        sorted_keys.push(key);
        classes.swap_indices(sorted_keys.len() - 1, current_index);

        let deps = match classes.get(&key) {
            Some(class) => class.deps_indexes(),
            None => return,
        };

        #[cfg(feature = "tracing")]
        tracing::trace!("index = {key}, deps_indexes = {deps:?}");

        for dep_key in deps {
            collect_deps(classes, dep_key, sorted_keys);
        }
    }

    let mut sorted_keys = Vec::with_capacity(classes.len());
    collect_deps(classes, root_key, &mut sorted_keys);

    #[cfg(feature = "tracing")]
    tracing::trace!("sorted_keys = {sorted_keys:?}");
}

fn sort_for_xml_with_root<V>(classes: &mut IndexMap<usize, V>, root_key: usize)
where
    V: HavokClass,
{
    fn collect_deps<V>(classes: &IndexMap<usize, V>, key: usize, sorted: &mut Vec<usize>)
    where
        V: HavokClass,
    {
        if sorted.contains(&key) {
            return;
        }

        let class = match classes.get(&key) {
            Some(class) => class,
            None => return,
        };

        let deps = class.deps_indexes();

        #[cfg(feature = "tracing")]
        tracing::trace!("index = {key}, deps_indexes = {deps:?}");

        for dep_key in deps {
            collect_deps(classes, dep_key, sorted);
        }

        sorted.push(key);
    }

    let mut sorted_keys = Vec::with_capacity(classes.len());
    collect_deps(classes, root_key, &mut sorted_keys);

    #[cfg(feature = "tracing")]
    tracing::trace!("sorted_keys = {sorted_keys:?}");

    let mut sorted_classes = IndexMap::with_capacity(classes.len());

    for key in sorted_keys {
        if let Some(class) = classes.swap_remove(&key) {
            sorted_classes.insert(key, class);
        }
    }

    *classes = sorted_classes;
}

/// find `hkRootLevelContainer`
pub(crate) fn find_root_ptr<V>(class_map: &IndexMap<usize, V>) -> Result<(usize, &V), SerError>
where
    V: HavokClass,
{
    let Some((&root_ptr, class)) = class_map
        .iter()
        .find(|class| class.1.name() == "hkRootLevelContainer")
    else {
        return Err(SerError::MissingRootClass);
    };

    Ok((root_ptr, class))
}

// It was created based on the code used to create `HavokTree`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Visiting,
    Visited,
}

fn check_cycle<V>(
    classes: &IndexMap<usize, V>,
    key: usize,
    states: &mut HashMap<usize, VisitState>,
    path: &mut Vec<usize>,
) -> Result<(), SerError>
where
    V: HavokClass,
{
    match states.get(&key) {
        Some(VisitState::Visited) => return Ok(()),
        Some(VisitState::Visiting) => {
            let start = path.iter().position(|&x| x == key).unwrap_or_default();

            let mut cycle = path[start..].to_vec();
            cycle.push(key);

            return Err(SerError::CycleDetected { cycle });
        }
        None => {}
    }

    states.insert(key, VisitState::Visiting);
    path.push(key);

    let class = if let Some(class) = classes.get(&key) {
        class
    } else {
        path.pop();
        states.insert(key, VisitState::Visited);
        return Ok(());
    };

    for dep in class.deps_indexes() {
        if dep == 0 {
            continue;
        }

        check_cycle(classes, dep, states, path)?;
    }

    path.pop();
    states.insert(key, VisitState::Visited);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mocks::new_defaultmale;
    use havok_classes::*;
    use havok_types::*;
    use indexmap::IndexMap;

    #[test]
    fn sort_for_bytes_defaultmale() {
        // ord: 8, 9, 10
        let mut classes = new_defaultmale();

        // ord: 9, 10
        let root = classes.swap_remove(&8).unwrap();
        classes.insert(8, root); // ord: 9, 10, 8
        classes.sort_for_bytes().unwrap();

        let keys: Vec<_> = classes.keys().copied().collect();
        #[rustfmt::skip]
        assert_eq!(keys, vec![8, 10, 9], "root must be serialized before dependencies");
    }

    #[test]
    fn checked_sort_for_bytes_defaultmale() {
        let mut classes = new_defaultmale();
        let result = classes.checked_sort_for_bytes();

        assert!(
            result.is_ok(),
            "defaultmale mock must not contain dependency cycles"
        );
    }

    #[test]
    fn sort_for_xml_defaultmale() {
        let mut classes = new_defaultmale();

        let root = classes
            .sort_for_xml()
            .expect("defaultmale must contain hkRootLevelContainer");
        assert_eq!(root, 8);

        let keys: Vec<_> = classes.keys().copied().collect();

        assert_eq!(
            keys,
            vec![9, 10, 8],
            "xml order must be dependency first and root last"
        );
    }

    #[test]
    fn checked_sort_for_xml_defaultmale() {
        let mut classes = new_defaultmale();
        let root = classes.checked_sort_for_xml();
        assert_eq!(root.unwrap(), 8);
    }

    #[test]
    fn sort_for_xml_missing_root() {
        let mut classes = IndexMap::<usize, Classes>::new();

        classes.insert(
            1,
            Classes::hkbProjectData(hkbProjectData {
                __ptr: Some(1.into()),
                ..Default::default()
            }),
        );

        let result = classes.sort_for_xml();

        assert!(matches!(result, Err(SerError::MissingRootClass)));
    }

    #[test]
    fn checked_sort_for_bytes_cycle_detected() {
        let mut classes = new_defaultmale();

        // Create an invalid Havok dependency graph:
        // - hkRootLevelContainer(8) -> hkRootLevelContainer.hkRootLevelContainerNamedVariant(8) -> Self
        //
        // A cycle in dependency pointers makes deterministic serialization order impossible.
        let root = classes.get_mut(&8).unwrap();
        if let Classes::hkRootLevelContainer(root) = root {
            root.m_namedVariants[0].m_variant = Pointer::new(8);
        }
        let result = classes.checked_sort_for_bytes();

        assert!(matches!(
            result,
            Err(SerError::CycleDetected { cycle })
                if cycle.contains(&8)
        ));
    }
}
