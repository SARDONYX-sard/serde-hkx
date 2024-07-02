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
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachine"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2171346379u32)
        }
    }
    impl<'a> __serde::Serialize for hkbStateMachine<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2171346379u32)));
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StartStateMode {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("START_STATE_MODE_DEFAULT") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("START_STATE_MODE_SYNC") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("START_STATE_MODE_RANDOM") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("START_STATE_MODE_CHOOSER") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<StartStateMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StartStateMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum StartStateMode",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                StartStateMode::START_STATE_MODE_DEFAULT,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(StartStateMode::START_STATE_MODE_SYNC)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                StartStateMode::START_STATE_MODE_RANDOM,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                StartStateMode::START_STATE_MODE_CHOOSER,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "START_STATE_MODE_DEFAULT",
                "START_STATE_MODE_SYNC",
                "START_STATE_MODE_RANDOM",
                "START_STATE_MODE_CHOOSER",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "StartStateMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<StartStateMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for StateMachineSelfTransitionMode {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SELF_TRANSITION_MODE_NO_TRANSITION",
                                    ) => _serde::__private::Ok(__Field::__field0),
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SELF_TRANSITION_MODE_TRANSITION_TO_START_STATE",
                                    ) => _serde::__private::Ok(__Field::__field1),
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SELF_TRANSITION_MODE_FORCE_TRANSITION_TO_START_STATE",
                                    ) => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<StateMachineSelfTransitionMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = StateMachineSelfTransitionMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum StateMachineSelfTransitionMode",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                StateMachineSelfTransitionMode::SELF_TRANSITION_MODE_NO_TRANSITION,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                StateMachineSelfTransitionMode::SELF_TRANSITION_MODE_TRANSITION_TO_START_STATE,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                StateMachineSelfTransitionMode::SELF_TRANSITION_MODE_FORCE_TRANSITION_TO_START_STATE,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "SELF_TRANSITION_MODE_NO_TRANSITION",
                "SELF_TRANSITION_MODE_TRANSITION_TO_START_STATE",
                "SELF_TRANSITION_MODE_FORCE_TRANSITION_TO_START_STATE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "StateMachineSelfTransitionMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        StateMachineSelfTransitionMode,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
