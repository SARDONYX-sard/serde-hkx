use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbStateMachineTransitionInfoReference`
/// - version: `1`
/// - signature: `0x9810c2d0`
/// - size: `  6`(x86)/`  6`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineTransitionInfoReference {
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
    /// - name: `fromStateIndex`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "fromStateIndex"))]
    pub m_fromStateIndex: i16,
    /// # C++ Info
    /// - name: `transitionIndex`(ctype: `hkInt16`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "transitionIndex"))]
    pub m_transitionIndex: i16,
    /// # C++ Info
    /// - name: `stateMachineId`(ctype: `hkInt16`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "stateMachineId"))]
    pub m_stateMachineId: i16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineTransitionInfoReference {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineTransitionInfoReference"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9810c2d0)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkbStateMachineTransitionInfoReference {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9810c2d0)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbStateMachineTransitionInfoReference",
                    class_meta,
                    (6u64, 6u64),
                )?;
            serializer.serialize_field("fromStateIndex", &self.m_fromStateIndex)?;
            serializer.serialize_field("transitionIndex", &self.m_transitionIndex)?;
            serializer.serialize_field("stateMachineId", &self.m_stateMachineId)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbStateMachineTransitionInfoReference {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_fromStateIndex,
                m_transitionIndex,
                m_stateMachineId,
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
                        "fromStateIndex" => Ok(__Field::m_fromStateIndex),
                        "transitionIndex" => Ok(__Field::m_transitionIndex),
                        "stateMachineId" => Ok(__Field::m_stateMachineId),
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
            struct __hkbStateMachineTransitionInfoReferenceVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbStateMachineTransitionInfoReference,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbStateMachineTransitionInfoReferenceVisitor<'de> {
                type Value = hkbStateMachineTransitionInfoReference;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbStateMachineTransitionInfoReference",
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
                    let mut m_fromStateIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_transitionIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_stateMachineId: _serde::__private::Option<i16> = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_fromStateIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromStateIndex",
                                        ),
                                    );
                                }
                                m_fromStateIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_transitionIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionIndex",
                                        ),
                                    );
                                }
                                m_transitionIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_stateMachineId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stateMachineId",
                                        ),
                                    );
                                }
                                m_stateMachineId = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
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
                    let m_fromStateIndex = match m_fromStateIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fromStateIndex",
                                ),
                            );
                        }
                    };
                    let m_transitionIndex = match m_transitionIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionIndex",
                                ),
                            );
                        }
                    };
                    let m_stateMachineId = match m_stateMachineId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stateMachineId",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbStateMachineTransitionInfoReference {
                        __ptr,
                        m_fromStateIndex,
                        m_transitionIndex,
                        m_stateMachineId,
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
                    let mut m_fromStateIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_transitionIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_stateMachineId: _serde::__private::Option<i16> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_fromStateIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fromStateIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromStateIndex",
                                        ),
                                    );
                                }
                                m_fromStateIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transitionIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transitionIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transitionIndex",
                                        ),
                                    );
                                }
                                m_transitionIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stateMachineId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_stateMachineId) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stateMachineId",
                                        ),
                                    );
                                }
                                m_stateMachineId = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
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
                    let m_fromStateIndex = match m_fromStateIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fromStateIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transitionIndex = match m_transitionIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transitionIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stateMachineId = match m_stateMachineId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stateMachineId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbStateMachineTransitionInfoReference {
                        __ptr,
                        m_fromStateIndex,
                        m_transitionIndex,
                        m_stateMachineId,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "fromStateIndex",
                "transitionIndex",
                "stateMachineId",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbStateMachineTransitionInfoReference",
                FIELDS,
                __hkbStateMachineTransitionInfoReferenceVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbStateMachineTransitionInfoReference,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
