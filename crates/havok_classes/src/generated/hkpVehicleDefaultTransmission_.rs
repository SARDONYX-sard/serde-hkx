use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleDefaultTransmission`
/// - version: `0`
/// - signature: `0x235d5d6b`
/// - size: ` 52`(x86)/` 72`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultTransmission {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleTransmission,
    /// # C++ Info
    /// - name: `downshiftRPM`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_downshiftRPM: f32,
    /// # C++ Info
    /// - name: `upshiftRPM`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_upshiftRPM: f32,
    /// # C++ Info
    /// - name: `primaryTransmissionRatio`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_primaryTransmissionRatio: f32,
    /// # C++ Info
    /// - name: `clutchDelayTime`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_clutchDelayTime: f32,
    /// # C++ Info
    /// - name: `reverseGearRatio`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_reverseGearRatio: f32,
    /// # C++ Info
    /// - name: `gearsRatio`(ctype: `hkArray<hkReal>`)
    /// - offset: ` 28`(x86)/` 40`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_gearsRatio: Vec<f32>,
    /// # C++ Info
    /// - name: `wheelsTorqueRatio`(ctype: `hkArray<hkReal>`)
    /// - offset: ` 40`(x86)/` 56`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_wheelsTorqueRatio: Vec<f32>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultTransmission {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultTransmission"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x235d5d6b)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultTransmission {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x235d5d6b)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultTransmission", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("downshiftRPM", &self.m_downshiftRPM)?;
            serializer.serialize_field("upshiftRPM", &self.m_upshiftRPM)?;
            serializer
                .serialize_field(
                    "primaryTransmissionRatio",
                    &self.m_primaryTransmissionRatio,
                )?;
            serializer.serialize_field("clutchDelayTime", &self.m_clutchDelayTime)?;
            serializer.serialize_field("reverseGearRatio", &self.m_reverseGearRatio)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("gearsRatio", &self.m_gearsRatio)?;
            serializer
                .serialize_array_meta_field(
                    "wheelsTorqueRatio",
                    &self.m_wheelsTorqueRatio,
                )?;
            serializer.serialize_array_field("gearsRatio", &self.m_gearsRatio)?;
            serializer
                .serialize_array_field("wheelsTorqueRatio", &self.m_wheelsTorqueRatio)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleDefaultTransmission {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_downshiftRPM,
                m_upshiftRPM,
                m_primaryTransmissionRatio,
                m_clutchDelayTime,
                m_reverseGearRatio,
                m_gearsRatio,
                m_wheelsTorqueRatio,
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
                        "downshiftRPM" => Ok(__Field::m_downshiftRPM),
                        "upshiftRPM" => Ok(__Field::m_upshiftRPM),
                        "primaryTransmissionRatio" => {
                            Ok(__Field::m_primaryTransmissionRatio)
                        }
                        "clutchDelayTime" => Ok(__Field::m_clutchDelayTime),
                        "reverseGearRatio" => Ok(__Field::m_reverseGearRatio),
                        "gearsRatio" => Ok(__Field::m_gearsRatio),
                        "wheelsTorqueRatio" => Ok(__Field::m_wheelsTorqueRatio),
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
            struct __hkpVehicleDefaultTransmissionVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpVehicleDefaultTransmission>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpVehicleDefaultTransmissionVisitor<'de> {
                type Value = hkpVehicleDefaultTransmission;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpVehicleDefaultTransmission",
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
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_downshiftRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_upshiftRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_primaryTransmissionRatio: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_clutchDelayTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_reverseGearRatio: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_gearsRatio: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_wheelsTorqueRatio: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_downshiftRPM) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "downshiftRPM",
                                        ),
                                    );
                                }
                                m_downshiftRPM = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_upshiftRPM) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "upshiftRPM",
                                        ),
                                    );
                                }
                                m_upshiftRPM = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_primaryTransmissionRatio,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primaryTransmissionRatio",
                                        ),
                                    );
                                }
                                m_primaryTransmissionRatio = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_clutchDelayTime) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clutchDelayTime",
                                        ),
                                    );
                                }
                                m_clutchDelayTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_reverseGearRatio) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "reverseGearRatio",
                                        ),
                                    );
                                }
                                m_reverseGearRatio = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_gearsRatio) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gearsRatio",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_gearsRatio = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wheelsTorqueRatio,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsTorqueRatio",
                                        ),
                                    );
                                }
                                m_wheelsTorqueRatio = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
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
                    let m_downshiftRPM = match m_downshiftRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "downshiftRPM",
                                ),
                            );
                        }
                    };
                    let m_upshiftRPM = match m_upshiftRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "upshiftRPM",
                                ),
                            );
                        }
                    };
                    let m_primaryTransmissionRatio = match m_primaryTransmissionRatio {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primaryTransmissionRatio",
                                ),
                            );
                        }
                    };
                    let m_clutchDelayTime = match m_clutchDelayTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clutchDelayTime",
                                ),
                            );
                        }
                    };
                    let m_reverseGearRatio = match m_reverseGearRatio {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "reverseGearRatio",
                                ),
                            );
                        }
                    };
                    let m_gearsRatio = match m_gearsRatio {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "gearsRatio",
                                ),
                            );
                        }
                    };
                    let m_wheelsTorqueRatio = match m_wheelsTorqueRatio {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsTorqueRatio",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpVehicleDefaultTransmission {
                        __ptr,
                        parent,
                        m_downshiftRPM,
                        m_upshiftRPM,
                        m_primaryTransmissionRatio,
                        m_clutchDelayTime,
                        m_reverseGearRatio,
                        m_gearsRatio,
                        m_wheelsTorqueRatio,
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
                    let mut m_downshiftRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_upshiftRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_primaryTransmissionRatio: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_clutchDelayTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_reverseGearRatio: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_gearsRatio: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_wheelsTorqueRatio: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_downshiftRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_downshiftRPM) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "downshiftRPM",
                                        ),
                                    );
                                }
                                m_downshiftRPM = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_upshiftRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_upshiftRPM) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "upshiftRPM",
                                        ),
                                    );
                                }
                                m_upshiftRPM = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_primaryTransmissionRatio => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_primaryTransmissionRatio,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primaryTransmissionRatio",
                                        ),
                                    );
                                }
                                m_primaryTransmissionRatio = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_clutchDelayTime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_clutchDelayTime) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clutchDelayTime",
                                        ),
                                    );
                                }
                                m_clutchDelayTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_reverseGearRatio => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_reverseGearRatio) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "reverseGearRatio",
                                        ),
                                    );
                                }
                                m_reverseGearRatio = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_gearsRatio => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_gearsRatio) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gearsRatio",
                                        ),
                                    );
                                }
                                m_gearsRatio = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wheelsTorqueRatio => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wheelsTorqueRatio,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsTorqueRatio",
                                        ),
                                    );
                                }
                                m_wheelsTorqueRatio = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
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
                    let m_downshiftRPM = match m_downshiftRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "downshiftRPM",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_upshiftRPM = match m_upshiftRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "upshiftRPM",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_primaryTransmissionRatio = match m_primaryTransmissionRatio {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primaryTransmissionRatio",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_clutchDelayTime = match m_clutchDelayTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clutchDelayTime",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_reverseGearRatio = match m_reverseGearRatio {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "reverseGearRatio",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_gearsRatio = match m_gearsRatio {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "gearsRatio",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wheelsTorqueRatio = match m_wheelsTorqueRatio {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsTorqueRatio",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkpVehicleTransmission {
                        __ptr,
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpVehicleDefaultTransmission {
                        __ptr,
                        parent,
                        m_downshiftRPM,
                        m_upshiftRPM,
                        m_primaryTransmissionRatio,
                        m_clutchDelayTime,
                        m_reverseGearRatio,
                        m_gearsRatio,
                        m_wheelsTorqueRatio,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "downshiftRPM",
                "upshiftRPM",
                "primaryTransmissionRatio",
                "clutchDelayTime",
                "reverseGearRatio",
                "gearsRatio",
                "wheelsTorqueRatio",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDefaultTransmission",
                FIELDS,
                __hkpVehicleDefaultTransmissionVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleDefaultTransmission,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
