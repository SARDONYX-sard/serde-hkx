use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbGeneratorTransitionEffectInternalState`
/// -         version: `0`
/// -       signature: `0xd6692b5d`
/// -          size:  32(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbGeneratorTransitionEffectInternalState {
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
    /// -          name: `timeInTransition`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timeInTransition: f32,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `effectiveBlendInDuration`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_effectiveBlendInDuration: f32,
    /// # C++ Info
    /// -          name: `effectiveBlendOutDuration`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_effectiveBlendOutDuration: f32,
    /// # C++ Info
    /// -          name: `toGeneratorState`(ctype: `enum ToGeneratorState`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_toGeneratorState: ToGeneratorState,
    /// # C++ Info
    /// -          name: `echoTransitionGenerator`(ctype: `hkBool`)
    /// -        offset:  25(x86)/ 33(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_echoTransitionGenerator: bool,
    /// # C++ Info
    /// -          name: `echoToGenerator`(ctype: `hkBool`)
    /// -        offset:  26(x86)/ 34(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_echoToGenerator: bool,
    /// # C++ Info
    /// -          name: `justActivated`(ctype: `hkBool`)
    /// -        offset:  27(x86)/ 35(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_justActivated: bool,
    /// # C++ Info
    /// -          name: `updateActiveNodes`(ctype: `hkBool`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_updateActiveNodes: bool,
    /// # C++ Info
    /// -          name: `stage`(ctype: `enum Stage`)
    /// -        offset:  29(x86)/ 37(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_stage: Stage,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbGeneratorTransitionEffectInternalState {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbGeneratorTransitionEffectInternalState"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3597216605u32)
        }
    }
    impl __serde::Serialize for hkbGeneratorTransitionEffectInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbGeneratorTransitionEffectInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("timeInTransition", &self.m_timeInTransition)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer
                .serialize_field(
                    "effectiveBlendInDuration",
                    &self.m_effectiveBlendInDuration,
                )?;
            serializer
                .serialize_field(
                    "effectiveBlendOutDuration",
                    &self.m_effectiveBlendOutDuration,
                )?;
            serializer.serialize_field("toGeneratorState", &self.m_toGeneratorState)?;
            serializer
                .serialize_field(
                    "echoTransitionGenerator",
                    &self.m_echoTransitionGenerator,
                )?;
            serializer.serialize_field("echoToGenerator", &self.m_echoToGenerator)?;
            serializer.serialize_field("justActivated", &self.m_justActivated)?;
            serializer.serialize_field("updateActiveNodes", &self.m_updateActiveNodes)?;
            serializer.serialize_field("stage", &self.m_stage)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.end()
        }
    }
};
