use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpAngFrictionConstraintAtom`
/// -         version: `0`
/// -       signature: `0xf313aa80`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAngFrictionConstraintAtom {
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
    /// -          name: `isEnabled`(ctype: `hkUint8`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isEnabled: u8,
    /// # C++ Info
    /// -          name: `firstFrictionAxis`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_firstFrictionAxis: u8,
    /// # C++ Info
    /// -          name: `numFrictionAxes`(ctype: `hkUint8`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numFrictionAxes: u8,
    /// # C++ Info
    /// -          name: `maxFrictionTorque`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxFrictionTorque: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpAngFrictionConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpAngFrictionConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf313aa80)
        }
    }
    impl _serde::Serialize for hkpAngFrictionConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf313aa80)));
            let mut serializer = __serializer
                .serialize_struct("hkpAngFrictionConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.serialize_field("firstFrictionAxis", &self.m_firstFrictionAxis)?;
            serializer.serialize_field("numFrictionAxes", &self.m_numFrictionAxes)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("maxFrictionTorque", &self.m_maxFrictionTorque)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_isEnabled,
    m_firstFrictionAxis,
    m_numFrictionAxes,
    m_maxFrictionTorque,
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
            "firstFrictionAxis" => Ok(__Field::m_firstFrictionAxis),
            "numFrictionAxes" => Ok(__Field::m_numFrictionAxes),
            "maxFrictionTorque" => Ok(__Field::m_maxFrictionTorque),
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
pub(super) struct __hkpAngFrictionConstraintAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpAngFrictionConstraintAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpAngFrictionConstraintAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpAngFrictionConstraintAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpAngFrictionConstraintAtom>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpAngFrictionConstraintAtomVisitor<'de> {
    type Value = hkpAngFrictionConstraintAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpAngFrictionConstraintAtom",
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
        let parent = __A::parent_value(&mut __map)?;
        let mut m_isEnabled: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_firstFrictionAxis: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_numFrictionAxes: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_maxFrictionTorque: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
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
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_firstFrictionAxis) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "firstFrictionAxis",
                            ),
                        );
                    }
                    m_firstFrictionAxis = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_numFrictionAxes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numFrictionAxes",
                            ),
                        );
                    }
                    m_numFrictionAxes = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_maxFrictionTorque) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxFrictionTorque",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_maxFrictionTorque = _serde::__private::Some(
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
        let m_isEnabled = match m_isEnabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isEnabled"),
                );
            }
        };
        let m_firstFrictionAxis = match m_firstFrictionAxis {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("firstFrictionAxis"),
                );
            }
        };
        let m_numFrictionAxes = match m_numFrictionAxes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numFrictionAxes"),
                );
            }
        };
        let m_maxFrictionTorque = match m_maxFrictionTorque {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFrictionTorque"),
                );
            }
        };
        _serde::__private::Ok(hkpAngFrictionConstraintAtom {
            __ptr,
            parent,
            m_isEnabled,
            m_firstFrictionAxis,
            m_numFrictionAxes,
            m_maxFrictionTorque,
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
        let parent = __hkpConstraintAtomVisitor::visit_as_parent(&mut __map)?;
        let mut m_isEnabled: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_firstFrictionAxis: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_numFrictionAxes: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_maxFrictionTorque: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..4usize {
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
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_firstFrictionAxis => {
                        if _serde::__private::Option::is_some(&m_firstFrictionAxis) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "firstFrictionAxis",
                                ),
                            );
                        }
                        m_firstFrictionAxis = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numFrictionAxes => {
                        if _serde::__private::Option::is_some(&m_numFrictionAxes) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numFrictionAxes",
                                ),
                            );
                        }
                        m_numFrictionAxes = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxFrictionTorque => {
                        if _serde::__private::Option::is_some(&m_maxFrictionTorque) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxFrictionTorque",
                                ),
                            );
                        }
                        m_maxFrictionTorque = _serde::__private::Some(
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
        let m_isEnabled = match m_isEnabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isEnabled"),
                );
            }
        };
        let m_firstFrictionAxis = match m_firstFrictionAxis {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("firstFrictionAxis"),
                );
            }
        };
        let m_numFrictionAxes = match m_numFrictionAxes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numFrictionAxes"),
                );
            }
        };
        let m_maxFrictionTorque = match m_maxFrictionTorque {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFrictionTorque"),
                );
            }
        };
        _serde::__private::Ok(hkpAngFrictionConstraintAtom {
            __ptr,
            parent,
            m_isEnabled,
            m_firstFrictionAxis,
            m_numFrictionAxes,
            m_maxFrictionTorque,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpAngFrictionConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "isEnabled",
                "firstFrictionAxis",
                "numFrictionAxes",
                "maxFrictionTorque",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpAngFrictionConstraintAtom",
                FIELDS,
                __hkpAngFrictionConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpAngFrictionConstraintAtom,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
