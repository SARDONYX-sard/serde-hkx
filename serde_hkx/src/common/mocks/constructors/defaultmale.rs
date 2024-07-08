use indexmap::IndexMap;

use crate::common::mocks::classes::*;
use crate::common::mocks::enums::EventMode;
use crate::common::mocks::mock_requires::*;

pub fn new_defaultmale<'a>() -> IndexMap<usize, Classes<'a>> {
    let hkb_project_string_data = HkbProjectStringData {
        __ptr: Some(9.into()),
        animation_filenames: vec![],
        behavior_filenames: vec![],
        character_filenames: vec!["Characters\\DefaultMale.hkx".into()],
        event_names: vec![],
        animation_path: "".into(),
        behavior_path: "".into(),
        character_path: "".into(),
        full_path_to_source: "".into(),
        root_path: None.into(),
        ..Default::default()
    };

    let hkb_project_data = HkbProjectData {
        _ptr: Some(10.into()),
        world_up_ws: Vector4::new(0.0, 0.0, 1.0, 0.0),
        string_data: Pointer::new(9),
        default_event_mode: EventMode::EventModeIgnoreFromGenerator,
        ..Default::default()
    };

    let hk_root_level_container = HkRootLevelContainer {
        _ptr: Some(8.into()),
        named_variants: vec![HkRootLevelContainerNamedVariant {
            __ptr: None,
            name: "hkbProjectData".into(),
            class_name: "hkbProjectData".into(),
            variant: Pointer::new(10),
        }],
    };

    let mut classes = indexmap::IndexMap::new();
    classes.insert(8, Classes::HkRootLevelContainer(hk_root_level_container));
    classes.insert(9, Classes::HkbProjectStringData(hkb_project_string_data));
    classes.insert(10, Classes::HkbProjectData(hkb_project_data));
    classes
}
