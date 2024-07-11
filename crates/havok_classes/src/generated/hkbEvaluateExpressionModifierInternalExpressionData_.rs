use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEvaluateExpressionModifierInternalExpressionData`
/// -         version: `0`
/// -       signature: `0xb8686f6b`
/// -          size:   2(x86)/  2(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEvaluateExpressionModifierInternalExpressionData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `raisedEvent`(ctype: `hkBool`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_raisedEvent: bool,
    /// # C++ Info
    /// -          name: `wasTrueInPreviousFrame`(ctype: `hkBool`)
    /// -        offset:   1(x86)/  1(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_wasTrueInPreviousFrame: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbEvaluateExpressionModifierInternalExpressionData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbEvaluateExpressionModifierInternalExpressionData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb8686f6b)
        }
    }
    impl _serde::Serialize for hkbEvaluateExpressionModifierInternalExpressionData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xb8686f6b)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbEvaluateExpressionModifierInternalExpressionData",
                    class_meta,
                )?;
            serializer.serialize_field("raisedEvent", &self.m_raisedEvent)?;
            serializer
                .serialize_field(
                    "wasTrueInPreviousFrame",
                    &self.m_wasTrueInPreviousFrame,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_raisedEvent,
    m_wasTrueInPreviousFrame,
    __ignore,
}
struct __FieldVisitor;
impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
    type Value = __Field;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "field identifier")
    }
    /// Intended for use in XML.
    #[allow(clippy::match_single_binding)]
    #[allow(clippy::reversed_empty_ranges)]
    #[allow(clippy::single_match)]
    fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
    where
        __E: _serde::de::Error,
    {
        match __value {
            "raisedEvent" => Ok(__Field::m_raisedEvent),
            "wasTrueInPreviousFrame" => Ok(__Field::m_wasTrueInPreviousFrame),
            _ => Ok(__Field::__ignore),
        }
    }
}
impl<'de> _serde::Deserialize<'de> for __Field {
    #[inline]
    fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
    }
}
pub(super) struct __hkbEvaluateExpressionModifierInternalExpressionDataVisitor<'de> {
    marker: core::marker::PhantomData<
        hkbEvaluateExpressionModifierInternalExpressionData,
    >,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbEvaluateExpressionModifierInternalExpressionDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkbEvaluateExpressionModifierInternalExpressionData,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbEvaluateExpressionModifierInternalExpressionData,
                >,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de>
for __hkbEvaluateExpressionModifierInternalExpressionDataVisitor<'de> {
    type Value = hkbEvaluateExpressionModifierInternalExpressionData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbEvaluateExpressionModifierInternalExpressionData",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_raisedEvent: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_wasTrueInPreviousFrame: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
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
                1usize => {
                    if _serde::__private::Option::is_some(&m_wasTrueInPreviousFrame) {
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
        let m_raisedEvent = match m_raisedEvent {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("raisedEvent"),
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
        _serde::__private::Ok(hkbEvaluateExpressionModifierInternalExpressionData {
            __ptr: __A::class_ptr(&mut __map),
            m_raisedEvent,
            m_wasTrueInPreviousFrame,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_raisedEvent: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_wasTrueInPreviousFrame: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_raisedEvent => {
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
                    __Field::m_wasTrueInPreviousFrame => {
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
        }
        let m_raisedEvent = match m_raisedEvent {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("raisedEvent"),
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
        _serde::__private::Ok(hkbEvaluateExpressionModifierInternalExpressionData {
            __ptr: __A::class_ptr(&mut __map),
            m_raisedEvent,
            m_wasTrueInPreviousFrame,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkbEvaluateExpressionModifierInternalExpressionData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["raisedEvent", "wasTrueInPreviousFrame"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbEvaluateExpressionModifierInternalExpressionData",
                FIELDS,
                __hkbEvaluateExpressionModifierInternalExpressionDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbEvaluateExpressionModifierInternalExpressionData,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
