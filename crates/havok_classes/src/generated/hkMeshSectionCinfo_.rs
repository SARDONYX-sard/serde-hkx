use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMeshSectionCinfo`
/// -         version: `1`
/// -       signature: `0x6075f3ff`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMeshSectionCinfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `vertexBuffer`(ctype: `struct hkMeshVertexBuffer*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_vertexBuffer: Pointer,
    /// # C++ Info
    /// -          name: `material`(ctype: `struct hkMeshMaterial*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_material: Pointer,
    /// # C++ Info
    /// -          name: `primitiveType`(ctype: `enum PrimitiveType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_primitiveType: PrimitiveType,
    /// # C++ Info
    /// -          name: `numPrimitives`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numPrimitives: i32,
    /// # C++ Info
    /// -          name: `indexType`(ctype: `enum MeshSectionIndexType`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_indexType: MeshSectionIndexType,
    /// # C++ Info
    /// -          name: `indices`(ctype: `void*`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_indices: Pointer,
    /// # C++ Info
    /// -          name: `vertexStartIndex`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vertexStartIndex: i32,
    /// # C++ Info
    /// -          name: `transformIndex`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_transformIndex: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkMeshSectionCinfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMeshSectionCinfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1618342911u32)
        }
    }
    impl _serde::Serialize for hkMeshSectionCinfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1618342911u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMeshSectionCinfo", class_meta)?;
            serializer.serialize_field("vertexBuffer", &self.m_vertexBuffer)?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.serialize_field("primitiveType", &self.m_primitiveType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("numPrimitives", &self.m_numPrimitives)?;
            serializer.serialize_field("indexType", &self.m_indexType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_field("indices", &self.m_indices)?;
            serializer.serialize_field("vertexStartIndex", &self.m_vertexStartIndex)?;
            serializer.serialize_field("transformIndex", &self.m_transformIndex)?;
            serializer.end()
        }
    }
};
