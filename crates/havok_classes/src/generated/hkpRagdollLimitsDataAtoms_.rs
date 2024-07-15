use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpRagdollLimitsDataAtoms`
/// - version: `0`
/// - signature: `0x82b894c3`
/// - size: `176`(x86)/`176`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRagdollLimitsDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `rotations`(ctype: `struct hkpSetLocalRotationsConstraintAtom`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `112`(x86)/`112`(x86_64)
    pub m_rotations: hkpSetLocalRotationsConstraintAtom,
    /// # C++ Info
    /// - name: `twistLimit`(ctype: `struct hkpTwistLimitConstraintAtom`)
    /// - offset: `112`(x86)/`112`(x86_64)
    /// - type_size: ` 20`(x86)/` 20`(x86_64)
    pub m_twistLimit: hkpTwistLimitConstraintAtom,
    /// # C++ Info
    /// - name: `coneLimit`(ctype: `struct hkpConeLimitConstraintAtom`)
    /// - offset: `132`(x86)/`132`(x86_64)
    /// - type_size: ` 20`(x86)/` 20`(x86_64)
    pub m_coneLimit: hkpConeLimitConstraintAtom,
    /// # C++ Info
    /// - name: `planesLimit`(ctype: `struct hkpConeLimitConstraintAtom`)
    /// - offset: `152`(x86)/`152`(x86_64)
    /// - type_size: ` 20`(x86)/` 20`(x86_64)
    pub m_planesLimit: hkpConeLimitConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpRagdollLimitsDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRagdollLimitsDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x82b894c3)
        }
    }
    impl _serde::Serialize for hkpRagdollLimitsDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x82b894c3)));
            let mut serializer = __serializer
                .serialize_struct("hkpRagdollLimitsDataAtoms", class_meta)?;
            serializer.serialize_field("rotations", &self.m_rotations)?;
            serializer.serialize_field("twistLimit", &self.m_twistLimit)?;
            serializer.serialize_field("coneLimit", &self.m_coneLimit)?;
            serializer.serialize_field("planesLimit", &self.m_planesLimit)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_rotations,
    m_twistLimit,
    m_coneLimit,
    m_planesLimit,
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
            "rotations" => Ok(__Field::m_rotations),
            "twistLimit" => Ok(__Field::m_twistLimit),
            "coneLimit" => Ok(__Field::m_coneLimit),
            "planesLimit" => Ok(__Field::m_planesLimit),
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
pub(super) struct __hkpRagdollLimitsDataAtomsVisitor<'de> {
    marker: core::marker::PhantomData<hkpRagdollLimitsDataAtoms>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpRagdollLimitsDataAtomsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpRagdollLimitsDataAtoms, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpRagdollLimitsDataAtoms>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpRagdollLimitsDataAtomsVisitor<'de> {
    type Value = hkpRagdollLimitsDataAtoms;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpRagdollLimitsDataAtoms")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_rotations: _serde::__private::Option<
            hkpSetLocalRotationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_twistLimit: _serde::__private::Option<hkpTwistLimitConstraintAtom> = _serde::__private::None;
        let mut m_coneLimit: _serde::__private::Option<hkpConeLimitConstraintAtom> = _serde::__private::None;
        let mut m_planesLimit: _serde::__private::Option<hkpConeLimitConstraintAtom> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_rotations) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotations",
                            ),
                        );
                    }
                    m_rotations = _serde::__private::Some(
                        match __A::next_value::<
                            hkpSetLocalRotationsConstraintAtom,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
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
                2usize => {
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
                3usize => {
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
                _ => {}
            }
        }
        __A::pad(&mut __map, 4usize, 4usize)?;
        let m_rotations = match m_rotations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotations"),
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
        _serde::__private::Ok(hkpRagdollLimitsDataAtoms {
            __ptr,
            m_rotations,
            m_twistLimit,
            m_coneLimit,
            m_planesLimit,
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
        let mut m_rotations: _serde::__private::Option<
            hkpSetLocalRotationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_twistLimit: _serde::__private::Option<hkpTwistLimitConstraintAtom> = _serde::__private::None;
        let mut m_coneLimit: _serde::__private::Option<hkpConeLimitConstraintAtom> = _serde::__private::None;
        let mut m_planesLimit: _serde::__private::Option<hkpConeLimitConstraintAtom> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_rotations => {
                        if _serde::__private::Option::is_some(&m_rotations) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotations",
                                ),
                            );
                        }
                        m_rotations = _serde::__private::Some(
                            match __A::next_value::<
                                hkpSetLocalRotationsConstraintAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
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
                                    return _serde::__private::Err(__err);
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
                                    return _serde::__private::Err(__err);
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
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        let m_rotations = match m_rotations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotations"),
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
        _serde::__private::Ok(hkpRagdollLimitsDataAtoms {
            __ptr,
            m_rotations,
            m_twistLimit,
            m_coneLimit,
            m_planesLimit,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpRagdollLimitsDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "rotations",
                "twistLimit",
                "coneLimit",
                "planesLimit",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpRagdollLimitsDataAtoms",
                FIELDS,
                __hkpRagdollLimitsDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<hkpRagdollLimitsDataAtoms>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
