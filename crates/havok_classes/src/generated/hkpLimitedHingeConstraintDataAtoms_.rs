use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpLimitedHingeConstraintDataAtoms`
/// - version: `1`
/// - signature: `0x54c7715b`
/// - size: `240`(x86)/`240`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpLimitedHingeConstraintDataAtoms {
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
    /// - name: `angMotor`(ctype: `struct hkpAngMotorConstraintAtom`)
    /// - offset: `160`(x86)/`160`(x86_64)
    /// - type_size: ` 20`(x86)/` 24`(x86_64)
    pub m_angMotor: hkpAngMotorConstraintAtom,
    /// # C++ Info
    /// - name: `angFriction`(ctype: `struct hkpAngFrictionConstraintAtom`)
    /// - offset: `180`(x86)/`184`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    pub m_angFriction: hkpAngFrictionConstraintAtom,
    /// # C++ Info
    /// - name: `angLimit`(ctype: `struct hkpAngLimitConstraintAtom`)
    /// - offset: `192`(x86)/`196`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_angLimit: hkpAngLimitConstraintAtom,
    /// # C++ Info
    /// - name: `2dAng`(ctype: `struct hkp2dAngConstraintAtom`)
    /// - offset: `208`(x86)/`212`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_2dAng: hkp2dAngConstraintAtom,
    /// # C++ Info
    /// - name: `ballSocket`(ctype: `struct hkpBallSocketConstraintAtom`)
    /// - offset: `212`(x86)/`216`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_ballSocket: hkpBallSocketConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpLimitedHingeConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpLimitedHingeConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x54c7715b)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_transforms.deps_indexes());
            v.extend(self.m_setupStabilization.deps_indexes());
            v.extend(self.m_angMotor.deps_indexes());
            v.extend(self.m_angFriction.deps_indexes());
            v.extend(self.m_angLimit.deps_indexes());
            v.extend(self.m_2dAng.deps_indexes());
            v.extend(self.m_ballSocket.deps_indexes());
            v
        }
    }
    impl _serde::Serialize for hkpLimitedHingeConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x54c7715b)));
            let mut serializer = __serializer
                .serialize_struct("hkpLimitedHingeConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer
                .serialize_field("setupStabilization", &self.m_setupStabilization)?;
            serializer.serialize_field("angMotor", &self.m_angMotor)?;
            serializer.serialize_field("angFriction", &self.m_angFriction)?;
            serializer.serialize_field("angLimit", &self.m_angLimit)?;
            serializer.serialize_field("2dAng", &self.m_2dAng)?;
            serializer.serialize_field("ballSocket", &self.m_ballSocket)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpLimitedHingeConstraintDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_transforms,
                m_setupStabilization,
                m_angMotor,
                m_angFriction,
                m_angLimit,
                m_2dAng,
                m_ballSocket,
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
                        "transforms" => Ok(__Field::m_transforms),
                        "setupStabilization" => Ok(__Field::m_setupStabilization),
                        "angMotor" => Ok(__Field::m_angMotor),
                        "angFriction" => Ok(__Field::m_angFriction),
                        "angLimit" => Ok(__Field::m_angLimit),
                        "2dAng" => Ok(__Field::m_2dAng),
                        "ballSocket" => Ok(__Field::m_ballSocket),
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
            struct __hkpLimitedHingeConstraintDataAtomsVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpLimitedHingeConstraintDataAtoms,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpLimitedHingeConstraintDataAtomsVisitor<'de> {
                type Value = hkpLimitedHingeConstraintDataAtoms;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpLimitedHingeConstraintDataAtoms",
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
                    let mut m_angMotor: _serde::__private::Option<
                        hkpAngMotorConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_angFriction: _serde::__private::Option<
                        hkpAngFrictionConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_angLimit: _serde::__private::Option<
                        hkpAngLimitConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_2dAng: _serde::__private::Option<hkp2dAngConstraintAtom> = _serde::__private::None;
                    let mut m_ballSocket: _serde::__private::Option<
                        hkpBallSocketConstraintAtom,
                    > = _serde::__private::None;
                    for i in 0..7usize {
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
                                if _serde::__private::Option::is_some(
                                    &m_setupStabilization,
                                ) {
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_angMotor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angMotor",
                                        ),
                                    );
                                }
                                m_angMotor = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpAngMotorConstraintAtom,
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
                                if _serde::__private::Option::is_some(&m_angLimit) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angLimit",
                                        ),
                                    );
                                }
                                m_angLimit = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpAngLimitConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_2dAng) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("2dAng"),
                                    );
                                }
                                m_2dAng = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkp2dAngConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
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
                    __A::pad(&mut __map, 12usize, 8usize)?;
                    let m_transforms = match m_transforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transforms",
                                ),
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
                    let m_angMotor = match m_angMotor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("angMotor"),
                            );
                        }
                    };
                    let m_angFriction = match m_angFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angFriction",
                                ),
                            );
                        }
                    };
                    let m_angLimit = match m_angLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("angLimit"),
                            );
                        }
                    };
                    let m_2dAng = match m_2dAng {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("2dAng"),
                            );
                        }
                    };
                    let m_ballSocket = match m_ballSocket {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ballSocket",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpLimitedHingeConstraintDataAtoms {
                        __ptr,
                        m_transforms,
                        m_setupStabilization,
                        m_angMotor,
                        m_angFriction,
                        m_angLimit,
                        m_2dAng,
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
                    let mut m_transforms: _serde::__private::Option<
                        hkpSetLocalTransformsConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_setupStabilization: _serde::__private::Option<
                        hkpSetupStabilizationAtom,
                    > = _serde::__private::None;
                    let mut m_angMotor: _serde::__private::Option<
                        hkpAngMotorConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_angFriction: _serde::__private::Option<
                        hkpAngFrictionConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_angLimit: _serde::__private::Option<
                        hkpAngLimitConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_2dAng: _serde::__private::Option<hkp2dAngConstraintAtom> = _serde::__private::None;
                    let mut m_ballSocket: _serde::__private::Option<
                        hkpBallSocketConstraintAtom,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_transforms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transforms) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                            __Field::m_setupStabilization => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_setupStabilization,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_angMotor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_angMotor) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angMotor",
                                        ),
                                    );
                                }
                                m_angMotor = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpAngMotorConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_angFriction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_angFriction) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                            __Field::m_angLimit => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_angLimit) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angLimit",
                                        ),
                                    );
                                }
                                m_angLimit = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpAngLimitConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_2dAng => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_2dAng) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("2dAng"),
                                    );
                                }
                                m_2dAng = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkp2dAngConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ballSocket => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ballSocket) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                    let m_transforms = match m_transforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transforms",
                                ),
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
                    let m_angMotor = match m_angMotor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("angMotor"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_angFriction = match m_angFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angFriction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_angLimit = match m_angLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("angLimit"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_2dAng = match m_2dAng {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("2dAng"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ballSocket = match m_ballSocket {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ballSocket",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpLimitedHingeConstraintDataAtoms {
                        __ptr,
                        m_transforms,
                        m_setupStabilization,
                        m_angMotor,
                        m_angFriction,
                        m_angLimit,
                        m_2dAng,
                        m_ballSocket,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transforms",
                "setupStabilization",
                "angMotor",
                "angFriction",
                "angLimit",
                "2dAng",
                "ballSocket",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpLimitedHingeConstraintDataAtoms",
                FIELDS,
                __hkpLimitedHingeConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpLimitedHingeConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
