use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBvTreeShape`
/// -         version: `0`
/// -       signature: `0xa823d623`
/// -          size:  20(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBvTreeShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShape,
    /// # C++ Info
    /// -          name: `bvTreeType`(ctype: `enum BvTreeType`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bvTreeType: BvTreeType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpBvTreeShape {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpBvTreeShape"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2820920867u32)
        }
    }
    impl __serde::Serialize for hkpBvTreeShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpBvTreeShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("bvTreeType", &self.m_bvTreeType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
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
pub enum BvTreeType {
    #[default]
    BVTREE_MOPP = 0isize,
    BVTREE_TRISAMPLED_HEIGHTFIELD = 1isize,
    BVTREE_USER = 2isize,
    BVTREE_MAX = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BvTreeType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BVTREE_MOPP => __serializer.serialize_field("BVTREE_MOPP", &0u64),
                Self::BVTREE_TRISAMPLED_HEIGHTFIELD => {
                    __serializer.serialize_field("BVTREE_TRISAMPLED_HEIGHTFIELD", &1u64)
                }
                Self::BVTREE_USER => __serializer.serialize_field("BVTREE_USER", &2u64),
                Self::BVTREE_MAX => __serializer.serialize_field("BVTREE_MAX", &3u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum BvTreeType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
