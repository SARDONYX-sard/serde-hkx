use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultSteering`
/// -         version: `0`
/// -       signature: `0x8f0411c8`
/// -          size:  28(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultSteering {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleSteering,
    /// # C++ Info
    /// -          name: `maxSteeringAngle`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSteeringAngle: f32,
    /// # C++ Info
    /// -          name: `maxSpeedFullSteeringAngle`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSpeedFullSteeringAngle: f32,
    /// # C++ Info
    /// -          name: `doesWheelSteer`(ctype: `hkArray<hkBool>`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_doesWheelSteer: Vec<bool>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultSteering {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultSteering"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x8f0411c8)
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultSteering {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x8f0411c8)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultSteering", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("maxSteeringAngle", &self.m_maxSteeringAngle)?;
            serializer
                .serialize_field(
                    "maxSpeedFullSteeringAngle",
                    &self.m_maxSpeedFullSteeringAngle,
                )?;
            serializer
                .serialize_array_meta_field("doesWheelSteer", &self.m_doesWheelSteer)?;
            serializer.serialize_array_field("doesWheelSteer", &self.m_doesWheelSteer)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_maxSteeringAngle,
    m_maxSpeedFullSteeringAngle,
    m_doesWheelSteer,
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
            "maxSteeringAngle" => Ok(__Field::m_maxSteeringAngle),
            "maxSpeedFullSteeringAngle" => Ok(__Field::m_maxSpeedFullSteeringAngle),
            "doesWheelSteer" => Ok(__Field::m_doesWheelSteer),
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
pub(super) struct __hkpVehicleDefaultSteeringVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleDefaultSteering>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleDefaultSteeringVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpVehicleDefaultSteering, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpVehicleDefaultSteering>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpVehicleDefaultSteeringVisitor<'de> {
    type Value = hkpVehicleDefaultSteering;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpVehicleDefaultSteering")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_maxSteeringAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxSpeedFullSteeringAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_doesWheelSteer: _serde::__private::Option<Vec<bool>> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_maxSteeringAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxSteeringAngle",
                            ),
                        );
                    }
                    m_maxSteeringAngle = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_maxSpeedFullSteeringAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxSpeedFullSteeringAngle",
                            ),
                        );
                    }
                    m_maxSpeedFullSteeringAngle = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_doesWheelSteer) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "doesWheelSteer",
                            ),
                        );
                    }
                    m_doesWheelSteer = _serde::__private::Some(
                        match __A::next_value::<Vec<bool>>(&mut __map) {
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
        let m_maxSteeringAngle = match m_maxSteeringAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxSteeringAngle"),
                );
            }
        };
        let m_maxSpeedFullSteeringAngle = match m_maxSpeedFullSteeringAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxSpeedFullSteeringAngle",
                    ),
                );
            }
        };
        let m_doesWheelSteer = match m_doesWheelSteer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("doesWheelSteer"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultSteering {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_maxSteeringAngle,
            m_maxSpeedFullSteeringAngle,
            m_doesWheelSteer,
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
        let parent = __hkpVehicleSteeringVisitor::visit_as_parent(&mut __map)?;
        let mut m_maxSteeringAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxSpeedFullSteeringAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_doesWheelSteer: _serde::__private::Option<Vec<bool>> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_maxSteeringAngle => {
                        if _serde::__private::Option::is_some(&m_maxSteeringAngle) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxSteeringAngle",
                                ),
                            );
                        }
                        m_maxSteeringAngle = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxSpeedFullSteeringAngle => {
                        if _serde::__private::Option::is_some(
                            &m_maxSpeedFullSteeringAngle,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxSpeedFullSteeringAngle",
                                ),
                            );
                        }
                        m_maxSpeedFullSteeringAngle = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_doesWheelSteer => {
                        if _serde::__private::Option::is_some(&m_doesWheelSteer) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "doesWheelSteer",
                                ),
                            );
                        }
                        m_doesWheelSteer = _serde::__private::Some(
                            match __A::next_value::<Vec<bool>>(&mut __map) {
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
        let m_maxSteeringAngle = match m_maxSteeringAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxSteeringAngle"),
                );
            }
        };
        let m_maxSpeedFullSteeringAngle = match m_maxSpeedFullSteeringAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxSpeedFullSteeringAngle",
                    ),
                );
            }
        };
        let m_doesWheelSteer = match m_doesWheelSteer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("doesWheelSteer"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultSteering {
            __ptr,
            parent,
            m_maxSteeringAngle,
            m_maxSpeedFullSteeringAngle,
            m_doesWheelSteer,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleDefaultSteering {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "maxSteeringAngle",
                "maxSpeedFullSteeringAngle",
                "doesWheelSteer",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDefaultSteering",
                FIELDS,
                __hkpVehicleDefaultSteeringVisitor {
                    marker: _serde::__private::PhantomData::<hkpVehicleDefaultSteering>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
