use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpHingeLimitsDataAtoms`
/// -         version: `0`
/// -       signature: `0x555876ff`
/// -          size: 144(x86)/144(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpHingeLimitsDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `rotations`(ctype: `struct hkpSetLocalRotationsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size: 112(x86)/112(x86_64)
    ///
    pub m_rotations: hkpSetLocalRotationsConstraintAtom,
    /// # C++ Info
    /// -          name: `angLimit`(ctype: `struct hkpAngLimitConstraintAtom`)
    /// -        offset: 112(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_angLimit: hkpAngLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `2dAng`(ctype: `struct hkp2dAngConstraintAtom`)
    /// -        offset: 128(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_2dAng: hkp2dAngConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpHingeLimitsDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpHingeLimitsDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x555876ff)
        }
    }
    impl _serde::Serialize for hkpHingeLimitsDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x555876ff)));
            let mut serializer = __serializer
                .serialize_struct("hkpHingeLimitsDataAtoms", class_meta)?;
            serializer.serialize_field("rotations", &self.m_rotations)?;
            serializer.serialize_field("angLimit", &self.m_angLimit)?;
            serializer.serialize_field("2dAng", &self.m_2dAng)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_rotations,
    m_angLimit,
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
            "rotations" => Ok(__Field::m_rotations),
            "angLimit" => Ok(__Field::m_angLimit),
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
pub(super) struct __hkpHingeLimitsDataAtomsVisitor<'de> {
    marker: core::marker::PhantomData<hkpHingeLimitsDataAtoms>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpHingeLimitsDataAtomsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpHingeLimitsDataAtoms, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpHingeLimitsDataAtoms>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpHingeLimitsDataAtomsVisitor<'de> {
    type Value = hkpHingeLimitsDataAtoms;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpHingeLimitsDataAtoms")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_rotations: _serde::__private::Option<
            hkpSetLocalRotationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_angLimit: _serde::__private::Option<hkpAngLimitConstraintAtom> = _serde::__private::None;
        let mut m_2dAng: _serde::__private::Option<hkp2dAngConstraintAtom> = _serde::__private::None;
        for i in 0..3usize {
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
                    if _serde::__private::Option::is_some(&m_angLimit) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "angLimit",
                            ),
                        );
                    }
                    m_angLimit = _serde::__private::Some(
                        match __A::next_value::<hkpAngLimitConstraintAtom>(&mut __map) {
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
                _ => {}
            }
        }
        __A::pad(&mut __map, 12usize, 12usize)?;
        let m_rotations = match m_rotations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotations"),
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
        _serde::__private::Ok(hkpHingeLimitsDataAtoms {
            __ptr: __A::class_ptr(&mut __map),
            m_rotations,
            m_angLimit,
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
        let mut m_rotations: _serde::__private::Option<
            hkpSetLocalRotationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_angLimit: _serde::__private::Option<hkpAngLimitConstraintAtom> = _serde::__private::None;
        let mut m_2dAng: _serde::__private::Option<hkp2dAngConstraintAtom> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
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
                __Field::m_angLimit => {
                    if _serde::__private::Option::is_some(&m_angLimit) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "angLimit",
                            ),
                        );
                    }
                    m_angLimit = _serde::__private::Some(
                        match __A::next_value::<hkpAngLimitConstraintAtom>(&mut __map) {
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
        let m_rotations = match m_rotations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotations"),
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
        _serde::__private::Ok(hkpHingeLimitsDataAtoms {
            __ptr: __A::class_ptr(&mut __map),
            m_rotations,
            m_angLimit,
            m_2dAng,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpHingeLimitsDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["rotations", "angLimit", "2dAng"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpHingeLimitsDataAtoms",
                FIELDS,
                __hkpHingeLimitsDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<hkpHingeLimitsDataAtoms>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
