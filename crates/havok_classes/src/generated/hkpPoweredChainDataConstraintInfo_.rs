use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpPoweredChainDataConstraintInfo`
/// - version: `0`
/// - signature: `0xf88aee25`
/// - size: ` 80`(x86)/` 96`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPoweredChainDataConstraintInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `pivotInA`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_pivotInA: Vector4,
    /// # C++ Info
    /// - name: `pivotInB`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_pivotInB: Vector4,
    /// # C++ Info
    /// - name: `aTc`(ctype: `hkQuaternion`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_aTc: Quaternion,
    /// # C++ Info
    /// - name: `bTc`(ctype: `hkQuaternion`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_bTc: Quaternion,
    /// # C++ Info
    /// - name: `motors`(ctype: `struct hkpConstraintMotor*`)
    /// - offset: ` 64`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 24`(x86_64)
    pub m_motors: [Pointer; 3usize],
    /// # C++ Info
    /// - name: `switchBodies`(ctype: `hkBool`)
    /// - offset: ` 76`(x86)/` 88`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_switchBodies: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPoweredChainDataConstraintInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPoweredChainDataConstraintInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf88aee25)
        }
    }
    impl _serde::Serialize for hkpPoweredChainDataConstraintInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf88aee25)));
            let mut serializer = __serializer
                .serialize_struct("hkpPoweredChainDataConstraintInfo", class_meta)?;
            serializer.serialize_field("pivotInA", &self.m_pivotInA)?;
            serializer.serialize_field("pivotInB", &self.m_pivotInB)?;
            serializer.serialize_field("aTc", &self.m_aTc)?;
            serializer.serialize_field("bTc", &self.m_bTc)?;
            serializer.serialize_field("motors", &self.m_motors.as_slice())?;
            serializer.serialize_field("switchBodies", &self.m_switchBodies)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_pivotInA,
    m_pivotInB,
    m_aTc,
    m_bTc,
    m_motors,
    m_switchBodies,
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
            "pivotInA" => Ok(__Field::m_pivotInA),
            "pivotInB" => Ok(__Field::m_pivotInB),
            "aTc" => Ok(__Field::m_aTc),
            "bTc" => Ok(__Field::m_bTc),
            "motors" => Ok(__Field::m_motors),
            "switchBodies" => Ok(__Field::m_switchBodies),
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
pub(super) struct __hkpPoweredChainDataConstraintInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkpPoweredChainDataConstraintInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPoweredChainDataConstraintInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpPoweredChainDataConstraintInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpPoweredChainDataConstraintInfo,
                >,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpPoweredChainDataConstraintInfoVisitor<'de> {
    type Value = hkpPoweredChainDataConstraintInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpPoweredChainDataConstraintInfo",
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
        let mut m_pivotInA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_pivotInB: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_aTc: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_bTc: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_motors: _serde::__private::Option<[Pointer; 3usize]> = _serde::__private::None;
        let mut m_switchBodies: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..6usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_pivotInA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pivotInA",
                            ),
                        );
                    }
                    m_pivotInA = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_pivotInB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pivotInB",
                            ),
                        );
                    }
                    m_pivotInB = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_aTc) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("aTc"),
                        );
                    }
                    m_aTc = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_bTc) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bTc"),
                        );
                    }
                    m_bTc = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
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
                5usize => {
                    if _serde::__private::Option::is_some(&m_switchBodies) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "switchBodies",
                            ),
                        );
                    }
                    m_switchBodies = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
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
        __A::pad(&mut __map, 3usize, 7usize)?;
        let m_pivotInA = match m_pivotInA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivotInA"),
                );
            }
        };
        let m_pivotInB = match m_pivotInB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivotInB"),
                );
            }
        };
        let m_aTc = match m_aTc {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("aTc"),
                );
            }
        };
        let m_bTc = match m_bTc {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bTc"),
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
        let m_switchBodies = match m_switchBodies {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("switchBodies"),
                );
            }
        };
        _serde::__private::Ok(hkpPoweredChainDataConstraintInfo {
            __ptr,
            m_pivotInA,
            m_pivotInB,
            m_aTc,
            m_bTc,
            m_motors,
            m_switchBodies,
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
        let mut m_pivotInA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_pivotInB: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_aTc: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_bTc: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_motors: _serde::__private::Option<[Pointer; 3usize]> = _serde::__private::None;
        let mut m_switchBodies: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_pivotInA => {
                        if _serde::__private::Option::is_some(&m_pivotInA) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pivotInA",
                                ),
                            );
                        }
                        m_pivotInA = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_pivotInB => {
                        if _serde::__private::Option::is_some(&m_pivotInB) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pivotInB",
                                ),
                            );
                        }
                        m_pivotInB = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_aTc => {
                        if _serde::__private::Option::is_some(&m_aTc) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("aTc"),
                            );
                        }
                        m_aTc = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bTc => {
                        if _serde::__private::Option::is_some(&m_bTc) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("bTc"),
                            );
                        }
                        m_bTc = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
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
                    __Field::m_switchBodies => {
                        if _serde::__private::Option::is_some(&m_switchBodies) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "switchBodies",
                                ),
                            );
                        }
                        m_switchBodies = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
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
        let m_pivotInA = match m_pivotInA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivotInA"),
                );
            }
        };
        let m_pivotInB = match m_pivotInB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivotInB"),
                );
            }
        };
        let m_aTc = match m_aTc {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("aTc"),
                );
            }
        };
        let m_bTc = match m_bTc {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bTc"),
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
        let m_switchBodies = match m_switchBodies {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("switchBodies"),
                );
            }
        };
        _serde::__private::Ok(hkpPoweredChainDataConstraintInfo {
            __ptr,
            m_pivotInA,
            m_pivotInB,
            m_aTc,
            m_bTc,
            m_motors,
            m_switchBodies,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPoweredChainDataConstraintInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "pivotInA",
                "pivotInB",
                "aTc",
                "bTc",
                "motors",
                "switchBodies",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPoweredChainDataConstraintInfo",
                FIELDS,
                __hkpPoweredChainDataConstraintInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpPoweredChainDataConstraintInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
