use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCompressedMeshShapeChunk`
/// -         version: `4`
/// -       signature: `0x5d0d67bd`
/// -          size:  80(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCompressedMeshShapeChunk {
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
    /// -          name: `indices`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  28(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_indices: Vec<u16>,
    /// # C++ Info
    /// -          name: `stripLengths`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  40(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_stripLengths: Vec<u16>,
    /// # C++ Info
    /// -          name: `weldingInfo`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  52(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_weldingInfo: Vec<u16>,
    /// # C++ Info
    /// -          name: `materialInfo`(ctype: `hkUint32`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_materialInfo: u32,
    /// # C++ Info
    /// -          name: `reference`(ctype: `hkUint16`)
    /// -        offset:  68(x86)/ 84(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_reference: u16,
    /// # C++ Info
    /// -          name: `transformIndex`(ctype: `hkUint16`)
    /// -        offset:  70(x86)/ 86(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_transformIndex: u16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCompressedMeshShapeChunk {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCompressedMeshShapeChunk"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5d0d67bd)
        }
    }
    impl _serde::Serialize for hkpCompressedMeshShapeChunk {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5d0d67bd)));
            let mut serializer = __serializer
                .serialize_struct("hkpCompressedMeshShapeChunk", class_meta)?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.serialize_array_meta_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_meta_field("indices", &self.m_indices)?;
            serializer.serialize_array_meta_field("stripLengths", &self.m_stripLengths)?;
            serializer.serialize_array_meta_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.serialize_field("materialInfo", &self.m_materialInfo)?;
            serializer.serialize_field("reference", &self.m_reference)?;
            serializer.serialize_field("transformIndex", &self.m_transformIndex)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_array_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_field("indices", &self.m_indices)?;
            serializer.serialize_array_field("stripLengths", &self.m_stripLengths)?;
            serializer.serialize_array_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.end()
        }
    }
};
