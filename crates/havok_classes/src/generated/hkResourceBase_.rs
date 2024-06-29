use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkResourceBase`
/// -         version: `0`
/// -       signature: `0x660d7cac`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkResourceBase {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkResourceBase {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkResourceBase"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1712159916u32)
        }
    }
    impl __serde::Serialize for hkResourceBase {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkResourceBase", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
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
pub enum Type {
    #[default]
    TYPE_RESOURCE = 0isize,
    TYPE_CONTAINER = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Type {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TYPE_RESOURCE => {
                    __serializer.serialize_field("TYPE_RESOURCE", &0u64)
                }
                Self::TYPE_CONTAINER => {
                    __serializer.serialize_field("TYPE_CONTAINER", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_u8().ok_or(S::Error::custom("Failed enum Type to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
