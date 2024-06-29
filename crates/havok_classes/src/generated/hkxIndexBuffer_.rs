use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxIndexBuffer`
/// -         version: `1`
/// -       signature: `0xc12c8197`
/// -          size:  44(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxIndexBuffer {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `indexType`(ctype: `enum IndexType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_indexType: IndexType,
    /// # C++ Info
    /// -          name: `indices16`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_indices16: Vec<u16>,
    /// # C++ Info
    /// -          name: `indices32`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_indices32: Vec<u32>,
    /// # C++ Info
    /// -          name: `vertexBaseOffset`(ctype: `hkUint32`)
    /// -        offset:  36(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vertexBaseOffset: u32,
    /// # C++ Info
    /// -          name: `length`(ctype: `hkUint32`)
    /// -        offset:  40(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_length: u32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxIndexBuffer {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxIndexBuffer"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3240919447u32)
        }
    }
    impl __serde::Serialize for hkxIndexBuffer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxIndexBuffer", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("indexType", &self.m_indexType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_meta_field("indices16", &self.m_indices16)?;
            serializer.serialize_array_meta_field("indices32", &self.m_indices32)?;
            serializer.serialize_field("vertexBaseOffset", &self.m_vertexBaseOffset)?;
            serializer.serialize_field("length", &self.m_length)?;
            serializer.serialize_array_field("indices16", &self.m_indices16)?;
            serializer.serialize_array_field("indices32", &self.m_indices32)?;
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
pub enum IndexType {
    #[default]
    INDEX_TYPE_INVALID = 0isize,
    INDEX_TYPE_TRI_LIST = 1isize,
    INDEX_TYPE_TRI_STRIP = 2isize,
    INDEX_TYPE_TRI_FAN = 3isize,
    INDEX_TYPE_MAX_ID = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for IndexType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::INDEX_TYPE_INVALID => {
                    __serializer.serialize_field("INDEX_TYPE_INVALID", &0u64)
                }
                Self::INDEX_TYPE_TRI_LIST => {
                    __serializer.serialize_field("INDEX_TYPE_TRI_LIST", &1u64)
                }
                Self::INDEX_TYPE_TRI_STRIP => {
                    __serializer.serialize_field("INDEX_TYPE_TRI_STRIP", &2u64)
                }
                Self::INDEX_TYPE_TRI_FAN => {
                    __serializer.serialize_field("INDEX_TYPE_TRI_FAN", &3u64)
                }
                Self::INDEX_TYPE_MAX_ID => {
                    __serializer.serialize_field("INDEX_TYPE_MAX_ID", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum IndexType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
