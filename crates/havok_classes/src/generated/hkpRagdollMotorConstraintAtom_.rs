use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRagdollMotorConstraintAtom`
/// -         version: `0`
/// -       signature: `0x71013826`
/// -          size:  80(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRagdollMotorConstraintAtom {
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
    /// -          name: `initializedOffset`(ctype: `hkInt16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_initializedOffset: i16,
    /// # C++ Info
    /// -          name: `previousTargetAnglesOffset`(ctype: `hkInt16`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_previousTargetAnglesOffset: i16,
    /// # C++ Info
    /// -          name: `target_bRca`(ctype: `hkMatrix3`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_target_bRca: Matrix3,
    /// # C++ Info
    /// -          name: `motors`(ctype: `struct hkpConstraintMotor*`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_motors: [Pointer; 3usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpRagdollMotorConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRagdollMotorConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x71013826)
        }
    }
    impl _serde::Serialize for hkpRagdollMotorConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x71013826)));
            let mut serializer = __serializer
                .serialize_struct("hkpRagdollMotorConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("initializedOffset", &self.m_initializedOffset)?;
            serializer
                .serialize_field(
                    "previousTargetAnglesOffset",
                    &self.m_previousTargetAnglesOffset,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("target_bRca", &self.m_target_bRca)?;
            serializer.serialize_field("motors", &self.m_motors.as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_isEnabled,
    m_initializedOffset,
    m_previousTargetAnglesOffset,
    m_target_bRca,
    m_motors,
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
            "initializedOffset" => Ok(__Field::m_initializedOffset),
            "previousTargetAnglesOffset" => Ok(__Field::m_previousTargetAnglesOffset),
            "target_bRca" => Ok(__Field::m_target_bRca),
            "motors" => Ok(__Field::m_motors),
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
pub(super) struct __hkpRagdollMotorConstraintAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpRagdollMotorConstraintAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpRagdollMotorConstraintAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpRagdollMotorConstraintAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpRagdollMotorConstraintAtom>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpRagdollMotorConstraintAtomVisitor<'de> {
    type Value = hkpRagdollMotorConstraintAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpRagdollMotorConstraintAtom",
        )
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
        let mut m_initializedOffset: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_previousTargetAnglesOffset: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_target_bRca: _serde::__private::Option<Matrix3> = _serde::__private::None;
        let mut m_motors: _serde::__private::Option<[Pointer; 3usize]> = _serde::__private::None;
        for i in 0..5usize {
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
                    if _serde::__private::Option::is_some(&m_initializedOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "initializedOffset",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_initializedOffset = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(
                        &m_previousTargetAnglesOffset,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "previousTargetAnglesOffset",
                            ),
                        );
                    }
                    m_previousTargetAnglesOffset = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_target_bRca) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "target_bRca",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 8usize, 8usize)?;
                    m_target_bRca = _serde::__private::Some(
                        match __A::next_value::<Matrix3>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_motors) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("motors"),
                        );
                    }
                    m_motors = _serde::__private::Some(
                        match __A::next_value::<[Pointer; 3usize]>(&mut __map) {
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
        __A::pad(&mut __map, 4usize, 8usize)?;
        let m_isEnabled = match m_isEnabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isEnabled"),
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
        let m_previousTargetAnglesOffset = match m_previousTargetAnglesOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "previousTargetAnglesOffset",
                    ),
                );
            }
        };
        let m_target_bRca = match m_target_bRca {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("target_bRca"),
                );
            }
        };
        let m_motors = match m_motors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motors"),
                );
            }
        };
        _serde::__private::Ok(hkpRagdollMotorConstraintAtom {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_isEnabled,
            m_initializedOffset,
            m_previousTargetAnglesOffset,
            m_target_bRca,
            m_motors,
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
        let mut m_initializedOffset: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_previousTargetAnglesOffset: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_target_bRca: _serde::__private::Option<Matrix3> = _serde::__private::None;
        let mut m_motors: _serde::__private::Option<[Pointer; 3usize]> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
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
                __Field::m_previousTargetAnglesOffset => {
                    if _serde::__private::Option::is_some(
                        &m_previousTargetAnglesOffset,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "previousTargetAnglesOffset",
                            ),
                        );
                    }
                    m_previousTargetAnglesOffset = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_target_bRca => {
                    if _serde::__private::Option::is_some(&m_target_bRca) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "target_bRca",
                            ),
                        );
                    }
                    m_target_bRca = _serde::__private::Some(
                        match __A::next_value::<Matrix3>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_motors => {
                    if _serde::__private::Option::is_some(&m_motors) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("motors"),
                        );
                    }
                    m_motors = _serde::__private::Some(
                        match __A::next_value::<[Pointer; 3usize]>(&mut __map) {
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
        let m_initializedOffset = match m_initializedOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initializedOffset"),
                );
            }
        };
        let m_previousTargetAnglesOffset = match m_previousTargetAnglesOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "previousTargetAnglesOffset",
                    ),
                );
            }
        };
        let m_target_bRca = match m_target_bRca {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("target_bRca"),
                );
            }
        };
        let m_motors = match m_motors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motors"),
                );
            }
        };
        _serde::__private::Ok(hkpRagdollMotorConstraintAtom {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_isEnabled,
            m_initializedOffset,
            m_previousTargetAnglesOffset,
            m_target_bRca,
            m_motors,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpRagdollMotorConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "isEnabled",
                "initializedOffset",
                "previousTargetAnglesOffset",
                "target_bRca",
                "motors",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpRagdollMotorConstraintAtom",
                FIELDS,
                __hkpRagdollMotorConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpRagdollMotorConstraintAtom,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
