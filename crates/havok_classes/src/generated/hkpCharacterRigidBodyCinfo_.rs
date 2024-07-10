use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCharacterRigidBodyCinfo`
/// -         version: `0`
/// -       signature: `0x892f441`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCharacterRigidBodyCinfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpCharacterControllerCinfo,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `shape`(ctype: `struct hkpShape*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_shape: Pointer,
    /// # C++ Info
    /// -          name: `position`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_position: Vector4,
    /// # C++ Info
    /// -          name: `rotation`(ctype: `hkQuaternion`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotation: Quaternion,
    /// # C++ Info
    /// -          name: `mass`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_mass: f32,
    /// # C++ Info
    /// -          name: `friction`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_friction: f32,
    /// # C++ Info
    /// -          name: `maxLinearVelocity`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxLinearVelocity: f32,
    /// # C++ Info
    /// -          name: `allowedPenetrationDepth`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_allowedPenetrationDepth: f32,
    /// # C++ Info
    /// -          name: `up`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_up: Vector4,
    /// # C++ Info
    /// -          name: `maxSlope`(ctype: `hkReal`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSlope: f32,
    /// # C++ Info
    /// -          name: `maxForce`(ctype: `hkReal`)
    /// -        offset:  84(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxForce: f32,
    /// # C++ Info
    /// -          name: `unweldingHeightOffsetFactor`(ctype: `hkReal`)
    /// -        offset:  88(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_unweldingHeightOffsetFactor: f32,
    /// # C++ Info
    /// -          name: `maxSpeedForSimplexSolver`(ctype: `hkReal`)
    /// -        offset:  92(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSpeedForSimplexSolver: f32,
    /// # C++ Info
    /// -          name: `supportDistance`(ctype: `hkReal`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_supportDistance: f32,
    /// # C++ Info
    /// -          name: `hardSupportDistance`(ctype: `hkReal`)
    /// -        offset: 100(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_hardSupportDistance: f32,
    /// # C++ Info
    /// -          name: `vdbColor`(ctype: `hkInt32`)
    /// -        offset: 104(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vdbColor: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCharacterRigidBodyCinfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCharacterRigidBodyCinfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x892f441)
        }
    }
    impl _serde::Serialize for hkpCharacterRigidBodyCinfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x892f441)));
            let mut serializer = __serializer
                .serialize_struct("hkpCharacterRigidBodyCinfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer.serialize_field("position", &self.m_position)?;
            serializer.serialize_field("rotation", &self.m_rotation)?;
            serializer.serialize_field("mass", &self.m_mass)?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("maxLinearVelocity", &self.m_maxLinearVelocity)?;
            serializer
                .serialize_field(
                    "allowedPenetrationDepth",
                    &self.m_allowedPenetrationDepth,
                )?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer.serialize_field("maxSlope", &self.m_maxSlope)?;
            serializer.serialize_field("maxForce", &self.m_maxForce)?;
            serializer
                .serialize_field(
                    "unweldingHeightOffsetFactor",
                    &self.m_unweldingHeightOffsetFactor,
                )?;
            serializer
                .serialize_field(
                    "maxSpeedForSimplexSolver",
                    &self.m_maxSpeedForSimplexSolver,
                )?;
            serializer.serialize_field("supportDistance", &self.m_supportDistance)?;
            serializer
                .serialize_field("hardSupportDistance", &self.m_hardSupportDistance)?;
            serializer.serialize_field("vdbColor", &self.m_vdbColor)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_collisionFilterInfo,
    m_shape,
    m_position,
    m_rotation,
    m_mass,
    m_friction,
    m_maxLinearVelocity,
    m_allowedPenetrationDepth,
    m_up,
    m_maxSlope,
    m_maxForce,
    m_unweldingHeightOffsetFactor,
    m_maxSpeedForSimplexSolver,
    m_supportDistance,
    m_hardSupportDistance,
    m_vdbColor,
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
            "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
            "shape" => Ok(__Field::m_shape),
            "position" => Ok(__Field::m_position),
            "rotation" => Ok(__Field::m_rotation),
            "mass" => Ok(__Field::m_mass),
            "friction" => Ok(__Field::m_friction),
            "maxLinearVelocity" => Ok(__Field::m_maxLinearVelocity),
            "allowedPenetrationDepth" => Ok(__Field::m_allowedPenetrationDepth),
            "up" => Ok(__Field::m_up),
            "maxSlope" => Ok(__Field::m_maxSlope),
            "maxForce" => Ok(__Field::m_maxForce),
            "unweldingHeightOffsetFactor" => Ok(__Field::m_unweldingHeightOffsetFactor),
            "maxSpeedForSimplexSolver" => Ok(__Field::m_maxSpeedForSimplexSolver),
            "supportDistance" => Ok(__Field::m_supportDistance),
            "hardSupportDistance" => Ok(__Field::m_hardSupportDistance),
            "vdbColor" => Ok(__Field::m_vdbColor),
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
pub(super) struct __hkpCharacterRigidBodyCinfoVisitor<'de> {
    marker: core::marker::PhantomData<hkpCharacterRigidBodyCinfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpCharacterRigidBodyCinfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpCharacterRigidBodyCinfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpCharacterRigidBodyCinfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpCharacterRigidBodyCinfoVisitor<'de> {
    type Value = hkpCharacterRigidBodyCinfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpCharacterRigidBodyCinfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_shape: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_position: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_mass: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_friction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxLinearVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_allowedPenetrationDepth: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_maxSlope: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxForce: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_unweldingHeightOffsetFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxSpeedForSimplexSolver: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_supportDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_hardSupportDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_vdbColor: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..16usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_collisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionFilterInfo",
                            ),
                        );
                    }
                    m_collisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_shape) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("shape"),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_shape = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_position) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "position",
                            ),
                        );
                    }
                    m_position = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_rotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotation",
                            ),
                        );
                    }
                    m_rotation = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
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
                5usize => {
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
                6usize => {
                    if _serde::__private::Option::is_some(&m_maxLinearVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxLinearVelocity",
                            ),
                        );
                    }
                    m_maxLinearVelocity = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_allowedPenetrationDepth) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "allowedPenetrationDepth",
                            ),
                        );
                    }
                    m_allowedPenetrationDepth = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_up) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("up"),
                        );
                    }
                    m_up = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_maxSlope) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxSlope",
                            ),
                        );
                    }
                    m_maxSlope = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_maxForce) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxForce",
                            ),
                        );
                    }
                    m_maxForce = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(
                        &m_unweldingHeightOffsetFactor,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "unweldingHeightOffsetFactor",
                            ),
                        );
                    }
                    m_unweldingHeightOffsetFactor = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_maxSpeedForSimplexSolver) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxSpeedForSimplexSolver",
                            ),
                        );
                    }
                    m_maxSpeedForSimplexSolver = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_supportDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "supportDistance",
                            ),
                        );
                    }
                    m_supportDistance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                14usize => {
                    if _serde::__private::Option::is_some(&m_hardSupportDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hardSupportDistance",
                            ),
                        );
                    }
                    m_hardSupportDistance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_vdbColor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vdbColor",
                            ),
                        );
                    }
                    m_vdbColor = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        __A::pad(&mut __map, 4usize, 4usize)?;
        let m_collisionFilterInfo = match m_collisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "collisionFilterInfo",
                    ),
                );
            }
        };
        let m_shape = match m_shape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shape"),
                );
            }
        };
        let m_position = match m_position {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("position"),
                );
            }
        };
        let m_rotation = match m_rotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotation"),
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
        let m_friction = match m_friction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("friction"),
                );
            }
        };
        let m_maxLinearVelocity = match m_maxLinearVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxLinearVelocity"),
                );
            }
        };
        let m_allowedPenetrationDepth = match m_allowedPenetrationDepth {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "allowedPenetrationDepth",
                    ),
                );
            }
        };
        let m_up = match m_up {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("up"),
                );
            }
        };
        let m_maxSlope = match m_maxSlope {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxSlope"),
                );
            }
        };
        let m_maxForce = match m_maxForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxForce"),
                );
            }
        };
        let m_unweldingHeightOffsetFactor = match m_unweldingHeightOffsetFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "unweldingHeightOffsetFactor",
                    ),
                );
            }
        };
        let m_maxSpeedForSimplexSolver = match m_maxSpeedForSimplexSolver {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxSpeedForSimplexSolver",
                    ),
                );
            }
        };
        let m_supportDistance = match m_supportDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("supportDistance"),
                );
            }
        };
        let m_hardSupportDistance = match m_hardSupportDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hardSupportDistance",
                    ),
                );
            }
        };
        let m_vdbColor = match m_vdbColor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vdbColor"),
                );
            }
        };
        _serde::__private::Ok(hkpCharacterRigidBodyCinfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_collisionFilterInfo,
            m_shape,
            m_position,
            m_rotation,
            m_mass,
            m_friction,
            m_maxLinearVelocity,
            m_allowedPenetrationDepth,
            m_up,
            m_maxSlope,
            m_maxForce,
            m_unweldingHeightOffsetFactor,
            m_maxSpeedForSimplexSolver,
            m_supportDistance,
            m_hardSupportDistance,
            m_vdbColor,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpCharacterControllerCinfoVisitor::visit_as_parent(&mut __map)?;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_shape: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_position: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_mass: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_friction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxLinearVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_allowedPenetrationDepth: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_maxSlope: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxForce: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_unweldingHeightOffsetFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxSpeedForSimplexSolver: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_supportDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_hardSupportDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_vdbColor: _serde::__private::Option<i32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_collisionFilterInfo => {
                    if _serde::__private::Option::is_some(&m_collisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionFilterInfo",
                            ),
                        );
                    }
                    m_collisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_shape => {
                    if _serde::__private::Option::is_some(&m_shape) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("shape"),
                        );
                    }
                    m_shape = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_position => {
                    if _serde::__private::Option::is_some(&m_position) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "position",
                            ),
                        );
                    }
                    m_position = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_rotation => {
                    if _serde::__private::Option::is_some(&m_rotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotation",
                            ),
                        );
                    }
                    m_rotation = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
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
                __Field::m_maxLinearVelocity => {
                    if _serde::__private::Option::is_some(&m_maxLinearVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxLinearVelocity",
                            ),
                        );
                    }
                    m_maxLinearVelocity = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_allowedPenetrationDepth => {
                    if _serde::__private::Option::is_some(&m_allowedPenetrationDepth) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "allowedPenetrationDepth",
                            ),
                        );
                    }
                    m_allowedPenetrationDepth = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_up => {
                    if _serde::__private::Option::is_some(&m_up) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("up"),
                        );
                    }
                    m_up = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_maxSlope => {
                    if _serde::__private::Option::is_some(&m_maxSlope) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxSlope",
                            ),
                        );
                    }
                    m_maxSlope = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_maxForce => {
                    if _serde::__private::Option::is_some(&m_maxForce) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxForce",
                            ),
                        );
                    }
                    m_maxForce = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_unweldingHeightOffsetFactor => {
                    if _serde::__private::Option::is_some(
                        &m_unweldingHeightOffsetFactor,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "unweldingHeightOffsetFactor",
                            ),
                        );
                    }
                    m_unweldingHeightOffsetFactor = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_maxSpeedForSimplexSolver => {
                    if _serde::__private::Option::is_some(&m_maxSpeedForSimplexSolver) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxSpeedForSimplexSolver",
                            ),
                        );
                    }
                    m_maxSpeedForSimplexSolver = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_supportDistance => {
                    if _serde::__private::Option::is_some(&m_supportDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "supportDistance",
                            ),
                        );
                    }
                    m_supportDistance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_hardSupportDistance => {
                    if _serde::__private::Option::is_some(&m_hardSupportDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hardSupportDistance",
                            ),
                        );
                    }
                    m_hardSupportDistance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_vdbColor => {
                    if _serde::__private::Option::is_some(&m_vdbColor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vdbColor",
                            ),
                        );
                    }
                    m_vdbColor = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        let m_collisionFilterInfo = match m_collisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "collisionFilterInfo",
                    ),
                );
            }
        };
        let m_shape = match m_shape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shape"),
                );
            }
        };
        let m_position = match m_position {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("position"),
                );
            }
        };
        let m_rotation = match m_rotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotation"),
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
        let m_friction = match m_friction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("friction"),
                );
            }
        };
        let m_maxLinearVelocity = match m_maxLinearVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxLinearVelocity"),
                );
            }
        };
        let m_allowedPenetrationDepth = match m_allowedPenetrationDepth {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "allowedPenetrationDepth",
                    ),
                );
            }
        };
        let m_up = match m_up {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("up"),
                );
            }
        };
        let m_maxSlope = match m_maxSlope {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxSlope"),
                );
            }
        };
        let m_maxForce = match m_maxForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxForce"),
                );
            }
        };
        let m_unweldingHeightOffsetFactor = match m_unweldingHeightOffsetFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "unweldingHeightOffsetFactor",
                    ),
                );
            }
        };
        let m_maxSpeedForSimplexSolver = match m_maxSpeedForSimplexSolver {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxSpeedForSimplexSolver",
                    ),
                );
            }
        };
        let m_supportDistance = match m_supportDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("supportDistance"),
                );
            }
        };
        let m_hardSupportDistance = match m_hardSupportDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hardSupportDistance",
                    ),
                );
            }
        };
        let m_vdbColor = match m_vdbColor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vdbColor"),
                );
            }
        };
        _serde::__private::Ok(hkpCharacterRigidBodyCinfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_collisionFilterInfo,
            m_shape,
            m_position,
            m_rotation,
            m_mass,
            m_friction,
            m_maxLinearVelocity,
            m_allowedPenetrationDepth,
            m_up,
            m_maxSlope,
            m_maxForce,
            m_unweldingHeightOffsetFactor,
            m_maxSpeedForSimplexSolver,
            m_supportDistance,
            m_hardSupportDistance,
            m_vdbColor,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCharacterRigidBodyCinfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "collisionFilterInfo",
                "shape",
                "position",
                "rotation",
                "mass",
                "friction",
                "maxLinearVelocity",
                "allowedPenetrationDepth",
                "up",
                "maxSlope",
                "maxForce",
                "unweldingHeightOffsetFactor",
                "maxSpeedForSimplexSolver",
                "supportDistance",
                "hardSupportDistance",
                "vdbColor",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCharacterRigidBodyCinfo",
                FIELDS,
                __hkpCharacterRigidBodyCinfoVisitor {
                    marker: _serde::__private::PhantomData::<hkpCharacterRigidBodyCinfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
