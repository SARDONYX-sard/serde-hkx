use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbClipTrigger`
/// -         version: `1`
/// -       signature: `0x7eb45cea`
/// -          size:  16(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbClipTrigger {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `localTime`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_localTime: f32,
    /// # C++ Info
    /// -          name: `event`(ctype: `struct hkbEventProperty`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_event: hkbEventProperty,
    /// # C++ Info
    /// -          name: `relativeToEndOfClip`(ctype: `hkBool`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_relativeToEndOfClip: bool,
    /// # C++ Info
    /// -          name: `acyclic`(ctype: `hkBool`)
    /// -        offset:  13(x86)/ 25(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_acyclic: bool,
    /// # C++ Info
    /// -          name: `isAnnotation`(ctype: `hkBool`)
    /// -        offset:  14(x86)/ 26(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isAnnotation: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbClipTrigger {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbClipTrigger"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2125749482u32)
        }
    }
    impl _serde::Serialize for hkbClipTrigger {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2125749482u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbClipTrigger", class_meta)?;
            serializer.serialize_field("localTime", &self.m_localTime)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("event", &self.m_event)?;
            serializer
                .serialize_field("relativeToEndOfClip", &self.m_relativeToEndOfClip)?;
            serializer.serialize_field("acyclic", &self.m_acyclic)?;
            serializer.serialize_field("isAnnotation", &self.m_isAnnotation)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.end()
        }
    }
};
