use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleDefaultEngine`
/// - version: `0`
/// - signature: `0x56f8ca24`
/// - size: ` 48`(x86)/` 56`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultEngine {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleEngine,
    /// # C++ Info
    /// - name: `minRPM`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minRPM: f32,
    /// # C++ Info
    /// - name: `optRPM`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_optRPM: f32,
    /// # C++ Info
    /// - name: `maxRPM`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxRPM: f32,
    /// # C++ Info
    /// - name: `maxTorque`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxTorque: f32,
    /// # C++ Info
    /// - name: `torqueFactorAtMinRPM`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_torqueFactorAtMinRPM: f32,
    /// # C++ Info
    /// - name: `torqueFactorAtMaxRPM`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_torqueFactorAtMaxRPM: f32,
    /// # C++ Info
    /// - name: `resistanceFactorAtMinRPM`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_resistanceFactorAtMinRPM: f32,
    /// # C++ Info
    /// - name: `resistanceFactorAtOptRPM`(ctype: `hkReal`)
    /// - offset: ` 36`(x86)/` 44`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_resistanceFactorAtOptRPM: f32,
    /// # C++ Info
    /// - name: `resistanceFactorAtMaxRPM`(ctype: `hkReal`)
    /// - offset: ` 40`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_resistanceFactorAtMaxRPM: f32,
    /// # C++ Info
    /// - name: `clutchSlipRPM`(ctype: `hkReal`)
    /// - offset: ` 44`(x86)/` 52`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_clutchSlipRPM: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultEngine {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultEngine"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x56f8ca24)
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultEngine {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x56f8ca24)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultEngine", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("minRPM", &self.m_minRPM)?;
            serializer.serialize_field("optRPM", &self.m_optRPM)?;
            serializer.serialize_field("maxRPM", &self.m_maxRPM)?;
            serializer.serialize_field("maxTorque", &self.m_maxTorque)?;
            serializer
                .serialize_field("torqueFactorAtMinRPM", &self.m_torqueFactorAtMinRPM)?;
            serializer
                .serialize_field("torqueFactorAtMaxRPM", &self.m_torqueFactorAtMaxRPM)?;
            serializer
                .serialize_field(
                    "resistanceFactorAtMinRPM",
                    &self.m_resistanceFactorAtMinRPM,
                )?;
            serializer
                .serialize_field(
                    "resistanceFactorAtOptRPM",
                    &self.m_resistanceFactorAtOptRPM,
                )?;
            serializer
                .serialize_field(
                    "resistanceFactorAtMaxRPM",
                    &self.m_resistanceFactorAtMaxRPM,
                )?;
            serializer.serialize_field("clutchSlipRPM", &self.m_clutchSlipRPM)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleDefaultEngine {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_minRPM,
                m_optRPM,
                m_maxRPM,
                m_maxTorque,
                m_torqueFactorAtMinRPM,
                m_torqueFactorAtMaxRPM,
                m_resistanceFactorAtMinRPM,
                m_resistanceFactorAtOptRPM,
                m_resistanceFactorAtMaxRPM,
                m_clutchSlipRPM,
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
                        "minRPM" => Ok(__Field::m_minRPM),
                        "optRPM" => Ok(__Field::m_optRPM),
                        "maxRPM" => Ok(__Field::m_maxRPM),
                        "maxTorque" => Ok(__Field::m_maxTorque),
                        "torqueFactorAtMinRPM" => Ok(__Field::m_torqueFactorAtMinRPM),
                        "torqueFactorAtMaxRPM" => Ok(__Field::m_torqueFactorAtMaxRPM),
                        "resistanceFactorAtMinRPM" => {
                            Ok(__Field::m_resistanceFactorAtMinRPM)
                        }
                        "resistanceFactorAtOptRPM" => {
                            Ok(__Field::m_resistanceFactorAtOptRPM)
                        }
                        "resistanceFactorAtMaxRPM" => {
                            Ok(__Field::m_resistanceFactorAtMaxRPM)
                        }
                        "clutchSlipRPM" => Ok(__Field::m_clutchSlipRPM),
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
            struct __hkpVehicleDefaultEngineVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpVehicleDefaultEngine>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpVehicleDefaultEngineVisitor<'de> {
                type Value = hkpVehicleDefaultEngine;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpVehicleDefaultEngine",
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
                    let mut m_minRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_optRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxTorque: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_torqueFactorAtMinRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_torqueFactorAtMaxRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_resistanceFactorAtMinRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_resistanceFactorAtOptRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_resistanceFactorAtMaxRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_clutchSlipRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..10usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_minRPM) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("minRPM"),
                                    );
                                }
                                m_minRPM = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_optRPM) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("optRPM"),
                                    );
                                }
                                m_optRPM = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_maxRPM) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("maxRPM"),
                                    );
                                }
                                m_maxRPM = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_maxTorque) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxTorque",
                                        ),
                                    );
                                }
                                m_maxTorque = _serde::__private::Some(
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
                                    &m_torqueFactorAtMinRPM,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "torqueFactorAtMinRPM",
                                        ),
                                    );
                                }
                                m_torqueFactorAtMinRPM = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_torqueFactorAtMaxRPM,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "torqueFactorAtMaxRPM",
                                        ),
                                    );
                                }
                                m_torqueFactorAtMaxRPM = _serde::__private::Some(
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
                                    &m_resistanceFactorAtMinRPM,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "resistanceFactorAtMinRPM",
                                        ),
                                    );
                                }
                                m_resistanceFactorAtMinRPM = _serde::__private::Some(
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
                                    &m_resistanceFactorAtOptRPM,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "resistanceFactorAtOptRPM",
                                        ),
                                    );
                                }
                                m_resistanceFactorAtOptRPM = _serde::__private::Some(
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
                                    &m_resistanceFactorAtMaxRPM,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "resistanceFactorAtMaxRPM",
                                        ),
                                    );
                                }
                                m_resistanceFactorAtMaxRPM = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_clutchSlipRPM) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clutchSlipRPM",
                                        ),
                                    );
                                }
                                m_clutchSlipRPM = _serde::__private::Some(
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
                    let m_minRPM = match m_minRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("minRPM"),
                            );
                        }
                    };
                    let m_optRPM = match m_optRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("optRPM"),
                            );
                        }
                    };
                    let m_maxRPM = match m_maxRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("maxRPM"),
                            );
                        }
                    };
                    let m_maxTorque = match m_maxTorque {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxTorque",
                                ),
                            );
                        }
                    };
                    let m_torqueFactorAtMinRPM = match m_torqueFactorAtMinRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "torqueFactorAtMinRPM",
                                ),
                            );
                        }
                    };
                    let m_torqueFactorAtMaxRPM = match m_torqueFactorAtMaxRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "torqueFactorAtMaxRPM",
                                ),
                            );
                        }
                    };
                    let m_resistanceFactorAtMinRPM = match m_resistanceFactorAtMinRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "resistanceFactorAtMinRPM",
                                ),
                            );
                        }
                    };
                    let m_resistanceFactorAtOptRPM = match m_resistanceFactorAtOptRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "resistanceFactorAtOptRPM",
                                ),
                            );
                        }
                    };
                    let m_resistanceFactorAtMaxRPM = match m_resistanceFactorAtMaxRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "resistanceFactorAtMaxRPM",
                                ),
                            );
                        }
                    };
                    let m_clutchSlipRPM = match m_clutchSlipRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clutchSlipRPM",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpVehicleDefaultEngine {
                        __ptr,
                        parent,
                        m_minRPM,
                        m_optRPM,
                        m_maxRPM,
                        m_maxTorque,
                        m_torqueFactorAtMinRPM,
                        m_torqueFactorAtMaxRPM,
                        m_resistanceFactorAtMinRPM,
                        m_resistanceFactorAtOptRPM,
                        m_resistanceFactorAtMaxRPM,
                        m_clutchSlipRPM,
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
                    let mut m_minRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_optRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxTorque: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_torqueFactorAtMinRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_torqueFactorAtMaxRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_resistanceFactorAtMinRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_resistanceFactorAtOptRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_resistanceFactorAtMaxRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_clutchSlipRPM: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_minRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_minRPM) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("minRPM"),
                                    );
                                }
                                m_minRPM = _serde::__private::Some(
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
                            __Field::m_optRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_optRPM) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("optRPM"),
                                    );
                                }
                                m_optRPM = _serde::__private::Some(
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
                            __Field::m_maxRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxRPM) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("maxRPM"),
                                    );
                                }
                                m_maxRPM = _serde::__private::Some(
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
                            __Field::m_maxTorque => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxTorque) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxTorque",
                                        ),
                                    );
                                }
                                m_maxTorque = _serde::__private::Some(
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
                            __Field::m_torqueFactorAtMinRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_torqueFactorAtMinRPM,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "torqueFactorAtMinRPM",
                                        ),
                                    );
                                }
                                m_torqueFactorAtMinRPM = _serde::__private::Some(
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
                            __Field::m_torqueFactorAtMaxRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_torqueFactorAtMaxRPM,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "torqueFactorAtMaxRPM",
                                        ),
                                    );
                                }
                                m_torqueFactorAtMaxRPM = _serde::__private::Some(
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
                            __Field::m_resistanceFactorAtMinRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_resistanceFactorAtMinRPM,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "resistanceFactorAtMinRPM",
                                        ),
                                    );
                                }
                                m_resistanceFactorAtMinRPM = _serde::__private::Some(
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
                            __Field::m_resistanceFactorAtOptRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_resistanceFactorAtOptRPM,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "resistanceFactorAtOptRPM",
                                        ),
                                    );
                                }
                                m_resistanceFactorAtOptRPM = _serde::__private::Some(
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
                            __Field::m_resistanceFactorAtMaxRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_resistanceFactorAtMaxRPM,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "resistanceFactorAtMaxRPM",
                                        ),
                                    );
                                }
                                m_resistanceFactorAtMaxRPM = _serde::__private::Some(
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
                            __Field::m_clutchSlipRPM => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_clutchSlipRPM) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clutchSlipRPM",
                                        ),
                                    );
                                }
                                m_clutchSlipRPM = _serde::__private::Some(
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
                            _ => {}
                        }
                    }
                    let m_minRPM = match m_minRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("minRPM"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_optRPM = match m_optRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("optRPM"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxRPM = match m_maxRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("maxRPM"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxTorque = match m_maxTorque {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxTorque",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_torqueFactorAtMinRPM = match m_torqueFactorAtMinRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "torqueFactorAtMinRPM",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_torqueFactorAtMaxRPM = match m_torqueFactorAtMaxRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "torqueFactorAtMaxRPM",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_resistanceFactorAtMinRPM = match m_resistanceFactorAtMinRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "resistanceFactorAtMinRPM",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_resistanceFactorAtOptRPM = match m_resistanceFactorAtOptRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "resistanceFactorAtOptRPM",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_resistanceFactorAtMaxRPM = match m_resistanceFactorAtMaxRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "resistanceFactorAtMaxRPM",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_clutchSlipRPM = match m_clutchSlipRPM {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clutchSlipRPM",
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
                    let parent = hkpVehicleEngine { __ptr, parent };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpVehicleDefaultEngine {
                        __ptr,
                        parent,
                        m_minRPM,
                        m_optRPM,
                        m_maxRPM,
                        m_maxTorque,
                        m_torqueFactorAtMinRPM,
                        m_torqueFactorAtMaxRPM,
                        m_resistanceFactorAtMinRPM,
                        m_resistanceFactorAtOptRPM,
                        m_resistanceFactorAtMaxRPM,
                        m_clutchSlipRPM,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "minRPM",
                "optRPM",
                "maxRPM",
                "maxTorque",
                "torqueFactorAtMinRPM",
                "torqueFactorAtMaxRPM",
                "resistanceFactorAtMinRPM",
                "resistanceFactorAtOptRPM",
                "resistanceFactorAtMaxRPM",
                "clutchSlipRPM",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDefaultEngine",
                FIELDS,
                __hkpVehicleDefaultEngineVisitor {
                    marker: _serde::__private::PhantomData::<hkpVehicleDefaultEngine>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
