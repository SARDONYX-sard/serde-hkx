use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSkeletonMapperDataChainMapping`
/// -         version: `0`
/// -       signature: `0xa528f7cf`
/// -          size: 112(x86)/112(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSkeletonMapperDataChainMapping {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `startBoneA`(ctype: `hkInt16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_startBoneA: i16,
    /// # C++ Info
    /// -          name: `endBoneA`(ctype: `hkInt16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_endBoneA: i16,
    /// # C++ Info
    /// -          name: `startBoneB`(ctype: `hkInt16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_startBoneB: i16,
    /// # C++ Info
    /// -          name: `endBoneB`(ctype: `hkInt16`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_endBoneB: i16,
    /// # C++ Info
    /// -          name: `startAFromBTransform`(ctype: `hkQsTransform`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_startAFromBTransform: QsTransform,
    /// # C++ Info
    /// -          name: `endAFromBTransform`(ctype: `hkQsTransform`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_endAFromBTransform: QsTransform,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaSkeletonMapperDataChainMapping {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSkeletonMapperDataChainMapping"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa528f7cf)
        }
    }
    impl _serde::Serialize for hkaSkeletonMapperDataChainMapping {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa528f7cf)));
            let mut serializer = __serializer
                .serialize_struct("hkaSkeletonMapperDataChainMapping", class_meta)?;
            serializer.serialize_field("startBoneA", &self.m_startBoneA)?;
            serializer.serialize_field("endBoneA", &self.m_endBoneA)?;
            serializer.serialize_field("startBoneB", &self.m_startBoneB)?;
            serializer.serialize_field("endBoneB", &self.m_endBoneB)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field("startAFromBTransform", &self.m_startAFromBTransform)?;
            serializer
                .serialize_field("endAFromBTransform", &self.m_endAFromBTransform)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_startBoneA,
    m_endBoneA,
    m_startBoneB,
    m_endBoneB,
    m_startAFromBTransform,
    m_endAFromBTransform,
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
            "startBoneA" => Ok(__Field::m_startBoneA),
            "endBoneA" => Ok(__Field::m_endBoneA),
            "startBoneB" => Ok(__Field::m_startBoneB),
            "endBoneB" => Ok(__Field::m_endBoneB),
            "startAFromBTransform" => Ok(__Field::m_startAFromBTransform),
            "endAFromBTransform" => Ok(__Field::m_endAFromBTransform),
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
pub(super) struct __hkaSkeletonMapperDataChainMappingVisitor<'de> {
    marker: core::marker::PhantomData<hkaSkeletonMapperDataChainMapping>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaSkeletonMapperDataChainMappingVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkaSkeletonMapperDataChainMapping, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkaSkeletonMapperDataChainMapping,
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
impl<'de> _serde::de::Visitor<'de> for __hkaSkeletonMapperDataChainMappingVisitor<'de> {
    type Value = hkaSkeletonMapperDataChainMapping;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkaSkeletonMapperDataChainMapping",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_startBoneA: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endBoneA: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_startBoneB: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endBoneB: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_startAFromBTransform: _serde::__private::Option<QsTransform> = _serde::__private::None;
        let mut m_endAFromBTransform: _serde::__private::Option<QsTransform> = _serde::__private::None;
        for i in 0..6usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_startBoneA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startBoneA",
                            ),
                        );
                    }
                    m_startBoneA = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_endBoneA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endBoneA",
                            ),
                        );
                    }
                    m_endBoneA = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_startBoneB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startBoneB",
                            ),
                        );
                    }
                    m_startBoneB = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_endBoneB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endBoneB",
                            ),
                        );
                    }
                    m_endBoneB = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_startAFromBTransform) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startAFromBTransform",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 8usize, 8usize)?;
                    m_startAFromBTransform = _serde::__private::Some(
                        match __A::next_value::<QsTransform>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_endAFromBTransform) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endAFromBTransform",
                            ),
                        );
                    }
                    m_endAFromBTransform = _serde::__private::Some(
                        match __A::next_value::<QsTransform>(&mut __map) {
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
        let m_startBoneA = match m_startBoneA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startBoneA"),
                );
            }
        };
        let m_endBoneA = match m_endBoneA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endBoneA"),
                );
            }
        };
        let m_startBoneB = match m_startBoneB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startBoneB"),
                );
            }
        };
        let m_endBoneB = match m_endBoneB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endBoneB"),
                );
            }
        };
        let m_startAFromBTransform = match m_startAFromBTransform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "startAFromBTransform",
                    ),
                );
            }
        };
        let m_endAFromBTransform = match m_endAFromBTransform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "endAFromBTransform",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkaSkeletonMapperDataChainMapping {
            __ptr: __A::class_ptr(&mut __map),
            m_startBoneA,
            m_endBoneA,
            m_startBoneB,
            m_endBoneB,
            m_startAFromBTransform,
            m_endAFromBTransform,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_startBoneA: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endBoneA: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_startBoneB: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endBoneB: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_startAFromBTransform: _serde::__private::Option<QsTransform> = _serde::__private::None;
        let mut m_endAFromBTransform: _serde::__private::Option<QsTransform> = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_startBoneA => {
                        if _serde::__private::Option::is_some(&m_startBoneA) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "startBoneA",
                                ),
                            );
                        }
                        m_startBoneA = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_endBoneA => {
                        if _serde::__private::Option::is_some(&m_endBoneA) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "endBoneA",
                                ),
                            );
                        }
                        m_endBoneA = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_startBoneB => {
                        if _serde::__private::Option::is_some(&m_startBoneB) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "startBoneB",
                                ),
                            );
                        }
                        m_startBoneB = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_endBoneB => {
                        if _serde::__private::Option::is_some(&m_endBoneB) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "endBoneB",
                                ),
                            );
                        }
                        m_endBoneB = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_startAFromBTransform => {
                        if _serde::__private::Option::is_some(&m_startAFromBTransform) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "startAFromBTransform",
                                ),
                            );
                        }
                        m_startAFromBTransform = _serde::__private::Some(
                            match __A::next_value::<QsTransform>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_endAFromBTransform => {
                        if _serde::__private::Option::is_some(&m_endAFromBTransform) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "endAFromBTransform",
                                ),
                            );
                        }
                        m_endAFromBTransform = _serde::__private::Some(
                            match __A::next_value::<QsTransform>(&mut __map) {
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
        let m_startBoneA = match m_startBoneA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startBoneA"),
                );
            }
        };
        let m_endBoneA = match m_endBoneA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endBoneA"),
                );
            }
        };
        let m_startBoneB = match m_startBoneB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startBoneB"),
                );
            }
        };
        let m_endBoneB = match m_endBoneB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endBoneB"),
                );
            }
        };
        let m_startAFromBTransform = match m_startAFromBTransform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "startAFromBTransform",
                    ),
                );
            }
        };
        let m_endAFromBTransform = match m_endAFromBTransform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "endAFromBTransform",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkaSkeletonMapperDataChainMapping {
            __ptr: __A::class_ptr(&mut __map),
            m_startBoneA,
            m_endBoneA,
            m_startBoneB,
            m_endBoneB,
            m_startAFromBTransform,
            m_endAFromBTransform,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaSkeletonMapperDataChainMapping {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "startBoneA",
                "endBoneA",
                "startBoneB",
                "endBoneB",
                "startAFromBTransform",
                "endAFromBTransform",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaSkeletonMapperDataChainMapping",
                FIELDS,
                __hkaSkeletonMapperDataChainMappingVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaSkeletonMapperDataChainMapping,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
