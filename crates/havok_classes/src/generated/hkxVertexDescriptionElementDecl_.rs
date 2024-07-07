use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxVertexDescriptionElementDecl`
/// -         version: `2`
/// -       signature: `0x483a429b`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxVertexDescriptionElementDecl {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `byteOffset`(ctype: `hkUint32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_byteOffset: u32,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum DataType`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_type: DataType,
    /// # C++ Info
    /// -          name: `usage`(ctype: `enum DataUsage`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_usage: DataUsage,
    /// # C++ Info
    /// -          name: `byteStride`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_byteStride: u32,
    /// # C++ Info
    /// -          name: `numElements`(ctype: `hkUint8`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numElements: u8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxVertexDescriptionElementDecl {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxVertexDescriptionElementDecl"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x483a429b)
        }
    }
    impl _serde::Serialize for hkxVertexDescriptionElementDecl {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x483a429b)));
            let mut serializer = __serializer
                .serialize_struct("hkxVertexDescriptionElementDecl", class_meta)?;
            serializer.serialize_field("byteOffset", &self.m_byteOffset)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("usage", &self.m_usage)?;
            serializer.serialize_field("byteStride", &self.m_byteStride)?;
            serializer.serialize_field("numElements", &self.m_numElements)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
