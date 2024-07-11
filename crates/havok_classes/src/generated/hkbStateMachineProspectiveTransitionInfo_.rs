use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineProspectiveTransitionInfo`
/// -         version: `2`
/// -       signature: `0x3ab09a2e`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineProspectiveTransitionInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `transitionInfoReference`(ctype: `struct hkbStateMachineTransitionInfoReference`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   6(x86)/  6(x86_64)
    ///
    pub m_transitionInfoReference: hkbStateMachineTransitionInfoReference,
    /// # C++ Info
    /// -          name: `transitionInfoReferenceForTE`(ctype: `struct hkbStateMachineTransitionInfoReference`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   6(x86)/  6(x86_64)
    ///
    pub m_transitionInfoReferenceForTE: hkbStateMachineTransitionInfoReference,
    /// # C++ Info
    /// -          name: `toStateId`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_toStateId: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineProspectiveTransitionInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineProspectiveTransitionInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3ab09a2e)
        }
    }
    impl _serde::Serialize for hkbStateMachineProspectiveTransitionInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3ab09a2e)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbStateMachineProspectiveTransitionInfo",
                    class_meta,
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
            serializer.serialize_field("toStateId", &self.m_toStateId)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_transitionInfoReference,
    m_transitionInfoReferenceForTE,
    m_toStateId,
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
            "transitionInfoReference" => Ok(__Field::m_transitionInfoReference),
            "transitionInfoReferenceForTE" => Ok(__Field::m_transitionInfoReferenceForTE),
            "toStateId" => Ok(__Field::m_toStateId),
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
pub(super) struct __hkbStateMachineProspectiveTransitionInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbStateMachineProspectiveTransitionInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbStateMachineProspectiveTransitionInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbStateMachineProspectiveTransitionInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbStateMachineProspectiveTransitionInfo,
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
for __hkbStateMachineProspectiveTransitionInfoVisitor<'de> {
    type Value = hkbStateMachineProspectiveTransitionInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbStateMachineProspectiveTransitionInfo",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_transitionInfoReference: _serde::__private::Option<
            hkbStateMachineTransitionInfoReference,
        > = _serde::__private::None;
        let mut m_transitionInfoReferenceForTE: _serde::__private::Option<
            hkbStateMachineTransitionInfoReference,
        > = _serde::__private::None;
        let mut m_toStateId: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_transitionInfoReference) {
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
                1usize => {
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
                2usize => {
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
                _ => {}
            }
        }
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
        let m_toStateId = match m_toStateId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("toStateId"),
                );
            }
        };
        _serde::__private::Ok(hkbStateMachineProspectiveTransitionInfo {
            __ptr: __A::class_ptr(&mut __map),
            m_transitionInfoReference,
            m_transitionInfoReferenceForTE,
            m_toStateId,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_transitionInfoReference: _serde::__private::Option<
            hkbStateMachineTransitionInfoReference,
        > = _serde::__private::None;
        let mut m_transitionInfoReferenceForTE: _serde::__private::Option<
            hkbStateMachineTransitionInfoReference,
        > = _serde::__private::None;
        let mut m_toStateId: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_transitionInfoReference => {
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
                    __Field::m_transitionInfoReferenceForTE => {
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
                    __Field::m_toStateId => {
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
                    _ => {}
                }
            }
        }
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
        let m_toStateId = match m_toStateId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("toStateId"),
                );
            }
        };
        _serde::__private::Ok(hkbStateMachineProspectiveTransitionInfo {
            __ptr,
            m_transitionInfoReference,
            m_transitionInfoReferenceForTE,
            m_toStateId,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbStateMachineProspectiveTransitionInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "transitionInfoReference",
                "transitionInfoReferenceForTE",
                "toStateId",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbStateMachineProspectiveTransitionInfo",
                FIELDS,
                __hkbStateMachineProspectiveTransitionInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbStateMachineProspectiveTransitionInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
