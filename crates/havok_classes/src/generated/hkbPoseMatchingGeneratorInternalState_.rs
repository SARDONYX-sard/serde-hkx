use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbPoseMatchingGeneratorInternalState`
/// -         version: `0`
/// -       signature: `0x552d9dd4`
/// -          size:  28(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbPoseMatchingGeneratorInternalState {
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
    /// -          name: `currentMatch`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_currentMatch: i32,
    /// # C++ Info
    /// -          name: `bestMatch`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bestMatch: i32,
    /// # C++ Info
    /// -          name: `timeSinceBetterMatch`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timeSinceBetterMatch: f32,
    /// # C++ Info
    /// -          name: `error`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_error: f32,
    /// # C++ Info
    /// -          name: `resetCurrentMatchLocalTime`(ctype: `hkBool`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_resetCurrentMatchLocalTime: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbPoseMatchingGeneratorInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbPoseMatchingGeneratorInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1429052884u32)
        }
    }
    impl _serde::Serialize for hkbPoseMatchingGeneratorInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1429052884u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbPoseMatchingGeneratorInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("currentMatch", &self.m_currentMatch)?;
            serializer.serialize_field("bestMatch", &self.m_bestMatch)?;
            serializer
                .serialize_field("timeSinceBetterMatch", &self.m_timeSinceBetterMatch)?;
            serializer.serialize_field("error", &self.m_error)?;
            serializer
                .serialize_field(
                    "resetCurrentMatchLocalTime",
                    &self.m_resetCurrentMatchLocalTime,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
