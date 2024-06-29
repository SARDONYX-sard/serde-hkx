use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachine`
/// -         version: `4`
/// -       signature: `0x816c1dcb`
/// -          size: 180(x86)/264(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachine<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// -          name: `eventToSendWhenStateOrTransitionChanges`(ctype: `struct hkbEvent`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_eventToSendWhenStateOrTransitionChanges: hkbEvent,
    /// # C++ Info
    /// -          name: `startStateChooser`(ctype: `struct hkbStateChooser*`)
    /// -        offset:  52(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_startStateChooser: Pointer,
    /// # C++ Info
    /// -          name: `startStateId`(ctype: `hkInt32`)
    /// -        offset:  56(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_startStateId: i32,
    /// # C++ Info
    /// -          name: `returnToPreviousStateEventId`(ctype: `hkInt32`)
    /// -        offset:  60(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_returnToPreviousStateEventId: i32,
    /// # C++ Info
    /// -          name: `randomTransitionEventId`(ctype: `hkInt32`)
    /// -        offset:  64(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_randomTransitionEventId: i32,
    /// # C++ Info
    /// -          name: `transitionToNextHigherStateEventId`(ctype: `hkInt32`)
    /// -        offset:  68(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_transitionToNextHigherStateEventId: i32,
    /// # C++ Info
    /// -          name: `transitionToNextLowerStateEventId`(ctype: `hkInt32`)
    /// -        offset:  72(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_transitionToNextLowerStateEventId: i32,
    /// # C++ Info
    /// -          name: `syncVariableIndex`(ctype: `hkInt32`)
    /// -        offset:  76(x86)/124(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_syncVariableIndex: i32,
    /// # C++ Info
    /// -          name: `currentStateId`(ctype: `hkInt32`)
    /// -        offset:  80(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_currentStateId: i32,
    /// # C++ Info
    /// -          name: `wrapAroundStateId`(ctype: `hkBool`)
    /// -        offset:  84(x86)/132(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_wrapAroundStateId: bool,
    /// # C++ Info
    /// -          name: `maxSimultaneousTransitions`(ctype: `hkInt8`)
    /// -        offset:  85(x86)/133(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_maxSimultaneousTransitions: i8,
    /// # C++ Info
    /// -          name: `startStateMode`(ctype: `enum StartStateMode`)
    /// -        offset:  86(x86)/134(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_startStateMode: StartStateMode,
    /// # C++ Info
    /// -          name: `selfTransitionMode`(ctype: `enum StateMachineSelfTransitionMode`)
    /// -        offset:  87(x86)/135(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_selfTransitionMode: StateMachineSelfTransitionMode,
    /// # C++ Info
    /// -          name: `isActive`(ctype: `hkBool`)
    /// -        offset:  88(x86)/136(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isActive: bool,
    /// # C++ Info
    /// -          name: `states`(ctype: `hkArray<hkbStateMachineStateInfo*>`)
    /// -        offset:  92(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_states: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `wildcardTransitions`(ctype: `struct hkbStateMachineTransitionInfoArray*`)
    /// -        offset: 104(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_wildcardTransitions: Pointer,
    /// # C++ Info
    /// -          name: `stateIdToIndexMap`(ctype: `void*`)
    /// -        offset: 108(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_stateIdToIndexMap: Pointer,
    /// # C++ Info
    /// -          name: `activeTransitions`(ctype: `hkArray<void>`)
    /// -        offset: 112(x86)/176(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_activeTransitions: Vec<()>,
    /// # C++ Info
    /// -          name: `transitionFlags`(ctype: `hkArray<void>`)
    /// -        offset: 124(x86)/192(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_transitionFlags: Vec<()>,
    /// # C++ Info
    /// -          name: `wildcardTransitionFlags`(ctype: `hkArray<void>`)
    /// -        offset: 136(x86)/208(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_wildcardTransitionFlags: Vec<()>,
    /// # C++ Info
    /// -          name: `delayedTransitions`(ctype: `hkArray<void>`)
    /// -        offset: 148(x86)/224(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_delayedTransitions: Vec<()>,
    /// # C++ Info
    /// -          name: `timeInState`(ctype: `hkReal`)
    /// -        offset: 160(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeInState: f32,
    /// # C++ Info
    /// -          name: `lastLocalTime`(ctype: `hkReal`)
    /// -        offset: 164(x86)/244(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_lastLocalTime: f32,
    /// # C++ Info
    /// -          name: `previousStateId`(ctype: `hkInt32`)
    /// -        offset: 168(x86)/248(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_previousStateId: i32,
    /// # C++ Info
    /// -          name: `nextStartStateIndexOverride`(ctype: `hkInt32`)
    /// -        offset: 172(x86)/252(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nextStartStateIndexOverride: i32,
    /// # C++ Info
    /// -          name: `stateOrTransitionChanged`(ctype: `hkBool`)
    /// -        offset: 176(x86)/256(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_stateOrTransitionChanged: bool,
    /// # C++ Info
    /// -          name: `echoNextUpdate`(ctype: `hkBool`)
    /// -        offset: 177(x86)/257(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_echoNextUpdate: bool,
    /// # C++ Info
    /// -          name: `sCurrentStateIndexAndEntered`(ctype: `hkUint16`)
    /// -        offset: 178(x86)/258(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_sCurrentStateIndexAndEntered: u16,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbStateMachine<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbStateMachine"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2171346379u32)
        }
    }
    impl<'a> __serde::Serialize for hkbStateMachine<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachine", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "eventToSendWhenStateOrTransitionChanges",
                    &self.m_eventToSendWhenStateOrTransitionChanges,
                )?;
            serializer.serialize_field("startStateChooser", &self.m_startStateChooser)?;
            serializer.serialize_field("startStateId", &self.m_startStateId)?;
            serializer
                .serialize_field(
                    "returnToPreviousStateEventId",
                    &self.m_returnToPreviousStateEventId,
                )?;
            serializer
                .serialize_field(
                    "randomTransitionEventId",
                    &self.m_randomTransitionEventId,
                )?;
            serializer
                .serialize_field(
                    "transitionToNextHigherStateEventId",
                    &self.m_transitionToNextHigherStateEventId,
                )?;
            serializer
                .serialize_field(
                    "transitionToNextLowerStateEventId",
                    &self.m_transitionToNextLowerStateEventId,
                )?;
            serializer.serialize_field("syncVariableIndex", &self.m_syncVariableIndex)?;
            serializer.skip_field("currentStateId", &self.m_currentStateId)?;
            serializer.serialize_field("wrapAroundStateId", &self.m_wrapAroundStateId)?;
            serializer
                .serialize_field(
                    "maxSimultaneousTransitions",
                    &self.m_maxSimultaneousTransitions,
                )?;
            serializer.serialize_field("startStateMode", &self.m_startStateMode)?;
            serializer
                .serialize_field("selfTransitionMode", &self.m_selfTransitionMode)?;
            serializer.skip_field("isActive", &self.m_isActive)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_meta_field("states", &self.m_states)?;
            serializer
                .serialize_field("wildcardTransitions", &self.m_wildcardTransitions)?;
            serializer.skip_field("stateIdToIndexMap", &self.m_stateIdToIndexMap)?;
            serializer
                .skip_array_meta_field("activeTransitions", &self.m_activeTransitions)?;
            serializer
                .skip_array_meta_field("transitionFlags", &self.m_transitionFlags)?;
            serializer
                .skip_array_meta_field(
                    "wildcardTransitionFlags",
                    &self.m_wildcardTransitionFlags,
                )?;
            serializer
                .skip_array_meta_field(
                    "delayedTransitions",
                    &self.m_delayedTransitions,
                )?;
            serializer.skip_field("timeInState", &self.m_timeInState)?;
            serializer.skip_field("lastLocalTime", &self.m_lastLocalTime)?;
            serializer.skip_field("previousStateId", &self.m_previousStateId)?;
            serializer
                .skip_field(
                    "nextStartStateIndexOverride",
                    &self.m_nextStartStateIndexOverride,
                )?;
            serializer
                .skip_field(
                    "stateOrTransitionChanged",
                    &self.m_stateOrTransitionChanged,
                )?;
            serializer.skip_field("echoNextUpdate", &self.m_echoNextUpdate)?;
            serializer
                .skip_field(
                    "sCurrentStateIndexAndEntered",
                    &self.m_sCurrentStateIndexAndEntered,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("states", &self.m_states)?;
            serializer
                .serialize_array_field("activeTransitions", &self.m_activeTransitions)?;
            serializer
                .serialize_array_field("transitionFlags", &self.m_transitionFlags)?;
            serializer
                .serialize_array_field(
                    "wildcardTransitionFlags",
                    &self.m_wildcardTransitionFlags,
                )?;
            serializer
                .serialize_array_field(
                    "delayedTransitions",
                    &self.m_delayedTransitions,
                )?;
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
pub enum StartStateMode {
    #[default]
    START_STATE_MODE_DEFAULT = 0isize,
    START_STATE_MODE_SYNC = 1isize,
    START_STATE_MODE_RANDOM = 2isize,
    START_STATE_MODE_CHOOSER = 3isize,
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
pub enum StateMachineSelfTransitionMode {
    #[default]
    SELF_TRANSITION_MODE_NO_TRANSITION = 0isize,
    SELF_TRANSITION_MODE_TRANSITION_TO_START_STATE = 1isize,
    SELF_TRANSITION_MODE_FORCE_TRANSITION_TO_START_STATE = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for StartStateMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::START_STATE_MODE_DEFAULT => {
                    __serializer.serialize_field("START_STATE_MODE_DEFAULT", &0u64)
                }
                Self::START_STATE_MODE_SYNC => {
                    __serializer.serialize_field("START_STATE_MODE_SYNC", &1u64)
                }
                Self::START_STATE_MODE_RANDOM => {
                    __serializer.serialize_field("START_STATE_MODE_RANDOM", &2u64)
                }
                Self::START_STATE_MODE_CHOOSER => {
                    __serializer.serialize_field("START_STATE_MODE_CHOOSER", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum StartStateMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for StateMachineSelfTransitionMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::SELF_TRANSITION_MODE_NO_TRANSITION => {
                    __serializer
                        .serialize_field("SELF_TRANSITION_MODE_NO_TRANSITION", &0u64)
                }
                Self::SELF_TRANSITION_MODE_TRANSITION_TO_START_STATE => {
                    __serializer
                        .serialize_field(
                            "SELF_TRANSITION_MODE_TRANSITION_TO_START_STATE",
                            &1u64,
                        )
                }
                Self::SELF_TRANSITION_MODE_FORCE_TRANSITION_TO_START_STATE => {
                    __serializer
                        .serialize_field(
                            "SELF_TRANSITION_MODE_FORCE_TRANSITION_TO_START_STATE",
                            &2u64,
                        )
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(
                    S::Error::custom("Failed enum StateMachineSelfTransitionMode to_i8"),
                )?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
