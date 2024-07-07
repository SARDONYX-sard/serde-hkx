use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMeshShapeSubpart`
/// -         version: `0`
/// -       signature: `0x27336e5d`
/// -          size:  56(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMeshShapeSubpart {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `vertexBase`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_vertexBase: Pointer,
    /// # C++ Info
    /// -          name: `vertexStriding`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vertexStriding: i32,
    /// # C++ Info
    /// -          name: `numVertices`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numVertices: i32,
    /// # C++ Info
    /// -          name: `indexBase`(ctype: `void*`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_indexBase: Pointer,
    /// # C++ Info
    /// -          name: `stridingType`(ctype: `enum MeshShapeIndexStridingType`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_stridingType: MeshShapeIndexStridingType,
    /// # C++ Info
    /// -          name: `materialIndexStridingType`(ctype: `enum MeshShapeMaterialIndexStridingType`)
    /// -        offset:  17(x86)/ 25(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_materialIndexStridingType: MeshShapeMaterialIndexStridingType,
    /// # C++ Info
    /// -          name: `indexStriding`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_indexStriding: i32,
    /// # C++ Info
    /// -          name: `flipAlternateTriangles`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_flipAlternateTriangles: i32,
    /// # C++ Info
    /// -          name: `numTriangles`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numTriangles: i32,
    /// # C++ Info
    /// -          name: `materialIndexBase`(ctype: `void*`)
    /// -        offset:  32(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialIndexBase: Pointer,
    /// # C++ Info
    /// -          name: `materialIndexStriding`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_materialIndexStriding: i32,
    /// # C++ Info
    /// -          name: `materialBase`(ctype: `void*`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialBase: Pointer,
    /// # C++ Info
    /// -          name: `materialStriding`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_materialStriding: i32,
    /// # C++ Info
    /// -          name: `numMaterials`(ctype: `hkInt32`)
    /// -        offset:  48(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numMaterials: i32,
    /// # C++ Info
    /// -          name: `triangleOffset`(ctype: `hkInt32`)
    /// -        offset:  52(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_triangleOffset: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMeshShapeSubpart {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMeshShapeSubpart"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x27336e5d)
        }
    }
    impl _serde::Serialize for hkpMeshShapeSubpart {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x27336e5d)));
            let mut serializer = __serializer
                .serialize_struct("hkpMeshShapeSubpart", class_meta)?;
            serializer.skip_field("vertexBase", &self.m_vertexBase)?;
            serializer.serialize_field("vertexStriding", &self.m_vertexStriding)?;
            serializer.serialize_field("numVertices", &self.m_numVertices)?;
            serializer.skip_field("indexBase", &self.m_indexBase)?;
            serializer.serialize_field("stridingType", &self.m_stridingType)?;
            serializer
                .serialize_field(
                    "materialIndexStridingType",
                    &self.m_materialIndexStridingType,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("indexStriding", &self.m_indexStriding)?;
            serializer
                .serialize_field(
                    "flipAlternateTriangles",
                    &self.m_flipAlternateTriangles,
                )?;
            serializer.serialize_field("numTriangles", &self.m_numTriangles)?;
            serializer.skip_field("materialIndexBase", &self.m_materialIndexBase)?;
            serializer
                .serialize_field(
                    "materialIndexStriding",
                    &self.m_materialIndexStriding,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialBase", &self.m_materialBase)?;
            serializer.serialize_field("materialStriding", &self.m_materialStriding)?;
            serializer.serialize_field("numMaterials", &self.m_numMaterials)?;
            serializer.serialize_field("triangleOffset", &self.m_triangleOffset)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
