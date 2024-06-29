use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBlendingTransitionEffect`
/// -         version: `1`
/// -       signature: `0xfd8584fe`
/// -          size:  88(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlendingTransitionEffect<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbTransitionEffect<'a>,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `toGeneratorStartTimeFraction`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_toGeneratorStartTimeFraction: f32,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags FlagBits`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_flags: FlagBits,
    /// # C++ Info
    /// -          name: `endMode`(ctype: `enum EndMode`)
    /// -        offset:  54(x86)/ 90(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_endMode: EndMode,
    /// # C++ Info
    /// -          name: `blendCurve`(ctype: `enum BlendCurve`)
    /// -        offset:  55(x86)/ 91(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_blendCurve: BlendCurve,
    /// # C++ Info
    /// -          name: `fromGenerator`(ctype: `void*`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_fromGenerator: Pointer,
    /// # C++ Info
    /// -          name: `toGenerator`(ctype: `void*`)
    /// -        offset:  60(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_toGenerator: Pointer,
    /// # C++ Info
    /// -          name: `characterPoseAtBeginningOfTransition`(ctype: `hkArray<void>`)
    /// -        offset:  64(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_characterPoseAtBeginningOfTransition: Vec<()>,
    /// # C++ Info
    /// -          name: `timeRemaining`(ctype: `hkReal`)
    /// -        offset:  76(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeRemaining: f32,
    /// # C++ Info
    /// -          name: `timeInTransition`(ctype: `hkReal`)
    /// -        offset:  80(x86)/132(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeInTransition: f32,
    /// # C++ Info
    /// -          name: `applySelfTransition`(ctype: `hkBool`)
    /// -        offset:  84(x86)/136(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_applySelfTransition: bool,
    /// # C++ Info
    /// -          name: `initializeCharacterPose`(ctype: `hkBool`)
    /// -        offset:  85(x86)/137(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_initializeCharacterPose: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbBlendingTransitionEffect<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbBlendingTransitionEffect"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4253385982u32)
        }
    }
    impl<'a> __serde::Serialize for hkbBlendingTransitionEffect<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbBlendingTransitionEffect", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field(
                    "name",
                    &self.parent.parent.parent.m_name,
                )?;
            serializer.skip_field("id", &self.parent.parent.parent.m_id)?;
            serializer
                .skip_field("cloneState", &self.parent.parent.parent.m_cloneState)?;
            serializer
                .skip_field("padNode", &self.parent.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "selfTransitionMode",
                    &self.parent.m_selfTransitionMode,
                )?;
            serializer.serialize_field("eventMode", &self.parent.m_eventMode)?;
            serializer.skip_field("defaultEventMode", &self.parent.m_defaultEventMode)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer
                .serialize_field(
                    "toGeneratorStartTimeFraction",
                    &self.m_toGeneratorStartTimeFraction,
                )?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("endMode", &self.m_endMode)?;
            serializer.serialize_field("blendCurve", &self.m_blendCurve)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("fromGenerator", &self.m_fromGenerator)?;
            serializer.skip_field("toGenerator", &self.m_toGenerator)?;
            serializer
                .skip_array_meta_field(
                    "characterPoseAtBeginningOfTransition",
                    &self.m_characterPoseAtBeginningOfTransition,
                )?;
            serializer.skip_field("timeRemaining", &self.m_timeRemaining)?;
            serializer.skip_field("timeInTransition", &self.m_timeInTransition)?;
            serializer.skip_field("applySelfTransition", &self.m_applySelfTransition)?;
            serializer
                .skip_field("initializeCharacterPose", &self.m_initializeCharacterPose)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .serialize_stringptr_field("name", &self.parent.parent.parent.m_name)?;
            serializer
                .serialize_array_field(
                    "characterPoseAtBeginningOfTransition",
                    &self.m_characterPoseAtBeginningOfTransition,
                )?;
            serializer.end()
        }
    }
};
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT16`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)] pub
    struct FlagBits : u16 { #[doc = "0"] const FLAG_NONE = 0u16; #[doc = "1"] const
    FLAG_IGNORE_FROM_WORLD_FROM_MODEL = 1u16; #[doc = "2"] const FLAG_SYNC = 2u16; #[doc
    = "4"] const FLAG_IGNORE_TO_WORLD_FROM_MODEL = 4u16; }
}
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum EndMode {
    #[default]
    END_MODE_NONE = 0isize,
    END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR = 1isize,
    END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for FlagBits {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::FLAG_NONE => {
                        __serializer.serialize_field("FLAG_NONE", &Self::FLAG_NONE)
                    }
                    Self::FLAG_IGNORE_FROM_WORLD_FROM_MODEL => {
                        __serializer
                            .serialize_field(
                                "FLAG_IGNORE_FROM_WORLD_FROM_MODEL",
                                &Self::FLAG_IGNORE_FROM_WORLD_FROM_MODEL,
                            )
                    }
                    Self::FLAG_SYNC => {
                        __serializer.serialize_field("FLAG_SYNC", &Self::FLAG_SYNC)
                    }
                    Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL => {
                        __serializer
                            .serialize_field(
                                "FLAG_IGNORE_TO_WORLD_FROM_MODEL",
                                &Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL,
                            )
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for EndMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::END_MODE_NONE => {
                    __serializer.serialize_field("END_MODE_NONE", &0u64)
                }
                Self::END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR => {
                    __serializer
                        .serialize_field(
                            "END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR",
                            &1u64,
                        )
                }
                Self::END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR => {
                    __serializer
                        .serialize_field(
                            "END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR",
                            &2u64,
                        )
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i8().ok_or(S::Error::custom("Failed enum EndMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
