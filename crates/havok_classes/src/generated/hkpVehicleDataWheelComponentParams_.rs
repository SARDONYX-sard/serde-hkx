use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDataWheelComponentParams`
/// -         version: `0`
/// -       signature: `0x82fe40e0`
/// -          size:  40(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDataWheelComponentParams {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `radius`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_radius: f32,
    /// # C++ Info
    /// -          name: `mass`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_mass: f32,
    /// # C++ Info
    /// -          name: `width`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_width: f32,
    /// # C++ Info
    /// -          name: `friction`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_friction: f32,
    /// # C++ Info
    /// -          name: `viscosityFriction`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_viscosityFriction: f32,
    /// # C++ Info
    /// -          name: `maxFriction`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxFriction: f32,
    /// # C++ Info
    /// -          name: `slipAngle`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_slipAngle: f32,
    /// # C++ Info
    /// -          name: `forceFeedbackMultiplier`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_forceFeedbackMultiplier: f32,
    /// # C++ Info
    /// -          name: `maxContactBodyAcceleration`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxContactBodyAcceleration: f32,
    /// # C++ Info
    /// -          name: `axle`(ctype: `hkInt8`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_axle: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDataWheelComponentParams {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDataWheelComponentParams"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x82fe40e0)
        }
    }
    impl _serde::Serialize for hkpVehicleDataWheelComponentParams {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x82fe40e0)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDataWheelComponentParams", class_meta)?;
            serializer.serialize_field("radius", &self.m_radius)?;
            serializer.serialize_field("mass", &self.m_mass)?;
            serializer.serialize_field("width", &self.m_width)?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("viscosityFriction", &self.m_viscosityFriction)?;
            serializer.serialize_field("maxFriction", &self.m_maxFriction)?;
            serializer.serialize_field("slipAngle", &self.m_slipAngle)?;
            serializer
                .serialize_field(
                    "forceFeedbackMultiplier",
                    &self.m_forceFeedbackMultiplier,
                )?;
            serializer
                .serialize_field(
                    "maxContactBodyAcceleration",
                    &self.m_maxContactBodyAcceleration,
                )?;
            serializer.serialize_field("axle", &self.m_axle)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_radius,
    m_mass,
    m_width,
    m_friction,
    m_viscosityFriction,
    m_maxFriction,
    m_slipAngle,
    m_forceFeedbackMultiplier,
    m_maxContactBodyAcceleration,
    m_axle,
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
            "radius" => Ok(__Field::m_radius),
            "mass" => Ok(__Field::m_mass),
            "width" => Ok(__Field::m_width),
            "friction" => Ok(__Field::m_friction),
            "viscosityFriction" => Ok(__Field::m_viscosityFriction),
            "maxFriction" => Ok(__Field::m_maxFriction),
            "slipAngle" => Ok(__Field::m_slipAngle),
            "forceFeedbackMultiplier" => Ok(__Field::m_forceFeedbackMultiplier),
            "maxContactBodyAcceleration" => Ok(__Field::m_maxContactBodyAcceleration),
            "axle" => Ok(__Field::m_axle),
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
pub(super) struct __hkpVehicleDataWheelComponentParamsVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleDataWheelComponentParams>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleDataWheelComponentParamsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpVehicleDataWheelComponentParams, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpVehicleDataWheelComponentParams,
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
impl<'de> _serde::de::Visitor<'de> for __hkpVehicleDataWheelComponentParamsVisitor<'de> {
    type Value = hkpVehicleDataWheelComponentParams;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleDataWheelComponentParams",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_mass: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_width: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_friction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_viscosityFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_slipAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_forceFeedbackMultiplier: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxContactBodyAcceleration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_axle: _serde::__private::Option<i8> = _serde::__private::None;
        for i in 0..10usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_radius) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("radius"),
                        );
                    }
                    m_radius = _serde::__private::Some(
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
                    if _serde::__private::Option::is_some(&m_width) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("width"),
                        );
                    }
                    m_width = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_friction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "friction",
                            ),
                        );
                    }
                    m_friction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_viscosityFriction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "viscosityFriction",
                            ),
                        );
                    }
                    m_viscosityFriction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_maxFriction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxFriction",
                            ),
                        );
                    }
                    m_maxFriction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_slipAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "slipAngle",
                            ),
                        );
                    }
                    m_slipAngle = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_forceFeedbackMultiplier) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "forceFeedbackMultiplier",
                            ),
                        );
                    }
                    m_forceFeedbackMultiplier = _serde::__private::Some(
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
                        &m_maxContactBodyAcceleration,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxContactBodyAcceleration",
                            ),
                        );
                    }
                    m_maxContactBodyAcceleration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_axle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("axle"),
                        );
                    }
                    m_axle = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
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
        let m_radius = match m_radius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("radius"),
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
        let m_width = match m_width {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("width"),
                );
            }
        };
        let m_friction = match m_friction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("friction"),
                );
            }
        };
        let m_viscosityFriction = match m_viscosityFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("viscosityFriction"),
                );
            }
        };
        let m_maxFriction = match m_maxFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFriction"),
                );
            }
        };
        let m_slipAngle = match m_slipAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("slipAngle"),
                );
            }
        };
        let m_forceFeedbackMultiplier = match m_forceFeedbackMultiplier {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "forceFeedbackMultiplier",
                    ),
                );
            }
        };
        let m_maxContactBodyAcceleration = match m_maxContactBodyAcceleration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxContactBodyAcceleration",
                    ),
                );
            }
        };
        let m_axle = match m_axle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("axle"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDataWheelComponentParams {
            __ptr: __A::class_ptr(&mut __map),
            m_radius,
            m_mass,
            m_width,
            m_friction,
            m_viscosityFriction,
            m_maxFriction,
            m_slipAngle,
            m_forceFeedbackMultiplier,
            m_maxContactBodyAcceleration,
            m_axle,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_mass: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_width: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_friction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_viscosityFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_slipAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_forceFeedbackMultiplier: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxContactBodyAcceleration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_axle: _serde::__private::Option<i8> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_radius => {
                    if _serde::__private::Option::is_some(&m_radius) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("radius"),
                        );
                    }
                    m_radius = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_mass => {
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
                __Field::m_width => {
                    if _serde::__private::Option::is_some(&m_width) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("width"),
                        );
                    }
                    m_width = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_friction => {
                    if _serde::__private::Option::is_some(&m_friction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "friction",
                            ),
                        );
                    }
                    m_friction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_viscosityFriction => {
                    if _serde::__private::Option::is_some(&m_viscosityFriction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "viscosityFriction",
                            ),
                        );
                    }
                    m_viscosityFriction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_maxFriction => {
                    if _serde::__private::Option::is_some(&m_maxFriction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxFriction",
                            ),
                        );
                    }
                    m_maxFriction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_slipAngle => {
                    if _serde::__private::Option::is_some(&m_slipAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "slipAngle",
                            ),
                        );
                    }
                    m_slipAngle = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_forceFeedbackMultiplier => {
                    if _serde::__private::Option::is_some(&m_forceFeedbackMultiplier) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "forceFeedbackMultiplier",
                            ),
                        );
                    }
                    m_forceFeedbackMultiplier = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_maxContactBodyAcceleration => {
                    if _serde::__private::Option::is_some(
                        &m_maxContactBodyAcceleration,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxContactBodyAcceleration",
                            ),
                        );
                    }
                    m_maxContactBodyAcceleration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_axle => {
                    if _serde::__private::Option::is_some(&m_axle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("axle"),
                        );
                    }
                    m_axle = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
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
        let m_radius = match m_radius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("radius"),
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
        let m_width = match m_width {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("width"),
                );
            }
        };
        let m_friction = match m_friction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("friction"),
                );
            }
        };
        let m_viscosityFriction = match m_viscosityFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("viscosityFriction"),
                );
            }
        };
        let m_maxFriction = match m_maxFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFriction"),
                );
            }
        };
        let m_slipAngle = match m_slipAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("slipAngle"),
                );
            }
        };
        let m_forceFeedbackMultiplier = match m_forceFeedbackMultiplier {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "forceFeedbackMultiplier",
                    ),
                );
            }
        };
        let m_maxContactBodyAcceleration = match m_maxContactBodyAcceleration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxContactBodyAcceleration",
                    ),
                );
            }
        };
        let m_axle = match m_axle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("axle"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDataWheelComponentParams {
            __ptr: __A::class_ptr(&mut __map),
            m_radius,
            m_mass,
            m_width,
            m_friction,
            m_viscosityFriction,
            m_maxFriction,
            m_slipAngle,
            m_forceFeedbackMultiplier,
            m_maxContactBodyAcceleration,
            m_axle,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleDataWheelComponentParams {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "radius",
                "mass",
                "width",
                "friction",
                "viscosityFriction",
                "maxFriction",
                "slipAngle",
                "forceFeedbackMultiplier",
                "maxContactBodyAcceleration",
                "axle",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDataWheelComponentParams",
                FIELDS,
                __hkpVehicleDataWheelComponentParamsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleDataWheelComponentParams,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
