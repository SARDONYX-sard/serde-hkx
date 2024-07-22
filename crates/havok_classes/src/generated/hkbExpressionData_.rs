use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbExpressionData`
/// - version: `0`
/// - signature: `0x6740042a`
/// - size: ` 16`(x86)/` 24`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbExpressionData<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `expression`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_expression: StringPtr<'a>,
    /// # C++ Info
    /// - name: `assignmentVariableIndex`(ctype: `hkInt32`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_assignmentVariableIndex: i32,
    /// # C++ Info
    /// - name: `assignmentEventIndex`(ctype: `hkInt32`)
    /// - offset: `  8`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_assignmentEventIndex: i32,
    /// # C++ Info
    /// - name: `eventMode`(ctype: `enum ExpressionEventMode`)
    /// - offset: ` 12`(x86)/` 16`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_eventMode: ExpressionEventMode,
    /// # C++ Info
    /// - name: `raisedEvent`(ctype: `hkBool`)
    /// - offset: ` 13`(x86)/` 17`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_raisedEvent: bool,
    /// # C++ Info
    /// - name: `wasTrueInPreviousFrame`(ctype: `hkBool`)
    /// - offset: ` 14`(x86)/` 18`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_wasTrueInPreviousFrame: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbExpressionData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbExpressionData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6740042a)
        }
    }
    impl<'a> _serde::Serialize for hkbExpressionData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6740042a)));
            let mut serializer = __serializer
                .serialize_struct("hkbExpressionData", class_meta)?;
            serializer.serialize_stringptr_meta_field("expression", &self.m_expression)?;
            serializer
                .serialize_field(
                    "assignmentVariableIndex",
                    &self.m_assignmentVariableIndex,
                )?;
            serializer
                .serialize_field("assignmentEventIndex", &self.m_assignmentEventIndex)?;
            serializer.serialize_field("eventMode", &self.m_eventMode)?;
            serializer.skip_field("raisedEvent", &self.m_raisedEvent)?;
            serializer
                .skip_field("wasTrueInPreviousFrame", &self.m_wasTrueInPreviousFrame)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_stringptr_field("expression", &self.m_expression)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbExpressionData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_eventMode,
                m_assignmentEventIndex,
                m_assignmentVariableIndex,
                m_expression,
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
                        "eventMode" => Ok(__Field::m_eventMode),
                        "assignmentEventIndex" => Ok(__Field::m_assignmentEventIndex),
                        "assignmentVariableIndex" => {
                            Ok(__Field::m_assignmentVariableIndex)
                        }
                        "expression" => Ok(__Field::m_expression),
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
            struct __hkbExpressionDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbExpressionData<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbExpressionDataVisitor<'de> {
                type Value = hkbExpressionData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbExpressionData",
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
                    let mut m_expression: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_assignmentVariableIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_assignmentEventIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_eventMode: _serde::__private::Option<
                        ExpressionEventMode,
                    > = _serde::__private::None;
                    let mut m_raisedEvent: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_wasTrueInPreviousFrame: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_expression) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expression",
                                        ),
                                    );
                                }
                                m_expression = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_assignmentVariableIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "assignmentVariableIndex",
                                        ),
                                    );
                                }
                                m_assignmentVariableIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_assignmentEventIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "assignmentEventIndex",
                                        ),
                                    );
                                }
                                m_assignmentEventIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_eventMode) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventMode",
                                        ),
                                    );
                                }
                                m_eventMode = _serde::__private::Some(
                                    match __A::next_value::<ExpressionEventMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_raisedEvent) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raisedEvent",
                                        ),
                                    );
                                }
                                m_raisedEvent = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wasTrueInPreviousFrame,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wasTrueInPreviousFrame",
                                        ),
                                    );
                                }
                                m_wasTrueInPreviousFrame = _serde::__private::Some(
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
                    __A::pad(&mut __map, 1usize, 5usize)?;
                    let m_expression = match m_expression {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expression",
                                ),
                            );
                        }
                    };
                    let m_assignmentVariableIndex = match m_assignmentVariableIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "assignmentVariableIndex",
                                ),
                            );
                        }
                    };
                    let m_assignmentEventIndex = match m_assignmentEventIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "assignmentEventIndex",
                                ),
                            );
                        }
                    };
                    let m_eventMode = match m_eventMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventMode",
                                ),
                            );
                        }
                    };
                    let m_raisedEvent = match m_raisedEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raisedEvent",
                                ),
                            );
                        }
                    };
                    let m_wasTrueInPreviousFrame = match m_wasTrueInPreviousFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wasTrueInPreviousFrame",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbExpressionData {
                        __ptr,
                        m_expression,
                        m_assignmentVariableIndex,
                        m_assignmentEventIndex,
                        m_eventMode,
                        m_raisedEvent,
                        m_wasTrueInPreviousFrame,
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
                    let mut m_eventMode: _serde::__private::Option<
                        ExpressionEventMode,
                    > = _serde::__private::None;
                    let mut m_assignmentEventIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_assignmentVariableIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_expression: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_eventMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eventMode) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventMode",
                                        ),
                                    );
                                }
                                m_eventMode = _serde::__private::Some(
                                    match __A::next_value::<ExpressionEventMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_assignmentEventIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_assignmentEventIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "assignmentEventIndex",
                                        ),
                                    );
                                }
                                m_assignmentEventIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_assignmentVariableIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_assignmentVariableIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "assignmentVariableIndex",
                                        ),
                                    );
                                }
                                m_assignmentVariableIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_expression => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_expression) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expression",
                                        ),
                                    );
                                }
                                m_expression = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    let m_eventMode = match m_eventMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_assignmentEventIndex = match m_assignmentEventIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "assignmentEventIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_assignmentVariableIndex = match m_assignmentVariableIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "assignmentVariableIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_expression = match m_expression {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expression",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbExpressionData {
                        __ptr,
                        m_expression,
                        m_assignmentVariableIndex,
                        m_assignmentEventIndex,
                        m_eventMode,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "expression",
                "assignmentVariableIndex",
                "assignmentEventIndex",
                "eventMode",
                "raisedEvent",
                "wasTrueInPreviousFrame",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbExpressionData",
                FIELDS,
                __hkbExpressionDataVisitor {
                    marker: _serde::__private::PhantomData::<hkbExpressionData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
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
pub enum ExpressionEventMode {
    #[default]
    EVENT_MODE_SEND_ONCE = 0isize,
    EVENT_MODE_SEND_ON_TRUE = 1isize,
    EVENT_MODE_SEND_ON_FALSE_TO_TRUE = 2isize,
    EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ExpressionEventMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::EVENT_MODE_SEND_ONCE => {
                    __serializer.serialize_field("EVENT_MODE_SEND_ONCE", &0u64)
                }
                Self::EVENT_MODE_SEND_ON_TRUE => {
                    __serializer.serialize_field("EVENT_MODE_SEND_ON_TRUE", &1u64)
                }
                Self::EVENT_MODE_SEND_ON_FALSE_TO_TRUE => {
                    __serializer
                        .serialize_field("EVENT_MODE_SEND_ON_FALSE_TO_TRUE", &2u64)
                }
                Self::EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE => {
                    __serializer
                        .serialize_field("EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum ExpressionEventMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for ExpressionEventMode {
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
                                || v.eq_ignore_ascii_case("EVENT_MODE_SEND_ONCE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("EVENT_MODE_SEND_ON_TRUE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case(
                                        "EVENT_MODE_SEND_ON_FALSE_TO_TRUE",
                                    ) => _serde::__private::Ok(__Field::__field2),
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case(
                                        "EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE",
                                    ) => _serde::__private::Ok(__Field::__field3),
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
                marker: _serde::__private::PhantomData<ExpressionEventMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ExpressionEventMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ExpressionEventMode",
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
                                ExpressionEventMode::EVENT_MODE_SEND_ONCE,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ExpressionEventMode::EVENT_MODE_SEND_ON_TRUE,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ExpressionEventMode::EVENT_MODE_SEND_ON_FALSE_TO_TRUE,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ExpressionEventMode::EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "EVENT_MODE_SEND_ONCE",
                "EVENT_MODE_SEND_ON_TRUE",
                "EVENT_MODE_SEND_ON_FALSE_TO_TRUE",
                "EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ExpressionEventMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ExpressionEventMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
