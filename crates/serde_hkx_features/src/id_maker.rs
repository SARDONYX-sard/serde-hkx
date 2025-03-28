//! Create event_id & variable map from `hkbBehaviorGraphStringData`.

use havok_classes::Classes;
use serde_hkx::{EventIdMap, VariableIdMap};

/// Create `eventID` & `variableId` maps from `hkbBehaviorGraphStringData` class
///
/// # Example
/// ```no_run
/// use serde_hkx::bytes::serde::hkx_header::HkxHeader;
/// use serde_hkx::errors::SerdeHkxError;
/// use serde_hkx::{ClassMapKey, EventIdMap, VariableIdMap};
/// use serde_hkx_features::ClassMap;
///
/// fn main() -> Result<(), SerdeHkxError> {
///     let class_map = ClassMap::new();
///     let ptr: ClassMapKey = "#0001".into(); // hkbBehaviorGraphStringData pointer
///     let (event_id_map, variable_id_map) = class_map
///         .get(&ptr)
///         .and_then(|class| crate_maps_from_id_class(class))
///         .unwrap_or_else(|| (EventIdMap::new(), VariableIdMap::new()));
///
///     let hkx = serde_hkx::to_bytes_with_maps(
///         &class_map,
///         &HkxHeader::new_skyrim_se(),
///         event_id_map,
///         variable_id_map,
///     )?;
///     std::fs::write("generated.hkx", &hkx)?;
///     Ok(())
/// }
/// ```
pub fn crate_maps_from_id_class(id_class: &Classes<'_>) -> Option<(EventIdMap, VariableIdMap)> {
    if let Classes::hkbBehaviorGraphStringData(graph_class) = id_class {
        let mut event_map = EventIdMap::new();
        let mut variable_map = VariableIdMap::new();
        for (id, event_id) in graph_class.m_eventNames.iter().enumerate() {
            event_map.0.insert(event_id.to_string(), id);
        }

        for (id, variable_id) in graph_class.m_variableNames.iter().enumerate() {
            variable_map.0.insert(variable_id.to_string(), id);
        }
        return Some((event_map, variable_map));
    }

    None
}
