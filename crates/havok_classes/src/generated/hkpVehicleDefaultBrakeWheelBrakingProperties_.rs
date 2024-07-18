use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleDefaultBrakeWheelBrakingProperties`
/// - version: `0`
/// - signature: `0x1ffad971`
/// - size: ` 12`(x86)/` 12`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultBrakeWheelBrakingProperties {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `maxBreakingTorque`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxBreakingTorque: f32,
    /// # C++ Info
    /// - name: `minPedalInputToBlock`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minPedalInputToBlock: f32,
    /// # C++ Info
    /// - name: `isConnectedToHandbrake`(ctype: `hkBool`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isConnectedToHandbrake: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultBrakeWheelBrakingProperties {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultBrakeWheelBrakingProperties"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x1ffad971)
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultBrakeWheelBrakingProperties {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x1ffad971)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpVehicleDefaultBrakeWheelBrakingProperties",
                    class_meta,
                )?;
            serializer.serialize_field("maxBreakingTorque", &self.m_maxBreakingTorque)?;
            serializer
                .serialize_field("minPedalInputToBlock", &self.m_minPedalInputToBlock)?;
            serializer
                .serialize_field(
                    "isConnectedToHandbrake",
                    &self.m_isConnectedToHandbrake,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_maxBreakingTorque,
    m_minPedalInputToBlock,
    m_isConnectedToHandbrake,
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
            "maxBreakingTorque" => Ok(__Field::m_maxBreakingTorque),
            "minPedalInputToBlock" => Ok(__Field::m_minPedalInputToBlock),
            "isConnectedToHandbrake" => Ok(__Field::m_isConnectedToHandbrake),
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
pub(super) struct __hkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleDefaultBrakeWheelBrakingProperties>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkpVehicleDefaultBrakeWheelBrakingProperties,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpVehicleDefaultBrakeWheelBrakingProperties,
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
impl<'de> _serde::de::Visitor<'de>
for __hkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor<'de> {
    type Value = hkpVehicleDefaultBrakeWheelBrakingProperties;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleDefaultBrakeWheelBrakingProperties",
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
        let mut m_maxBreakingTorque: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minPedalInputToBlock: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_isConnectedToHandbrake: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_maxBreakingTorque) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxBreakingTorque",
                            ),
                        );
                    }
                    m_maxBreakingTorque = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_minPedalInputToBlock) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minPedalInputToBlock",
                            ),
                        );
                    }
                    m_minPedalInputToBlock = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_isConnectedToHandbrake) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isConnectedToHandbrake",
                            ),
                        );
                    }
                    m_isConnectedToHandbrake = _serde::__private::Some(
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
        __A::pad(&mut __map, 3usize, 3usize)?;
        let m_maxBreakingTorque = match m_maxBreakingTorque {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxBreakingTorque"),
                );
            }
        };
        let m_minPedalInputToBlock = match m_minPedalInputToBlock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minPedalInputToBlock",
                    ),
                );
            }
        };
        let m_isConnectedToHandbrake = match m_isConnectedToHandbrake {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "isConnectedToHandbrake",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultBrakeWheelBrakingProperties {
            __ptr,
            m_maxBreakingTorque,
            m_minPedalInputToBlock,
            m_isConnectedToHandbrake,
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
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_maxBreakingTorque: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minPedalInputToBlock: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_isConnectedToHandbrake: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..3usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_maxBreakingTorque => {
                        if _serde::__private::Option::is_some(&m_maxBreakingTorque) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxBreakingTorque",
                                ),
                            );
                        }
                        m_maxBreakingTorque = _serde::__private::Some(
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
                    __Field::m_minPedalInputToBlock => {
                        if _serde::__private::Option::is_some(&m_minPedalInputToBlock) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "minPedalInputToBlock",
                                ),
                            );
                        }
                        m_minPedalInputToBlock = _serde::__private::Some(
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
                    __Field::m_isConnectedToHandbrake => {
                        if _serde::__private::Option::is_some(
                            &m_isConnectedToHandbrake,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isConnectedToHandbrake",
                                ),
                            );
                        }
                        m_isConnectedToHandbrake = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
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
        }
        let m_maxBreakingTorque = match m_maxBreakingTorque {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxBreakingTorque"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_minPedalInputToBlock = match m_minPedalInputToBlock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minPedalInputToBlock",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_isConnectedToHandbrake = match m_isConnectedToHandbrake {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "isConnectedToHandbrake",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultBrakeWheelBrakingProperties {
            __ptr,
            m_maxBreakingTorque,
            m_minPedalInputToBlock,
            m_isConnectedToHandbrake,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleDefaultBrakeWheelBrakingProperties {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "maxBreakingTorque",
                "minPedalInputToBlock",
                "isConnectedToHandbrake",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDefaultBrakeWheelBrakingProperties",
                FIELDS,
                __hkpVehicleDefaultBrakeWheelBrakingPropertiesVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleDefaultBrakeWheelBrakingProperties,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
