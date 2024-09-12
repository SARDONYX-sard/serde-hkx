use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbStateMachineInternalState`
/// - version: `0`
/// - signature: `0xbd1a7502`
/// - size: ` 80`(x86)/`104`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineInternalState {
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
    /// - name: `activeTransitions`(ctype: `hkArray<struct hkbStateMachineActiveTransitionInfo>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_activeTransitions: Vec<hkbStateMachineActiveTransitionInfo>,
    /// # C++ Info
    /// - name: `transitionFlags`(ctype: `hkArray<hkUint8>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_transitionFlags: Vec<u8>,
    /// # C++ Info
    /// - name: `wildcardTransitionFlags`(ctype: `hkArray<hkUint8>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_wildcardTransitionFlags: Vec<u8>,
    /// # C++ Info
    /// - name: `delayedTransitions`(ctype: `hkArray<struct hkbStateMachineDelayedTransitionInfo>`)
    /// - offset: ` 44`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_delayedTransitions: Vec<hkbStateMachineDelayedTransitionInfo>,
    /// # C++ Info
    /// - name: `timeInState`(ctype: `hkReal`)
    /// - offset: ` 56`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_timeInState: f32,
    /// # C++ Info
    /// - name: `lastLocalTime`(ctype: `hkReal`)
    /// - offset: ` 60`(x86)/` 84`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_lastLocalTime: f32,
    /// # C++ Info
    /// - name: `currentStateId`(ctype: `hkInt32`)
    /// - offset: ` 64`(x86)/` 88`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_currentStateId: i32,
    /// # C++ Info
    /// - name: `previousStateId`(ctype: `hkInt32`)
    /// - offset: ` 68`(x86)/` 92`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_previousStateId: i32,
    /// # C++ Info
    /// - name: `nextStartStateIndexOverride`(ctype: `hkInt32`)
    /// - offset: ` 72`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_nextStartStateIndexOverride: i32,
    /// # C++ Info
    /// - name: `stateOrTransitionChanged`(ctype: `hkBool`)
    /// - offset: ` 76`(x86)/`100`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_stateOrTransitionChanged: bool,
    /// # C++ Info
    /// - name: `echoNextUpdate`(ctype: `hkBool`)
    /// - offset: ` 77`(x86)/`101`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_echoNextUpdate: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xbd1a7502)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_activeTransitions
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_delayedTransitions
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v
        }
    }
    impl _serde::Serialize for hkbStateMachineInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xbd1a7502)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbStateMachineInternalState",
                    class_meta,
                    (80u64, 104u64),
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "activeTransitions",
                    &self.m_activeTransitions,
                    TypeSize::Struct {
                        size_x86: 32u64,
                        size_x86_64: 40u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "transitionFlags",
                    &self.m_transitionFlags,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "wildcardTransitionFlags",
                    &self.m_wildcardTransitionFlags,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "delayedTransitions",
                    &self.m_delayedTransitions,
                    TypeSize::Struct {
                        size_x86: 24u64,
                        size_x86_64: 24u64,
                    },
                )?;
            serializer.serialize_field("timeInState", &self.m_timeInState)?;
            serializer.serialize_field("lastLocalTime", &self.m_lastLocalTime)?;
            serializer.serialize_field("currentStateId", &self.m_currentStateId)?;
            serializer.serialize_field("previousStateId", &self.m_previousStateId)?;
            serializer
                .serialize_field(
                    "nextStartStateIndexOverride",
                    &self.m_nextStartStateIndexOverride,
                )?;
            serializer
                .serialize_field(
                    "stateOrTransitionChanged",
                    &self.m_stateOrTransitionChanged,
                )?;
            serializer.serialize_field("echoNextUpdate", &self.m_echoNextUpdate)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbStateMachineInternalState {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_activeTransitions,
                m_transitionFlags,
                m_wildcardTransitionFlags,
                m_delayedTransitions,
                m_timeInState,
                m_lastLocalTime,
                m_currentStateId,
                m_previousStateId,
                m_nextStartStateIndexOverride,
                m_stateOrTransitionChanged,
                m_echoNextUpdate,
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
                        "activeTransitions" => Ok(__Field::m_activeTransitions),
                        "transitionFlags" => Ok(__Field::m_transitionFlags),
                        "wildcardTransitionFlags" => {
                            Ok(__Field::m_wildcardTransitionFlags)
                        }
                        "delayedTransitions" => Ok(__Field::m_delayedTransitions),
                        "timeInState" => Ok(__Field::m_timeInState),
                        "lastLocalTime" => Ok(__Field::m_lastLocalTime),
                        "currentStateId" => Ok(__Field::m_currentStateId),
                        "previousStateId" => Ok(__Field::m_previousStateId),
                        "nextStartStateIndexOverride" => {
                            Ok(__Field::m_nextStartStateIndexOverride)
                        }
                        "stateOrTransitionChanged" => {
                            Ok(__Field::m_stateOrTransitionChanged)
                        }
                        "echoNextUpdate" => Ok(__Field::m_echoNextUpdate),
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
            struct __hkbStateMachineInternalStateVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbStateMachineInternalState>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbStateMachineInternalStateVisitor<'de> {
                type Value = hkbStateMachineInternalState;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbStateMachineInternalState",
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
                    let mut m_activeTransitions: _serde::__private::Option<
                        Vec<hkbStateMachineActiveTransitionInfo>,
                    > = _serde::__private::None;
                    let mut m_transitionFlags: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    let mut m_wildcardTransitionFlags: _serde::__private::Option<
                        Vec<u8>,
                    > = _serde::__private::None;
                    let mut m_delayedTransitions: _serde::__private::Option<
                        Vec<hkbStateMachineDelayedTransitionInfo>,
                    > = _serde::__private::None;
                    let mut m_timeInState: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lastLocalTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_currentStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_previousStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_nextStartStateIndexOverride: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_stateOrTransitionChanged: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_echoNextUpdate: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
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
                                    match __A::next_value::<
                                        Vec<hkbStateMachineActiveTransitionInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_transitionFlags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionFlags",
                                        ),
                                    );
                                }
                                m_transitionFlags = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
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
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
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
                                    match __A::next_value::<
                                        Vec<hkbStateMachineDelayedTransitionInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
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
                            5usize => {
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
                            6usize => {
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
                            7usize => {
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
                            8usize => {
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
                            9usize => {
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
                            10usize => {
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
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 2usize, 2usize)?;
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
                    _serde::__private::Ok(hkbStateMachineInternalState {
                        __ptr,
                        parent,
                        m_activeTransitions,
                        m_transitionFlags,
                        m_wildcardTransitionFlags,
                        m_delayedTransitions,
                        m_timeInState,
                        m_lastLocalTime,
                        m_currentStateId,
                        m_previousStateId,
                        m_nextStartStateIndexOverride,
                        m_stateOrTransitionChanged,
                        m_echoNextUpdate,
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
                    let mut m_activeTransitions: _serde::__private::Option<
                        Vec<hkbStateMachineActiveTransitionInfo>,
                    > = _serde::__private::None;
                    let mut m_transitionFlags: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    let mut m_wildcardTransitionFlags: _serde::__private::Option<
                        Vec<u8>,
                    > = _serde::__private::None;
                    let mut m_delayedTransitions: _serde::__private::Option<
                        Vec<hkbStateMachineDelayedTransitionInfo>,
                    > = _serde::__private::None;
                    let mut m_timeInState: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lastLocalTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_currentStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_previousStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_nextStartStateIndexOverride: _serde::__private::Option<
                        i32,
                    > = _serde::__private::None;
                    let mut m_stateOrTransitionChanged: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_echoNextUpdate: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_activeTransitions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_activeTransitions,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "activeTransitions",
                                        ),
                                    );
                                }
                                m_activeTransitions = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbStateMachineActiveTransitionInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transitionFlags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transitionFlags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionFlags",
                                        ),
                                    );
                                }
                                m_transitionFlags = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wildcardTransitionFlags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wildcardTransitionFlags,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wildcardTransitionFlags",
                                        ),
                                    );
                                }
                                m_wildcardTransitionFlags = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_delayedTransitions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_delayedTransitions,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "delayedTransitions",
                                        ),
                                    );
                                }
                                m_delayedTransitions = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbStateMachineDelayedTransitionInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_timeInState => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_timeInState) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_lastLocalTime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lastLocalTime) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_currentStateId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_currentStateId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_previousStateId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_previousStateId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_nextStartStateIndexOverride => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nextStartStateIndexOverride,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_stateOrTransitionChanged => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_stateOrTransitionChanged,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_echoNextUpdate => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_echoNextUpdate) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_activeTransitions = match m_activeTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "activeTransitions",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transitionFlags = match m_transitionFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionFlags",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wildcardTransitionFlags = match m_wildcardTransitionFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wildcardTransitionFlags",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_delayedTransitions = match m_delayedTransitions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "delayedTransitions",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_timeInState = match m_timeInState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeInState",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lastLocalTime = match m_lastLocalTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastLocalTime",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_currentStateId = match m_currentStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentStateId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_previousStateId = match m_previousStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "previousStateId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nextStartStateIndexOverride = match m_nextStartStateIndexOverride {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextStartStateIndexOverride",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stateOrTransitionChanged = match m_stateOrTransitionChanged {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stateOrTransitionChanged",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_echoNextUpdate = match m_echoNextUpdate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "echoNextUpdate",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbStateMachineInternalState {
                        __ptr,
                        parent,
                        m_activeTransitions,
                        m_transitionFlags,
                        m_wildcardTransitionFlags,
                        m_delayedTransitions,
                        m_timeInState,
                        m_lastLocalTime,
                        m_currentStateId,
                        m_previousStateId,
                        m_nextStartStateIndexOverride,
                        m_stateOrTransitionChanged,
                        m_echoNextUpdate,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "activeTransitions",
                "transitionFlags",
                "wildcardTransitionFlags",
                "delayedTransitions",
                "timeInState",
                "lastLocalTime",
                "currentStateId",
                "previousStateId",
                "nextStartStateIndexOverride",
                "stateOrTransitionChanged",
                "echoNextUpdate",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbStateMachineInternalState",
                FIELDS,
                __hkbStateMachineInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbStateMachineInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
