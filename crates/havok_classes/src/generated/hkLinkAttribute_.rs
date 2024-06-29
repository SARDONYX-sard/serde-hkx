use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkLinkAttribute`
/// -         version: `0`
/// -       signature: `0x255d8164`
/// -          size:   1(x86)/  1(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkLinkAttribute {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum Link`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: Link,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkLinkAttribute {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkLinkAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(626884964u32)
        }
    }
    impl __serde::Serialize for hkLinkAttribute {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkLinkAttribute", class_meta)?;
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
pub enum Link {
    #[default]
    NONE = 0isize,
    DIRECT_LINK = 1isize,
    CHILD = 2isize,
    MESH = 3isize,
    PARENT_NAME = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Link {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NONE => __serializer.serialize_field("NONE", &0u64),
                Self::DIRECT_LINK => __serializer.serialize_field("DIRECT_LINK", &1u64),
                Self::CHILD => __serializer.serialize_field("CHILD", &2u64),
                Self::MESH => __serializer.serialize_field("MESH", &3u64),
                Self::PARENT_NAME => __serializer.serialize_field("PARENT_NAME", &4u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i8().ok_or(S::Error::custom("Failed enum Link to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
