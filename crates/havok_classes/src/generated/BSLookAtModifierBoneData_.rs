use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSLookAtModifierBoneData`
/// -         version: `0`
/// -       signature: `0x29efee59`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSLookAtModifierBoneData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `index`(ctype: `hkInt16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_index: i16,
    /// # C++ Info
    /// -          name: `fwdAxisLS`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_fwdAxisLS: Vector4,
    /// # C++ Info
    /// -          name: `limitAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `onGain`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_onGain: f32,
    /// # C++ Info
    /// -          name: `offGain`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offGain: f32,
    /// # C++ Info
    /// -          name: `enabled`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 44(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enabled: bool,
    /// # C++ Info
    /// -          name: `currentFwdAxisLS`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_currentFwdAxisLS: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for BSLookAtModifierBoneData {
        #[inline]
        fn name(&self) -> &'static str {
            "BSLookAtModifierBoneData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x29efee59)
        }
    }
    impl _serde::Serialize for BSLookAtModifierBoneData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x29efee59)));
            let mut serializer = __serializer
                .serialize_struct("BSLookAtModifierBoneData", class_meta)?;
            serializer.serialize_field("index", &self.m_index)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("fwdAxisLS", &self.m_fwdAxisLS)?;
            serializer.serialize_field("limitAngleDegrees", &self.m_limitAngleDegrees)?;
            serializer.serialize_field("onGain", &self.m_onGain)?;
            serializer.serialize_field("offGain", &self.m_offGain)?;
            serializer.serialize_field("enabled", &self.m_enabled)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("currentFwdAxisLS", &self.m_currentFwdAxisLS)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_index,
    m_fwdAxisLS,
    m_limitAngleDegrees,
    m_onGain,
    m_offGain,
    m_enabled,
    m_currentFwdAxisLS,
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
            "index" => Ok(__Field::m_index),
            "fwdAxisLS" => Ok(__Field::m_fwdAxisLS),
            "limitAngleDegrees" => Ok(__Field::m_limitAngleDegrees),
            "onGain" => Ok(__Field::m_onGain),
            "offGain" => Ok(__Field::m_offGain),
            "enabled" => Ok(__Field::m_enabled),
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
pub(super) struct __BSLookAtModifierBoneDataVisitor<'de> {
    marker: core::marker::PhantomData<BSLookAtModifierBoneData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSLookAtModifierBoneDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSLookAtModifierBoneData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSLookAtModifierBoneData>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSLookAtModifierBoneDataVisitor<'de> {
    type Value = BSLookAtModifierBoneData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSLookAtModifierBoneData")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_index: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_fwdAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_enabled: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_currentFwdAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_index) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("index"),
                        );
                    }
                    m_index = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_fwdAxisLS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "fwdAxisLS",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 14usize, 14usize)?;
                    m_fwdAxisLS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_limitAngleDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitAngleDegrees",
                            ),
                        );
                    }
                    m_limitAngleDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_onGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("onGain"),
                        );
                    }
                    m_onGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_offGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("offGain"),
                        );
                    }
                    m_offGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_enabled) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("enabled"),
                        );
                    }
                    m_enabled = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_currentFwdAxisLS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentFwdAxisLS",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_currentFwdAxisLS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
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
        let m_index = match m_index {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("index"),
                );
            }
        };
        let m_fwdAxisLS = match m_fwdAxisLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fwdAxisLS"),
                );
            }
        };
        let m_limitAngleDegrees = match m_limitAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDegrees"),
                );
            }
        };
        let m_onGain = match m_onGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onGain"),
                );
            }
        };
        let m_offGain = match m_offGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offGain"),
                );
            }
        };
        let m_enabled = match m_enabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enabled"),
                );
            }
        };
        let m_currentFwdAxisLS = match m_currentFwdAxisLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("currentFwdAxisLS"),
                );
            }
        };
        _serde::__private::Ok(BSLookAtModifierBoneData {
            __ptr: __A::class_ptr(&mut __map),
            m_index,
            m_fwdAxisLS,
            m_limitAngleDegrees,
            m_onGain,
            m_offGain,
            m_enabled,
            m_currentFwdAxisLS,
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
        let mut m_index: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_fwdAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_enabled: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_index => {
                        if _serde::__private::Option::is_some(&m_index) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("index"),
                            );
                        }
                        m_index = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_fwdAxisLS => {
                        if _serde::__private::Option::is_some(&m_fwdAxisLS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "fwdAxisLS",
                                ),
                            );
                        }
                        m_fwdAxisLS = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_limitAngleDegrees => {
                        if _serde::__private::Option::is_some(&m_limitAngleDegrees) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "limitAngleDegrees",
                                ),
                            );
                        }
                        m_limitAngleDegrees = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_onGain => {
                        if _serde::__private::Option::is_some(&m_onGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("onGain"),
                            );
                        }
                        m_onGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_offGain => {
                        if _serde::__private::Option::is_some(&m_offGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "offGain",
                                ),
                            );
                        }
                        m_offGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_enabled => {
                        if _serde::__private::Option::is_some(&m_enabled) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "enabled",
                                ),
                            );
                        }
                        m_enabled = _serde::__private::Some(
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
        let m_index = match m_index {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("index"),
                );
            }
        };
        let m_fwdAxisLS = match m_fwdAxisLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fwdAxisLS"),
                );
            }
        };
        let m_limitAngleDegrees = match m_limitAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDegrees"),
                );
            }
        };
        let m_onGain = match m_onGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onGain"),
                );
            }
        };
        let m_offGain = match m_offGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offGain"),
                );
            }
        };
        let m_enabled = match m_enabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enabled"),
                );
            }
        };
        _serde::__private::Ok(BSLookAtModifierBoneData {
            __ptr,
            m_index,
            m_fwdAxisLS,
            m_limitAngleDegrees,
            m_onGain,
            m_offGain,
            m_enabled,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSLookAtModifierBoneData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "index",
                "fwdAxisLS",
                "limitAngleDegrees",
                "onGain",
                "offGain",
                "enabled",
                "currentFwdAxisLS",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSLookAtModifierBoneData",
                FIELDS,
                __BSLookAtModifierBoneDataVisitor {
                    marker: _serde::__private::PhantomData::<BSLookAtModifierBoneData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
