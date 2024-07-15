use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbFootIkGains`
/// - version: `0`
/// - signature: `0xa681b7f0`
/// - size: ` 48`(x86)/` 48`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkGains {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `onOffGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_onOffGain: f32,
    /// # C++ Info
    /// - name: `groundAscendingGain`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_groundAscendingGain: f32,
    /// # C++ Info
    /// - name: `groundDescendingGain`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_groundDescendingGain: f32,
    /// # C++ Info
    /// - name: `footPlantedGain`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_footPlantedGain: f32,
    /// # C++ Info
    /// - name: `footRaisedGain`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_footRaisedGain: f32,
    /// # C++ Info
    /// - name: `footUnlockGain`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_footUnlockGain: f32,
    /// # C++ Info
    /// - name: `worldFromModelFeedbackGain`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_worldFromModelFeedbackGain: f32,
    /// # C++ Info
    /// - name: `errorUpDownBias`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_errorUpDownBias: f32,
    /// # C++ Info
    /// - name: `alignWorldFromModelGain`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_alignWorldFromModelGain: f32,
    /// # C++ Info
    /// - name: `hipOrientationGain`(ctype: `hkReal`)
    /// - offset: ` 36`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_hipOrientationGain: f32,
    /// # C++ Info
    /// - name: `maxKneeAngleDifference`(ctype: `hkReal`)
    /// - offset: ` 40`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxKneeAngleDifference: f32,
    /// # C++ Info
    /// - name: `ankleOrientationGain`(ctype: `hkReal`)
    /// - offset: ` 44`(x86)/` 44`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_ankleOrientationGain: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbFootIkGains {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkGains"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa681b7f0)
        }
    }
    impl _serde::Serialize for hkbFootIkGains {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa681b7f0)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkGains", class_meta)?;
            serializer.serialize_field("onOffGain", &self.m_onOffGain)?;
            serializer
                .serialize_field("groundAscendingGain", &self.m_groundAscendingGain)?;
            serializer
                .serialize_field("groundDescendingGain", &self.m_groundDescendingGain)?;
            serializer.serialize_field("footPlantedGain", &self.m_footPlantedGain)?;
            serializer.serialize_field("footRaisedGain", &self.m_footRaisedGain)?;
            serializer.serialize_field("footUnlockGain", &self.m_footUnlockGain)?;
            serializer
                .serialize_field(
                    "worldFromModelFeedbackGain",
                    &self.m_worldFromModelFeedbackGain,
                )?;
            serializer.serialize_field("errorUpDownBias", &self.m_errorUpDownBias)?;
            serializer
                .serialize_field(
                    "alignWorldFromModelGain",
                    &self.m_alignWorldFromModelGain,
                )?;
            serializer
                .serialize_field("hipOrientationGain", &self.m_hipOrientationGain)?;
            serializer
                .serialize_field(
                    "maxKneeAngleDifference",
                    &self.m_maxKneeAngleDifference,
                )?;
            serializer
                .serialize_field("ankleOrientationGain", &self.m_ankleOrientationGain)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_onOffGain,
    m_groundAscendingGain,
    m_groundDescendingGain,
    m_footPlantedGain,
    m_footRaisedGain,
    m_footUnlockGain,
    m_worldFromModelFeedbackGain,
    m_errorUpDownBias,
    m_alignWorldFromModelGain,
    m_hipOrientationGain,
    m_maxKneeAngleDifference,
    m_ankleOrientationGain,
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
            "onOffGain" => Ok(__Field::m_onOffGain),
            "groundAscendingGain" => Ok(__Field::m_groundAscendingGain),
            "groundDescendingGain" => Ok(__Field::m_groundDescendingGain),
            "footPlantedGain" => Ok(__Field::m_footPlantedGain),
            "footRaisedGain" => Ok(__Field::m_footRaisedGain),
            "footUnlockGain" => Ok(__Field::m_footUnlockGain),
            "worldFromModelFeedbackGain" => Ok(__Field::m_worldFromModelFeedbackGain),
            "errorUpDownBias" => Ok(__Field::m_errorUpDownBias),
            "alignWorldFromModelGain" => Ok(__Field::m_alignWorldFromModelGain),
            "hipOrientationGain" => Ok(__Field::m_hipOrientationGain),
            "maxKneeAngleDifference" => Ok(__Field::m_maxKneeAngleDifference),
            "ankleOrientationGain" => Ok(__Field::m_ankleOrientationGain),
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
pub(super) struct __hkbFootIkGainsVisitor<'de> {
    marker: core::marker::PhantomData<hkbFootIkGains>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbFootIkGainsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbFootIkGains, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbFootIkGains>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbFootIkGainsVisitor<'de> {
    type Value = hkbFootIkGains;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbFootIkGains")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_onOffGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_groundAscendingGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_groundDescendingGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_footPlantedGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_footRaisedGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_footUnlockGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_worldFromModelFeedbackGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_errorUpDownBias: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_alignWorldFromModelGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_hipOrientationGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxKneeAngleDifference: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_ankleOrientationGain: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..12usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_onOffGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "onOffGain",
                            ),
                        );
                    }
                    m_onOffGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_groundAscendingGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "groundAscendingGain",
                            ),
                        );
                    }
                    m_groundAscendingGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_groundDescendingGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "groundDescendingGain",
                            ),
                        );
                    }
                    m_groundDescendingGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_footPlantedGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "footPlantedGain",
                            ),
                        );
                    }
                    m_footPlantedGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_footRaisedGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "footRaisedGain",
                            ),
                        );
                    }
                    m_footRaisedGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_footUnlockGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "footUnlockGain",
                            ),
                        );
                    }
                    m_footUnlockGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(
                        &m_worldFromModelFeedbackGain,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "worldFromModelFeedbackGain",
                            ),
                        );
                    }
                    m_worldFromModelFeedbackGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_errorUpDownBias) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "errorUpDownBias",
                            ),
                        );
                    }
                    m_errorUpDownBias = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_alignWorldFromModelGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "alignWorldFromModelGain",
                            ),
                        );
                    }
                    m_alignWorldFromModelGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_hipOrientationGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hipOrientationGain",
                            ),
                        );
                    }
                    m_hipOrientationGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_maxKneeAngleDifference) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxKneeAngleDifference",
                            ),
                        );
                    }
                    m_maxKneeAngleDifference = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_ankleOrientationGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ankleOrientationGain",
                            ),
                        );
                    }
                    m_ankleOrientationGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
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
        let m_onOffGain = match m_onOffGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onOffGain"),
                );
            }
        };
        let m_groundAscendingGain = match m_groundAscendingGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "groundAscendingGain",
                    ),
                );
            }
        };
        let m_groundDescendingGain = match m_groundDescendingGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "groundDescendingGain",
                    ),
                );
            }
        };
        let m_footPlantedGain = match m_footPlantedGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("footPlantedGain"),
                );
            }
        };
        let m_footRaisedGain = match m_footRaisedGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("footRaisedGain"),
                );
            }
        };
        let m_footUnlockGain = match m_footUnlockGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("footUnlockGain"),
                );
            }
        };
        let m_worldFromModelFeedbackGain = match m_worldFromModelFeedbackGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "worldFromModelFeedbackGain",
                    ),
                );
            }
        };
        let m_errorUpDownBias = match m_errorUpDownBias {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("errorUpDownBias"),
                );
            }
        };
        let m_alignWorldFromModelGain = match m_alignWorldFromModelGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "alignWorldFromModelGain",
                    ),
                );
            }
        };
        let m_hipOrientationGain = match m_hipOrientationGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hipOrientationGain",
                    ),
                );
            }
        };
        let m_maxKneeAngleDifference = match m_maxKneeAngleDifference {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxKneeAngleDifference",
                    ),
                );
            }
        };
        let m_ankleOrientationGain = match m_ankleOrientationGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "ankleOrientationGain",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbFootIkGains {
            __ptr,
            m_onOffGain,
            m_groundAscendingGain,
            m_groundDescendingGain,
            m_footPlantedGain,
            m_footRaisedGain,
            m_footUnlockGain,
            m_worldFromModelFeedbackGain,
            m_errorUpDownBias,
            m_alignWorldFromModelGain,
            m_hipOrientationGain,
            m_maxKneeAngleDifference,
            m_ankleOrientationGain,
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
        let mut m_onOffGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_groundAscendingGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_groundDescendingGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_footPlantedGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_footRaisedGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_footUnlockGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_worldFromModelFeedbackGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_errorUpDownBias: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_alignWorldFromModelGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_hipOrientationGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxKneeAngleDifference: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_ankleOrientationGain: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..12usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_onOffGain => {
                        if _serde::__private::Option::is_some(&m_onOffGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "onOffGain",
                                ),
                            );
                        }
                        m_onOffGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_groundAscendingGain => {
                        if _serde::__private::Option::is_some(&m_groundAscendingGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "groundAscendingGain",
                                ),
                            );
                        }
                        m_groundAscendingGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_groundDescendingGain => {
                        if _serde::__private::Option::is_some(&m_groundDescendingGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "groundDescendingGain",
                                ),
                            );
                        }
                        m_groundDescendingGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_footPlantedGain => {
                        if _serde::__private::Option::is_some(&m_footPlantedGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "footPlantedGain",
                                ),
                            );
                        }
                        m_footPlantedGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_footRaisedGain => {
                        if _serde::__private::Option::is_some(&m_footRaisedGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "footRaisedGain",
                                ),
                            );
                        }
                        m_footRaisedGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_footUnlockGain => {
                        if _serde::__private::Option::is_some(&m_footUnlockGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "footUnlockGain",
                                ),
                            );
                        }
                        m_footUnlockGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_worldFromModelFeedbackGain => {
                        if _serde::__private::Option::is_some(
                            &m_worldFromModelFeedbackGain,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "worldFromModelFeedbackGain",
                                ),
                            );
                        }
                        m_worldFromModelFeedbackGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_errorUpDownBias => {
                        if _serde::__private::Option::is_some(&m_errorUpDownBias) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "errorUpDownBias",
                                ),
                            );
                        }
                        m_errorUpDownBias = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_alignWorldFromModelGain => {
                        if _serde::__private::Option::is_some(
                            &m_alignWorldFromModelGain,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "alignWorldFromModelGain",
                                ),
                            );
                        }
                        m_alignWorldFromModelGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_hipOrientationGain => {
                        if _serde::__private::Option::is_some(&m_hipOrientationGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "hipOrientationGain",
                                ),
                            );
                        }
                        m_hipOrientationGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxKneeAngleDifference => {
                        if _serde::__private::Option::is_some(
                            &m_maxKneeAngleDifference,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxKneeAngleDifference",
                                ),
                            );
                        }
                        m_maxKneeAngleDifference = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_ankleOrientationGain => {
                        if _serde::__private::Option::is_some(&m_ankleOrientationGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ankleOrientationGain",
                                ),
                            );
                        }
                        m_ankleOrientationGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
        let m_onOffGain = match m_onOffGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onOffGain"),
                );
            }
        };
        let m_groundAscendingGain = match m_groundAscendingGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "groundAscendingGain",
                    ),
                );
            }
        };
        let m_groundDescendingGain = match m_groundDescendingGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "groundDescendingGain",
                    ),
                );
            }
        };
        let m_footPlantedGain = match m_footPlantedGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("footPlantedGain"),
                );
            }
        };
        let m_footRaisedGain = match m_footRaisedGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("footRaisedGain"),
                );
            }
        };
        let m_footUnlockGain = match m_footUnlockGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("footUnlockGain"),
                );
            }
        };
        let m_worldFromModelFeedbackGain = match m_worldFromModelFeedbackGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "worldFromModelFeedbackGain",
                    ),
                );
            }
        };
        let m_errorUpDownBias = match m_errorUpDownBias {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("errorUpDownBias"),
                );
            }
        };
        let m_alignWorldFromModelGain = match m_alignWorldFromModelGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "alignWorldFromModelGain",
                    ),
                );
            }
        };
        let m_hipOrientationGain = match m_hipOrientationGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hipOrientationGain",
                    ),
                );
            }
        };
        let m_maxKneeAngleDifference = match m_maxKneeAngleDifference {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxKneeAngleDifference",
                    ),
                );
            }
        };
        let m_ankleOrientationGain = match m_ankleOrientationGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "ankleOrientationGain",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbFootIkGains {
            __ptr,
            m_onOffGain,
            m_groundAscendingGain,
            m_groundDescendingGain,
            m_footPlantedGain,
            m_footRaisedGain,
            m_footUnlockGain,
            m_worldFromModelFeedbackGain,
            m_errorUpDownBias,
            m_alignWorldFromModelGain,
            m_hipOrientationGain,
            m_maxKneeAngleDifference,
            m_ankleOrientationGain,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbFootIkGains {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "onOffGain",
                "groundAscendingGain",
                "groundDescendingGain",
                "footPlantedGain",
                "footRaisedGain",
                "footUnlockGain",
                "worldFromModelFeedbackGain",
                "errorUpDownBias",
                "alignWorldFromModelGain",
                "hipOrientationGain",
                "maxKneeAngleDifference",
                "ankleOrientationGain",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbFootIkGains",
                FIELDS,
                __hkbFootIkGainsVisitor {
                    marker: _serde::__private::PhantomData::<hkbFootIkGains>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
