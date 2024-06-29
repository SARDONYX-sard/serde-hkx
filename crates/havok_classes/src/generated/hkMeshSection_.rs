use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMeshSection`
/// -         version: `1`
/// -       signature: `0x1893c365`
/// -          size:  40(x86)/ 56(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMeshSection {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `primitiveType`(ctype: `enum PrimitiveType`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_primitiveType: PrimitiveType,
    /// # C++ Info
    /// -          name: `numPrimitives`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numPrimitives: i32,
    /// # C++ Info
    /// -          name: `numIndices`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numIndices: i32,
    /// # C++ Info
    /// -          name: `vertexStartIndex`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vertexStartIndex: i32,
    /// # C++ Info
    /// -          name: `transformIndex`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_transformIndex: i32,
    /// # C++ Info
    /// -          name: `indexType`(ctype: `enum MeshSectionIndexType`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_indexType: MeshSectionIndexType,
    /// # C++ Info
    /// -          name: `indices`(ctype: `void*`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_indices: Pointer,
    /// # C++ Info
    /// -          name: `vertexBuffer`(ctype: `struct hkMeshVertexBuffer*`)
    /// -        offset:  28(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_vertexBuffer: Pointer,
    /// # C++ Info
    /// -          name: `material`(ctype: `struct hkMeshMaterial*`)
    /// -        offset:  32(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_material: Pointer,
    /// # C++ Info
    /// -          name: `sectionIndex`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sectionIndex: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkMeshSection {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMeshSection"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(412336997u32)
        }
    }
    impl __serde::Serialize for hkMeshSection {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMeshSection", class_meta)?;
            serializer.serialize_field("primitiveType", &self.m_primitiveType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("numPrimitives", &self.m_numPrimitives)?;
            serializer.serialize_field("numIndices", &self.m_numIndices)?;
            serializer.serialize_field("vertexStartIndex", &self.m_vertexStartIndex)?;
            serializer.serialize_field("transformIndex", &self.m_transformIndex)?;
            serializer.serialize_field("indexType", &self.m_indexType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("indices", &self.m_indices)?;
            serializer.serialize_field("vertexBuffer", &self.m_vertexBuffer)?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.serialize_field("sectionIndex", &self.m_sectionIndex)?;
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
pub enum MeshSectionIndexType {
    #[default]
    INDEX_TYPE_NONE = 0isize,
    INDEX_TYPE_UINT16 = 1isize,
    INDEX_TYPE_UINT32 = 2isize,
}
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
pub enum PrimitiveType {
    #[default]
    PRIMITIVE_TYPE_UNKNOWN = 0isize,
    PRIMITIVE_TYPE_POINT_LIST = 1isize,
    PRIMITIVE_TYPE_LINE_LIST = 2isize,
    PRIMITIVE_TYPE_TRIANGLE_LIST = 3isize,
    PRIMITIVE_TYPE_TRIANGLE_STRIP = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MeshSectionIndexType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::INDEX_TYPE_NONE => {
                    __serializer.serialize_field("INDEX_TYPE_NONE", &0u64)
                }
                Self::INDEX_TYPE_UINT16 => {
                    __serializer.serialize_field("INDEX_TYPE_UINT16", &1u64)
                }
                Self::INDEX_TYPE_UINT32 => {
                    __serializer.serialize_field("INDEX_TYPE_UINT32", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum MeshSectionIndexType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for PrimitiveType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::PRIMITIVE_TYPE_UNKNOWN => {
                    __serializer.serialize_field("PRIMITIVE_TYPE_UNKNOWN", &0u64)
                }
                Self::PRIMITIVE_TYPE_POINT_LIST => {
                    __serializer.serialize_field("PRIMITIVE_TYPE_POINT_LIST", &1u64)
                }
                Self::PRIMITIVE_TYPE_LINE_LIST => {
                    __serializer.serialize_field("PRIMITIVE_TYPE_LINE_LIST", &2u64)
                }
                Self::PRIMITIVE_TYPE_TRIANGLE_LIST => {
                    __serializer.serialize_field("PRIMITIVE_TYPE_TRIANGLE_LIST", &3u64)
                }
                Self::PRIMITIVE_TYPE_TRIANGLE_STRIP => {
                    __serializer.serialize_field("PRIMITIVE_TYPE_TRIANGLE_STRIP", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum PrimitiveType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
