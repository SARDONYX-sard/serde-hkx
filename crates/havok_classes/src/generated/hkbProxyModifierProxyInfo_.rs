use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbProxyModifierProxyInfo`
/// -         version: `0`
/// -       signature: `0x39de637e`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbProxyModifierProxyInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `dynamicFriction`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dynamicFriction: f32,
    /// # C++ Info
    /// -          name: `staticFriction`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticFriction: f32,
    /// # C++ Info
    /// -          name: `keepContactTolerance`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_keepContactTolerance: f32,
    /// # C++ Info
    /// -          name: `up`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_up: Vector4,
    /// # C++ Info
    /// -          name: `keepDistance`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_keepDistance: f32,
    /// # C++ Info
    /// -          name: `contactAngleSensitivity`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_contactAngleSensitivity: f32,
    /// # C++ Info
    /// -          name: `userPlanes`(ctype: `hkUint32`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_userPlanes: u32,
    /// # C++ Info
    /// -          name: `maxCharacterSpeedForSolver`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxCharacterSpeedForSolver: f32,
    /// # C++ Info
    /// -          name: `characterStrength`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_characterStrength: f32,
    /// # C++ Info
    /// -          name: `characterMass`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_characterMass: f32,
    /// # C++ Info
    /// -          name: `maxSlope`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSlope: f32,
    /// # C++ Info
    /// -          name: `penetrationRecoverySpeed`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_penetrationRecoverySpeed: f32,
    /// # C++ Info
    /// -          name: `maxCastIterations`(ctype: `hkInt32`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxCastIterations: i32,
    /// # C++ Info
    /// -          name: `refreshManifoldInCheckSupport`(ctype: `hkBool`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_refreshManifoldInCheckSupport: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbProxyModifierProxyInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbProxyModifierProxyInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x39de637e)
        }
    }
    impl _serde::Serialize for hkbProxyModifierProxyInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x39de637e)));
            let mut serializer = __serializer
                .serialize_struct("hkbProxyModifierProxyInfo", class_meta)?;
            serializer.serialize_field("dynamicFriction", &self.m_dynamicFriction)?;
            serializer.serialize_field("staticFriction", &self.m_staticFriction)?;
            serializer
                .serialize_field("keepContactTolerance", &self.m_keepContactTolerance)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer.serialize_field("keepDistance", &self.m_keepDistance)?;
            serializer
                .serialize_field(
                    "contactAngleSensitivity",
                    &self.m_contactAngleSensitivity,
                )?;
            serializer.serialize_field("userPlanes", &self.m_userPlanes)?;
            serializer
                .serialize_field(
                    "maxCharacterSpeedForSolver",
                    &self.m_maxCharacterSpeedForSolver,
                )?;
            serializer.serialize_field("characterStrength", &self.m_characterStrength)?;
            serializer.serialize_field("characterMass", &self.m_characterMass)?;
            serializer.serialize_field("maxSlope", &self.m_maxSlope)?;
            serializer
                .serialize_field(
                    "penetrationRecoverySpeed",
                    &self.m_penetrationRecoverySpeed,
                )?;
            serializer.serialize_field("maxCastIterations", &self.m_maxCastIterations)?;
            serializer
                .serialize_field(
                    "refreshManifoldInCheckSupport",
                    &self.m_refreshManifoldInCheckSupport,
                )?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_dynamicFriction,
    m_staticFriction,
    m_keepContactTolerance,
    m_up,
    m_keepDistance,
    m_contactAngleSensitivity,
    m_userPlanes,
    m_maxCharacterSpeedForSolver,
    m_characterStrength,
    m_characterMass,
    m_maxSlope,
    m_penetrationRecoverySpeed,
    m_maxCastIterations,
    m_refreshManifoldInCheckSupport,
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
            "dynamicFriction" => Ok(__Field::m_dynamicFriction),
            "staticFriction" => Ok(__Field::m_staticFriction),
            "keepContactTolerance" => Ok(__Field::m_keepContactTolerance),
            "up" => Ok(__Field::m_up),
            "keepDistance" => Ok(__Field::m_keepDistance),
            "contactAngleSensitivity" => Ok(__Field::m_contactAngleSensitivity),
            "userPlanes" => Ok(__Field::m_userPlanes),
            "maxCharacterSpeedForSolver" => Ok(__Field::m_maxCharacterSpeedForSolver),
            "characterStrength" => Ok(__Field::m_characterStrength),
            "characterMass" => Ok(__Field::m_characterMass),
            "maxSlope" => Ok(__Field::m_maxSlope),
            "penetrationRecoverySpeed" => Ok(__Field::m_penetrationRecoverySpeed),
            "maxCastIterations" => Ok(__Field::m_maxCastIterations),
            "refreshManifoldInCheckSupport" => {
                Ok(__Field::m_refreshManifoldInCheckSupport)
            }
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
pub(super) struct __hkbProxyModifierProxyInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbProxyModifierProxyInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbProxyModifierProxyInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbProxyModifierProxyInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbProxyModifierProxyInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbProxyModifierProxyInfoVisitor<'de> {
    type Value = hkbProxyModifierProxyInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbProxyModifierProxyInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_dynamicFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_staticFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_keepContactTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_keepDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_contactAngleSensitivity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_userPlanes: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_maxCharacterSpeedForSolver: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_characterStrength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_characterMass: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxSlope: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_penetrationRecoverySpeed: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxCastIterations: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_refreshManifoldInCheckSupport: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..14usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_dynamicFriction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dynamicFriction",
                            ),
                        );
                    }
                    m_dynamicFriction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_staticFriction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "staticFriction",
                            ),
                        );
                    }
                    m_staticFriction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_keepContactTolerance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "keepContactTolerance",
                            ),
                        );
                    }
                    m_keepContactTolerance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_up) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("up"),
                        );
                    }
                    __A::pad(&mut __map, 4usize, 4usize)?;
                    m_up = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_keepDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "keepDistance",
                            ),
                        );
                    }
                    m_keepDistance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_contactAngleSensitivity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactAngleSensitivity",
                            ),
                        );
                    }
                    m_contactAngleSensitivity = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_userPlanes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userPlanes",
                            ),
                        );
                    }
                    m_userPlanes = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(
                        &m_maxCharacterSpeedForSolver,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxCharacterSpeedForSolver",
                            ),
                        );
                    }
                    m_maxCharacterSpeedForSolver = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_characterStrength) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "characterStrength",
                            ),
                        );
                    }
                    m_characterStrength = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_characterMass) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "characterMass",
                            ),
                        );
                    }
                    m_characterMass = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
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
                11usize => {
                    if _serde::__private::Option::is_some(&m_penetrationRecoverySpeed) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "penetrationRecoverySpeed",
                            ),
                        );
                    }
                    m_penetrationRecoverySpeed = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_maxCastIterations) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxCastIterations",
                            ),
                        );
                    }
                    m_maxCastIterations = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(
                        &m_refreshManifoldInCheckSupport,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "refreshManifoldInCheckSupport",
                            ),
                        );
                    }
                    m_refreshManifoldInCheckSupport = _serde::__private::Some(
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
        __A::pad(&mut __map, 11usize, 11usize)?;
        let m_dynamicFriction = match m_dynamicFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dynamicFriction"),
                );
            }
        };
        let m_staticFriction = match m_staticFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticFriction"),
                );
            }
        };
        let m_keepContactTolerance = match m_keepContactTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "keepContactTolerance",
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
        let m_keepDistance = match m_keepDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("keepDistance"),
                );
            }
        };
        let m_contactAngleSensitivity = match m_contactAngleSensitivity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contactAngleSensitivity",
                    ),
                );
            }
        };
        let m_userPlanes = match m_userPlanes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userPlanes"),
                );
            }
        };
        let m_maxCharacterSpeedForSolver = match m_maxCharacterSpeedForSolver {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxCharacterSpeedForSolver",
                    ),
                );
            }
        };
        let m_characterStrength = match m_characterStrength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterStrength"),
                );
            }
        };
        let m_characterMass = match m_characterMass {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterMass"),
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
        let m_penetrationRecoverySpeed = match m_penetrationRecoverySpeed {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "penetrationRecoverySpeed",
                    ),
                );
            }
        };
        let m_maxCastIterations = match m_maxCastIterations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxCastIterations"),
                );
            }
        };
        let m_refreshManifoldInCheckSupport = match m_refreshManifoldInCheckSupport {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "refreshManifoldInCheckSupport",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbProxyModifierProxyInfo {
            __ptr: __A::class_ptr(&mut __map),
            m_dynamicFriction,
            m_staticFriction,
            m_keepContactTolerance,
            m_up,
            m_keepDistance,
            m_contactAngleSensitivity,
            m_userPlanes,
            m_maxCharacterSpeedForSolver,
            m_characterStrength,
            m_characterMass,
            m_maxSlope,
            m_penetrationRecoverySpeed,
            m_maxCastIterations,
            m_refreshManifoldInCheckSupport,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_dynamicFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_staticFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_keepContactTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_keepDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_contactAngleSensitivity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_userPlanes: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_maxCharacterSpeedForSolver: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_characterStrength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_characterMass: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxSlope: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_penetrationRecoverySpeed: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxCastIterations: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_refreshManifoldInCheckSupport: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..14usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_dynamicFriction => {
                        if _serde::__private::Option::is_some(&m_dynamicFriction) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "dynamicFriction",
                                ),
                            );
                        }
                        m_dynamicFriction = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_staticFriction => {
                        if _serde::__private::Option::is_some(&m_staticFriction) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "staticFriction",
                                ),
                            );
                        }
                        m_staticFriction = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_keepContactTolerance => {
                        if _serde::__private::Option::is_some(&m_keepContactTolerance) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "keepContactTolerance",
                                ),
                            );
                        }
                        m_keepContactTolerance = _serde::__private::Some(
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
                    __Field::m_keepDistance => {
                        if _serde::__private::Option::is_some(&m_keepDistance) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "keepDistance",
                                ),
                            );
                        }
                        m_keepDistance = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_contactAngleSensitivity => {
                        if _serde::__private::Option::is_some(
                            &m_contactAngleSensitivity,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contactAngleSensitivity",
                                ),
                            );
                        }
                        m_contactAngleSensitivity = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_userPlanes => {
                        if _serde::__private::Option::is_some(&m_userPlanes) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "userPlanes",
                                ),
                            );
                        }
                        m_userPlanes = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxCharacterSpeedForSolver => {
                        if _serde::__private::Option::is_some(
                            &m_maxCharacterSpeedForSolver,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxCharacterSpeedForSolver",
                                ),
                            );
                        }
                        m_maxCharacterSpeedForSolver = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_characterStrength => {
                        if _serde::__private::Option::is_some(&m_characterStrength) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "characterStrength",
                                ),
                            );
                        }
                        m_characterStrength = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_characterMass => {
                        if _serde::__private::Option::is_some(&m_characterMass) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "characterMass",
                                ),
                            );
                        }
                        m_characterMass = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
                    __Field::m_penetrationRecoverySpeed => {
                        if _serde::__private::Option::is_some(
                            &m_penetrationRecoverySpeed,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "penetrationRecoverySpeed",
                                ),
                            );
                        }
                        m_penetrationRecoverySpeed = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxCastIterations => {
                        if _serde::__private::Option::is_some(&m_maxCastIterations) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxCastIterations",
                                ),
                            );
                        }
                        m_maxCastIterations = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_refreshManifoldInCheckSupport => {
                        if _serde::__private::Option::is_some(
                            &m_refreshManifoldInCheckSupport,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "refreshManifoldInCheckSupport",
                                ),
                            );
                        }
                        m_refreshManifoldInCheckSupport = _serde::__private::Some(
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
        }
        let m_dynamicFriction = match m_dynamicFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dynamicFriction"),
                );
            }
        };
        let m_staticFriction = match m_staticFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticFriction"),
                );
            }
        };
        let m_keepContactTolerance = match m_keepContactTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "keepContactTolerance",
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
        let m_keepDistance = match m_keepDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("keepDistance"),
                );
            }
        };
        let m_contactAngleSensitivity = match m_contactAngleSensitivity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contactAngleSensitivity",
                    ),
                );
            }
        };
        let m_userPlanes = match m_userPlanes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userPlanes"),
                );
            }
        };
        let m_maxCharacterSpeedForSolver = match m_maxCharacterSpeedForSolver {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxCharacterSpeedForSolver",
                    ),
                );
            }
        };
        let m_characterStrength = match m_characterStrength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterStrength"),
                );
            }
        };
        let m_characterMass = match m_characterMass {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterMass"),
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
        let m_penetrationRecoverySpeed = match m_penetrationRecoverySpeed {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "penetrationRecoverySpeed",
                    ),
                );
            }
        };
        let m_maxCastIterations = match m_maxCastIterations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxCastIterations"),
                );
            }
        };
        let m_refreshManifoldInCheckSupport = match m_refreshManifoldInCheckSupport {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "refreshManifoldInCheckSupport",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbProxyModifierProxyInfo {
            __ptr: __A::class_ptr(&mut __map),
            m_dynamicFriction,
            m_staticFriction,
            m_keepContactTolerance,
            m_up,
            m_keepDistance,
            m_contactAngleSensitivity,
            m_userPlanes,
            m_maxCharacterSpeedForSolver,
            m_characterStrength,
            m_characterMass,
            m_maxSlope,
            m_penetrationRecoverySpeed,
            m_maxCastIterations,
            m_refreshManifoldInCheckSupport,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbProxyModifierProxyInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "dynamicFriction",
                "staticFriction",
                "keepContactTolerance",
                "up",
                "keepDistance",
                "contactAngleSensitivity",
                "userPlanes",
                "maxCharacterSpeedForSolver",
                "characterStrength",
                "characterMass",
                "maxSlope",
                "penetrationRecoverySpeed",
                "maxCastIterations",
                "refreshManifoldInCheckSupport",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbProxyModifierProxyInfo",
                FIELDS,
                __hkbProxyModifierProxyInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkbProxyModifierProxyInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
