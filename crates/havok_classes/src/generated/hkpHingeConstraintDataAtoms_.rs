use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpHingeConstraintDataAtoms`
/// -         version: `1`
/// -       signature: `0x6958371c`
/// -          size: 192(x86)/192(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpHingeConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `transforms`(ctype: `struct hkpSetLocalTransformsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size: 144(x86)/144(x86_64)
    ///
    pub m_transforms: hkpSetLocalTransformsConstraintAtom,
    /// # C++ Info
    /// -          name: `setupStabilization`(ctype: `struct hkpSetupStabilizationAtom`)
    /// -        offset: 144(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_setupStabilization: hkpSetupStabilizationAtom,
    /// # C++ Info
    /// -          name: `2dAng`(ctype: `struct hkp2dAngConstraintAtom`)
    /// -        offset: 160(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_2dAng: hkp2dAngConstraintAtom,
    /// # C++ Info
    /// -          name: `ballSocket`(ctype: `struct hkpBallSocketConstraintAtom`)
    /// -        offset: 164(x86)/164(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_ballSocket: hkpBallSocketConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpHingeConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpHingeConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6958371c)
        }
    }
    impl _serde::Serialize for hkpHingeConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6958371c)));
            let mut serializer = __serializer
                .serialize_struct("hkpHingeConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer
                .serialize_field("setupStabilization", &self.m_setupStabilization)?;
            serializer.serialize_field("2dAng", &self.m_2dAng)?;
            serializer.serialize_field("ballSocket", &self.m_ballSocket)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_transforms,
    m_setupStabilization,
    m_2dAng,
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
            "2dAng" => Ok(__Field::m_2dAng),
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
pub(super) struct __hkpHingeConstraintDataAtomsVisitor<'de> {
    marker: core::marker::PhantomData<hkpHingeConstraintDataAtoms>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpHingeConstraintDataAtomsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpHingeConstraintDataAtoms, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpHingeConstraintDataAtoms>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpHingeConstraintDataAtomsVisitor<'de> {
    type Value = hkpHingeConstraintDataAtoms;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpHingeConstraintDataAtoms",
        )
    }
    fn visit_struct_for_bytes<__A>(
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
        let mut m_2dAng: _serde::__private::Option<hkp2dAngConstraintAtom> = _serde::__private::None;
        let mut m_ballSocket: _serde::__private::Option<hkpBallSocketConstraintAtom> = _serde::__private::None;
        for i in 0..4usize {
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
                3usize => {
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
        __A::pad(&mut __map, 12usize, 12usize)?;
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
                    <__A::Error as _serde::de::Error>::missing_field("ballSocket"),
                );
            }
        };
        _serde::__private::Ok(hkpHingeConstraintDataAtoms {
            __ptr: __A::class_ptr(&mut __map),
            m_transforms,
            m_setupStabilization,
            m_2dAng,
            m_ballSocket,
        })
    }
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
        let mut m_2dAng: _serde::__private::Option<hkp2dAngConstraintAtom> = _serde::__private::None;
        let mut m_ballSocket: _serde::__private::Option<hkpBallSocketConstraintAtom> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
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
                                    return _serde::__private::Err(__err);
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
                                    return _serde::__private::Err(__err);
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
                    <__A::Error as _serde::de::Error>::missing_field("ballSocket"),
                );
            }
        };
        _serde::__private::Ok(hkpHingeConstraintDataAtoms {
            __ptr: __A::class_ptr(&mut __map),
            m_transforms,
            m_setupStabilization,
            m_2dAng,
            m_ballSocket,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpHingeConstraintDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "transforms",
                "setupStabilization",
                "2dAng",
                "ballSocket",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpHingeConstraintDataAtoms",
                FIELDS,
                __hkpHingeConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpHingeConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
