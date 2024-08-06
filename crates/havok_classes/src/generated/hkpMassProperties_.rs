use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpMassProperties`
/// - version: `0`
/// - signature: `0x68a56834`
/// - size: ` 80`(x86)/` 80`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMassProperties {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `volume`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_volume: f32,
    /// # C++ Info
    /// - name: `mass`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_mass: f32,
    /// # C++ Info
    /// - name: `centerOfMass`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_centerOfMass: Vector4,
    /// # C++ Info
    /// - name: `inertiaTensor`(ctype: `hkMatrix3`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_inertiaTensor: Matrix3,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMassProperties {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMassProperties"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x68a56834)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkpMassProperties {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x68a56834)));
            let mut serializer = __serializer
                .serialize_struct("hkpMassProperties", class_meta)?;
            serializer.serialize_field("volume", &self.m_volume)?;
            serializer.serialize_field("mass", &self.m_mass)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("centerOfMass", &self.m_centerOfMass)?;
            serializer.serialize_field("inertiaTensor", &self.m_inertiaTensor)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpMassProperties {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_volume,
                m_mass,
                m_centerOfMass,
                m_inertiaTensor,
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
                        "volume" => Ok(__Field::m_volume),
                        "mass" => Ok(__Field::m_mass),
                        "centerOfMass" => Ok(__Field::m_centerOfMass),
                        "inertiaTensor" => Ok(__Field::m_inertiaTensor),
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
            struct __hkpMassPropertiesVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpMassProperties>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpMassPropertiesVisitor<'de> {
                type Value = hkpMassProperties;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpMassProperties",
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
                    let mut m_volume: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_mass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_centerOfMass: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_inertiaTensor: _serde::__private::Option<Matrix3> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_volume) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("volume"),
                                    );
                                }
                                m_volume = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_mass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("mass"),
                                    );
                                }
                                m_mass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_centerOfMass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "centerOfMass",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 8usize)?;
                                m_centerOfMass = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_inertiaTensor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inertiaTensor",
                                        ),
                                    );
                                }
                                m_inertiaTensor = _serde::__private::Some(
                                    match __A::next_value::<Matrix3>(&mut __map) {
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
                    let m_volume = match m_volume {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("volume"),
                            );
                        }
                    };
                    let m_mass = match m_mass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("mass"),
                            );
                        }
                    };
                    let m_centerOfMass = match m_centerOfMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "centerOfMass",
                                ),
                            );
                        }
                    };
                    let m_inertiaTensor = match m_inertiaTensor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inertiaTensor",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpMassProperties {
                        __ptr,
                        m_volume,
                        m_mass,
                        m_centerOfMass,
                        m_inertiaTensor,
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
                    let mut m_volume: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_mass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_centerOfMass: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_inertiaTensor: _serde::__private::Option<Matrix3> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_volume => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_volume) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("volume"),
                                    );
                                }
                                m_volume = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_mass) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("mass"),
                                    );
                                }
                                m_mass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_centerOfMass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_centerOfMass) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "centerOfMass",
                                        ),
                                    );
                                }
                                m_centerOfMass = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_inertiaTensor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_inertiaTensor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inertiaTensor",
                                        ),
                                    );
                                }
                                m_inertiaTensor = _serde::__private::Some(
                                    match __A::next_value::<Matrix3>(&mut __map) {
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
                    let m_volume = match m_volume {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("volume"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mass = match m_mass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("mass"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_centerOfMass = match m_centerOfMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "centerOfMass",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_inertiaTensor = match m_inertiaTensor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inertiaTensor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpMassProperties {
                        __ptr,
                        m_volume,
                        m_mass,
                        m_centerOfMass,
                        m_inertiaTensor,
                    })
                }
            }
            const FIELDS: &[&str] = &["volume", "mass", "centerOfMass", "inertiaTensor"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpMassProperties",
                FIELDS,
                __hkpMassPropertiesVisitor {
                    marker: _serde::__private::PhantomData::<hkpMassProperties>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
