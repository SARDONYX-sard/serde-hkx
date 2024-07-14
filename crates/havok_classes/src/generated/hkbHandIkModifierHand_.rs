use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbHandIkModifierHand`
/// -         version: `3`
/// -       signature: `0x14dfe1dd`
/// -          size:  96(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbHandIkModifierHand<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `elbowAxisLS`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_elbowAxisLS: Vector4,
    /// # C++ Info
    /// -          name: `backHandNormalLS`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_backHandNormalLS: Vector4,
    /// # C++ Info
    /// -          name: `handOffsetLS`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_handOffsetLS: Vector4,
    /// # C++ Info
    /// -          name: `handOrienationOffsetLS`(ctype: `hkQuaternion`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_handOrienationOffsetLS: Quaternion,
    /// # C++ Info
    /// -          name: `maxElbowAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxElbowAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `minElbowAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minElbowAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `shoulderIndex`(ctype: `hkInt16`)
    /// -        offset:  72(x86)/ 72(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_shoulderIndex: i16,
    /// # C++ Info
    /// -          name: `shoulderSiblingIndex`(ctype: `hkInt16`)
    /// -        offset:  74(x86)/ 74(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_shoulderSiblingIndex: i16,
    /// # C++ Info
    /// -          name: `elbowIndex`(ctype: `hkInt16`)
    /// -        offset:  76(x86)/ 76(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_elbowIndex: i16,
    /// # C++ Info
    /// -          name: `elbowSiblingIndex`(ctype: `hkInt16`)
    /// -        offset:  78(x86)/ 78(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_elbowSiblingIndex: i16,
    /// # C++ Info
    /// -          name: `wristIndex`(ctype: `hkInt16`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_wristIndex: i16,
    /// # C++ Info
    /// -          name: `enforceEndPosition`(ctype: `hkBool`)
    /// -        offset:  82(x86)/ 82(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enforceEndPosition: bool,
    /// # C++ Info
    /// -          name: `enforceEndRotation`(ctype: `hkBool`)
    /// -        offset:  83(x86)/ 83(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enforceEndRotation: bool,
    /// # C++ Info
    /// -          name: `localFrameName`(ctype: `hkStringPtr`)
    /// -        offset:  84(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_localFrameName: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbHandIkModifierHand<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbHandIkModifierHand"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x14dfe1dd)
        }
    }
    impl<'a> _serde::Serialize for hkbHandIkModifierHand<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x14dfe1dd)));
            let mut serializer = __serializer
                .serialize_struct("hkbHandIkModifierHand", class_meta)?;
            serializer.serialize_field("elbowAxisLS", &self.m_elbowAxisLS)?;
            serializer.serialize_field("backHandNormalLS", &self.m_backHandNormalLS)?;
            serializer.serialize_field("handOffsetLS", &self.m_handOffsetLS)?;
            serializer
                .serialize_field(
                    "handOrienationOffsetLS",
                    &self.m_handOrienationOffsetLS,
                )?;
            serializer
                .serialize_field("maxElbowAngleDegrees", &self.m_maxElbowAngleDegrees)?;
            serializer
                .serialize_field("minElbowAngleDegrees", &self.m_minElbowAngleDegrees)?;
            serializer.serialize_field("shoulderIndex", &self.m_shoulderIndex)?;
            serializer
                .serialize_field("shoulderSiblingIndex", &self.m_shoulderSiblingIndex)?;
            serializer.serialize_field("elbowIndex", &self.m_elbowIndex)?;
            serializer.serialize_field("elbowSiblingIndex", &self.m_elbowSiblingIndex)?;
            serializer.serialize_field("wristIndex", &self.m_wristIndex)?;
            serializer
                .serialize_field("enforceEndPosition", &self.m_enforceEndPosition)?;
            serializer
                .serialize_field("enforceEndRotation", &self.m_enforceEndRotation)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "localFrameName",
                    &self.m_localFrameName,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_stringptr_field("localFrameName", &self.m_localFrameName)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_elbowAxisLS,
    m_backHandNormalLS,
    m_handOffsetLS,
    m_handOrienationOffsetLS,
    m_maxElbowAngleDegrees,
    m_minElbowAngleDegrees,
    m_shoulderIndex,
    m_shoulderSiblingIndex,
    m_elbowIndex,
    m_elbowSiblingIndex,
    m_wristIndex,
    m_enforceEndPosition,
    m_enforceEndRotation,
    m_localFrameName,
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
            "elbowAxisLS" => Ok(__Field::m_elbowAxisLS),
            "backHandNormalLS" => Ok(__Field::m_backHandNormalLS),
            "handOffsetLS" => Ok(__Field::m_handOffsetLS),
            "handOrienationOffsetLS" => Ok(__Field::m_handOrienationOffsetLS),
            "maxElbowAngleDegrees" => Ok(__Field::m_maxElbowAngleDegrees),
            "minElbowAngleDegrees" => Ok(__Field::m_minElbowAngleDegrees),
            "shoulderIndex" => Ok(__Field::m_shoulderIndex),
            "shoulderSiblingIndex" => Ok(__Field::m_shoulderSiblingIndex),
            "elbowIndex" => Ok(__Field::m_elbowIndex),
            "elbowSiblingIndex" => Ok(__Field::m_elbowSiblingIndex),
            "wristIndex" => Ok(__Field::m_wristIndex),
            "enforceEndPosition" => Ok(__Field::m_enforceEndPosition),
            "enforceEndRotation" => Ok(__Field::m_enforceEndRotation),
            "localFrameName" => Ok(__Field::m_localFrameName),
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
pub(super) struct __hkbHandIkModifierHandVisitor<'de> {
    marker: core::marker::PhantomData<hkbHandIkModifierHand<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbHandIkModifierHandVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbHandIkModifierHand<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbHandIkModifierHand<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbHandIkModifierHandVisitor<'de> {
    type Value = hkbHandIkModifierHand<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbHandIkModifierHand")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_elbowAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_backHandNormalLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_handOffsetLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_handOrienationOffsetLS: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_maxElbowAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minElbowAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_shoulderIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_shoulderSiblingIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_elbowIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_elbowSiblingIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_wristIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_enforceEndPosition: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_enforceEndRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_localFrameName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for i in 0..14usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_elbowAxisLS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "elbowAxisLS",
                            ),
                        );
                    }
                    m_elbowAxisLS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_backHandNormalLS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "backHandNormalLS",
                            ),
                        );
                    }
                    m_backHandNormalLS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_handOffsetLS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "handOffsetLS",
                            ),
                        );
                    }
                    m_handOffsetLS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_handOrienationOffsetLS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "handOrienationOffsetLS",
                            ),
                        );
                    }
                    m_handOrienationOffsetLS = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_maxElbowAngleDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxElbowAngleDegrees",
                            ),
                        );
                    }
                    m_maxElbowAngleDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_minElbowAngleDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minElbowAngleDegrees",
                            ),
                        );
                    }
                    m_minElbowAngleDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_shoulderIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "shoulderIndex",
                            ),
                        );
                    }
                    m_shoulderIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_shoulderSiblingIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "shoulderSiblingIndex",
                            ),
                        );
                    }
                    m_shoulderSiblingIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_elbowIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "elbowIndex",
                            ),
                        );
                    }
                    m_elbowIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_elbowSiblingIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "elbowSiblingIndex",
                            ),
                        );
                    }
                    m_elbowSiblingIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_wristIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wristIndex",
                            ),
                        );
                    }
                    m_wristIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_enforceEndPosition) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enforceEndPosition",
                            ),
                        );
                    }
                    m_enforceEndPosition = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_enforceEndRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enforceEndRotation",
                            ),
                        );
                    }
                    m_enforceEndRotation = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_localFrameName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "localFrameName",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_localFrameName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        __A::pad(&mut __map, 8usize, 0usize)?;
        let m_elbowAxisLS = match m_elbowAxisLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("elbowAxisLS"),
                );
            }
        };
        let m_backHandNormalLS = match m_backHandNormalLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("backHandNormalLS"),
                );
            }
        };
        let m_handOffsetLS = match m_handOffsetLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("handOffsetLS"),
                );
            }
        };
        let m_handOrienationOffsetLS = match m_handOrienationOffsetLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "handOrienationOffsetLS",
                    ),
                );
            }
        };
        let m_maxElbowAngleDegrees = match m_maxElbowAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxElbowAngleDegrees",
                    ),
                );
            }
        };
        let m_minElbowAngleDegrees = match m_minElbowAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minElbowAngleDegrees",
                    ),
                );
            }
        };
        let m_shoulderIndex = match m_shoulderIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shoulderIndex"),
                );
            }
        };
        let m_shoulderSiblingIndex = match m_shoulderSiblingIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "shoulderSiblingIndex",
                    ),
                );
            }
        };
        let m_elbowIndex = match m_elbowIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("elbowIndex"),
                );
            }
        };
        let m_elbowSiblingIndex = match m_elbowSiblingIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("elbowSiblingIndex"),
                );
            }
        };
        let m_wristIndex = match m_wristIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wristIndex"),
                );
            }
        };
        let m_enforceEndPosition = match m_enforceEndPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enforceEndPosition",
                    ),
                );
            }
        };
        let m_enforceEndRotation = match m_enforceEndRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enforceEndRotation",
                    ),
                );
            }
        };
        let m_localFrameName = match m_localFrameName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localFrameName"),
                );
            }
        };
        _serde::__private::Ok(hkbHandIkModifierHand {
            __ptr,
            m_elbowAxisLS,
            m_backHandNormalLS,
            m_handOffsetLS,
            m_handOrienationOffsetLS,
            m_maxElbowAngleDegrees,
            m_minElbowAngleDegrees,
            m_shoulderIndex,
            m_shoulderSiblingIndex,
            m_elbowIndex,
            m_elbowSiblingIndex,
            m_wristIndex,
            m_enforceEndPosition,
            m_enforceEndRotation,
            m_localFrameName,
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
        let mut m_elbowAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_backHandNormalLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_handOffsetLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_handOrienationOffsetLS: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_maxElbowAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minElbowAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_shoulderIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_shoulderSiblingIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_elbowIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_elbowSiblingIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_wristIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_enforceEndPosition: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_enforceEndRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_localFrameName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for _ in 0..14usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_elbowAxisLS => {
                        if _serde::__private::Option::is_some(&m_elbowAxisLS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "elbowAxisLS",
                                ),
                            );
                        }
                        m_elbowAxisLS = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_backHandNormalLS => {
                        if _serde::__private::Option::is_some(&m_backHandNormalLS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "backHandNormalLS",
                                ),
                            );
                        }
                        m_backHandNormalLS = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_handOffsetLS => {
                        if _serde::__private::Option::is_some(&m_handOffsetLS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "handOffsetLS",
                                ),
                            );
                        }
                        m_handOffsetLS = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_handOrienationOffsetLS => {
                        if _serde::__private::Option::is_some(
                            &m_handOrienationOffsetLS,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "handOrienationOffsetLS",
                                ),
                            );
                        }
                        m_handOrienationOffsetLS = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxElbowAngleDegrees => {
                        if _serde::__private::Option::is_some(&m_maxElbowAngleDegrees) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxElbowAngleDegrees",
                                ),
                            );
                        }
                        m_maxElbowAngleDegrees = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_minElbowAngleDegrees => {
                        if _serde::__private::Option::is_some(&m_minElbowAngleDegrees) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "minElbowAngleDegrees",
                                ),
                            );
                        }
                        m_minElbowAngleDegrees = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_shoulderIndex => {
                        if _serde::__private::Option::is_some(&m_shoulderIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "shoulderIndex",
                                ),
                            );
                        }
                        m_shoulderIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_shoulderSiblingIndex => {
                        if _serde::__private::Option::is_some(&m_shoulderSiblingIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "shoulderSiblingIndex",
                                ),
                            );
                        }
                        m_shoulderSiblingIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_elbowIndex => {
                        if _serde::__private::Option::is_some(&m_elbowIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "elbowIndex",
                                ),
                            );
                        }
                        m_elbowIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_elbowSiblingIndex => {
                        if _serde::__private::Option::is_some(&m_elbowSiblingIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "elbowSiblingIndex",
                                ),
                            );
                        }
                        m_elbowSiblingIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_wristIndex => {
                        if _serde::__private::Option::is_some(&m_wristIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "wristIndex",
                                ),
                            );
                        }
                        m_wristIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_enforceEndPosition => {
                        if _serde::__private::Option::is_some(&m_enforceEndPosition) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "enforceEndPosition",
                                ),
                            );
                        }
                        m_enforceEndPosition = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_enforceEndRotation => {
                        if _serde::__private::Option::is_some(&m_enforceEndRotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "enforceEndRotation",
                                ),
                            );
                        }
                        m_enforceEndRotation = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_localFrameName => {
                        if _serde::__private::Option::is_some(&m_localFrameName) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "localFrameName",
                                ),
                            );
                        }
                        m_localFrameName = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        let m_elbowAxisLS = match m_elbowAxisLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("elbowAxisLS"),
                );
            }
        };
        let m_backHandNormalLS = match m_backHandNormalLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("backHandNormalLS"),
                );
            }
        };
        let m_handOffsetLS = match m_handOffsetLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("handOffsetLS"),
                );
            }
        };
        let m_handOrienationOffsetLS = match m_handOrienationOffsetLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "handOrienationOffsetLS",
                    ),
                );
            }
        };
        let m_maxElbowAngleDegrees = match m_maxElbowAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxElbowAngleDegrees",
                    ),
                );
            }
        };
        let m_minElbowAngleDegrees = match m_minElbowAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minElbowAngleDegrees",
                    ),
                );
            }
        };
        let m_shoulderIndex = match m_shoulderIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shoulderIndex"),
                );
            }
        };
        let m_shoulderSiblingIndex = match m_shoulderSiblingIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "shoulderSiblingIndex",
                    ),
                );
            }
        };
        let m_elbowIndex = match m_elbowIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("elbowIndex"),
                );
            }
        };
        let m_elbowSiblingIndex = match m_elbowSiblingIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("elbowSiblingIndex"),
                );
            }
        };
        let m_wristIndex = match m_wristIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wristIndex"),
                );
            }
        };
        let m_enforceEndPosition = match m_enforceEndPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enforceEndPosition",
                    ),
                );
            }
        };
        let m_enforceEndRotation = match m_enforceEndRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enforceEndRotation",
                    ),
                );
            }
        };
        let m_localFrameName = match m_localFrameName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localFrameName"),
                );
            }
        };
        _serde::__private::Ok(hkbHandIkModifierHand {
            __ptr,
            m_elbowAxisLS,
            m_backHandNormalLS,
            m_handOffsetLS,
            m_handOrienationOffsetLS,
            m_maxElbowAngleDegrees,
            m_minElbowAngleDegrees,
            m_shoulderIndex,
            m_shoulderSiblingIndex,
            m_elbowIndex,
            m_elbowSiblingIndex,
            m_wristIndex,
            m_enforceEndPosition,
            m_enforceEndRotation,
            m_localFrameName,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbHandIkModifierHand<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "elbowAxisLS",
                "backHandNormalLS",
                "handOffsetLS",
                "handOrienationOffsetLS",
                "maxElbowAngleDegrees",
                "minElbowAngleDegrees",
                "shoulderIndex",
                "shoulderSiblingIndex",
                "elbowIndex",
                "elbowSiblingIndex",
                "wristIndex",
                "enforceEndPosition",
                "enforceEndRotation",
                "localFrameName",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbHandIkModifierHand",
                FIELDS,
                __hkbHandIkModifierHandVisitor {
                    marker: _serde::__private::PhantomData::<hkbHandIkModifierHand>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
