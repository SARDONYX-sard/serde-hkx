use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpAngMotorConstraintAtom`
/// -         version: `0`
/// -       signature: `0x81f087ff`
/// -          size:  20(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAngMotorConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// -          name: `isEnabled`(ctype: `hkBool`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isEnabled: bool,
    /// # C++ Info
    /// -          name: `motorAxis`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_motorAxis: u8,
    /// # C++ Info
    /// -          name: `initializedOffset`(ctype: `hkInt16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_initializedOffset: i16,
    /// # C++ Info
    /// -          name: `previousTargetAngleOffset`(ctype: `hkInt16`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_previousTargetAngleOffset: i16,
    /// # C++ Info
    /// -          name: `correspondingAngLimitSolverResultOffset`(ctype: `hkInt16`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_correspondingAngLimitSolverResultOffset: i16,
    /// # C++ Info
    /// -          name: `targetAngle`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_targetAngle: f32,
    /// # C++ Info
    /// -          name: `motor`(ctype: `struct hkpConstraintMotor*`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_motor: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpAngMotorConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpAngMotorConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x81f087ff)
        }
    }
    impl _serde::Serialize for hkpAngMotorConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x81f087ff)));
            let mut serializer = __serializer
                .serialize_struct("hkpAngMotorConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.serialize_field("motorAxis", &self.m_motorAxis)?;
            serializer.serialize_field("initializedOffset", &self.m_initializedOffset)?;
            serializer
                .serialize_field(
                    "previousTargetAngleOffset",
                    &self.m_previousTargetAngleOffset,
                )?;
            serializer
                .serialize_field(
                    "correspondingAngLimitSolverResultOffset",
                    &self.m_correspondingAngLimitSolverResultOffset,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("targetAngle", &self.m_targetAngle)?;
            serializer.serialize_field("motor", &self.m_motor)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_isEnabled,
    m_motorAxis,
    m_initializedOffset,
    m_previousTargetAngleOffset,
    m_correspondingAngLimitSolverResultOffset,
    m_targetAngle,
    m_motor,
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
            "isEnabled" => Ok(__Field::m_isEnabled),
            "motorAxis" => Ok(__Field::m_motorAxis),
            "initializedOffset" => Ok(__Field::m_initializedOffset),
            "previousTargetAngleOffset" => Ok(__Field::m_previousTargetAngleOffset),
            "correspondingAngLimitSolverResultOffset" => {
                Ok(__Field::m_correspondingAngLimitSolverResultOffset)
            }
            "targetAngle" => Ok(__Field::m_targetAngle),
            "motor" => Ok(__Field::m_motor),
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
pub(super) struct __hkpAngMotorConstraintAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpAngMotorConstraintAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpAngMotorConstraintAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpAngMotorConstraintAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpAngMotorConstraintAtom>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpAngMotorConstraintAtomVisitor<'de> {
    type Value = hkpAngMotorConstraintAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpAngMotorConstraintAtom")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_isEnabled: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_motorAxis: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_initializedOffset: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_previousTargetAngleOffset: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_correspondingAngLimitSolverResultOffset: _serde::__private::Option<
            i16,
        > = _serde::__private::None;
        let mut m_targetAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_motor: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_isEnabled) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isEnabled",
                            ),
                        );
                    }
                    m_isEnabled = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_motorAxis) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "motorAxis",
                            ),
                        );
                    }
                    m_motorAxis = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_initializedOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "initializedOffset",
                            ),
                        );
                    }
                    m_initializedOffset = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_previousTargetAngleOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "previousTargetAngleOffset",
                            ),
                        );
                    }
                    m_previousTargetAngleOffset = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(
                        &m_correspondingAngLimitSolverResultOffset,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "correspondingAngLimitSolverResultOffset",
                            ),
                        );
                    }
                    m_correspondingAngLimitSolverResultOffset = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_targetAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "targetAngle",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 2usize)?;
                    m_targetAngle = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_motor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("motor"),
                        );
                    }
                    m_motor = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
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
        let m_isEnabled = match m_isEnabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isEnabled"),
                );
            }
        };
        let m_motorAxis = match m_motorAxis {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motorAxis"),
                );
            }
        };
        let m_initializedOffset = match m_initializedOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initializedOffset"),
                );
            }
        };
        let m_previousTargetAngleOffset = match m_previousTargetAngleOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "previousTargetAngleOffset",
                    ),
                );
            }
        };
        let m_correspondingAngLimitSolverResultOffset = match m_correspondingAngLimitSolverResultOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "correspondingAngLimitSolverResultOffset",
                    ),
                );
            }
        };
        let m_targetAngle = match m_targetAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetAngle"),
                );
            }
        };
        let m_motor = match m_motor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motor"),
                );
            }
        };
        _serde::__private::Ok(hkpAngMotorConstraintAtom {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_isEnabled,
            m_motorAxis,
            m_initializedOffset,
            m_previousTargetAngleOffset,
            m_correspondingAngLimitSolverResultOffset,
            m_targetAngle,
            m_motor,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpConstraintAtomVisitor::visit_as_parent(&mut __map)?;
        let mut m_isEnabled: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_motorAxis: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_initializedOffset: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_previousTargetAngleOffset: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_correspondingAngLimitSolverResultOffset: _serde::__private::Option<
            i16,
        > = _serde::__private::None;
        let mut m_targetAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_motor: _serde::__private::Option<Pointer> = _serde::__private::None;
        for _ in 0..7usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_isEnabled => {
                        if _serde::__private::Option::is_some(&m_isEnabled) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isEnabled",
                                ),
                            );
                        }
                        m_isEnabled = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_motorAxis => {
                        if _serde::__private::Option::is_some(&m_motorAxis) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "motorAxis",
                                ),
                            );
                        }
                        m_motorAxis = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_initializedOffset => {
                        if _serde::__private::Option::is_some(&m_initializedOffset) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "initializedOffset",
                                ),
                            );
                        }
                        m_initializedOffset = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_previousTargetAngleOffset => {
                        if _serde::__private::Option::is_some(
                            &m_previousTargetAngleOffset,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "previousTargetAngleOffset",
                                ),
                            );
                        }
                        m_previousTargetAngleOffset = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_correspondingAngLimitSolverResultOffset => {
                        if _serde::__private::Option::is_some(
                            &m_correspondingAngLimitSolverResultOffset,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "correspondingAngLimitSolverResultOffset",
                                ),
                            );
                        }
                        m_correspondingAngLimitSolverResultOffset = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_targetAngle => {
                        if _serde::__private::Option::is_some(&m_targetAngle) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "targetAngle",
                                ),
                            );
                        }
                        m_targetAngle = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_motor => {
                        if _serde::__private::Option::is_some(&m_motor) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("motor"),
                            );
                        }
                        m_motor = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
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
        let m_isEnabled = match m_isEnabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isEnabled"),
                );
            }
        };
        let m_motorAxis = match m_motorAxis {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motorAxis"),
                );
            }
        };
        let m_initializedOffset = match m_initializedOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initializedOffset"),
                );
            }
        };
        let m_previousTargetAngleOffset = match m_previousTargetAngleOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "previousTargetAngleOffset",
                    ),
                );
            }
        };
        let m_correspondingAngLimitSolverResultOffset = match m_correspondingAngLimitSolverResultOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "correspondingAngLimitSolverResultOffset",
                    ),
                );
            }
        };
        let m_targetAngle = match m_targetAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetAngle"),
                );
            }
        };
        let m_motor = match m_motor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motor"),
                );
            }
        };
        _serde::__private::Ok(hkpAngMotorConstraintAtom {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_isEnabled,
            m_motorAxis,
            m_initializedOffset,
            m_previousTargetAngleOffset,
            m_correspondingAngLimitSolverResultOffset,
            m_targetAngle,
            m_motor,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpAngMotorConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "isEnabled",
                "motorAxis",
                "initializedOffset",
                "previousTargetAngleOffset",
                "correspondingAngLimitSolverResultOffset",
                "targetAngle",
                "motor",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpAngMotorConstraintAtom",
                FIELDS,
                __hkpAngMotorConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<hkpAngMotorConstraintAtom>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
