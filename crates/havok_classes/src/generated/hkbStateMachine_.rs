use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbStateMachine`
/// - version: `4`
/// - signature: `0x816c1dcb`
/// - size: `180`(x86)/`264`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// - name: `eventToSendWhenStateOrTransitionChanges`(ctype: `struct hkbEvent`)
    /// - offset: ` 40`(x86)/` 72`(x86_64)
    /// - type_size: ` 12`(x86)/` 24`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "eventToSendWhenStateOrTransitionChanges")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "eventToSendWhenStateOrTransitionChanges")
    )]
    pub m_eventToSendWhenStateOrTransitionChanges: hkbEvent,
    /// # C++ Info
    /// - name: `startStateChooser`(ctype: `struct hkbStateChooser*`)
    /// - offset: ` 52`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "startStateChooser"))]
    #[cfg_attr(feature = "serde", serde(rename = "startStateChooser"))]
    pub m_startStateChooser: Pointer,
    /// # C++ Info
    /// - name: `startStateId`(ctype: `hkInt32`)
    /// - offset: ` 56`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "startStateId"))]
    #[cfg_attr(feature = "serde", serde(rename = "startStateId"))]
    pub m_startStateId: i32,
    /// # C++ Info
    /// - name: `returnToPreviousStateEventId`(ctype: `hkInt32`)
    /// - offset: ` 60`(x86)/`108`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "returnToPreviousStateEventId")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "returnToPreviousStateEventId"))]
    pub m_returnToPreviousStateEventId: i32,
    /// # C++ Info
    /// - name: `randomTransitionEventId`(ctype: `hkInt32`)
    /// - offset: ` 64`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "randomTransitionEventId"))]
    #[cfg_attr(feature = "serde", serde(rename = "randomTransitionEventId"))]
    pub m_randomTransitionEventId: i32,
    /// # C++ Info
    /// - name: `transitionToNextHigherStateEventId`(ctype: `hkInt32`)
    /// - offset: ` 68`(x86)/`116`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "transitionToNextHigherStateEventId")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "transitionToNextHigherStateEventId"))]
    pub m_transitionToNextHigherStateEventId: i32,
    /// # C++ Info
    /// - name: `transitionToNextLowerStateEventId`(ctype: `hkInt32`)
    /// - offset: ` 72`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "transitionToNextLowerStateEventId")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "transitionToNextLowerStateEventId"))]
    pub m_transitionToNextLowerStateEventId: i32,
    /// # C++ Info
    /// - name: `syncVariableIndex`(ctype: `hkInt32`)
    /// - offset: ` 76`(x86)/`124`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "syncVariableIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "syncVariableIndex"))]
    pub m_syncVariableIndex: i32,
    /// # C++ Info
    /// - name: `currentStateId`(ctype: `hkInt32`)
    /// - offset: ` 80`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentStateId"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentStateId"))]
    pub m_currentStateId: i32,
    /// # C++ Info
    /// - name: `wrapAroundStateId`(ctype: `hkBool`)
    /// - offset: ` 84`(x86)/`132`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "wrapAroundStateId"))]
    #[cfg_attr(feature = "serde", serde(rename = "wrapAroundStateId"))]
    pub m_wrapAroundStateId: bool,
    /// # C++ Info
    /// - name: `maxSimultaneousTransitions`(ctype: `hkInt8`)
    /// - offset: ` 85`(x86)/`133`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxSimultaneousTransitions"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxSimultaneousTransitions"))]
    pub m_maxSimultaneousTransitions: i8,
    /// # C++ Info
    /// - name: `startStateMode`(ctype: `enum StartStateMode`)
    /// - offset: ` 86`(x86)/`134`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "startStateMode"))]
    #[cfg_attr(feature = "serde", serde(rename = "startStateMode"))]
    pub m_startStateMode: StartStateMode,
    /// # C++ Info
    /// - name: `selfTransitionMode`(ctype: `enum StateMachineSelfTransitionMode`)
    /// - offset: ` 87`(x86)/`135`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "selfTransitionMode"))]
    #[cfg_attr(feature = "serde", serde(rename = "selfTransitionMode"))]
    pub m_selfTransitionMode: StateMachineSelfTransitionMode,
    /// # C++ Info
    /// - name: `isActive`(ctype: `hkBool`)
    /// - offset: ` 88`(x86)/`136`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "isActive"))]
    #[cfg_attr(feature = "serde", serde(rename = "isActive"))]
    pub m_isActive: bool,
    /// # C++ Info
    /// - name: `states`(ctype: `hkArray<hkbStateMachineStateInfo*>`)
    /// - offset: ` 92`(x86)/`144`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "states"))]
    #[cfg_attr(feature = "serde", serde(rename = "states"))]
    pub m_states: Vec<Pointer>,
    /// # C++ Info
    /// - name: `wildcardTransitions`(ctype: `struct hkbStateMachineTransitionInfoArray*`)
    /// - offset: `104`(x86)/`160`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "wildcardTransitions"))]
    #[cfg_attr(feature = "serde", serde(rename = "wildcardTransitions"))]
    pub m_wildcardTransitions: Pointer,
    /// # C++ Info
    /// - name: `stateIdToIndexMap`(ctype: `void*`)
    /// - offset: `108`(x86)/`168`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "stateIdToIndexMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "stateIdToIndexMap"))]
    pub m_stateIdToIndexMap: Pointer,
    /// # C++ Info
    /// - name: `activeTransitions`(ctype: `hkArray<void>`)
    /// - offset: `112`(x86)/`176`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "activeTransitions"))]
    #[cfg_attr(feature = "serde", serde(rename = "activeTransitions"))]
    pub m_activeTransitions: Vec<()>,
    /// # C++ Info
    /// - name: `transitionFlags`(ctype: `hkArray<void>`)
    /// - offset: `124`(x86)/`192`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "transitionFlags"))]
    #[cfg_attr(feature = "serde", serde(rename = "transitionFlags"))]
    pub m_transitionFlags: Vec<()>,
    /// # C++ Info
    /// - name: `wildcardTransitionFlags`(ctype: `hkArray<void>`)
    /// - offset: `136`(x86)/`208`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "wildcardTransitionFlags"))]
    #[cfg_attr(feature = "serde", serde(rename = "wildcardTransitionFlags"))]
    pub m_wildcardTransitionFlags: Vec<()>,
    /// # C++ Info
    /// - name: `delayedTransitions`(ctype: `hkArray<void>`)
    /// - offset: `148`(x86)/`224`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "delayedTransitions"))]
    #[cfg_attr(feature = "serde", serde(rename = "delayedTransitions"))]
    pub m_delayedTransitions: Vec<()>,
    /// # C++ Info
    /// - name: `timeInState`(ctype: `hkReal`)
    /// - offset: `160`(x86)/`240`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeInState"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeInState"))]
    pub m_timeInState: f32,
    /// # C++ Info
    /// - name: `lastLocalTime`(ctype: `hkReal`)
    /// - offset: `164`(x86)/`244`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "lastLocalTime"))]
    #[cfg_attr(feature = "serde", serde(rename = "lastLocalTime"))]
    pub m_lastLocalTime: f32,
    /// # C++ Info
    /// - name: `previousStateId`(ctype: `hkInt32`)
    /// - offset: `168`(x86)/`248`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "previousStateId"))]
    #[cfg_attr(feature = "serde", serde(rename = "previousStateId"))]
    pub m_previousStateId: i32,
    /// # C++ Info
    /// - name: `nextStartStateIndexOverride`(ctype: `hkInt32`)
    /// - offset: `172`(x86)/`252`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "nextStartStateIndexOverride")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "nextStartStateIndexOverride"))]
    pub m_nextStartStateIndexOverride: i32,
    /// # C++ Info
    /// - name: `stateOrTransitionChanged`(ctype: `hkBool`)
    /// - offset: `176`(x86)/`256`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "stateOrTransitionChanged"))]
    #[cfg_attr(feature = "serde", serde(rename = "stateOrTransitionChanged"))]
    pub m_stateOrTransitionChanged: bool,
    /// # C++ Info
    /// - name: `echoNextUpdate`(ctype: `hkBool`)
    /// - offset: `177`(x86)/`257`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "echoNextUpdate"))]
    #[cfg_attr(feature = "serde", serde(rename = "echoNextUpdate"))]
    pub m_echoNextUpdate: bool,
    /// # C++ Info
    /// - name: `sCurrentStateIndexAndEntered`(ctype: `hkUint16`)
    /// - offset: `178`(x86)/`258`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "sCurrentStateIndexAndEntered")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "sCurrentStateIndexAndEntered"))]
    pub m_sCurrentStateIndexAndEntered: u16,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbStateMachine<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachine"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x816c1dcb)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v.extend(self.m_eventToSendWhenStateOrTransitionChanges.deps_indexes());
            v.push(self.m_startStateChooser.get());
            v.extend(self.m_states.iter().map(|ptr| ptr.get()));
            v.push(self.m_wildcardTransitions.get());
            v.push(self.m_stateIdToIndexMap.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbStateMachine<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x816c1dcb)));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachine", class_meta, (180u64, 264u64))?;
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
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer
                .skip_fixed_array_field(
                    "padNode",
                    self.parent.parent.m_padNode.as_slice(),
                    TypeSize::NonPtr,
                )?;
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
            serializer
                .serialize_array_field("states", &self.m_states, TypeSize::NonPtr)?;
            serializer
                .serialize_field("wildcardTransitions", &self.m_wildcardTransitions)?;
            serializer.skip_field("stateIdToIndexMap", &self.m_stateIdToIndexMap)?;
            serializer
                .skip_array_field(
                    "activeTransitions",
                    &self.m_activeTransitions,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "transitionFlags",
                    &self.m_transitionFlags,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "wildcardTransitionFlags",
                    &self.m_wildcardTransitionFlags,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "delayedTransitions",
                    &self.m_delayedTransitions,
                    TypeSize::NonPtr,
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
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbStateMachine<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_eventToSendWhenStateOrTransitionChanges,
                m_startStateChooser,
                m_startStateId,
                m_returnToPreviousStateEventId,
                m_randomTransitionEventId,
                m_transitionToNextHigherStateEventId,
                m_transitionToNextLowerStateEventId,
                m_syncVariableIndex,
                m_wrapAroundStateId,
                m_maxSimultaneousTransitions,
                m_startStateMode,
                m_selfTransitionMode,
                m_states,
                m_wildcardTransitions,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "eventToSendWhenStateOrTransitionChanges" => {
                            Ok(__Field::m_eventToSendWhenStateOrTransitionChanges)
                        }
                        "startStateChooser" => Ok(__Field::m_startStateChooser),
                        "startStateId" => Ok(__Field::m_startStateId),
                        "returnToPreviousStateEventId" => {
                            Ok(__Field::m_returnToPreviousStateEventId)
                        }
                        "randomTransitionEventId" => {
                            Ok(__Field::m_randomTransitionEventId)
                        }
                        "transitionToNextHigherStateEventId" => {
                            Ok(__Field::m_transitionToNextHigherStateEventId)
                        }
                        "transitionToNextLowerStateEventId" => {
                            Ok(__Field::m_transitionToNextLowerStateEventId)
                        }
                        "syncVariableIndex" => Ok(__Field::m_syncVariableIndex),
                        "wrapAroundStateId" => Ok(__Field::m_wrapAroundStateId),
                        "maxSimultaneousTransitions" => {
                            Ok(__Field::m_maxSimultaneousTransitions)
                        }
                        "startStateMode" => Ok(__Field::m_startStateMode),
                        "selfTransitionMode" => Ok(__Field::m_selfTransitionMode),
                        "states" => Ok(__Field::m_states),
                        "wildcardTransitions" => Ok(__Field::m_wildcardTransitions),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkbStateMachineVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbStateMachine<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbStateMachineVisitor<'de> {
                type Value = hkbStateMachine<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbStateMachine",
                    )
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_eventToSendWhenStateOrTransitionChanges: _serde::__private::Option<
                        hkbEvent,
                    > = _serde::__private::None;
                    let mut m_startStateChooser: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_startStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_returnToPreviousStateEventId: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_randomTransitionEventId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_transitionToNextHigherStateEventId: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_transitionToNextLowerStateEventId: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_syncVariableIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_currentStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_wrapAroundStateId: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_maxSimultaneousTransitions: _serde::__private::Option<
                        i8,
                    > = _serde::__private::None;
                    let mut m_startStateMode: _serde::__private::Option<
                        StartStateMode,
                    > = _serde::__private::None;
                    let mut m_selfTransitionMode: _serde::__private::Option<
                        StateMachineSelfTransitionMode,
                    > = _serde::__private::None;
                    let mut m_isActive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_states: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_wildcardTransitions: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_stateIdToIndexMap: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_activeTransitions: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_transitionFlags: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_wildcardTransitionFlags: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    let mut m_delayedTransitions: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_timeInState: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lastLocalTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_previousStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_nextStartStateIndexOverride: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_stateOrTransitionChanged: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_echoNextUpdate: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_sCurrentStateIndexAndEntered: _serde::__private::Option<
                        u16,
                    > = _serde::__private::None;
                    for i in 0..28usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_eventToSendWhenStateOrTransitionChanges,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventToSendWhenStateOrTransitionChanges",
                                        ),
                                    );
                                }
                                m_eventToSendWhenStateOrTransitionChanges = _serde::__private::Some(
                                    match __A::next_value::<hkbEvent>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_startStateChooser,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startStateChooser",
                                        ),
                                    );
                                }
                                m_startStateChooser = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_startStateId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startStateId",
                                        ),
                                    );
                                }
                                m_startStateId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_returnToPreviousStateEventId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "returnToPreviousStateEventId",
                                        ),
                                    );
                                }
                                m_returnToPreviousStateEventId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_randomTransitionEventId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "randomTransitionEventId",
                                        ),
                                    );
                                }
                                m_randomTransitionEventId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transitionToNextHigherStateEventId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionToNextHigherStateEventId",
                                        ),
                                    );
                                }
                                m_transitionToNextHigherStateEventId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transitionToNextLowerStateEventId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionToNextLowerStateEventId",
                                        ),
                                    );
                                }
                                m_transitionToNextLowerStateEventId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_syncVariableIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "syncVariableIndex",
                                        ),
                                    );
                                }
                                m_syncVariableIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_currentStateId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentStateId",
                                        ),
                                    );
                                }
                                m_currentStateId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wrapAroundStateId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wrapAroundStateId",
                                        ),
                                    );
                                }
                                m_wrapAroundStateId = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxSimultaneousTransitions,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxSimultaneousTransitions",
                                        ),
                                    );
                                }
                                m_maxSimultaneousTransitions = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_startStateMode) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startStateMode",
                                        ),
                                    );
                                }
                                m_startStateMode = _serde::__private::Some(
                                    match __A::next_value::<StartStateMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_selfTransitionMode,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "selfTransitionMode",
                                        ),
                                    );
                                }
                                m_selfTransitionMode = _serde::__private::Some(
                                    match __A::next_value::<
                                        StateMachineSelfTransitionMode,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_isActive) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isActive",
                                        ),
                                    );
                                }
                                m_isActive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_states) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("states"),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 7usize)?;
                                m_states = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wildcardTransitions,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wildcardTransitions",
                                        ),
                                    );
                                }
                                m_wildcardTransitions = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_stateIdToIndexMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stateIdToIndexMap",
                                        ),
                                    );
                                }
                                m_stateIdToIndexMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_activeTransitions,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "activeTransitions",
                                        ),
                                    );
                                }
                                m_activeTransitions = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_transitionFlags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionFlags",
                                        ),
                                    );
                                }
                                m_transitionFlags = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wildcardTransitionFlags,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wildcardTransitionFlags",
                                        ),
                                    );
                                }
                                m_wildcardTransitionFlags = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_delayedTransitions,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "delayedTransitions",
                                        ),
                                    );
                                }
                                m_delayedTransitions = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(&m_timeInState) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeInState",
                                        ),
                                    );
                                }
                                m_timeInState = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(&m_lastLocalTime) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastLocalTime",
                                        ),
                                    );
                                }
                                m_lastLocalTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(&m_previousStateId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "previousStateId",
                                        ),
                                    );
                                }
                                m_previousStateId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_nextStartStateIndexOverride,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextStartStateIndexOverride",
                                        ),
                                    );
                                }
                                m_nextStartStateIndexOverride = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            25usize => {
                                if _serde::__private::Option::is_some(
                                    &m_stateOrTransitionChanged,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stateOrTransitionChanged",
                                        ),
                                    );
                                }
                                m_stateOrTransitionChanged = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(&m_echoNextUpdate) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "echoNextUpdate",
                                        ),
                                    );
                                }
                                m_echoNextUpdate = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sCurrentStateIndexAndEntered,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sCurrentStateIndexAndEntered",
                                        ),
                                    );
                                }
                                m_sCurrentStateIndexAndEntered = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    let m_eventToSendWhenStateOrTransitionChanges = match m_eventToSendWhenStateOrTransitionChanges {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventToSendWhenStateOrTransitionChanges",
                                ),
                            );
                        }
                    };
                    let m_startStateChooser = match m_startStateChooser {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startStateChooser",
                                ),
                            );
                        }
                    };
                    let m_startStateId = match m_startStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startStateId",
                                ),
                            );
                        }
                    };
                    let m_returnToPreviousStateEventId = match m_returnToPreviousStateEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "returnToPreviousStateEventId",
                                ),
                            );
                        }
                    };
                    let m_randomTransitionEventId = match m_randomTransitionEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "randomTransitionEventId",
                                ),
                            );
                        }
                    };
                    let m_transitionToNextHigherStateEventId = match m_transitionToNextHigherStateEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionToNextHigherStateEventId",
                                ),
                            );
                        }
                    };
                    let m_transitionToNextLowerStateEventId = match m_transitionToNextLowerStateEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionToNextLowerStateEventId",
                                ),
                            );
                        }
                    };
                    let m_syncVariableIndex = match m_syncVariableIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "syncVariableIndex",
                                ),
                            );
                        }
                    };
                    let m_currentStateId = match m_currentStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentStateId",
                                ),
                            );
                        }
                    };
                    let m_wrapAroundStateId = match m_wrapAroundStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wrapAroundStateId",
                                ),
                            );
                        }
                    };
                    let m_maxSimultaneousTransitions = match m_maxSimultaneousTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxSimultaneousTransitions",
                                ),
                            );
                        }
                    };
                    let m_startStateMode = match m_startStateMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startStateMode",
                                ),
                            );
                        }
                    };
                    let m_selfTransitionMode = match m_selfTransitionMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "selfTransitionMode",
                                ),
                            );
                        }
                    };
                    let m_isActive = match m_isActive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isActive"),
                            );
                        }
                    };
                    let m_states = match m_states {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("states"),
                            );
                        }
                    };
                    let m_wildcardTransitions = match m_wildcardTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wildcardTransitions",
                                ),
                            );
                        }
                    };
                    let m_stateIdToIndexMap = match m_stateIdToIndexMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stateIdToIndexMap",
                                ),
                            );
                        }
                    };
                    let m_activeTransitions = match m_activeTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "activeTransitions",
                                ),
                            );
                        }
                    };
                    let m_transitionFlags = match m_transitionFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionFlags",
                                ),
                            );
                        }
                    };
                    let m_wildcardTransitionFlags = match m_wildcardTransitionFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wildcardTransitionFlags",
                                ),
                            );
                        }
                    };
                    let m_delayedTransitions = match m_delayedTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "delayedTransitions",
                                ),
                            );
                        }
                    };
                    let m_timeInState = match m_timeInState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeInState",
                                ),
                            );
                        }
                    };
                    let m_lastLocalTime = match m_lastLocalTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastLocalTime",
                                ),
                            );
                        }
                    };
                    let m_previousStateId = match m_previousStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "previousStateId",
                                ),
                            );
                        }
                    };
                    let m_nextStartStateIndexOverride = match m_nextStartStateIndexOverride {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextStartStateIndexOverride",
                                ),
                            );
                        }
                    };
                    let m_stateOrTransitionChanged = match m_stateOrTransitionChanged {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stateOrTransitionChanged",
                                ),
                            );
                        }
                    };
                    let m_echoNextUpdate = match m_echoNextUpdate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "echoNextUpdate",
                                ),
                            );
                        }
                    };
                    let m_sCurrentStateIndexAndEntered = match m_sCurrentStateIndexAndEntered {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sCurrentStateIndexAndEntered",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbStateMachine {
                        __ptr,
                        parent,
                        m_eventToSendWhenStateOrTransitionChanges,
                        m_startStateChooser,
                        m_startStateId,
                        m_returnToPreviousStateEventId,
                        m_randomTransitionEventId,
                        m_transitionToNextHigherStateEventId,
                        m_transitionToNextLowerStateEventId,
                        m_syncVariableIndex,
                        m_currentStateId,
                        m_wrapAroundStateId,
                        m_maxSimultaneousTransitions,
                        m_startStateMode,
                        m_selfTransitionMode,
                        m_isActive,
                        m_states,
                        m_wildcardTransitions,
                        m_stateIdToIndexMap,
                        m_activeTransitions,
                        m_transitionFlags,
                        m_wildcardTransitionFlags,
                        m_delayedTransitions,
                        m_timeInState,
                        m_lastLocalTime,
                        m_previousStateId,
                        m_nextStartStateIndexOverride,
                        m_stateOrTransitionChanged,
                        m_echoNextUpdate,
                        m_sCurrentStateIndexAndEntered,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_variableBindingSet: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_eventToSendWhenStateOrTransitionChanges: _serde::__private::Option<
                        hkbEvent,
                    > = _serde::__private::None;
                    let mut m_startStateChooser: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_startStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_returnToPreviousStateEventId: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_randomTransitionEventId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_transitionToNextHigherStateEventId: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_transitionToNextLowerStateEventId: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_syncVariableIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_wrapAroundStateId: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_maxSimultaneousTransitions: _serde::__private::Option<
                        i8,
                    > = _serde::__private::None;
                    let mut m_startStateMode: _serde::__private::Option<
                        StartStateMode,
                    > = _serde::__private::None;
                    let mut m_selfTransitionMode: _serde::__private::Option<
                        StateMachineSelfTransitionMode,
                    > = _serde::__private::None;
                    let mut m_states: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_wildcardTransitions: _serde::__private::Option<Pointer> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_eventToSendWhenStateOrTransitionChanges => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_eventToSendWhenStateOrTransitionChanges,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventToSendWhenStateOrTransitionChanges",
                                        ),
                                    );
                                }
                                m_eventToSendWhenStateOrTransitionChanges = _serde::__private::Some(
                                    match __A::next_value::<hkbEvent>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_startStateChooser => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_startStateChooser,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startStateChooser",
                                        ),
                                    );
                                }
                                m_startStateChooser = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_startStateId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_startStateId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startStateId",
                                        ),
                                    );
                                }
                                m_startStateId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_returnToPreviousStateEventId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_returnToPreviousStateEventId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "returnToPreviousStateEventId",
                                        ),
                                    );
                                }
                                m_returnToPreviousStateEventId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_randomTransitionEventId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_randomTransitionEventId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "randomTransitionEventId",
                                        ),
                                    );
                                }
                                m_randomTransitionEventId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transitionToNextHigherStateEventId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transitionToNextHigherStateEventId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionToNextHigherStateEventId",
                                        ),
                                    );
                                }
                                m_transitionToNextHigherStateEventId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transitionToNextLowerStateEventId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transitionToNextLowerStateEventId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionToNextLowerStateEventId",
                                        ),
                                    );
                                }
                                m_transitionToNextLowerStateEventId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_syncVariableIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_syncVariableIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "syncVariableIndex",
                                        ),
                                    );
                                }
                                m_syncVariableIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wrapAroundStateId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wrapAroundStateId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wrapAroundStateId",
                                        ),
                                    );
                                }
                                m_wrapAroundStateId = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxSimultaneousTransitions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxSimultaneousTransitions,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxSimultaneousTransitions",
                                        ),
                                    );
                                }
                                m_maxSimultaneousTransitions = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_startStateMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_startStateMode) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startStateMode",
                                        ),
                                    );
                                }
                                m_startStateMode = _serde::__private::Some(
                                    match __A::next_value::<StartStateMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_selfTransitionMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_selfTransitionMode,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "selfTransitionMode",
                                        ),
                                    );
                                }
                                m_selfTransitionMode = _serde::__private::Some(
                                    match __A::next_value::<
                                        StateMachineSelfTransitionMode,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_states => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_states) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("states"),
                                    );
                                }
                                m_states = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wildcardTransitions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wildcardTransitions,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wildcardTransitions",
                                        ),
                                    );
                                }
                                m_wildcardTransitions = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_eventToSendWhenStateOrTransitionChanges = match m_eventToSendWhenStateOrTransitionChanges {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventToSendWhenStateOrTransitionChanges",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_startStateChooser = match m_startStateChooser {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startStateChooser",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_startStateId = match m_startStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startStateId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_returnToPreviousStateEventId = match m_returnToPreviousStateEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "returnToPreviousStateEventId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_randomTransitionEventId = match m_randomTransitionEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "randomTransitionEventId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transitionToNextHigherStateEventId = match m_transitionToNextHigherStateEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionToNextHigherStateEventId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transitionToNextLowerStateEventId = match m_transitionToNextLowerStateEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionToNextLowerStateEventId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_syncVariableIndex = match m_syncVariableIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "syncVariableIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wrapAroundStateId = match m_wrapAroundStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wrapAroundStateId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxSimultaneousTransitions = match m_maxSimultaneousTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxSimultaneousTransitions",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_startStateMode = match m_startStateMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startStateMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_selfTransitionMode = match m_selfTransitionMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "selfTransitionMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_states = match m_states {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("states"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wildcardTransitions = match m_wildcardTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wildcardTransitions",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkbBindable {
                        __ptr,
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr,
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbGenerator { __ptr, parent };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbStateMachine {
                        __ptr,
                        parent,
                        m_eventToSendWhenStateOrTransitionChanges,
                        m_startStateChooser,
                        m_startStateId,
                        m_returnToPreviousStateEventId,
                        m_randomTransitionEventId,
                        m_transitionToNextHigherStateEventId,
                        m_transitionToNextLowerStateEventId,
                        m_syncVariableIndex,
                        m_wrapAroundStateId,
                        m_maxSimultaneousTransitions,
                        m_startStateMode,
                        m_selfTransitionMode,
                        m_states,
                        m_wildcardTransitions,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "eventToSendWhenStateOrTransitionChanges",
                "startStateChooser",
                "startStateId",
                "returnToPreviousStateEventId",
                "randomTransitionEventId",
                "transitionToNextHigherStateEventId",
                "transitionToNextLowerStateEventId",
                "syncVariableIndex",
                "currentStateId",
                "wrapAroundStateId",
                "maxSimultaneousTransitions",
                "startStateMode",
                "selfTransitionMode",
                "isActive",
                "states",
                "wildcardTransitions",
                "stateIdToIndexMap",
                "activeTransitions",
                "transitionFlags",
                "wildcardTransitionFlags",
                "delayedTransitions",
                "timeInState",
                "lastLocalTime",
                "previousStateId",
                "nextStartStateIndexOverride",
                "stateOrTransitionChanged",
                "echoNextUpdate",
                "sCurrentStateIndexAndEntered",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbStateMachine",
                FIELDS,
                __hkbStateMachineVisitor {
                    marker: _serde::__private::PhantomData::<hkbStateMachine>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `StartStateMode`(ctype: `hkEnum<StartStateMode, hkInt8>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
/// # C++ Info
/// - name: `StateMachineSelfTransitionMode`(ctype: `hkEnum<StateMachineSelfTransitionMode, hkInt8>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
