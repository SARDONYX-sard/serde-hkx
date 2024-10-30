use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbStateMachineDelayedTransitionInfo`
/// - version: `1`
/// - signature: `0x26d5499`
/// - size: ` 24`(x86)/` 24`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineDelayedTransitionInfo {
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
    /// # C++ Info
    /// - name: `delayedTransition`(ctype: `struct hkbStateMachineProspectiveTransitionInfo`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "delayedTransition"))]
    #[cfg_attr(feature = "serde", serde(rename = "delayedTransition"))]
    pub m_delayedTransition: hkbStateMachineProspectiveTransitionInfo,
    /// # C++ Info
    /// - name: `timeDelayed`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeDelayed"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeDelayed"))]
    pub m_timeDelayed: f32,
    /// # C++ Info
    /// - name: `isDelayedTransitionReturnToPreviousState`(ctype: `hkBool`)
    /// - offset: ` 20`(x86)/` 20`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "isDelayedTransitionReturnToPreviousState")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "isDelayedTransitionReturnToPreviousState")
    )]
    pub m_isDelayedTransitionReturnToPreviousState: bool,
    /// # C++ Info
    /// - name: `wasInAbutRangeLastFrame`(ctype: `hkBool`)
    /// - offset: ` 21`(x86)/` 21`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "wasInAbutRangeLastFrame"))]
    #[cfg_attr(feature = "serde", serde(rename = "wasInAbutRangeLastFrame"))]
    pub m_wasInAbutRangeLastFrame: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineDelayedTransitionInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineDelayedTransitionInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x26d5499)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_delayedTransition.deps_indexes());
            v
        }
    }
    impl _serde::Serialize for hkbStateMachineDelayedTransitionInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x26d5499)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbStateMachineDelayedTransitionInfo",
                    class_meta,
                    (24u64, 24u64),
                )?;
            serializer.serialize_field("delayedTransition", &self.m_delayedTransition)?;
            serializer.serialize_field("timeDelayed", &self.m_timeDelayed)?;
            serializer
                .serialize_field(
                    "isDelayedTransitionReturnToPreviousState",
                    &self.m_isDelayedTransitionReturnToPreviousState,
                )?;
            serializer
                .serialize_field(
                    "wasInAbutRangeLastFrame",
                    &self.m_wasInAbutRangeLastFrame,
                )?;
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
    impl<'de> _serde::Deserialize<'de> for hkbStateMachineDelayedTransitionInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_delayedTransition,
                m_timeDelayed,
                m_isDelayedTransitionReturnToPreviousState,
                m_wasInAbutRangeLastFrame,
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
                        "delayedTransition" => Ok(__Field::m_delayedTransition),
                        "timeDelayed" => Ok(__Field::m_timeDelayed),
                        "isDelayedTransitionReturnToPreviousState" => {
                            Ok(__Field::m_isDelayedTransitionReturnToPreviousState)
                        }
                        "wasInAbutRangeLastFrame" => {
                            Ok(__Field::m_wasInAbutRangeLastFrame)
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
            struct __hkbStateMachineDelayedTransitionInfoVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbStateMachineDelayedTransitionInfo,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbStateMachineDelayedTransitionInfoVisitor<'de> {
                type Value = hkbStateMachineDelayedTransitionInfo;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbStateMachineDelayedTransitionInfo",
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
                    let mut m_delayedTransition: _serde::__private::Option<
                        hkbStateMachineProspectiveTransitionInfo,
                    > = _serde::__private::None;
                    let mut m_timeDelayed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_isDelayedTransitionReturnToPreviousState: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_wasInAbutRangeLastFrame: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_delayedTransition,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "delayedTransition",
                                        ),
                                    );
                                }
                                m_delayedTransition = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbStateMachineProspectiveTransitionInfo,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_timeDelayed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeDelayed",
                                        ),
                                    );
                                }
                                m_timeDelayed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_isDelayedTransitionReturnToPreviousState,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isDelayedTransitionReturnToPreviousState",
                                        ),
                                    );
                                }
                                m_isDelayedTransitionReturnToPreviousState = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wasInAbutRangeLastFrame,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wasInAbutRangeLastFrame",
                                        ),
                                    );
                                }
                                m_wasInAbutRangeLastFrame = _serde::__private::Some(
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
                    let m_delayedTransition = match m_delayedTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "delayedTransition",
                                ),
                            );
                        }
                    };
                    let m_timeDelayed = match m_timeDelayed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeDelayed",
                                ),
                            );
                        }
                    };
                    let m_isDelayedTransitionReturnToPreviousState = match m_isDelayedTransitionReturnToPreviousState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isDelayedTransitionReturnToPreviousState",
                                ),
                            );
                        }
                    };
                    let m_wasInAbutRangeLastFrame = match m_wasInAbutRangeLastFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wasInAbutRangeLastFrame",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbStateMachineDelayedTransitionInfo {
                        __ptr,
                        m_delayedTransition,
                        m_timeDelayed,
                        m_isDelayedTransitionReturnToPreviousState,
                        m_wasInAbutRangeLastFrame,
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
                    let mut m_delayedTransition: _serde::__private::Option<
                        hkbStateMachineProspectiveTransitionInfo,
                    > = _serde::__private::None;
                    let mut m_timeDelayed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_isDelayedTransitionReturnToPreviousState: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_wasInAbutRangeLastFrame: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_delayedTransition => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_delayedTransition,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "delayedTransition",
                                        ),
                                    );
                                }
                                m_delayedTransition = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbStateMachineProspectiveTransitionInfo,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_timeDelayed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_timeDelayed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeDelayed",
                                        ),
                                    );
                                }
                                m_timeDelayed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isDelayedTransitionReturnToPreviousState => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_isDelayedTransitionReturnToPreviousState,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isDelayedTransitionReturnToPreviousState",
                                        ),
                                    );
                                }
                                m_isDelayedTransitionReturnToPreviousState = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wasInAbutRangeLastFrame => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wasInAbutRangeLastFrame,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wasInAbutRangeLastFrame",
                                        ),
                                    );
                                }
                                m_wasInAbutRangeLastFrame = _serde::__private::Some(
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
                    let m_delayedTransition = match m_delayedTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "delayedTransition",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_timeDelayed = match m_timeDelayed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeDelayed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isDelayedTransitionReturnToPreviousState = match m_isDelayedTransitionReturnToPreviousState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isDelayedTransitionReturnToPreviousState",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wasInAbutRangeLastFrame = match m_wasInAbutRangeLastFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wasInAbutRangeLastFrame",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbStateMachineDelayedTransitionInfo {
                        __ptr,
                        m_delayedTransition,
                        m_timeDelayed,
                        m_isDelayedTransitionReturnToPreviousState,
                        m_wasInAbutRangeLastFrame,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "delayedTransition",
                "timeDelayed",
                "isDelayedTransitionReturnToPreviousState",
                "wasInAbutRangeLastFrame",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbStateMachineDelayedTransitionInfo",
                FIELDS,
                __hkbStateMachineDelayedTransitionInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbStateMachineDelayedTransitionInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
