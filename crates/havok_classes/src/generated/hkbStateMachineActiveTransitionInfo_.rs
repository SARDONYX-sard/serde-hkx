use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbStateMachineActiveTransitionInfo`
/// - version: `1`
/// - signature: `0xbb90d54f`
/// - size: ` 32`(x86)/` 40`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineActiveTransitionInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `transitionEffect`(ctype: `void*`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_transitionEffect: Pointer,
    /// # C++ Info
    /// - name: `transitionEffectInternalStateInfo`(ctype: `struct hkbNodeInternalStateInfo*`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_transitionEffectInternalStateInfo: Pointer,
    /// # C++ Info
    /// - name: `transitionInfoReference`(ctype: `struct hkbStateMachineTransitionInfoReference`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  6`(x86)/`  6`(x86_64)
    pub m_transitionInfoReference: hkbStateMachineTransitionInfoReference,
    /// # C++ Info
    /// - name: `transitionInfoReferenceForTE`(ctype: `struct hkbStateMachineTransitionInfoReference`)
    /// - offset: ` 14`(x86)/` 22`(x86_64)
    /// - type_size: `  6`(x86)/`  6`(x86_64)
    pub m_transitionInfoReferenceForTE: hkbStateMachineTransitionInfoReference,
    /// # C++ Info
    /// - name: `fromStateId`(ctype: `hkInt32`)
    /// - offset: ` 20`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_fromStateId: i32,
    /// # C++ Info
    /// - name: `toStateId`(ctype: `hkInt32`)
    /// - offset: ` 24`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_toStateId: i32,
    /// # C++ Info
    /// - name: `isReturnToPreviousState`(ctype: `hkBool`)
    /// - offset: ` 28`(x86)/` 36`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isReturnToPreviousState: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineActiveTransitionInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineActiveTransitionInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xbb90d54f)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_transitionEffect.get());
            v.push(self.m_transitionEffectInternalStateInfo.get());
            v.extend(self.m_transitionInfoReference.deps_indexes());
            v.extend(self.m_transitionInfoReferenceForTE.deps_indexes());
            v
        }
    }
    impl _serde::Serialize for hkbStateMachineActiveTransitionInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xbb90d54f)));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineActiveTransitionInfo", class_meta)?;
            serializer.skip_field("transitionEffect", &self.m_transitionEffect)?;
            serializer
                .serialize_field(
                    "transitionEffectInternalStateInfo",
                    &self.m_transitionEffectInternalStateInfo,
                )?;
            serializer
                .serialize_field(
                    "transitionInfoReference",
                    &self.m_transitionInfoReference,
                )?;
            serializer
                .serialize_field(
                    "transitionInfoReferenceForTE",
                    &self.m_transitionInfoReferenceForTE,
                )?;
            serializer.serialize_field("fromStateId", &self.m_fromStateId)?;
            serializer.serialize_field("toStateId", &self.m_toStateId)?;
            serializer
                .serialize_field(
                    "isReturnToPreviousState",
                    &self.m_isReturnToPreviousState,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbStateMachineActiveTransitionInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_transitionEffectInternalStateInfo,
                m_transitionInfoReference,
                m_transitionInfoReferenceForTE,
                m_fromStateId,
                m_toStateId,
                m_isReturnToPreviousState,
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
                        "transitionEffectInternalStateInfo" => {
                            Ok(__Field::m_transitionEffectInternalStateInfo)
                        }
                        "transitionInfoReference" => {
                            Ok(__Field::m_transitionInfoReference)
                        }
                        "transitionInfoReferenceForTE" => {
                            Ok(__Field::m_transitionInfoReferenceForTE)
                        }
                        "fromStateId" => Ok(__Field::m_fromStateId),
                        "toStateId" => Ok(__Field::m_toStateId),
                        "isReturnToPreviousState" => {
                            Ok(__Field::m_isReturnToPreviousState)
                        }
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
            struct __hkbStateMachineActiveTransitionInfoVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbStateMachineActiveTransitionInfo,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbStateMachineActiveTransitionInfoVisitor<'de> {
                type Value = hkbStateMachineActiveTransitionInfo;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbStateMachineActiveTransitionInfo",
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
                    let mut m_transitionEffect: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_transitionEffectInternalStateInfo: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_transitionInfoReference: _serde::__private::Option<
                        hkbStateMachineTransitionInfoReference,
                    > = _serde::__private::None;
                    let mut m_transitionInfoReferenceForTE: _serde::__private::Option<
                        hkbStateMachineTransitionInfoReference,
                    > = _serde::__private::None;
                    let mut m_fromStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_toStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_isReturnToPreviousState: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_transitionEffect) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionEffect",
                                        ),
                                    );
                                }
                                m_transitionEffect = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transitionEffectInternalStateInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionEffectInternalStateInfo",
                                        ),
                                    );
                                }
                                m_transitionEffectInternalStateInfo = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transitionInfoReference,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionInfoReference",
                                        ),
                                    );
                                }
                                m_transitionInfoReference = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbStateMachineTransitionInfoReference,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transitionInfoReferenceForTE,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionInfoReferenceForTE",
                                        ),
                                    );
                                }
                                m_transitionInfoReferenceForTE = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbStateMachineTransitionInfoReference,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_fromStateId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromStateId",
                                        ),
                                    );
                                }
                                m_fromStateId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_toStateId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toStateId",
                                        ),
                                    );
                                }
                                m_toStateId = _serde::__private::Some(
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
                                    &m_isReturnToPreviousState,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isReturnToPreviousState",
                                        ),
                                    );
                                }
                                m_isReturnToPreviousState = _serde::__private::Some(
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
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    let m_transitionEffect = match m_transitionEffect {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionEffect",
                                ),
                            );
                        }
                    };
                    let m_transitionEffectInternalStateInfo = match m_transitionEffectInternalStateInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionEffectInternalStateInfo",
                                ),
                            );
                        }
                    };
                    let m_transitionInfoReference = match m_transitionInfoReference {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionInfoReference",
                                ),
                            );
                        }
                    };
                    let m_transitionInfoReferenceForTE = match m_transitionInfoReferenceForTE {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionInfoReferenceForTE",
                                ),
                            );
                        }
                    };
                    let m_fromStateId = match m_fromStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fromStateId",
                                ),
                            );
                        }
                    };
                    let m_toStateId = match m_toStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toStateId",
                                ),
                            );
                        }
                    };
                    let m_isReturnToPreviousState = match m_isReturnToPreviousState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isReturnToPreviousState",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbStateMachineActiveTransitionInfo {
                        __ptr,
                        m_transitionEffect,
                        m_transitionEffectInternalStateInfo,
                        m_transitionInfoReference,
                        m_transitionInfoReferenceForTE,
                        m_fromStateId,
                        m_toStateId,
                        m_isReturnToPreviousState,
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
                    let mut m_transitionEffectInternalStateInfo: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_transitionInfoReference: _serde::__private::Option<
                        hkbStateMachineTransitionInfoReference,
                    > = _serde::__private::None;
                    let mut m_transitionInfoReferenceForTE: _serde::__private::Option<
                        hkbStateMachineTransitionInfoReference,
                    > = _serde::__private::None;
                    let mut m_fromStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_toStateId: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_isReturnToPreviousState: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_transitionEffectInternalStateInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transitionEffectInternalStateInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionEffectInternalStateInfo",
                                        ),
                                    );
                                }
                                m_transitionEffectInternalStateInfo = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transitionInfoReference => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transitionInfoReference,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionInfoReference",
                                        ),
                                    );
                                }
                                m_transitionInfoReference = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbStateMachineTransitionInfoReference,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transitionInfoReferenceForTE => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transitionInfoReferenceForTE,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionInfoReferenceForTE",
                                        ),
                                    );
                                }
                                m_transitionInfoReferenceForTE = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbStateMachineTransitionInfoReference,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fromStateId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fromStateId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromStateId",
                                        ),
                                    );
                                }
                                m_fromStateId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toStateId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_toStateId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toStateId",
                                        ),
                                    );
                                }
                                m_toStateId = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isReturnToPreviousState => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_isReturnToPreviousState,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isReturnToPreviousState",
                                        ),
                                    );
                                }
                                m_isReturnToPreviousState = _serde::__private::Some(
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
                    let m_transitionEffectInternalStateInfo = match m_transitionEffectInternalStateInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionEffectInternalStateInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transitionInfoReference = match m_transitionInfoReference {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionInfoReference",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transitionInfoReferenceForTE = match m_transitionInfoReferenceForTE {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionInfoReferenceForTE",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fromStateId = match m_fromStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fromStateId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toStateId = match m_toStateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toStateId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isReturnToPreviousState = match m_isReturnToPreviousState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isReturnToPreviousState",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbStateMachineActiveTransitionInfo {
                        __ptr,
                        m_transitionEffectInternalStateInfo,
                        m_transitionInfoReference,
                        m_transitionInfoReferenceForTE,
                        m_fromStateId,
                        m_toStateId,
                        m_isReturnToPreviousState,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transitionEffect",
                "transitionEffectInternalStateInfo",
                "transitionInfoReference",
                "transitionInfoReferenceForTE",
                "fromStateId",
                "toStateId",
                "isReturnToPreviousState",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbStateMachineActiveTransitionInfo",
                FIELDS,
                __hkbStateMachineActiveTransitionInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbStateMachineActiveTransitionInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
