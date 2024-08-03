/// Reduce the burden of individual imports by importing a set of types needed to create a havok class structure here.
pub use crate::lib::*;
use havok_classes::*;
pub use havok_serde::{
    de::{self, Deserialize, Deserializer, MapAccess, Visitor},
    ser::{Error as _, Serialize, SerializeFlags, SerializeStruct, Serializer},
    HavokClass,
};
pub use havok_types::*;
use indexmap::IndexMap;

pub fn new_defaultmale<'a>() -> IndexMap<usize, Classes<'a>> {
    let hkb_project_string_data = hkbProjectStringData {
        __ptr: Some(9.into()),
        m_animationFilenames: vec![],
        m_behaviorFilenames: vec![],
        m_characterFilenames: vec!["Characters\\DefaultMale.hkx".into()],
        m_eventNames: vec![],
        m_animationPath: "".into(),
        m_behaviorPath: "".into(),
        m_characterPath: "".into(),
        m_fullPathToSource: "".into(),
        m_rootPath: None.into(),
        ..Default::default()
    };

    let hkb_project_data = hkbProjectData {
        __ptr: Some(10.into()),
        m_worldUpWS: Vector4::new(0.0, 0.0, 1.0, 0.0),
        m_stringData: Pointer::new(9),
        m_defaultEventMode: EventMode::EVENT_MODE_IGNORE_FROM_GENERATOR,
        ..Default::default()
    };

    let hk_root_level_container = hkRootLevelContainer {
        __ptr: Some(8.into()),
        m_namedVariants: vec![hkRootLevelContainerNamedVariant {
            __ptr: None,
            m_name: "hkbProjectData".into(),
            m_className: "hkbProjectData".into(),
            m_variant: Pointer::new(10),
        }],
    };

    let mut classes = indexmap::IndexMap::new();
    classes.insert(8, Classes::hkRootLevelContainer(hk_root_level_container));
    classes.insert(9, Classes::hkbProjectStringData(hkb_project_string_data));
    classes.insert(10, Classes::hkbProjectData(hkb_project_data));
    classes
}

pub fn new_defaultmale_vec<'a>() -> Vec<Classes<'a>> {
    let hkb_project_string_data = hkbProjectStringData {
        __ptr: Some(9.into()),
        m_animationFilenames: vec![],
        m_behaviorFilenames: vec![],
        m_characterFilenames: vec!["Characters\\DefaultMale.hkx".into()],
        m_eventNames: vec![],
        m_animationPath: "".into(),
        m_behaviorPath: "".into(),
        m_characterPath: "".into(),
        m_fullPathToSource: "".into(),
        m_rootPath: None.into(),
        ..Default::default()
    };

    let hkb_project_data = hkbProjectData {
        __ptr: Some(10.into()),
        m_worldUpWS: Vector4::new(0.0, 0.0, 1.0, 0.0),
        m_stringData: Pointer::new(9),
        m_defaultEventMode: EventMode::EVENT_MODE_IGNORE_FROM_GENERATOR,
        ..Default::default()
    };

    let hk_root_level_container = hkRootLevelContainer {
        __ptr: Some(8.into()),
        m_namedVariants: vec![hkRootLevelContainerNamedVariant {
            __ptr: None,
            m_name: "hkbProjectData".into(),
            m_className: "hkbProjectData".into(),
            m_variant: Pointer::new(10),
        }],
    };

    vec![
        Classes::hkRootLevelContainer(hk_root_level_container),
        Classes::hkbProjectStringData(hkb_project_string_data),
        Classes::hkbProjectData(hkb_project_data),
    ]
}
