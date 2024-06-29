use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCompressedMeshShapeConvexPiece`
/// -         version: `3`
/// -       signature: `0x385bb842`
/// -          size:  64(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCompressedMeshShapeConvexPiece {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `offset`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_offset: Vector4,
    /// # C++ Info
    /// -          name: `vertices`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vertices: Vec<u16>,
    /// # C++ Info
    /// -          name: `faceVertices`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  28(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_faceVertices: Vec<u8>,
    /// # C++ Info
    /// -          name: `faceOffsets`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  40(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_faceOffsets: Vec<u16>,
    /// # C++ Info
    /// -          name: `reference`(ctype: `hkUint16`)
    /// -        offset:  52(x86)/ 64(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_reference: u16,
    /// # C++ Info
    /// -          name: `transformIndex`(ctype: `hkUint16`)
    /// -        offset:  54(x86)/ 66(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_transformIndex: u16,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCompressedMeshShapeConvexPiece {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpCompressedMeshShapeConvexPiece"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(945535042u32)
        }
    }
    impl __serde::Serialize for hkpCompressedMeshShapeConvexPiece {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpCompressedMeshShapeConvexPiece", class_meta)?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.serialize_array_meta_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_meta_field("faceVertices", &self.m_faceVertices)?;
            serializer.serialize_array_meta_field("faceOffsets", &self.m_faceOffsets)?;
            serializer.serialize_field("reference", &self.m_reference)?;
            serializer.serialize_field("transformIndex", &self.m_transformIndex)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_array_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_field("faceVertices", &self.m_faceVertices)?;
            serializer.serialize_array_field("faceOffsets", &self.m_faceOffsets)?;
            serializer.end()
        }
    }
};
