use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxTextureInplace`
/// -         version: `1`
/// -       signature: `0xd45841d6`
/// -          size:  32(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxTextureInplace<'a> {
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
    /// -          name: `fileType`(ctype: `hkChar[4]`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fileType: [char; 4usize],
    /// # C++ Info
    /// -          name: `data`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_data: Vec<u8>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `originalFilename`(ctype: `hkStringPtr`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_originalFilename: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxTextureInplace<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxTextureInplace"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3562553814u32)
        }
    }
    impl<'a> _serde::Serialize for hkxTextureInplace<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3562553814u32)));
            let mut serializer = __serializer
                .serialize_struct("hkxTextureInplace", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("fileType", &self.m_fileType.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer
                .serialize_stringptr_meta_field(
                    "originalFilename",
                    &self.m_originalFilename,
                )?;
            serializer.serialize_array_field("data", &self.m_data)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer
                .serialize_stringptr_field(
                    "originalFilename",
                    &self.m_originalFilename,
                )?;
            serializer.end()
        }
    }
};
