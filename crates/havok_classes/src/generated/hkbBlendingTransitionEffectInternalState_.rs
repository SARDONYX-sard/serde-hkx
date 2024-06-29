use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBlendingTransitionEffectInternalState`
/// -         version: `0`
/// -       signature: `0xb18c70c2`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlendingTransitionEffectInternalState {
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
    /// -          name: `characterPoseAtBeginningOfTransition`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_characterPoseAtBeginningOfTransition: Vec<QsTransform>,
    /// # C++ Info
    /// -          name: `timeRemaining`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timeRemaining: f32,
    /// # C++ Info
    /// -          name: `timeInTransition`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timeInTransition: f32,
    /// # C++ Info
    /// -          name: `applySelfTransition`(ctype: `hkBool`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_applySelfTransition: bool,
    /// # C++ Info
    /// -          name: `initializeCharacterPose`(ctype: `hkBool`)
    /// -        offset:  29(x86)/ 41(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_initializeCharacterPose: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbBlendingTransitionEffectInternalState {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbBlendingTransitionEffectInternalState"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2978771138u32)
        }
    }
    impl __serde::Serialize for hkbBlendingTransitionEffectInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbBlendingTransitionEffectInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "characterPoseAtBeginningOfTransition",
                    &self.m_characterPoseAtBeginningOfTransition,
                )?;
            serializer.serialize_field("timeRemaining", &self.m_timeRemaining)?;
            serializer.serialize_field("timeInTransition", &self.m_timeInTransition)?;
            serializer
                .serialize_field("applySelfTransition", &self.m_applySelfTransition)?;
            serializer
                .serialize_field(
                    "initializeCharacterPose",
                    &self.m_initializeCharacterPose,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "characterPoseAtBeginningOfTransition",
                    &self.m_characterPoseAtBeginningOfTransition,
                )?;
            serializer.end()
        }
    }
};
