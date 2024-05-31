#![allow(unused)]

mod all_types_test_class;
mod hk_base_object;
mod hk_referenced_object;
mod hk_root_container;
mod hk_root_level_container_named_variant;
mod hkb_project_data;
mod hkb_project_string_data;
mod hkp_shape_info;

use crate::mocks::mock_requires::*;
pub use all_types_test_class::*;
pub use hk_base_object::*;
pub use hk_referenced_object::*;
pub use hk_root_container::*;
pub use hk_root_level_container_named_variant::*;
pub use hkb_project_data::*;
pub use hkb_project_string_data::*;
pub use hkp_shape_info::*;

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

impl HavokClass for Classes<'_> {}
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
