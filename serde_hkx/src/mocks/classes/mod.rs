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
    /// For binary writing, the youngest pointer index must be first after sorting in reverse order.
    ///
    /// To speed up the process, swap the first and last indexes instead of using shift.
    /// This dummy class exists to reserve space for this purpose.
    PhantomData,

    AllTypesTestClass(AllTypesTestClass),
    HkBaseObject(HkBaseObject),
    HkReferencedObject(HkReferencedObject),
    HkRootLevelContainer(HkRootLevelContainer<'a>),
    HkRootLevelContainerNamedVariant(HkRootLevelContainerNamedVariant<'a>),
    HkbProjectData(HkbProjectData),
    HkbProjectStringData(HkbProjectStringData<'a>),
    HkpShapeInfo(HkpShapeInfo<'a>),
}

// impl core::str::FromStr for Classes<'a> {
//     type Err ="";

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match
//     }
// }

impl HavokClass for Classes<'_> {
    fn name(&self) -> &'static str {
        match &self {
            Classes::PhantomData => panic!("The dummy class is used only for sorting, so being called name is not a good use of the API."),

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
            Classes::PhantomData => panic!("The dummy class is used only for sorting, so being called signature is not a good use of the API."),

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
            Classes::PhantomData => panic!("The dummy class is used only for sorting, so being called serialize is not a good use of the API."),

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

impl<'a, 'de: 'a> Deserialize<'de> for Classes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClassesVisitor<'a> {
            marker: std::marker::PhantomData<Classes<'a>>,
        }

        impl<'a, 'de: 'a> Visitor<'de> for ClassesVisitor<'a> {
            type Value = Classes<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid class enum")
            }

            fn visit_class_index<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::ClassIndexAccess<'de>,
            {
                let class_name = map.next_key()?;
                match class_name {
                    // "AllTypesTestClass" => Ok(Classes::AllTypesTestClass(map.next_value()?)),
                    // "HkBaseObject" => Ok(Classes::HkBaseObject(map.next_value()?)),
                    // "HkbProjectData" => Ok(Classes::HkbProjectData(map.next_value()?)),
                    // "HkbProjectStringData" => Ok(Classes::HkbProjectStringData(map.next_value()?)),
                    "hkReferencedObject" => Ok(Classes::HkReferencedObject(map.next_value()?)),
                    // "HkRootLevelContainer" => Ok(Classes::HkRootLevelContainer(map.next_value()?)),
                    // "HkRootLevelContainerNamedVariant" => {
                    //     Ok(Classes::HkRootLevelContainerNamedVariant(map.next_value()?))
                    // }
                    // "HkpShapeInfo" => Ok(Classes::HkpShapeInfo(map.next_value()?)),
                    _ => Err(de::Error::unknown_field(
                        class_name,
                        &[
                            "AllTypesTestClass",
                            "hkBaseObject",
                            "hkbProjectData",
                            "hkbProjectStringData",
                            "hkReferencedObject",
                            "hkRootLevelContainer",
                            "hkRootLevelContainerNamedVariant",
                            "hkpShapeInfo",
                        ],
                    )),
                }
            }
        }

        deserializer.deserialize_class_index(ClassesVisitor {
            marker: std::marker::PhantomData,
        })
    }
}
