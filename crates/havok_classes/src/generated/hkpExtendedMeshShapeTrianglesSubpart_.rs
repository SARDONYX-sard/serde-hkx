use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpExtendedMeshShapeTrianglesSubpart`
/// -         version: `3`
/// -       signature: `0x44c32df6`
/// -          size: 112(x86)/160(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpExtendedMeshShapeTrianglesSubpart {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpExtendedMeshShapeSubpart,
    /// # C++ Info
    /// -          name: `numTriangleShapes`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numTriangleShapes: i32,
    /// # C++ Info
    /// -          name: `vertexBase`(ctype: `void*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_vertexBase: Pointer,
    /// # C++ Info
    /// -          name: `numVertices`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numVertices: i32,
    /// # C++ Info
    /// -          name: `indexBase`(ctype: `void*`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_indexBase: Pointer,
    /// # C++ Info
    /// -          name: `vertexStriding`(ctype: `hkUint16`)
    /// -        offset:  36(x86)/ 72(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_vertexStriding: u16,
    /// # C++ Info
    /// -          name: `triangleOffset`(ctype: `hkInt32`)
    /// -        offset:  40(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_triangleOffset: i32,
    /// # C++ Info
    /// -          name: `indexStriding`(ctype: `hkUint16`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_indexStriding: u16,
    /// # C++ Info
    /// -          name: `stridingType`(ctype: `enum IndexStridingType`)
    /// -        offset:  46(x86)/ 82(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_stridingType: IndexStridingType,
    /// # C++ Info
    /// -          name: `flipAlternateTriangles`(ctype: `hkInt8`)
    /// -        offset:  47(x86)/ 83(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_flipAlternateTriangles: i8,
    /// # C++ Info
    /// -          name: `extrusion`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_extrusion: Vector4,
    /// # C++ Info
    /// -          name: `transform`(ctype: `hkQsTransform`)
    /// -        offset:  64(x86)/112(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_transform: QsTransform,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpExtendedMeshShapeTrianglesSubpart {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpExtendedMeshShapeTrianglesSubpart"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x44c32df6)
        }
    }
    impl _serde::Serialize for hkpExtendedMeshShapeTrianglesSubpart {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x44c32df6)));
            let mut serializer = __serializer
                .serialize_struct("hkpExtendedMeshShapeTrianglesSubpart", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer
                .serialize_field(
                    "materialIndexStridingType",
                    &self.parent.m_materialIndexStridingType,
                )?;
            serializer.skip_field("materialStriding", &self.parent.m_materialStriding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field("materialIndexBase", &self.parent.m_materialIndexBase)?;
            serializer
                .serialize_field(
                    "materialIndexStriding",
                    &self.parent.m_materialIndexStriding,
                )?;
            serializer.serialize_field("numMaterials", &self.parent.m_numMaterials)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialBase", &self.parent.m_materialBase)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("numTriangleShapes", &self.m_numTriangleShapes)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("vertexBase", &self.m_vertexBase)?;
            serializer.serialize_field("numVertices", &self.m_numVertices)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("indexBase", &self.m_indexBase)?;
            serializer.serialize_field("vertexStriding", &self.m_vertexStriding)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("triangleOffset", &self.m_triangleOffset)?;
            serializer.serialize_field("indexStriding", &self.m_indexStriding)?;
            serializer.serialize_field("stridingType", &self.m_stridingType)?;
            serializer
                .serialize_field(
                    "flipAlternateTriangles",
                    &self.m_flipAlternateTriangles,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_field("extrusion", &self.m_extrusion)?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer.end()
        }
    }
};
