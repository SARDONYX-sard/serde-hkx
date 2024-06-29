use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbGeneratorTransitionEffect`
/// -         version: `1`
/// -       signature: `0x5f771b12`
/// -          size:  92(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbGeneratorTransitionEffect<'a> {
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
    /// -          name: `transitionGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_transitionGenerator: Pointer,
    /// # C++ Info
    /// -          name: `blendInDuration`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blendInDuration: f32,
    /// # C++ Info
    /// -          name: `blendOutDuration`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blendOutDuration: f32,
    /// # C++ Info
    /// -          name: `syncToGeneratorStartTime`(ctype: `hkBool`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_syncToGeneratorStartTime: bool,
    /// # C++ Info
    /// -          name: `fromGenerator`(ctype: `void*`)
    /// -        offset:  60(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_fromGenerator: Pointer,
    /// # C++ Info
    /// -          name: `toGenerator`(ctype: `void*`)
    /// -        offset:  64(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_toGenerator: Pointer,
    /// # C++ Info
    /// -          name: `timeInTransition`(ctype: `hkReal`)
    /// -        offset:  68(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeInTransition: f32,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  72(x86)/124(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `effectiveBlendInDuration`(ctype: `hkReal`)
    /// -        offset:  76(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_effectiveBlendInDuration: f32,
    /// # C++ Info
    /// -          name: `effectiveBlendOutDuration`(ctype: `hkReal`)
    /// -        offset:  80(x86)/132(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_effectiveBlendOutDuration: f32,
    /// # C++ Info
    /// -          name: `toGeneratorState`(ctype: `enum unknown`)
    /// -        offset:  84(x86)/136(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_toGeneratorState: i8,
    /// # C++ Info
    /// -          name: `echoTransitionGenerator`(ctype: `hkBool`)
    /// -        offset:  85(x86)/137(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_echoTransitionGenerator: bool,
    /// # C++ Info
    /// -          name: `echoToGenerator`(ctype: `hkBool`)
    /// -        offset:  86(x86)/138(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_echoToGenerator: bool,
    /// # C++ Info
    /// -          name: `justActivated`(ctype: `hkBool`)
    /// -        offset:  87(x86)/139(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_justActivated: bool,
    /// # C++ Info
    /// -          name: `updateActiveNodes`(ctype: `hkBool`)
    /// -        offset:  88(x86)/140(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_updateActiveNodes: bool,
    /// # C++ Info
    /// -          name: `stage`(ctype: `enum unknown`)
    /// -        offset:  89(x86)/141(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_stage: i8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbGeneratorTransitionEffect<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbGeneratorTransitionEffect"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1601641234u32)
        }
    }
    impl<'a> __serde::Serialize for hkbGeneratorTransitionEffect<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbGeneratorTransitionEffect", class_meta)?;
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
            serializer
                .serialize_field("transitionGenerator", &self.m_transitionGenerator)?;
            serializer.serialize_field("blendInDuration", &self.m_blendInDuration)?;
            serializer.serialize_field("blendOutDuration", &self.m_blendOutDuration)?;
            serializer
                .serialize_field(
                    "syncToGeneratorStartTime",
                    &self.m_syncToGeneratorStartTime,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_field("fromGenerator", &self.m_fromGenerator)?;
            serializer.skip_field("toGenerator", &self.m_toGenerator)?;
            serializer.skip_field("timeInTransition", &self.m_timeInTransition)?;
            serializer.skip_field("duration", &self.m_duration)?;
            serializer
                .skip_field(
                    "effectiveBlendInDuration",
                    &self.m_effectiveBlendInDuration,
                )?;
            serializer
                .skip_field(
                    "effectiveBlendOutDuration",
                    &self.m_effectiveBlendOutDuration,
                )?;
            serializer.skip_field("toGeneratorState", &self.m_toGeneratorState)?;
            serializer
                .skip_field("echoTransitionGenerator", &self.m_echoTransitionGenerator)?;
            serializer.skip_field("echoToGenerator", &self.m_echoToGenerator)?;
            serializer.skip_field("justActivated", &self.m_justActivated)?;
            serializer.skip_field("updateActiveNodes", &self.m_updateActiveNodes)?;
            serializer.skip_field("stage", &self.m_stage)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .serialize_stringptr_field("name", &self.parent.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
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
pub enum ToGeneratorState {
    #[default]
    STATE_INACTIVE = 0isize,
    STATE_READY_FOR_SET_LOCAL_TIME = 1isize,
    STATE_READY_FOR_APPLY_SELF_TRANSITION_MODE = 2isize,
    STATE_ACTIVE = 3isize,
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
pub enum Stage {
    #[default]
    STAGE_BLENDING_IN = 0isize,
    STAGE_PLAYING_TRANSITION_GENERATOR = 1isize,
    STAGE_BLENDING_OUT = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ToGeneratorState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::STATE_INACTIVE => {
                    __serializer.serialize_field("STATE_INACTIVE", &0u64)
                }
                Self::STATE_READY_FOR_SET_LOCAL_TIME => {
                    __serializer.serialize_field("STATE_READY_FOR_SET_LOCAL_TIME", &1u64)
                }
                Self::STATE_READY_FOR_APPLY_SELF_TRANSITION_MODE => {
                    __serializer
                        .serialize_field(
                            "STATE_READY_FOR_APPLY_SELF_TRANSITION_MODE",
                            &2u64,
                        )
                }
                Self::STATE_ACTIVE => __serializer.serialize_field("STATE_ACTIVE", &3u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum ToGeneratorState to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Stage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::STAGE_BLENDING_IN => {
                    __serializer.serialize_field("STAGE_BLENDING_IN", &0u64)
                }
                Self::STAGE_PLAYING_TRANSITION_GENERATOR => {
                    __serializer
                        .serialize_field("STAGE_PLAYING_TRANSITION_GENERATOR", &1u64)
                }
                Self::STAGE_BLENDING_OUT => {
                    __serializer.serialize_field("STAGE_BLENDING_OUT", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i8().ok_or(S::Error::custom("Failed enum Stage to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
