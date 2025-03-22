//! Reduce the burden of individual imports by importing a set of types needed to create a havok class structure here.
use havok_classes::*;
pub use havok_types::*;

use super::ClassMap;

pub fn new_defaultmale<'a>() -> ClassMap<'a> {
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
        m_stringData: 9.into(),
        m_defaultEventMode: EventMode::EVENT_MODE_IGNORE_FROM_GENERATOR,
        ..Default::default()
    };

    let hk_root_level_container = hkRootLevelContainer {
        __ptr: Some(8.into()),
        m_namedVariants: vec![hkRootLevelContainerNamedVariant {
            __ptr: None,
            m_name: "hkbProjectData".into(),
            m_className: "hkbProjectData".into(),
            m_variant: 10.into(),
        }],
    };

    let mut classes = ClassMap::new();
    classes.insert(
        "#0008".into(),
        Classes::hkRootLevelContainer(Box::new(hk_root_level_container)),
    );
    classes.insert(
        "#0009".into(),
        Classes::hkbProjectStringData(Box::new(hkb_project_string_data)),
    );
    classes.insert(
        "#0010".into(),
        Classes::hkbProjectData(Box::new(hkb_project_data)),
    );
    classes
}
