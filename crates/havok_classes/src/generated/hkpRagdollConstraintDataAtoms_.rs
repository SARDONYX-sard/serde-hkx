use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpRagdollConstraintDataAtoms`
/// - version: `1`
/// - signature: `0xeed76b00`
/// - size: `336`(x86)/`352`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRagdollConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `transforms`(ctype: `struct hkpSetLocalTransformsConstraintAtom`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `144`(x86)/`144`(x86_64)
    pub m_transforms: hkpSetLocalTransformsConstraintAtom,
    /// # C++ Info
    /// - name: `setupStabilization`(ctype: `struct hkpSetupStabilizationAtom`)
    /// - offset: `144`(x86)/`144`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_setupStabilization: hkpSetupStabilizationAtom,
    /// # C++ Info
    /// - name: `ragdollMotors`(ctype: `struct hkpRagdollMotorConstraintAtom`)
    /// - offset: `160`(x86)/`160`(x86_64)
    /// - type_size: ` 80`(x86)/` 96`(x86_64)
    pub m_ragdollMotors: hkpRagdollMotorConstraintAtom,
    /// # C++ Info
    /// - name: `angFriction`(ctype: `struct hkpAngFrictionConstraintAtom`)
    /// - offset: `240`(x86)/`256`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    pub m_angFriction: hkpAngFrictionConstraintAtom,
    /// # C++ Info
    /// - name: `twistLimit`(ctype: `struct hkpTwistLimitConstraintAtom`)
    /// - offset: `252`(x86)/`268`(x86_64)
    /// - type_size: ` 20`(x86)/` 20`(x86_64)
    pub m_twistLimit: hkpTwistLimitConstraintAtom,
    /// # C++ Info
    /// - name: `coneLimit`(ctype: `struct hkpConeLimitConstraintAtom`)
    /// - offset: `272`(x86)/`288`(x86_64)
    /// - type_size: ` 20`(x86)/` 20`(x86_64)
    pub m_coneLimit: hkpConeLimitConstraintAtom,
    /// # C++ Info
    /// - name: `planesLimit`(ctype: `struct hkpConeLimitConstraintAtom`)
    /// - offset: `292`(x86)/`308`(x86_64)
    /// - type_size: ` 20`(x86)/` 20`(x86_64)
    pub m_planesLimit: hkpConeLimitConstraintAtom,
    /// # C++ Info
    /// - name: `ballSocket`(ctype: `struct hkpBallSocketConstraintAtom`)
    /// - offset: `312`(x86)/`328`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_ballSocket: hkpBallSocketConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpRagdollConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRagdollConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xeed76b00)
        }
    }
    impl _serde::Serialize for hkpRagdollConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xeed76b00)));
            let mut serializer = __serializer
                .serialize_struct("hkpRagdollConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer
                .serialize_field("setupStabilization", &self.m_setupStabilization)?;
            serializer.serialize_field("ragdollMotors", &self.m_ragdollMotors)?;
            serializer.serialize_field("angFriction", &self.m_angFriction)?;
            serializer.serialize_field("twistLimit", &self.m_twistLimit)?;
            serializer.serialize_field("coneLimit", &self.m_coneLimit)?;
            serializer.serialize_field("planesLimit", &self.m_planesLimit)?;
            serializer.serialize_field("ballSocket", &self.m_ballSocket)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_transforms,
    m_setupStabilization,
    m_ragdollMotors,
    m_angFriction,
    m_twistLimit,
    m_coneLimit,
    m_planesLimit,
    m_ballSocket,
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
            "transforms" => Ok(__Field::m_transforms),
            "setupStabilization" => Ok(__Field::m_setupStabilization),
            "ragdollMotors" => Ok(__Field::m_ragdollMotors),
            "angFriction" => Ok(__Field::m_angFriction),
            "twistLimit" => Ok(__Field::m_twistLimit),
            "coneLimit" => Ok(__Field::m_coneLimit),
            "planesLimit" => Ok(__Field::m_planesLimit),
            "ballSocket" => Ok(__Field::m_ballSocket),
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
pub(super) struct __hkpRagdollConstraintDataAtomsVisitor<'de> {
    marker: core::marker::PhantomData<hkpRagdollConstraintDataAtoms>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpRagdollConstraintDataAtomsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpRagdollConstraintDataAtoms, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpRagdollConstraintDataAtoms>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpRagdollConstraintDataAtomsVisitor<'de> {
    type Value = hkpRagdollConstraintDataAtoms;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpRagdollConstraintDataAtoms",
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
        let mut m_transforms: _serde::__private::Option<
            hkpSetLocalTransformsConstraintAtom,
        > = _serde::__private::None;
        let mut m_setupStabilization: _serde::__private::Option<
            hkpSetupStabilizationAtom,
        > = _serde::__private::None;
        let mut m_ragdollMotors: _serde::__private::Option<
            hkpRagdollMotorConstraintAtom,
        > = _serde::__private::None;
        let mut m_angFriction: _serde::__private::Option<hkpAngFrictionConstraintAtom> = _serde::__private::None;
        let mut m_twistLimit: _serde::__private::Option<hkpTwistLimitConstraintAtom> = _serde::__private::None;
        let mut m_coneLimit: _serde::__private::Option<hkpConeLimitConstraintAtom> = _serde::__private::None;
        let mut m_planesLimit: _serde::__private::Option<hkpConeLimitConstraintAtom> = _serde::__private::None;
        let mut m_ballSocket: _serde::__private::Option<hkpBallSocketConstraintAtom> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_transforms) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transforms",
                            ),
                        );
                    }
                    m_transforms = _serde::__private::Some(
                        match __A::next_value::<
                            hkpSetLocalTransformsConstraintAtom,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_setupStabilization) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "setupStabilization",
                            ),
                        );
                    }
                    m_setupStabilization = _serde::__private::Some(
                        match __A::next_value::<hkpSetupStabilizationAtom>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_ragdollMotors) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ragdollMotors",
                            ),
                        );
                    }
                    m_ragdollMotors = _serde::__private::Some(
                        match __A::next_value::<
                            hkpRagdollMotorConstraintAtom,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_angFriction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "angFriction",
                            ),
                        );
                    }
                    m_angFriction = _serde::__private::Some(
                        match __A::next_value::<
                            hkpAngFrictionConstraintAtom,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_twistLimit) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "twistLimit",
                            ),
                        );
                    }
                    m_twistLimit = _serde::__private::Some(
                        match __A::next_value::<
                            hkpTwistLimitConstraintAtom,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_coneLimit) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "coneLimit",
                            ),
                        );
                    }
                    m_coneLimit = _serde::__private::Some(
                        match __A::next_value::<hkpConeLimitConstraintAtom>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_planesLimit) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "planesLimit",
                            ),
                        );
                    }
                    m_planesLimit = _serde::__private::Some(
                        match __A::next_value::<hkpConeLimitConstraintAtom>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_ballSocket) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ballSocket",
                            ),
                        );
                    }
                    m_ballSocket = _serde::__private::Some(
                        match __A::next_value::<
                            hkpBallSocketConstraintAtom,
                        >(&mut __map) {
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
        __A::pad(&mut __map, 8usize, 8usize)?;
        let m_transforms = match m_transforms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transforms"),
                );
            }
        };
        let m_setupStabilization = match m_setupStabilization {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "setupStabilization",
                    ),
                );
            }
        };
        let m_ragdollMotors = match m_ragdollMotors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ragdollMotors"),
                );
            }
        };
        let m_angFriction = match m_angFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("angFriction"),
                );
            }
        };
        let m_twistLimit = match m_twistLimit {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("twistLimit"),
                );
            }
        };
        let m_coneLimit = match m_coneLimit {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("coneLimit"),
                );
            }
        };
        let m_planesLimit = match m_planesLimit {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("planesLimit"),
                );
            }
        };
        let m_ballSocket = match m_ballSocket {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ballSocket"),
                );
            }
        };
        _serde::__private::Ok(hkpRagdollConstraintDataAtoms {
            __ptr,
            m_transforms,
            m_setupStabilization,
            m_ragdollMotors,
            m_angFriction,
            m_twistLimit,
            m_coneLimit,
            m_planesLimit,
            m_ballSocket,
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
        let mut m_transforms: _serde::__private::Option<
            hkpSetLocalTransformsConstraintAtom,
        > = _serde::__private::None;
        let mut m_setupStabilization: _serde::__private::Option<
            hkpSetupStabilizationAtom,
        > = _serde::__private::None;
        let mut m_ragdollMotors: _serde::__private::Option<
            hkpRagdollMotorConstraintAtom,
        > = _serde::__private::None;
        let mut m_angFriction: _serde::__private::Option<hkpAngFrictionConstraintAtom> = _serde::__private::None;
        let mut m_twistLimit: _serde::__private::Option<hkpTwistLimitConstraintAtom> = _serde::__private::None;
        let mut m_coneLimit: _serde::__private::Option<hkpConeLimitConstraintAtom> = _serde::__private::None;
        let mut m_planesLimit: _serde::__private::Option<hkpConeLimitConstraintAtom> = _serde::__private::None;
        let mut m_ballSocket: _serde::__private::Option<hkpBallSocketConstraintAtom> = _serde::__private::None;
        for _ in 0..8usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_transforms => {
                        if _serde::__private::Option::is_some(&m_transforms) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "transforms",
                                ),
                            );
                        }
                        m_transforms = _serde::__private::Some(
                            match __A::next_value::<
                                hkpSetLocalTransformsConstraintAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_setupStabilization => {
                        if _serde::__private::Option::is_some(&m_setupStabilization) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "setupStabilization",
                                ),
                            );
                        }
                        m_setupStabilization = _serde::__private::Some(
                            match __A::next_value::<
                                hkpSetupStabilizationAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_ragdollMotors => {
                        if _serde::__private::Option::is_some(&m_ragdollMotors) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ragdollMotors",
                                ),
                            );
                        }
                        m_ragdollMotors = _serde::__private::Some(
                            match __A::next_value::<
                                hkpRagdollMotorConstraintAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_angFriction => {
                        if _serde::__private::Option::is_some(&m_angFriction) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "angFriction",
                                ),
                            );
                        }
                        m_angFriction = _serde::__private::Some(
                            match __A::next_value::<
                                hkpAngFrictionConstraintAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_twistLimit => {
                        if _serde::__private::Option::is_some(&m_twistLimit) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "twistLimit",
                                ),
                            );
                        }
                        m_twistLimit = _serde::__private::Some(
                            match __A::next_value::<
                                hkpTwistLimitConstraintAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_coneLimit => {
                        if _serde::__private::Option::is_some(&m_coneLimit) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "coneLimit",
                                ),
                            );
                        }
                        m_coneLimit = _serde::__private::Some(
                            match __A::next_value::<
                                hkpConeLimitConstraintAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_planesLimit => {
                        if _serde::__private::Option::is_some(&m_planesLimit) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "planesLimit",
                                ),
                            );
                        }
                        m_planesLimit = _serde::__private::Some(
                            match __A::next_value::<
                                hkpConeLimitConstraintAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_ballSocket => {
                        if _serde::__private::Option::is_some(&m_ballSocket) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ballSocket",
                                ),
                            );
                        }
                        m_ballSocket = _serde::__private::Some(
                            match __A::next_value::<
                                hkpBallSocketConstraintAtom,
                            >(&mut __map) {
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
        let m_transforms = match m_transforms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transforms"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_setupStabilization = match m_setupStabilization {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "setupStabilization",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_ragdollMotors = match m_ragdollMotors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ragdollMotors"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_angFriction = match m_angFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("angFriction"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_twistLimit = match m_twistLimit {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("twistLimit"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_coneLimit = match m_coneLimit {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("coneLimit"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_planesLimit = match m_planesLimit {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("planesLimit"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_ballSocket = match m_ballSocket {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ballSocket"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpRagdollConstraintDataAtoms {
            __ptr,
            m_transforms,
            m_setupStabilization,
            m_ragdollMotors,
            m_angFriction,
            m_twistLimit,
            m_coneLimit,
            m_planesLimit,
            m_ballSocket,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpRagdollConstraintDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "transforms",
                "setupStabilization",
                "ragdollMotors",
                "angFriction",
                "twistLimit",
                "coneLimit",
                "planesLimit",
                "ballSocket",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpRagdollConstraintDataAtoms",
                FIELDS,
                __hkpRagdollConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpRagdollConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
