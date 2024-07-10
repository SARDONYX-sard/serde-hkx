use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleFrictionDescriptionAxisDescription`
/// -         version: `0`
/// -       signature: `0x59ce153f`
/// -          size: 100(x86)/100(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleFrictionDescriptionAxisDescription {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `frictionCircleYtab`(ctype: `hkReal[16]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_frictionCircleYtab: [f32; 16usize],
    /// # C++ Info
    /// -          name: `xStep`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_xStep: f32,
    /// # C++ Info
    /// -          name: `xStart`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_xStart: f32,
    /// # C++ Info
    /// -          name: `wheelSurfaceInertia`(ctype: `hkReal`)
    /// -        offset:  72(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelSurfaceInertia: f32,
    /// # C++ Info
    /// -          name: `wheelSurfaceInertiaInv`(ctype: `hkReal`)
    /// -        offset:  76(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelSurfaceInertiaInv: f32,
    /// # C++ Info
    /// -          name: `wheelChassisMassRatio`(ctype: `hkReal`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelChassisMassRatio: f32,
    /// # C++ Info
    /// -          name: `wheelRadius`(ctype: `hkReal`)
    /// -        offset:  84(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelRadius: f32,
    /// # C++ Info
    /// -          name: `wheelRadiusInv`(ctype: `hkReal`)
    /// -        offset:  88(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelRadiusInv: f32,
    /// # C++ Info
    /// -          name: `wheelDownForceFactor`(ctype: `hkReal`)
    /// -        offset:  92(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelDownForceFactor: f32,
    /// # C++ Info
    /// -          name: `wheelDownForceSumFactor`(ctype: `hkReal`)
    /// -        offset:  96(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelDownForceSumFactor: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleFrictionDescriptionAxisDescription {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleFrictionDescriptionAxisDescription"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x59ce153f)
        }
    }
    impl _serde::Serialize for hkpVehicleFrictionDescriptionAxisDescription {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x59ce153f)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpVehicleFrictionDescriptionAxisDescription",
                    class_meta,
                )?;
            serializer
                .serialize_field(
                    "frictionCircleYtab",
                    &self.m_frictionCircleYtab.as_slice(),
                )?;
            serializer.serialize_field("xStep", &self.m_xStep)?;
            serializer.serialize_field("xStart", &self.m_xStart)?;
            serializer
                .serialize_field("wheelSurfaceInertia", &self.m_wheelSurfaceInertia)?;
            serializer
                .serialize_field(
                    "wheelSurfaceInertiaInv",
                    &self.m_wheelSurfaceInertiaInv,
                )?;
            serializer
                .serialize_field(
                    "wheelChassisMassRatio",
                    &self.m_wheelChassisMassRatio,
                )?;
            serializer.serialize_field("wheelRadius", &self.m_wheelRadius)?;
            serializer.serialize_field("wheelRadiusInv", &self.m_wheelRadiusInv)?;
            serializer
                .serialize_field("wheelDownForceFactor", &self.m_wheelDownForceFactor)?;
            serializer
                .serialize_field(
                    "wheelDownForceSumFactor",
                    &self.m_wheelDownForceSumFactor,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_frictionCircleYtab,
    m_xStep,
    m_xStart,
    m_wheelSurfaceInertia,
    m_wheelSurfaceInertiaInv,
    m_wheelChassisMassRatio,
    m_wheelRadius,
    m_wheelRadiusInv,
    m_wheelDownForceFactor,
    m_wheelDownForceSumFactor,
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
            "frictionCircleYtab" => Ok(__Field::m_frictionCircleYtab),
            "xStep" => Ok(__Field::m_xStep),
            "xStart" => Ok(__Field::m_xStart),
            "wheelSurfaceInertia" => Ok(__Field::m_wheelSurfaceInertia),
            "wheelSurfaceInertiaInv" => Ok(__Field::m_wheelSurfaceInertiaInv),
            "wheelChassisMassRatio" => Ok(__Field::m_wheelChassisMassRatio),
            "wheelRadius" => Ok(__Field::m_wheelRadius),
            "wheelRadiusInv" => Ok(__Field::m_wheelRadiusInv),
            "wheelDownForceFactor" => Ok(__Field::m_wheelDownForceFactor),
            "wheelDownForceSumFactor" => Ok(__Field::m_wheelDownForceSumFactor),
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
pub(super) struct __hkpVehicleFrictionDescriptionAxisDescriptionVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleFrictionDescriptionAxisDescription>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleFrictionDescriptionAxisDescriptionVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkpVehicleFrictionDescriptionAxisDescription,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpVehicleFrictionDescriptionAxisDescription,
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
for __hkpVehicleFrictionDescriptionAxisDescriptionVisitor<'de> {
    type Value = hkpVehicleFrictionDescriptionAxisDescription;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleFrictionDescriptionAxisDescription",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_frictionCircleYtab: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
        let mut m_xStep: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_xStart: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelSurfaceInertia: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelSurfaceInertiaInv: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelChassisMassRatio: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelRadius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelRadiusInv: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelDownForceFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelDownForceSumFactor: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..10usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_frictionCircleYtab) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "frictionCircleYtab",
                            ),
                        );
                    }
                    m_frictionCircleYtab = _serde::__private::Some(
                        match __A::next_value::<[f32; 16usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_xStep) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("xStep"),
                        );
                    }
                    m_xStep = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_xStart) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("xStart"),
                        );
                    }
                    m_xStart = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_wheelSurfaceInertia) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelSurfaceInertia",
                            ),
                        );
                    }
                    m_wheelSurfaceInertia = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_wheelSurfaceInertiaInv) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelSurfaceInertiaInv",
                            ),
                        );
                    }
                    m_wheelSurfaceInertiaInv = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_wheelChassisMassRatio) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelChassisMassRatio",
                            ),
                        );
                    }
                    m_wheelChassisMassRatio = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_wheelRadius) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelRadius",
                            ),
                        );
                    }
                    m_wheelRadius = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_wheelRadiusInv) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelRadiusInv",
                            ),
                        );
                    }
                    m_wheelRadiusInv = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_wheelDownForceFactor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelDownForceFactor",
                            ),
                        );
                    }
                    m_wheelDownForceFactor = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_wheelDownForceSumFactor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelDownForceSumFactor",
                            ),
                        );
                    }
                    m_wheelDownForceSumFactor = _serde::__private::Some(
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
        let m_frictionCircleYtab = match m_frictionCircleYtab {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "frictionCircleYtab",
                    ),
                );
            }
        };
        let m_xStep = match m_xStep {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("xStep"),
                );
            }
        };
        let m_xStart = match m_xStart {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("xStart"),
                );
            }
        };
        let m_wheelSurfaceInertia = match m_wheelSurfaceInertia {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelSurfaceInertia",
                    ),
                );
            }
        };
        let m_wheelSurfaceInertiaInv = match m_wheelSurfaceInertiaInv {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelSurfaceInertiaInv",
                    ),
                );
            }
        };
        let m_wheelChassisMassRatio = match m_wheelChassisMassRatio {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelChassisMassRatio",
                    ),
                );
            }
        };
        let m_wheelRadius = match m_wheelRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelRadius"),
                );
            }
        };
        let m_wheelRadiusInv = match m_wheelRadiusInv {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelRadiusInv"),
                );
            }
        };
        let m_wheelDownForceFactor = match m_wheelDownForceFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelDownForceFactor",
                    ),
                );
            }
        };
        let m_wheelDownForceSumFactor = match m_wheelDownForceSumFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelDownForceSumFactor",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleFrictionDescriptionAxisDescription {
            __ptr: __A::class_ptr(&mut __map),
            m_frictionCircleYtab,
            m_xStep,
            m_xStart,
            m_wheelSurfaceInertia,
            m_wheelSurfaceInertiaInv,
            m_wheelChassisMassRatio,
            m_wheelRadius,
            m_wheelRadiusInv,
            m_wheelDownForceFactor,
            m_wheelDownForceSumFactor,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_frictionCircleYtab: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
        let mut m_xStep: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_xStart: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelSurfaceInertia: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelSurfaceInertiaInv: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelChassisMassRatio: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelRadius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelRadiusInv: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelDownForceFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelDownForceSumFactor: _serde::__private::Option<f32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_frictionCircleYtab => {
                    if _serde::__private::Option::is_some(&m_frictionCircleYtab) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "frictionCircleYtab",
                            ),
                        );
                    }
                    m_frictionCircleYtab = _serde::__private::Some(
                        match __A::next_value::<[f32; 16usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_xStep => {
                    if _serde::__private::Option::is_some(&m_xStep) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("xStep"),
                        );
                    }
                    m_xStep = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_xStart => {
                    if _serde::__private::Option::is_some(&m_xStart) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("xStart"),
                        );
                    }
                    m_xStart = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_wheelSurfaceInertia => {
                    if _serde::__private::Option::is_some(&m_wheelSurfaceInertia) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelSurfaceInertia",
                            ),
                        );
                    }
                    m_wheelSurfaceInertia = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_wheelSurfaceInertiaInv => {
                    if _serde::__private::Option::is_some(&m_wheelSurfaceInertiaInv) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelSurfaceInertiaInv",
                            ),
                        );
                    }
                    m_wheelSurfaceInertiaInv = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_wheelChassisMassRatio => {
                    if _serde::__private::Option::is_some(&m_wheelChassisMassRatio) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelChassisMassRatio",
                            ),
                        );
                    }
                    m_wheelChassisMassRatio = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_wheelRadius => {
                    if _serde::__private::Option::is_some(&m_wheelRadius) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelRadius",
                            ),
                        );
                    }
                    m_wheelRadius = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_wheelRadiusInv => {
                    if _serde::__private::Option::is_some(&m_wheelRadiusInv) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelRadiusInv",
                            ),
                        );
                    }
                    m_wheelRadiusInv = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_wheelDownForceFactor => {
                    if _serde::__private::Option::is_some(&m_wheelDownForceFactor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelDownForceFactor",
                            ),
                        );
                    }
                    m_wheelDownForceFactor = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_wheelDownForceSumFactor => {
                    if _serde::__private::Option::is_some(&m_wheelDownForceSumFactor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelDownForceSumFactor",
                            ),
                        );
                    }
                    m_wheelDownForceSumFactor = _serde::__private::Some(
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
        let m_frictionCircleYtab = match m_frictionCircleYtab {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "frictionCircleYtab",
                    ),
                );
            }
        };
        let m_xStep = match m_xStep {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("xStep"),
                );
            }
        };
        let m_xStart = match m_xStart {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("xStart"),
                );
            }
        };
        let m_wheelSurfaceInertia = match m_wheelSurfaceInertia {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelSurfaceInertia",
                    ),
                );
            }
        };
        let m_wheelSurfaceInertiaInv = match m_wheelSurfaceInertiaInv {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelSurfaceInertiaInv",
                    ),
                );
            }
        };
        let m_wheelChassisMassRatio = match m_wheelChassisMassRatio {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelChassisMassRatio",
                    ),
                );
            }
        };
        let m_wheelRadius = match m_wheelRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelRadius"),
                );
            }
        };
        let m_wheelRadiusInv = match m_wheelRadiusInv {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelRadiusInv"),
                );
            }
        };
        let m_wheelDownForceFactor = match m_wheelDownForceFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelDownForceFactor",
                    ),
                );
            }
        };
        let m_wheelDownForceSumFactor = match m_wheelDownForceSumFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelDownForceSumFactor",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleFrictionDescriptionAxisDescription {
            __ptr: __A::class_ptr(&mut __map),
            m_frictionCircleYtab,
            m_xStep,
            m_xStart,
            m_wheelSurfaceInertia,
            m_wheelSurfaceInertiaInv,
            m_wheelChassisMassRatio,
            m_wheelRadius,
            m_wheelRadiusInv,
            m_wheelDownForceFactor,
            m_wheelDownForceSumFactor,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleFrictionDescriptionAxisDescription {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "frictionCircleYtab",
                "xStep",
                "xStart",
                "wheelSurfaceInertia",
                "wheelSurfaceInertiaInv",
                "wheelChassisMassRatio",
                "wheelRadius",
                "wheelRadiusInv",
                "wheelDownForceFactor",
                "wheelDownForceSumFactor",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleFrictionDescriptionAxisDescription",
                FIELDS,
                __hkpVehicleFrictionDescriptionAxisDescriptionVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleFrictionDescriptionAxisDescription,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
