use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkSemanticsAttribute`
/// -         version: `0`
/// -       signature: `0x837099c3`
/// -          size:   1(x86)/  1(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkSemanticsAttribute {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum Semantics`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: Semantics,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkSemanticsAttribute {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkSemanticsAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2205194691u32)
        }
    }
    impl __serde::Serialize for hkSemanticsAttribute {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkSemanticsAttribute", class_meta)?;
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
pub enum Semantics {
    #[default]
    UNKNOWN = 0isize,
    DISTANCE = 1isize,
    ANGLE = 2isize,
    NORMAL = 3isize,
    POSITION = 4isize,
    COSINE_ANGLE = 5isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Semantics {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::UNKNOWN => __serializer.serialize_field("UNKNOWN", &0u64),
                Self::DISTANCE => __serializer.serialize_field("DISTANCE", &1u64),
                Self::ANGLE => __serializer.serialize_field("ANGLE", &2u64),
                Self::NORMAL => __serializer.serialize_field("NORMAL", &3u64),
                Self::POSITION => __serializer.serialize_field("POSITION", &4u64),
                Self::COSINE_ANGLE => __serializer.serialize_field("COSINE_ANGLE", &5u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum Semantics to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
