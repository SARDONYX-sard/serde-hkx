use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbFootIkDriverInfoLeg`
/// - version: `0`
/// - signature: `0x224b18d1`
/// - size: ` 96`(x86)/` 96`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkDriverInfoLeg {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `prevAnkleRotLS`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_prevAnkleRotLS: Quaternion,
    /// # C++ Info
    /// - name: `kneeAxisLS`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_kneeAxisLS: Vector4,
    /// # C++ Info
    /// - name: `footEndLS`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_footEndLS: Vector4,
    /// # C++ Info
    /// - name: `footPlantedAnkleHeightMS`(ctype: `hkReal`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_footPlantedAnkleHeightMS: f32,
    /// # C++ Info
    /// - name: `footRaisedAnkleHeightMS`(ctype: `hkReal`)
    /// - offset: ` 52`(x86)/` 52`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_footRaisedAnkleHeightMS: f32,
    /// # C++ Info
    /// - name: `maxAnkleHeightMS`(ctype: `hkReal`)
    /// - offset: ` 56`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxAnkleHeightMS: f32,
    /// # C++ Info
    /// - name: `minAnkleHeightMS`(ctype: `hkReal`)
    /// - offset: ` 60`(x86)/` 60`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minAnkleHeightMS: f32,
    /// # C++ Info
    /// - name: `maxKneeAngleDegrees`(ctype: `hkReal`)
    /// - offset: ` 64`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxKneeAngleDegrees: f32,
    /// # C++ Info
    /// - name: `minKneeAngleDegrees`(ctype: `hkReal`)
    /// - offset: ` 68`(x86)/` 68`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minKneeAngleDegrees: f32,
    /// # C++ Info
    /// - name: `maxAnkleAngleDegrees`(ctype: `hkReal`)
    /// - offset: ` 72`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxAnkleAngleDegrees: f32,
    /// # C++ Info
    /// - name: `hipIndex`(ctype: `hkInt16`)
    /// - offset: ` 76`(x86)/` 76`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_hipIndex: i16,
    /// # C++ Info
    /// - name: `kneeIndex`(ctype: `hkInt16`)
    /// - offset: ` 78`(x86)/` 78`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_kneeIndex: i16,
    /// # C++ Info
    /// - name: `ankleIndex`(ctype: `hkInt16`)
    /// - offset: ` 80`(x86)/` 80`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_ankleIndex: i16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbFootIkDriverInfoLeg {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkDriverInfoLeg"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x224b18d1)
        }
    }
    impl _serde::Serialize for hkbFootIkDriverInfoLeg {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x224b18d1)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkDriverInfoLeg", class_meta)?;
            serializer.skip_field("prevAnkleRotLS", &self.m_prevAnkleRotLS)?;
            serializer.serialize_field("kneeAxisLS", &self.m_kneeAxisLS)?;
            serializer.serialize_field("footEndLS", &self.m_footEndLS)?;
            serializer
                .serialize_field(
                    "footPlantedAnkleHeightMS",
                    &self.m_footPlantedAnkleHeightMS,
                )?;
            serializer
                .serialize_field(
                    "footRaisedAnkleHeightMS",
                    &self.m_footRaisedAnkleHeightMS,
                )?;
            serializer.serialize_field("maxAnkleHeightMS", &self.m_maxAnkleHeightMS)?;
            serializer.serialize_field("minAnkleHeightMS", &self.m_minAnkleHeightMS)?;
            serializer
                .serialize_field("maxKneeAngleDegrees", &self.m_maxKneeAngleDegrees)?;
            serializer
                .serialize_field("minKneeAngleDegrees", &self.m_minKneeAngleDegrees)?;
            serializer
                .serialize_field("maxAnkleAngleDegrees", &self.m_maxAnkleAngleDegrees)?;
            serializer.serialize_field("hipIndex", &self.m_hipIndex)?;
            serializer.serialize_field("kneeIndex", &self.m_kneeIndex)?;
            serializer.serialize_field("ankleIndex", &self.m_ankleIndex)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbFootIkDriverInfoLeg {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_ankleIndex,
                m_kneeIndex,
                m_hipIndex,
                m_maxAnkleAngleDegrees,
                m_minKneeAngleDegrees,
                m_maxKneeAngleDegrees,
                m_minAnkleHeightMS,
                m_maxAnkleHeightMS,
                m_footRaisedAnkleHeightMS,
                m_footPlantedAnkleHeightMS,
                m_footEndLS,
                m_kneeAxisLS,
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
                        "ankleIndex" => Ok(__Field::m_ankleIndex),
                        "kneeIndex" => Ok(__Field::m_kneeIndex),
                        "hipIndex" => Ok(__Field::m_hipIndex),
                        "maxAnkleAngleDegrees" => Ok(__Field::m_maxAnkleAngleDegrees),
                        "minKneeAngleDegrees" => Ok(__Field::m_minKneeAngleDegrees),
                        "maxKneeAngleDegrees" => Ok(__Field::m_maxKneeAngleDegrees),
                        "minAnkleHeightMS" => Ok(__Field::m_minAnkleHeightMS),
                        "maxAnkleHeightMS" => Ok(__Field::m_maxAnkleHeightMS),
                        "footRaisedAnkleHeightMS" => {
                            Ok(__Field::m_footRaisedAnkleHeightMS)
                        }
                        "footPlantedAnkleHeightMS" => {
                            Ok(__Field::m_footPlantedAnkleHeightMS)
                        }
                        "footEndLS" => Ok(__Field::m_footEndLS),
                        "kneeAxisLS" => Ok(__Field::m_kneeAxisLS),
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
            struct __hkbFootIkDriverInfoLegVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbFootIkDriverInfoLeg>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbFootIkDriverInfoLegVisitor<'de> {
                type Value = hkbFootIkDriverInfoLeg;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbFootIkDriverInfoLeg",
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
                    let mut m_prevAnkleRotLS: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_kneeAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_footEndLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_footPlantedAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_footRaisedAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxKneeAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minKneeAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAnkleAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_hipIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_kneeIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_ankleIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    for i in 0..13usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_prevAnkleRotLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "prevAnkleRotLS",
                                        ),
                                    );
                                }
                                m_prevAnkleRotLS = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_kneeAxisLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "kneeAxisLS",
                                        ),
                                    );
                                }
                                m_kneeAxisLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_footEndLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footEndLS",
                                        ),
                                    );
                                }
                                m_footEndLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_footPlantedAnkleHeightMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footPlantedAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_footPlantedAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_footRaisedAnkleHeightMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footRaisedAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_footRaisedAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_maxAnkleHeightMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_maxAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_minAnkleHeightMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_minAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxKneeAngleDegrees,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxKneeAngleDegrees",
                                        ),
                                    );
                                }
                                m_maxKneeAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minKneeAngleDegrees,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minKneeAngleDegrees",
                                        ),
                                    );
                                }
                                m_minKneeAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxAnkleAngleDegrees,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAnkleAngleDegrees",
                                        ),
                                    );
                                }
                                m_maxAnkleAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_hipIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hipIndex",
                                        ),
                                    );
                                }
                                m_hipIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_kneeIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "kneeIndex",
                                        ),
                                    );
                                }
                                m_kneeIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_ankleIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ankleIndex",
                                        ),
                                    );
                                }
                                m_ankleIndex = _serde::__private::Some(
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
                    __A::pad(&mut __map, 14usize, 14usize)?;
                    let m_prevAnkleRotLS = match m_prevAnkleRotLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "prevAnkleRotLS",
                                ),
                            );
                        }
                    };
                    let m_kneeAxisLS = match m_kneeAxisLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "kneeAxisLS",
                                ),
                            );
                        }
                    };
                    let m_footEndLS = match m_footEndLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footEndLS",
                                ),
                            );
                        }
                    };
                    let m_footPlantedAnkleHeightMS = match m_footPlantedAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footPlantedAnkleHeightMS",
                                ),
                            );
                        }
                    };
                    let m_footRaisedAnkleHeightMS = match m_footRaisedAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footRaisedAnkleHeightMS",
                                ),
                            );
                        }
                    };
                    let m_maxAnkleHeightMS = match m_maxAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAnkleHeightMS",
                                ),
                            );
                        }
                    };
                    let m_minAnkleHeightMS = match m_minAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minAnkleHeightMS",
                                ),
                            );
                        }
                    };
                    let m_maxKneeAngleDegrees = match m_maxKneeAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxKneeAngleDegrees",
                                ),
                            );
                        }
                    };
                    let m_minKneeAngleDegrees = match m_minKneeAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minKneeAngleDegrees",
                                ),
                            );
                        }
                    };
                    let m_maxAnkleAngleDegrees = match m_maxAnkleAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAnkleAngleDegrees",
                                ),
                            );
                        }
                    };
                    let m_hipIndex = match m_hipIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("hipIndex"),
                            );
                        }
                    };
                    let m_kneeIndex = match m_kneeIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "kneeIndex",
                                ),
                            );
                        }
                    };
                    let m_ankleIndex = match m_ankleIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ankleIndex",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbFootIkDriverInfoLeg {
                        __ptr,
                        m_prevAnkleRotLS,
                        m_kneeAxisLS,
                        m_footEndLS,
                        m_footPlantedAnkleHeightMS,
                        m_footRaisedAnkleHeightMS,
                        m_maxAnkleHeightMS,
                        m_minAnkleHeightMS,
                        m_maxKneeAngleDegrees,
                        m_minKneeAngleDegrees,
                        m_maxAnkleAngleDegrees,
                        m_hipIndex,
                        m_kneeIndex,
                        m_ankleIndex,
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
                    let mut m_ankleIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_kneeIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_hipIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_maxAnkleAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minKneeAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxKneeAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_footRaisedAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_footPlantedAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_footEndLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_kneeAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_ankleIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ankleIndex) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ankleIndex",
                                        ),
                                    );
                                }
                                m_ankleIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_kneeIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_kneeIndex) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "kneeIndex",
                                        ),
                                    );
                                }
                                m_kneeIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_hipIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_hipIndex) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hipIndex",
                                        ),
                                    );
                                }
                                m_hipIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_maxAnkleAngleDegrees => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxAnkleAngleDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAnkleAngleDegrees",
                                        ),
                                    );
                                }
                                m_maxAnkleAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_minKneeAngleDegrees => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minKneeAngleDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minKneeAngleDegrees",
                                        ),
                                    );
                                }
                                m_minKneeAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_maxKneeAngleDegrees => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxKneeAngleDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxKneeAngleDegrees",
                                        ),
                                    );
                                }
                                m_maxKneeAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_minAnkleHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_minAnkleHeightMS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_minAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_maxAnkleHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxAnkleHeightMS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_maxAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_footRaisedAnkleHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_footRaisedAnkleHeightMS,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footRaisedAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_footRaisedAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_footPlantedAnkleHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_footPlantedAnkleHeightMS,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footPlantedAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_footPlantedAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_footEndLS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_footEndLS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footEndLS",
                                        ),
                                    );
                                }
                                m_footEndLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_kneeAxisLS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_kneeAxisLS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "kneeAxisLS",
                                        ),
                                    );
                                }
                                m_kneeAxisLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
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
                    let m_ankleIndex = match m_ankleIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ankleIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_kneeIndex = match m_kneeIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "kneeIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_hipIndex = match m_hipIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("hipIndex"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxAnkleAngleDegrees = match m_maxAnkleAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAnkleAngleDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minKneeAngleDegrees = match m_minKneeAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minKneeAngleDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxKneeAngleDegrees = match m_maxKneeAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxKneeAngleDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minAnkleHeightMS = match m_minAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minAnkleHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxAnkleHeightMS = match m_maxAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAnkleHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_footRaisedAnkleHeightMS = match m_footRaisedAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footRaisedAnkleHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_footPlantedAnkleHeightMS = match m_footPlantedAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footPlantedAnkleHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_footEndLS = match m_footEndLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footEndLS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_kneeAxisLS = match m_kneeAxisLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "kneeAxisLS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbFootIkDriverInfoLeg {
                        __ptr,
                        m_kneeAxisLS,
                        m_footEndLS,
                        m_footPlantedAnkleHeightMS,
                        m_footRaisedAnkleHeightMS,
                        m_maxAnkleHeightMS,
                        m_minAnkleHeightMS,
                        m_maxKneeAngleDegrees,
                        m_minKneeAngleDegrees,
                        m_maxAnkleAngleDegrees,
                        m_hipIndex,
                        m_kneeIndex,
                        m_ankleIndex,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "prevAnkleRotLS",
                "kneeAxisLS",
                "footEndLS",
                "footPlantedAnkleHeightMS",
                "footRaisedAnkleHeightMS",
                "maxAnkleHeightMS",
                "minAnkleHeightMS",
                "maxKneeAngleDegrees",
                "minKneeAngleDegrees",
                "maxAnkleAngleDegrees",
                "hipIndex",
                "kneeIndex",
                "ankleIndex",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbFootIkDriverInfoLeg",
                FIELDS,
                __hkbFootIkDriverInfoLegVisitor {
                    marker: _serde::__private::PhantomData::<hkbFootIkDriverInfoLeg>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
