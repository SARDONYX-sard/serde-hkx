mod all_types_test_class;
mod hk_referenced_object;
mod hkp_shape_info;

pub use all_types_test_class::AllTypesTestClass;
pub use hk_referenced_object::*;
pub use hkp_shape_info::*;

use crate::ser::Serialize;

pub enum Classes<'a> {
    HkpShapeInfo(HkpShapeInfo<'a>),
    HkReferencedObject(HkReferencedObject),
    AllTypesTestClass(AllTypesTestClass),
}

impl crate::HavokClass for Classes<'_> {}
impl<'a> Serialize for Classes<'a> {
    fn serialize<S: crate::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Classes::HkpShapeInfo(class) => class.serialize(serializer),
            Classes::HkReferencedObject(class) => class.serialize(serializer),
            Classes::AllTypesTestClass(class) => class.serialize(serializer),
        }
    }
}
