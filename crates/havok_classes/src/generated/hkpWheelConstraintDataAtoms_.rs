use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpWheelConstraintDataAtoms`
/// -         version: `0`
/// -       signature: `0x1188cbe1`
/// -          size: 304(x86)/304(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpWheelConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `suspensionBase`(ctype: `struct hkpSetLocalTransformsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size: 144(x86)/144(x86_64)
    ///
    pub m_suspensionBase: hkpSetLocalTransformsConstraintAtom,
    /// # C++ Info
    /// -          name: `lin0Limit`(ctype: `struct hkpLinLimitConstraintAtom`)
    /// -        offset: 144(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_lin0Limit: hkpLinLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `lin0Soft`(ctype: `struct hkpLinSoftConstraintAtom`)
    /// -        offset: 156(x86)/156(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_lin0Soft: hkpLinSoftConstraintAtom,
    /// # C++ Info
    /// -          name: `lin1`(ctype: `struct hkpLinConstraintAtom`)
    /// -        offset: 168(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lin1: hkpLinConstraintAtom,
    /// # C++ Info
    /// -          name: `lin2`(ctype: `struct hkpLinConstraintAtom`)
    /// -        offset: 172(x86)/172(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lin2: hkpLinConstraintAtom,
    /// # C++ Info
    /// -          name: `steeringBase`(ctype: `struct hkpSetLocalRotationsConstraintAtom`)
    /// -        offset: 176(x86)/176(x86_64)
    /// -     type_size: 112(x86)/112(x86_64)
    ///
    pub m_steeringBase: hkpSetLocalRotationsConstraintAtom,
    /// # C++ Info
    /// -          name: `2dAng`(ctype: `struct hkp2dAngConstraintAtom`)
    /// -        offset: 288(x86)/288(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_2dAng: hkp2dAngConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpWheelConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpWheelConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x1188cbe1)
        }
    }
    impl _serde::Serialize for hkpWheelConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x1188cbe1)));
            let mut serializer = __serializer
                .serialize_struct("hkpWheelConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("suspensionBase", &self.m_suspensionBase)?;
            serializer.serialize_field("lin0Limit", &self.m_lin0Limit)?;
            serializer.serialize_field("lin0Soft", &self.m_lin0Soft)?;
            serializer.serialize_field("lin1", &self.m_lin1)?;
            serializer.serialize_field("lin2", &self.m_lin2)?;
            serializer.serialize_field("steeringBase", &self.m_steeringBase)?;
            serializer.serialize_field("2dAng", &self.m_2dAng)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_suspensionBase,
    m_lin0Limit,
    m_lin0Soft,
    m_lin1,
    m_lin2,
    m_steeringBase,
    m_2dAng,
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
            "suspensionBase" => Ok(__Field::m_suspensionBase),
            "lin0Limit" => Ok(__Field::m_lin0Limit),
            "lin0Soft" => Ok(__Field::m_lin0Soft),
            "lin1" => Ok(__Field::m_lin1),
            "lin2" => Ok(__Field::m_lin2),
            "steeringBase" => Ok(__Field::m_steeringBase),
            "2dAng" => Ok(__Field::m_2dAng),
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
pub(super) struct __hkpWheelConstraintDataAtomsVisitor<'de> {
    marker: core::marker::PhantomData<hkpWheelConstraintDataAtoms>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpWheelConstraintDataAtomsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpWheelConstraintDataAtoms, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpWheelConstraintDataAtoms>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpWheelConstraintDataAtomsVisitor<'de> {
    type Value = hkpWheelConstraintDataAtoms;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpWheelConstraintDataAtoms",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_suspensionBase: _serde::__private::Option<
            hkpSetLocalTransformsConstraintAtom,
        > = _serde::__private::None;
        let mut m_lin0Limit: _serde::__private::Option<hkpLinLimitConstraintAtom> = _serde::__private::None;
        let mut m_lin0Soft: _serde::__private::Option<hkpLinSoftConstraintAtom> = _serde::__private::None;
        let mut m_lin1: _serde::__private::Option<hkpLinConstraintAtom> = _serde::__private::None;
        let mut m_lin2: _serde::__private::Option<hkpLinConstraintAtom> = _serde::__private::None;
        let mut m_steeringBase: _serde::__private::Option<
            hkpSetLocalRotationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_2dAng: _serde::__private::Option<hkp2dAngConstraintAtom> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_suspensionBase) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "suspensionBase",
                            ),
                        );
                    }
                    m_suspensionBase = _serde::__private::Some(
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
                    if _serde::__private::Option::is_some(&m_lin0Limit) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "lin0Limit",
                            ),
                        );
                    }
                    m_lin0Limit = _serde::__private::Some(
                        match __A::next_value::<hkpLinLimitConstraintAtom>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_lin0Soft) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "lin0Soft",
                            ),
                        );
                    }
                    m_lin0Soft = _serde::__private::Some(
                        match __A::next_value::<hkpLinSoftConstraintAtom>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_lin1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("lin1"),
                        );
                    }
                    m_lin1 = _serde::__private::Some(
                        match __A::next_value::<hkpLinConstraintAtom>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_lin2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("lin2"),
                        );
                    }
                    m_lin2 = _serde::__private::Some(
                        match __A::next_value::<hkpLinConstraintAtom>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_steeringBase) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "steeringBase",
                            ),
                        );
                    }
                    m_steeringBase = _serde::__private::Some(
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
                6usize => {
                    if _serde::__private::Option::is_some(&m_2dAng) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("2dAng"),
                        );
                    }
                    m_2dAng = _serde::__private::Some(
                        match __A::next_value::<hkp2dAngConstraintAtom>(&mut __map) {
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
        __A::pad(&mut __map, 12usize, 12usize)?;
        let m_suspensionBase = match m_suspensionBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("suspensionBase"),
                );
            }
        };
        let m_lin0Limit = match m_lin0Limit {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lin0Limit"),
                );
            }
        };
        let m_lin0Soft = match m_lin0Soft {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lin0Soft"),
                );
            }
        };
        let m_lin1 = match m_lin1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lin1"),
                );
            }
        };
        let m_lin2 = match m_lin2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lin2"),
                );
            }
        };
        let m_steeringBase = match m_steeringBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("steeringBase"),
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
        _serde::__private::Ok(hkpWheelConstraintDataAtoms {
            __ptr: __A::class_ptr(&mut __map),
            m_suspensionBase,
            m_lin0Limit,
            m_lin0Soft,
            m_lin1,
            m_lin2,
            m_steeringBase,
            m_2dAng,
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
        let mut m_suspensionBase: _serde::__private::Option<
            hkpSetLocalTransformsConstraintAtom,
        > = _serde::__private::None;
        let mut m_lin0Limit: _serde::__private::Option<hkpLinLimitConstraintAtom> = _serde::__private::None;
        let mut m_lin0Soft: _serde::__private::Option<hkpLinSoftConstraintAtom> = _serde::__private::None;
        let mut m_lin1: _serde::__private::Option<hkpLinConstraintAtom> = _serde::__private::None;
        let mut m_lin2: _serde::__private::Option<hkpLinConstraintAtom> = _serde::__private::None;
        let mut m_steeringBase: _serde::__private::Option<
            hkpSetLocalRotationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_2dAng: _serde::__private::Option<hkp2dAngConstraintAtom> = _serde::__private::None;
        for _ in 0..7usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_suspensionBase => {
                        if _serde::__private::Option::is_some(&m_suspensionBase) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "suspensionBase",
                                ),
                            );
                        }
                        m_suspensionBase = _serde::__private::Some(
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
                    __Field::m_lin0Limit => {
                        if _serde::__private::Option::is_some(&m_lin0Limit) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "lin0Limit",
                                ),
                            );
                        }
                        m_lin0Limit = _serde::__private::Some(
                            match __A::next_value::<
                                hkpLinLimitConstraintAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_lin0Soft => {
                        if _serde::__private::Option::is_some(&m_lin0Soft) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "lin0Soft",
                                ),
                            );
                        }
                        m_lin0Soft = _serde::__private::Some(
                            match __A::next_value::<
                                hkpLinSoftConstraintAtom,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_lin1 => {
                        if _serde::__private::Option::is_some(&m_lin1) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("lin1"),
                            );
                        }
                        m_lin1 = _serde::__private::Some(
                            match __A::next_value::<hkpLinConstraintAtom>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_lin2 => {
                        if _serde::__private::Option::is_some(&m_lin2) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("lin2"),
                            );
                        }
                        m_lin2 = _serde::__private::Some(
                            match __A::next_value::<hkpLinConstraintAtom>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_steeringBase => {
                        if _serde::__private::Option::is_some(&m_steeringBase) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "steeringBase",
                                ),
                            );
                        }
                        m_steeringBase = _serde::__private::Some(
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
                    __Field::m_2dAng => {
                        if _serde::__private::Option::is_some(&m_2dAng) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("2dAng"),
                            );
                        }
                        m_2dAng = _serde::__private::Some(
                            match __A::next_value::<hkp2dAngConstraintAtom>(&mut __map) {
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
        let m_suspensionBase = match m_suspensionBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("suspensionBase"),
                );
            }
        };
        let m_lin0Limit = match m_lin0Limit {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lin0Limit"),
                );
            }
        };
        let m_lin0Soft = match m_lin0Soft {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lin0Soft"),
                );
            }
        };
        let m_lin1 = match m_lin1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lin1"),
                );
            }
        };
        let m_lin2 = match m_lin2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lin2"),
                );
            }
        };
        let m_steeringBase = match m_steeringBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("steeringBase"),
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
        _serde::__private::Ok(hkpWheelConstraintDataAtoms {
            __ptr,
            m_suspensionBase,
            m_lin0Limit,
            m_lin0Soft,
            m_lin1,
            m_lin2,
            m_steeringBase,
            m_2dAng,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpWheelConstraintDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "suspensionBase",
                "lin0Limit",
                "lin0Soft",
                "lin1",
                "lin2",
                "steeringBase",
                "2dAng",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpWheelConstraintDataAtoms",
                FIELDS,
                __hkpWheelConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpWheelConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
