use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPulleyConstraintAtom`
/// -         version: `0`
/// -       signature: `0x94a08848`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPulleyConstraintAtom {
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
    /// -          name: `fixedPivotAinWorld`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_fixedPivotAinWorld: Vector4,
    /// # C++ Info
    /// -          name: `fixedPivotBinWorld`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_fixedPivotBinWorld: Vector4,
    /// # C++ Info
    /// -          name: `ropeLength`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_ropeLength: f32,
    /// # C++ Info
    /// -          name: `leverageOnBodyB`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_leverageOnBodyB: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPulleyConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPulleyConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x94a08848)
        }
    }
    impl _serde::Serialize for hkpPulleyConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x94a08848)));
            let mut serializer = __serializer
                .serialize_struct("hkpPulleyConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer
                .serialize_field("fixedPivotAinWorld", &self.m_fixedPivotAinWorld)?;
            serializer
                .serialize_field("fixedPivotBinWorld", &self.m_fixedPivotBinWorld)?;
            serializer.serialize_field("ropeLength", &self.m_ropeLength)?;
            serializer.serialize_field("leverageOnBodyB", &self.m_leverageOnBodyB)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_fixedPivotAinWorld,
    m_fixedPivotBinWorld,
    m_ropeLength,
    m_leverageOnBodyB,
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
            "fixedPivotAinWorld" => Ok(__Field::m_fixedPivotAinWorld),
            "fixedPivotBinWorld" => Ok(__Field::m_fixedPivotBinWorld),
            "ropeLength" => Ok(__Field::m_ropeLength),
            "leverageOnBodyB" => Ok(__Field::m_leverageOnBodyB),
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
pub(super) struct __hkpPulleyConstraintAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpPulleyConstraintAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPulleyConstraintAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpPulleyConstraintAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpPulleyConstraintAtom>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpPulleyConstraintAtomVisitor<'de> {
    type Value = hkpPulleyConstraintAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpPulleyConstraintAtom")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_fixedPivotAinWorld: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_fixedPivotBinWorld: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_ropeLength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_leverageOnBodyB: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_fixedPivotAinWorld) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "fixedPivotAinWorld",
                            ),
                        );
                    }
                    m_fixedPivotAinWorld = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_fixedPivotBinWorld) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "fixedPivotBinWorld",
                            ),
                        );
                    }
                    m_fixedPivotBinWorld = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_ropeLength) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ropeLength",
                            ),
                        );
                    }
                    m_ropeLength = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_leverageOnBodyB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "leverageOnBodyB",
                            ),
                        );
                    }
                    m_leverageOnBodyB = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
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
        let m_fixedPivotAinWorld = match m_fixedPivotAinWorld {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "fixedPivotAinWorld",
                    ),
                );
            }
        };
        let m_fixedPivotBinWorld = match m_fixedPivotBinWorld {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "fixedPivotBinWorld",
                    ),
                );
            }
        };
        let m_ropeLength = match m_ropeLength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ropeLength"),
                );
            }
        };
        let m_leverageOnBodyB = match m_leverageOnBodyB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("leverageOnBodyB"),
                );
            }
        };
        _serde::__private::Ok(hkpPulleyConstraintAtom {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_fixedPivotAinWorld,
            m_fixedPivotBinWorld,
            m_ropeLength,
            m_leverageOnBodyB,
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
        let mut m_fixedPivotAinWorld: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_fixedPivotBinWorld: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_ropeLength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_leverageOnBodyB: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_fixedPivotAinWorld => {
                        if _serde::__private::Option::is_some(&m_fixedPivotAinWorld) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "fixedPivotAinWorld",
                                ),
                            );
                        }
                        m_fixedPivotAinWorld = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_fixedPivotBinWorld => {
                        if _serde::__private::Option::is_some(&m_fixedPivotBinWorld) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "fixedPivotBinWorld",
                                ),
                            );
                        }
                        m_fixedPivotBinWorld = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_ropeLength => {
                        if _serde::__private::Option::is_some(&m_ropeLength) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ropeLength",
                                ),
                            );
                        }
                        m_ropeLength = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_leverageOnBodyB => {
                        if _serde::__private::Option::is_some(&m_leverageOnBodyB) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "leverageOnBodyB",
                                ),
                            );
                        }
                        m_leverageOnBodyB = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
        let m_fixedPivotAinWorld = match m_fixedPivotAinWorld {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "fixedPivotAinWorld",
                    ),
                );
            }
        };
        let m_fixedPivotBinWorld = match m_fixedPivotBinWorld {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "fixedPivotBinWorld",
                    ),
                );
            }
        };
        let m_ropeLength = match m_ropeLength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ropeLength"),
                );
            }
        };
        let m_leverageOnBodyB = match m_leverageOnBodyB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("leverageOnBodyB"),
                );
            }
        };
        _serde::__private::Ok(hkpPulleyConstraintAtom {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_fixedPivotAinWorld,
            m_fixedPivotBinWorld,
            m_ropeLength,
            m_leverageOnBodyB,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPulleyConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "fixedPivotAinWorld",
                "fixedPivotBinWorld",
                "ropeLength",
                "leverageOnBodyB",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPulleyConstraintAtom",
                FIELDS,
                __hkpPulleyConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<hkpPulleyConstraintAtom>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
