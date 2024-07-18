use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpBallSocketConstraintAtom`
/// - version: `3`
/// - signature: `0xe70e4dfa`
/// - size: ` 16`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBallSocketConstraintAtom {
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
    /// - name: `solvingMethod`(ctype: `enum SolvingMethod`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_solvingMethod: SolvingMethod,
    /// # C++ Info
    /// - name: `bodiesToNotify`(ctype: `hkUint8`)
    /// - offset: `  3`(x86)/`  3`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_bodiesToNotify: u8,
    /// # C++ Info
    /// - name: `velocityStabilizationFactor`(ctype: `hkUint8`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_velocityStabilizationFactor: u8,
    /// # C++ Info
    /// - name: `maxImpulse`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxImpulse: f32,
    /// # C++ Info
    /// - name: `inertiaStabilizationFactor`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_inertiaStabilizationFactor: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpBallSocketConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBallSocketConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe70e4dfa)
        }
    }
    impl _serde::Serialize for hkpBallSocketConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe70e4dfa)));
            let mut serializer = __serializer
                .serialize_struct("hkpBallSocketConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("solvingMethod", &self.m_solvingMethod)?;
            serializer.serialize_field("bodiesToNotify", &self.m_bodiesToNotify)?;
            serializer
                .serialize_field(
                    "velocityStabilizationFactor",
                    &self.m_velocityStabilizationFactor,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("maxImpulse", &self.m_maxImpulse)?;
            serializer
                .serialize_field(
                    "inertiaStabilizationFactor",
                    &self.m_inertiaStabilizationFactor,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_solvingMethod,
    m_bodiesToNotify,
    m_velocityStabilizationFactor,
    m_maxImpulse,
    m_inertiaStabilizationFactor,
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
            "solvingMethod" => Ok(__Field::m_solvingMethod),
            "bodiesToNotify" => Ok(__Field::m_bodiesToNotify),
            "velocityStabilizationFactor" => Ok(__Field::m_velocityStabilizationFactor),
            "maxImpulse" => Ok(__Field::m_maxImpulse),
            "inertiaStabilizationFactor" => Ok(__Field::m_inertiaStabilizationFactor),
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
pub(super) struct __hkpBallSocketConstraintAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpBallSocketConstraintAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpBallSocketConstraintAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpBallSocketConstraintAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpBallSocketConstraintAtom>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpBallSocketConstraintAtomVisitor<'de> {
    type Value = hkpBallSocketConstraintAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpBallSocketConstraintAtom",
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
        let mut m_solvingMethod: _serde::__private::Option<SolvingMethod> = _serde::__private::None;
        let mut m_bodiesToNotify: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_velocityStabilizationFactor: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_maxImpulse: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_inertiaStabilizationFactor: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_solvingMethod) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "solvingMethod",
                            ),
                        );
                    }
                    m_solvingMethod = _serde::__private::Some(
                        match __A::next_value::<SolvingMethod>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_bodiesToNotify) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bodiesToNotify",
                            ),
                        );
                    }
                    m_bodiesToNotify = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(
                        &m_velocityStabilizationFactor,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "velocityStabilizationFactor",
                            ),
                        );
                    }
                    m_velocityStabilizationFactor = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_maxImpulse) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxImpulse",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_maxImpulse = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(
                        &m_inertiaStabilizationFactor,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "inertiaStabilizationFactor",
                            ),
                        );
                    }
                    m_inertiaStabilizationFactor = _serde::__private::Some(
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
        let m_solvingMethod = match m_solvingMethod {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("solvingMethod"),
                );
            }
        };
        let m_bodiesToNotify = match m_bodiesToNotify {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodiesToNotify"),
                );
            }
        };
        let m_velocityStabilizationFactor = match m_velocityStabilizationFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "velocityStabilizationFactor",
                    ),
                );
            }
        };
        let m_maxImpulse = match m_maxImpulse {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxImpulse"),
                );
            }
        };
        let m_inertiaStabilizationFactor = match m_inertiaStabilizationFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "inertiaStabilizationFactor",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpBallSocketConstraintAtom {
            __ptr,
            parent,
            m_solvingMethod,
            m_bodiesToNotify,
            m_velocityStabilizationFactor,
            m_maxImpulse,
            m_inertiaStabilizationFactor,
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
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkpConstraintAtomVisitor::visit_as_parent(&mut __map)?;
        let mut m_solvingMethod: _serde::__private::Option<SolvingMethod> = _serde::__private::None;
        let mut m_bodiesToNotify: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_velocityStabilizationFactor: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_maxImpulse: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_inertiaStabilizationFactor: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..5usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_solvingMethod => {
                        if _serde::__private::Option::is_some(&m_solvingMethod) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "solvingMethod",
                                ),
                            );
                        }
                        m_solvingMethod = _serde::__private::Some(
                            match __A::next_value::<SolvingMethod>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_bodiesToNotify => {
                        if _serde::__private::Option::is_some(&m_bodiesToNotify) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bodiesToNotify",
                                ),
                            );
                        }
                        m_bodiesToNotify = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_velocityStabilizationFactor => {
                        if _serde::__private::Option::is_some(
                            &m_velocityStabilizationFactor,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "velocityStabilizationFactor",
                                ),
                            );
                        }
                        m_velocityStabilizationFactor = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_maxImpulse => {
                        if _serde::__private::Option::is_some(&m_maxImpulse) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxImpulse",
                                ),
                            );
                        }
                        m_maxImpulse = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_inertiaStabilizationFactor => {
                        if _serde::__private::Option::is_some(
                            &m_inertiaStabilizationFactor,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "inertiaStabilizationFactor",
                                ),
                            );
                        }
                        m_inertiaStabilizationFactor = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        let m_solvingMethod = match m_solvingMethod {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("solvingMethod"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_bodiesToNotify = match m_bodiesToNotify {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodiesToNotify"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_velocityStabilizationFactor = match m_velocityStabilizationFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "velocityStabilizationFactor",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_maxImpulse = match m_maxImpulse {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxImpulse"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_inertiaStabilizationFactor = match m_inertiaStabilizationFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "inertiaStabilizationFactor",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpBallSocketConstraintAtom {
            __ptr,
            parent,
            m_solvingMethod,
            m_bodiesToNotify,
            m_velocityStabilizationFactor,
            m_maxImpulse,
            m_inertiaStabilizationFactor,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpBallSocketConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "solvingMethod",
                "bodiesToNotify",
                "velocityStabilizationFactor",
                "maxImpulse",
                "inertiaStabilizationFactor",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpBallSocketConstraintAtom",
                FIELDS,
                __hkpBallSocketConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpBallSocketConstraintAtom,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
