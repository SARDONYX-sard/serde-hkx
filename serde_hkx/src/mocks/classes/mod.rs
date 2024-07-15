#[rustfmt::skip]
mod all_types_test_class;
#[rustfmt::skip]
mod hkBaseObject_;
#[rustfmt::skip]
mod hkReferencedObject_;
#[rustfmt::skip]
mod hkRootLevelContainerNamedVariant_;
#[rustfmt::skip]
mod hkRootLevelContainer_;
#[rustfmt::skip]
mod hkbProjectData_;
#[rustfmt::skip]
mod hkbProjectStringData_;

pub(super) use super::class_requires;
use super::class_requires::*;
pub use all_types_test_class::*;
pub use hkBaseObject_::*;
pub use hkReferencedObject_::*;
pub use hkRootLevelContainerNamedVariant_::*;
pub use hkRootLevelContainer_::*;
pub use hkbProjectData_::*;
pub use hkbProjectStringData_::*;

pub use super::enums::EventMode;

#[derive(Debug, Clone, PartialEq)]
pub enum Classes<'a> {
    /// For binary writing, the youngest pointer index must be first after sorting in reverse order.
    ///
    /// To speed up the process, swap the first and last indexes instead of using shift.
    /// This dummy class exists to reserve space for this purpose.
    PhantomData,

    AllTypesTestClass(AllTypesTestClass),
    hkBaseObject(hkBaseObject),
    hkReferencedObject(hkReferencedObject),
    hkRootLevelContainer(hkRootLevelContainer<'a>),
    HkRootLevelContainerNamedVariant(hkRootLevelContainerNamedVariant<'a>),
    hkbProjectData(hkbProjectData),
    hkbProjectStringData(hkbProjectStringData<'a>),
}

impl HavokClass for Classes<'_> {
    fn name(&self) -> &'static str {
        match &self {
            Classes::PhantomData => panic!("The dummy class is used only for sorting, so being called name is not a good use of the API."),

            Classes::AllTypesTestClass(class) => class.name(),
            Classes::hkBaseObject(class) => class.name(),
            Classes::hkbProjectData(class) => class.name(),
            Classes::hkbProjectStringData(class) => class.name(),
            Classes::hkReferencedObject(class) => class.name(),
            Classes::hkRootLevelContainer(class) => class.name(),
            Classes::HkRootLevelContainerNamedVariant(class) => class.name(),
        }
    }

    fn signature(&self) -> Signature {
        match &self {
            Classes::PhantomData => panic!("The dummy class is used only for sorting, so being called signature is not a good use of the API."),

            Classes::AllTypesTestClass(class) => class.signature(),
            Classes::hkBaseObject(class) => class.signature(),
            Classes::hkbProjectData(class) => class.signature(),
            Classes::hkbProjectStringData(class) => class.signature(),
            Classes::hkReferencedObject(class) => class.signature(),
            Classes::hkRootLevelContainer(class) => class.signature(),
            Classes::HkRootLevelContainerNamedVariant(class) => class.signature(),
        }
    }
}

impl<'a> Serialize for Classes<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Classes::PhantomData => panic!("The dummy class is used only for sorting, so being called serialize is not a good use of the API."),

            Classes::AllTypesTestClass(class) => class.serialize(serializer),
            Classes::hkBaseObject(class) => class.serialize(serializer),
            Classes::hkbProjectData(class) => class.serialize(serializer),
            Classes::hkbProjectStringData(class) => class.serialize(serializer),
            Classes::hkReferencedObject(class) => class.serialize(serializer),
            Classes::hkRootLevelContainer(class) => class.serialize(serializer),
            Classes::HkRootLevelContainerNamedVariant(class) => class.serialize(serializer),
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
                    "hkBaseObject" => Ok(Classes::hkBaseObject(map.next_value()?)),
                    "hkbProjectData" => Ok(Classes::hkbProjectData(map.next_value()?)),
                    "hkbProjectStringData" => Ok(Classes::hkbProjectStringData(map.next_value()?)),
                    "hkReferencedObject" => Ok(Classes::hkReferencedObject(map.next_value()?)),
                    "hkRootLevelContainer" => Ok(Classes::hkRootLevelContainer(map.next_value()?)),
                    "hkRootLevelContainerNamedVariant" => {
                        Ok(Classes::HkRootLevelContainerNamedVariant(map.next_value()?))
                    }
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
