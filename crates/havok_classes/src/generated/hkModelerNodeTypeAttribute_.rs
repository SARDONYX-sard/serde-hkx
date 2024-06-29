use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkModelerNodeTypeAttribute`
/// -         version: `0`
/// -       signature: `0x338c092f`
/// -          size:   1(x86)/  1(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkModelerNodeTypeAttribute {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum ModelerType`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: ModelerType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkModelerNodeTypeAttribute {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkModelerNodeTypeAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(864815407u32)
        }
    }
    impl __serde::Serialize for hkModelerNodeTypeAttribute {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkModelerNodeTypeAttribute", class_meta)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum ModelerType {
    #[default]
    DEFAULT = 0isize,
    LOCATOR = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ModelerType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::DEFAULT => __serializer.serialize_field("DEFAULT", &0u64),
                Self::LOCATOR => __serializer.serialize_field("LOCATOR", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum ModelerType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
