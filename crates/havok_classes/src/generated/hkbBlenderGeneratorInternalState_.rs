use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBlenderGeneratorInternalState`
/// -         version: `0`
/// -       signature: `0x84717488`
/// -          size:  48(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlenderGeneratorInternalState {
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
    /// -          name: `childrenInternalStates`(ctype: `hkArray<struct hkbBlenderGeneratorChildInternalState>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childrenInternalStates: Vec<hkbBlenderGeneratorChildInternalState>,
    /// # C++ Info
    /// -          name: `sortedChildren`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_sortedChildren: Vec<i16>,
    /// # C++ Info
    /// -          name: `endIntervalWeight`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_endIntervalWeight: f32,
    /// # C++ Info
    /// -          name: `numActiveChildren`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numActiveChildren: i32,
    /// # C++ Info
    /// -          name: `beginIntervalIndex`(ctype: `hkInt16`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_beginIntervalIndex: i16,
    /// # C++ Info
    /// -          name: `endIntervalIndex`(ctype: `hkInt16`)
    /// -        offset:  42(x86)/ 58(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_endIntervalIndex: i16,
    /// # C++ Info
    /// -          name: `initSync`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 60(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_initSync: bool,
    /// # C++ Info
    /// -          name: `doSubtractiveBlend`(ctype: `hkBool`)
    /// -        offset:  45(x86)/ 61(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_doSubtractiveBlend: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBlenderGeneratorInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBlenderGeneratorInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x84717488)
        }
    }
    impl _serde::Serialize for hkbBlenderGeneratorInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x84717488)));
            let mut serializer = __serializer
                .serialize_struct("hkbBlenderGeneratorInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "childrenInternalStates",
                    &self.m_childrenInternalStates,
                )?;
            serializer
                .serialize_array_meta_field("sortedChildren", &self.m_sortedChildren)?;
            serializer.serialize_field("endIntervalWeight", &self.m_endIntervalWeight)?;
            serializer.serialize_field("numActiveChildren", &self.m_numActiveChildren)?;
            serializer
                .serialize_field("beginIntervalIndex", &self.m_beginIntervalIndex)?;
            serializer.serialize_field("endIntervalIndex", &self.m_endIntervalIndex)?;
            serializer.serialize_field("initSync", &self.m_initSync)?;
            serializer
                .serialize_field("doSubtractiveBlend", &self.m_doSubtractiveBlend)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "childrenInternalStates",
                    &self.m_childrenInternalStates,
                )?;
            serializer.serialize_array_field("sortedChildren", &self.m_sortedChildren)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_childrenInternalStates,
    m_sortedChildren,
    m_endIntervalWeight,
    m_numActiveChildren,
    m_beginIntervalIndex,
    m_endIntervalIndex,
    m_initSync,
    m_doSubtractiveBlend,
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
            "childrenInternalStates" => Ok(__Field::m_childrenInternalStates),
            "sortedChildren" => Ok(__Field::m_sortedChildren),
            "endIntervalWeight" => Ok(__Field::m_endIntervalWeight),
            "numActiveChildren" => Ok(__Field::m_numActiveChildren),
            "beginIntervalIndex" => Ok(__Field::m_beginIntervalIndex),
            "endIntervalIndex" => Ok(__Field::m_endIntervalIndex),
            "initSync" => Ok(__Field::m_initSync),
            "doSubtractiveBlend" => Ok(__Field::m_doSubtractiveBlend),
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
pub(super) struct __hkbBlenderGeneratorInternalStateVisitor<'de> {
    marker: core::marker::PhantomData<hkbBlenderGeneratorInternalState>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbBlenderGeneratorInternalStateVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbBlenderGeneratorInternalState, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbBlenderGeneratorInternalState,
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
impl<'de> _serde::de::Visitor<'de> for __hkbBlenderGeneratorInternalStateVisitor<'de> {
    type Value = hkbBlenderGeneratorInternalState;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbBlenderGeneratorInternalState",
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
        let parent = __A::next_value(&mut __map)?;
        let mut m_childrenInternalStates: _serde::__private::Option<
            Vec<hkbBlenderGeneratorChildInternalState>,
        > = _serde::__private::None;
        let mut m_sortedChildren: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
        let mut m_endIntervalWeight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_numActiveChildren: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_beginIntervalIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endIntervalIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_initSync: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_doSubtractiveBlend: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_childrenInternalStates) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childrenInternalStates",
                            ),
                        );
                    }
                    m_childrenInternalStates = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkbBlenderGeneratorChildInternalState>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_sortedChildren) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sortedChildren",
                            ),
                        );
                    }
                    m_sortedChildren = _serde::__private::Some(
                        match __A::next_value::<Vec<i16>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_endIntervalWeight) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endIntervalWeight",
                            ),
                        );
                    }
                    m_endIntervalWeight = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_numActiveChildren) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numActiveChildren",
                            ),
                        );
                    }
                    m_numActiveChildren = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_beginIntervalIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "beginIntervalIndex",
                            ),
                        );
                    }
                    m_beginIntervalIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_endIntervalIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endIntervalIndex",
                            ),
                        );
                    }
                    m_endIntervalIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_initSync) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "initSync",
                            ),
                        );
                    }
                    m_initSync = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_doSubtractiveBlend) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "doSubtractiveBlend",
                            ),
                        );
                    }
                    m_doSubtractiveBlend = _serde::__private::Some(
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
        let m_childrenInternalStates = match m_childrenInternalStates {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "childrenInternalStates",
                    ),
                );
            }
        };
        let m_sortedChildren = match m_sortedChildren {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sortedChildren"),
                );
            }
        };
        let m_endIntervalWeight = match m_endIntervalWeight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endIntervalWeight"),
                );
            }
        };
        let m_numActiveChildren = match m_numActiveChildren {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numActiveChildren"),
                );
            }
        };
        let m_beginIntervalIndex = match m_beginIntervalIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "beginIntervalIndex",
                    ),
                );
            }
        };
        let m_endIntervalIndex = match m_endIntervalIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endIntervalIndex"),
                );
            }
        };
        let m_initSync = match m_initSync {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initSync"),
                );
            }
        };
        let m_doSubtractiveBlend = match m_doSubtractiveBlend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "doSubtractiveBlend",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbBlenderGeneratorInternalState {
            __ptr,
            parent,
            m_childrenInternalStates,
            m_sortedChildren,
            m_endIntervalWeight,
            m_numActiveChildren,
            m_beginIntervalIndex,
            m_endIntervalIndex,
            m_initSync,
            m_doSubtractiveBlend,
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
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_childrenInternalStates: _serde::__private::Option<
            Vec<hkbBlenderGeneratorChildInternalState>,
        > = _serde::__private::None;
        let mut m_sortedChildren: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
        let mut m_endIntervalWeight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_numActiveChildren: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_beginIntervalIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endIntervalIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_initSync: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_doSubtractiveBlend: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..8usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_childrenInternalStates => {
                        if _serde::__private::Option::is_some(
                            &m_childrenInternalStates,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "childrenInternalStates",
                                ),
                            );
                        }
                        m_childrenInternalStates = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkbBlenderGeneratorChildInternalState>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_sortedChildren => {
                        if _serde::__private::Option::is_some(&m_sortedChildren) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sortedChildren",
                                ),
                            );
                        }
                        m_sortedChildren = _serde::__private::Some(
                            match __A::next_value::<Vec<i16>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_endIntervalWeight => {
                        if _serde::__private::Option::is_some(&m_endIntervalWeight) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "endIntervalWeight",
                                ),
                            );
                        }
                        m_endIntervalWeight = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numActiveChildren => {
                        if _serde::__private::Option::is_some(&m_numActiveChildren) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numActiveChildren",
                                ),
                            );
                        }
                        m_numActiveChildren = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_beginIntervalIndex => {
                        if _serde::__private::Option::is_some(&m_beginIntervalIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "beginIntervalIndex",
                                ),
                            );
                        }
                        m_beginIntervalIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_endIntervalIndex => {
                        if _serde::__private::Option::is_some(&m_endIntervalIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "endIntervalIndex",
                                ),
                            );
                        }
                        m_endIntervalIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_initSync => {
                        if _serde::__private::Option::is_some(&m_initSync) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "initSync",
                                ),
                            );
                        }
                        m_initSync = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_doSubtractiveBlend => {
                        if _serde::__private::Option::is_some(&m_doSubtractiveBlend) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "doSubtractiveBlend",
                                ),
                            );
                        }
                        m_doSubtractiveBlend = _serde::__private::Some(
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
        let m_childrenInternalStates = match m_childrenInternalStates {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "childrenInternalStates",
                    ),
                );
            }
        };
        let m_sortedChildren = match m_sortedChildren {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sortedChildren"),
                );
            }
        };
        let m_endIntervalWeight = match m_endIntervalWeight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endIntervalWeight"),
                );
            }
        };
        let m_numActiveChildren = match m_numActiveChildren {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numActiveChildren"),
                );
            }
        };
        let m_beginIntervalIndex = match m_beginIntervalIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "beginIntervalIndex",
                    ),
                );
            }
        };
        let m_endIntervalIndex = match m_endIntervalIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endIntervalIndex"),
                );
            }
        };
        let m_initSync = match m_initSync {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initSync"),
                );
            }
        };
        let m_doSubtractiveBlend = match m_doSubtractiveBlend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "doSubtractiveBlend",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbBlenderGeneratorInternalState {
            __ptr,
            parent,
            m_childrenInternalStates,
            m_sortedChildren,
            m_endIntervalWeight,
            m_numActiveChildren,
            m_beginIntervalIndex,
            m_endIntervalIndex,
            m_initSync,
            m_doSubtractiveBlend,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbBlenderGeneratorInternalState {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "childrenInternalStates",
                "sortedChildren",
                "endIntervalWeight",
                "numActiveChildren",
                "beginIntervalIndex",
                "endIntervalIndex",
                "initSync",
                "doSubtractiveBlend",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBlenderGeneratorInternalState",
                FIELDS,
                __hkbBlenderGeneratorInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbBlenderGeneratorInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
