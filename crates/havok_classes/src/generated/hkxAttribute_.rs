use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxAttribute`
/// -         version: `0`
/// -       signature: `0x7375cae3`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxAttribute<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `value`(ctype: `struct hkReferencedObject*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_value: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkxAttribute<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1937099491u32)
        }
    }
    impl<'a> __serde::Serialize for hkxAttribute<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxAttribute", class_meta)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("value", &self.m_value)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT8`
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
pub enum Hint {
    #[default]
    HINT_NONE = 0isize,
    HINT_IGNORE = 1isize,
    HINT_TRANSFORM = 2isize,
    HINT_SCALE = 4isize,
    HINT_TRANSFORM_AND_SCALE = 6isize,
    HINT_FLIP = 8isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Hint {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HINT_NONE => __serializer.serialize_field("HINT_NONE", &0u64),
                Self::HINT_IGNORE => __serializer.serialize_field("HINT_IGNORE", &1u64),
                Self::HINT_TRANSFORM => {
                    __serializer.serialize_field("HINT_TRANSFORM", &2u64)
                }
                Self::HINT_SCALE => __serializer.serialize_field("HINT_SCALE", &4u64),
                Self::HINT_TRANSFORM_AND_SCALE => {
                    __serializer.serialize_field("HINT_TRANSFORM_AND_SCALE", &6u64)
                }
                Self::HINT_FLIP => __serializer.serialize_field("HINT_FLIP", &8u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_u8().ok_or(S::Error::custom("Failed enum Hint to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
