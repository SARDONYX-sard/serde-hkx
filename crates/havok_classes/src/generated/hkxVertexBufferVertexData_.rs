use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxVertexBufferVertexData`
/// -         version: `0`
/// -       signature: `0xd72b6fd0`
/// -          size:  84(x86)/104(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxVertexBufferVertexData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `vectorData`(ctype: `hkArray<hkVector4>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vectorData: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `floatData`(ctype: `hkArray<hkReal>`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_floatData: Vec<f32>,
    /// # C++ Info
    /// -          name: `uint32Data`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_uint32Data: Vec<u32>,
    /// # C++ Info
    /// -          name: `uint16Data`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  36(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_uint16Data: Vec<u16>,
    /// # C++ Info
    /// -          name: `uint8Data`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_uint8Data: Vec<u8>,
    /// # C++ Info
    /// -          name: `numVerts`(ctype: `hkUint32`)
    /// -        offset:  60(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numVerts: u32,
    /// # C++ Info
    /// -          name: `vectorStride`(ctype: `hkUint32`)
    /// -        offset:  64(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vectorStride: u32,
    /// # C++ Info
    /// -          name: `floatStride`(ctype: `hkUint32`)
    /// -        offset:  68(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_floatStride: u32,
    /// # C++ Info
    /// -          name: `uint32Stride`(ctype: `hkUint32`)
    /// -        offset:  72(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_uint32Stride: u32,
    /// # C++ Info
    /// -          name: `uint16Stride`(ctype: `hkUint32`)
    /// -        offset:  76(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_uint16Stride: u32,
    /// # C++ Info
    /// -          name: `uint8Stride`(ctype: `hkUint32`)
    /// -        offset:  80(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_uint8Stride: u32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxVertexBufferVertexData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxVertexBufferVertexData"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3609948112u32)
        }
    }
    impl __serde::Serialize for hkxVertexBufferVertexData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3609948112u32)));
            let mut serializer = __serializer
                .serialize_struct("hkxVertexBufferVertexData", class_meta)?;
            serializer.serialize_array_meta_field("vectorData", &self.m_vectorData)?;
            serializer.serialize_array_meta_field("floatData", &self.m_floatData)?;
            serializer.serialize_array_meta_field("uint32Data", &self.m_uint32Data)?;
            serializer.serialize_array_meta_field("uint16Data", &self.m_uint16Data)?;
            serializer.serialize_array_meta_field("uint8Data", &self.m_uint8Data)?;
            serializer.serialize_field("numVerts", &self.m_numVerts)?;
            serializer.serialize_field("vectorStride", &self.m_vectorStride)?;
            serializer.serialize_field("floatStride", &self.m_floatStride)?;
            serializer.serialize_field("uint32Stride", &self.m_uint32Stride)?;
            serializer.serialize_field("uint16Stride", &self.m_uint16Stride)?;
            serializer.serialize_field("uint8Stride", &self.m_uint8Stride)?;
            serializer.serialize_array_field("vectorData", &self.m_vectorData)?;
            serializer.serialize_array_field("floatData", &self.m_floatData)?;
            serializer.serialize_array_field("uint32Data", &self.m_uint32Data)?;
            serializer.serialize_array_field("uint16Data", &self.m_uint16Data)?;
            serializer.serialize_array_field("uint8Data", &self.m_uint8Data)?;
            serializer.end()
        }
    }
};
