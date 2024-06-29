use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryMeshTexture`
/// -         version: `2`
/// -       signature: `0x2db6577c`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryMeshTexture<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkMeshTexture,
    /// # C++ Info
    /// -          name: `filename`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_filename: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_data: Vec<u8>,
    /// # C++ Info
    /// -          name: `format`(ctype: `enum Format`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_format: Format,
    /// # C++ Info
    /// -          name: `hasMipMaps`(ctype: `hkBool`)
    /// -        offset:  25(x86)/ 41(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hasMipMaps: bool,
    /// # C++ Info
    /// -          name: `filterMode`(ctype: `enum FilterMode`)
    /// -        offset:  26(x86)/ 42(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_filterMode: FilterMode,
    /// # C++ Info
    /// -          name: `usageHint`(ctype: `enum TextureUsageType`)
    /// -        offset:  27(x86)/ 43(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_usageHint: TextureUsageType,
    /// # C++ Info
    /// -          name: `textureCoordChannel`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_textureCoordChannel: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkMemoryMeshTexture<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMemoryMeshTexture"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(766924668u32)
        }
    }
    impl<'a> __serde::Serialize for hkMemoryMeshTexture<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryMeshTexture", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("filename", &self.m_filename)?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_field("format", &self.m_format)?;
            serializer.serialize_field("hasMipMaps", &self.m_hasMipMaps)?;
            serializer.serialize_field("filterMode", &self.m_filterMode)?;
            serializer.serialize_field("usageHint", &self.m_usageHint)?;
            serializer
                .serialize_field("textureCoordChannel", &self.m_textureCoordChannel)?;
            serializer.serialize_stringptr_field("filename", &self.m_filename)?;
            serializer.serialize_array_field("data", &self.m_data)?;
            serializer.end()
        }
    }
};
