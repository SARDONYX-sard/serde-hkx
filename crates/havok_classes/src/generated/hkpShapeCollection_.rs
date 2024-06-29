use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpShapeCollection`
/// -         version: `0`
/// -       signature: `0xe8c3991d`
/// -          size:  24(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpShapeCollection {
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
    /// -          name: `disableWelding`(ctype: `hkBool`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_disableWelding: bool,
    /// # C++ Info
    /// -          name: `collectionType`(ctype: `enum CollectionType`)
    /// -        offset:  21(x86)/ 41(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_collectionType: CollectionType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpShapeCollection {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpShapeCollection"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3905132829u32)
        }
    }
    impl __serde::Serialize for hkpShapeCollection {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpShapeCollection", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("disableWelding", &self.m_disableWelding)?;
            serializer.serialize_field("collectionType", &self.m_collectionType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
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
pub enum CollectionType {
    #[default]
    COLLECTION_LIST = 0isize,
    COLLECTION_EXTENDED_MESH = 1isize,
    COLLECTION_TRISAMPLED_HEIGHTFIELD = 2isize,
    COLLECTION_USER = 3isize,
    COLLECTION_SIMPLE_MESH = 4isize,
    COLLECTION_MESH_SHAPE = 5isize,
    COLLECTION_COMPRESSED_MESH = 6isize,
    COLLECTION_MAX = 7isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for CollectionType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::COLLECTION_LIST => {
                    __serializer.serialize_field("COLLECTION_LIST", &0u64)
                }
                Self::COLLECTION_EXTENDED_MESH => {
                    __serializer.serialize_field("COLLECTION_EXTENDED_MESH", &1u64)
                }
                Self::COLLECTION_TRISAMPLED_HEIGHTFIELD => {
                    __serializer
                        .serialize_field("COLLECTION_TRISAMPLED_HEIGHTFIELD", &2u64)
                }
                Self::COLLECTION_USER => {
                    __serializer.serialize_field("COLLECTION_USER", &3u64)
                }
                Self::COLLECTION_SIMPLE_MESH => {
                    __serializer.serialize_field("COLLECTION_SIMPLE_MESH", &4u64)
                }
                Self::COLLECTION_MESH_SHAPE => {
                    __serializer.serialize_field("COLLECTION_MESH_SHAPE", &5u64)
                }
                Self::COLLECTION_COMPRESSED_MESH => {
                    __serializer.serialize_field("COLLECTION_COMPRESSED_MESH", &6u64)
                }
                Self::COLLECTION_MAX => {
                    __serializer.serialize_field("COLLECTION_MAX", &7u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum CollectionType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
