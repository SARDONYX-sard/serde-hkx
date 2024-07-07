use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkPackfileHeader`
/// -         version: `1`
/// -       signature: `0x79f9ffda`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkPackfileHeader {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `magic`(ctype: `hkInt32[2]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_magic: [i32; 2usize],
    /// # C++ Info
    /// -          name: `userTag`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_userTag: i32,
    /// # C++ Info
    /// -          name: `fileVersion`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fileVersion: i32,
    /// # C++ Info
    /// -          name: `layoutRules`(ctype: `hkUint8[4]`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_layoutRules: [u8; 4usize],
    /// # C++ Info
    /// -          name: `numSections`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numSections: i32,
    /// # C++ Info
    /// -          name: `contentsSectionIndex`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_contentsSectionIndex: i32,
    /// # C++ Info
    /// -          name: `contentsSectionOffset`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_contentsSectionOffset: i32,
    /// # C++ Info
    /// -          name: `contentsClassNameSectionIndex`(ctype: `hkInt32`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_contentsClassNameSectionIndex: i32,
    /// # C++ Info
    /// -          name: `contentsClassNameSectionOffset`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_contentsClassNameSectionOffset: i32,
    /// # C++ Info
    /// -          name: `contentsVersion`(ctype: `hkChar[16]`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_contentsVersion: [char; 16usize],
    /// # C++ Info
    /// -          name: `flags`(ctype: `hkInt32`)
    /// -        offset:  56(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_flags: i32,
    /// # C++ Info
    /// -          name: `pad`(ctype: `hkInt32[1]`)
    /// -        offset:  60(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_pad: [i32; 1usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkPackfileHeader {
        #[inline]
        fn name(&self) -> &'static str {
            "hkPackfileHeader"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x79f9ffda)
        }
    }
    impl _serde::Serialize for hkPackfileHeader {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x79f9ffda)));
            let mut serializer = __serializer
                .serialize_struct("hkPackfileHeader", class_meta)?;
            serializer.serialize_field("magic", &self.m_magic.as_slice())?;
            serializer.serialize_field("userTag", &self.m_userTag)?;
            serializer.serialize_field("fileVersion", &self.m_fileVersion)?;
            serializer.serialize_field("layoutRules", &self.m_layoutRules.as_slice())?;
            serializer.serialize_field("numSections", &self.m_numSections)?;
            serializer
                .serialize_field("contentsSectionIndex", &self.m_contentsSectionIndex)?;
            serializer
                .serialize_field(
                    "contentsSectionOffset",
                    &self.m_contentsSectionOffset,
                )?;
            serializer
                .serialize_field(
                    "contentsClassNameSectionIndex",
                    &self.m_contentsClassNameSectionIndex,
                )?;
            serializer
                .serialize_field(
                    "contentsClassNameSectionOffset",
                    &self.m_contentsClassNameSectionOffset,
                )?;
            serializer
                .serialize_field("contentsVersion", &self.m_contentsVersion.as_slice())?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("pad", &self.m_pad.as_slice())?;
            serializer.end()
        }
    }
};
