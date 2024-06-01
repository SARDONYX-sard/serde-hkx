#![allow(unused)]

mod all_types_test_class;
mod hk_base_object;
mod hk_referenced_object;
mod hk_root_container;
mod hk_root_level_container_named_variant;
mod hkb_project_data;
mod hkb_project_string_data;
mod hkp_shape_info;

use super::mock_requires::*;
pub use all_types_test_class::*;
pub use hk_base_object::*;
pub use hk_referenced_object::*;
pub use hk_root_container::*;
pub use hk_root_level_container_named_variant::*;
pub use hkb_project_data::*;
pub use hkb_project_string_data::*;
pub use hkp_shape_info::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Classes<'a> {
    AllTypesTestClass(AllTypesTestClass),
    HkBaseObject(HkBaseObject),
    HkReferencedObject(HkReferencedObject),
    HkRootLevelContainer(HkRootLevelContainer<'a>),
    HkRootLevelContainerNamedVariant(HkRootLevelContainerNamedVariant<'a>),
    HkbProjectData(HkbProjectData),
    HkbProjectStringData(HkbProjectStringData<'a>),

    HkpShapeInfo(HkpShapeInfo<'a>),
}

impl HavokClass for Classes<'_> {
    fn name(&self) -> &'static CStr {
        match &self {
            Classes::AllTypesTestClass(class) => class.name(),
            Classes::HkBaseObject(class) => class.name(),
            Classes::HkbProjectData(class) => class.name(),
            Classes::HkbProjectStringData(class) => class.name(),
            Classes::HkReferencedObject(class) => class.name(),
            Classes::HkRootLevelContainer(class) => class.name(),
            Classes::HkRootLevelContainerNamedVariant(class) => class.name(),
            Classes::HkpShapeInfo(class) => class.name(),
        }
    }

    fn signature(&self) -> Signature {
        match &self {
            Classes::AllTypesTestClass(class) => class.signature(),
            Classes::HkBaseObject(class) => class.signature(),
            Classes::HkbProjectData(class) => class.signature(),
            Classes::HkbProjectStringData(class) => class.signature(),
            Classes::HkReferencedObject(class) => class.signature(),
            Classes::HkRootLevelContainer(class) => class.signature(),
            Classes::HkRootLevelContainerNamedVariant(class) => class.signature(),
            Classes::HkpShapeInfo(class) => class.signature(),
        }
    }
}

impl<'a> Serialize for Classes<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Classes::AllTypesTestClass(class) => class.serialize(serializer),
            Classes::HkBaseObject(class) => class.serialize(serializer),
            Classes::HkbProjectData(class) => class.serialize(serializer),
            Classes::HkbProjectStringData(class) => class.serialize(serializer),
            Classes::HkReferencedObject(class) => class.serialize(serializer),
            Classes::HkRootLevelContainer(class) => class.serialize(serializer),
            Classes::HkRootLevelContainerNamedVariant(class) => class.serialize(serializer),
            Classes::HkpShapeInfo(class) => class.serialize(serializer),
        }
    }
}
