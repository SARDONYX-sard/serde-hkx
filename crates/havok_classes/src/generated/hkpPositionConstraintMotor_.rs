use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpPositionConstraintMotor`
/// - version: `0`
/// - signature: `0x748fb303`
/// - size: ` 36`(x86)/` 48`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPositionConstraintMotor {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpLimitedForceConstraintMotor,
    /// # C++ Info
    /// - name: `tau`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_tau: f32,
    /// # C++ Info
    /// - name: `damping`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_damping: f32,
    /// # C++ Info
    /// - name: `proportionalRecoveryVelocity`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_proportionalRecoveryVelocity: f32,
    /// # C++ Info
    /// - name: `constantRecoveryVelocity`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 44`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_constantRecoveryVelocity: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPositionConstraintMotor {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPositionConstraintMotor"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x748fb303)
        }
    }
    impl _serde::Serialize for hkpPositionConstraintMotor {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x748fb303)));
            let mut serializer = __serializer
                .serialize_struct("hkpPositionConstraintMotor", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("minForce", &self.parent.m_minForce)?;
            serializer.serialize_field("maxForce", &self.parent.m_maxForce)?;
            serializer.serialize_field("tau", &self.m_tau)?;
            serializer.serialize_field("damping", &self.m_damping)?;
            serializer
                .serialize_field(
                    "proportionalRecoveryVelocity",
                    &self.m_proportionalRecoveryVelocity,
                )?;
            serializer
                .serialize_field(
                    "constantRecoveryVelocity",
                    &self.m_constantRecoveryVelocity,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_tau,
    m_damping,
    m_proportionalRecoveryVelocity,
    m_constantRecoveryVelocity,
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
            "tau" => Ok(__Field::m_tau),
            "damping" => Ok(__Field::m_damping),
            "proportionalRecoveryVelocity" => Ok(__Field::m_proportionalRecoveryVelocity),
            "constantRecoveryVelocity" => Ok(__Field::m_constantRecoveryVelocity),
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
pub(super) struct __hkpPositionConstraintMotorVisitor<'de> {
    marker: core::marker::PhantomData<hkpPositionConstraintMotor>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPositionConstraintMotorVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpPositionConstraintMotor, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpPositionConstraintMotor>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpPositionConstraintMotorVisitor<'de> {
    type Value = hkpPositionConstraintMotor;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpPositionConstraintMotor")
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
        let mut m_tau: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_damping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_proportionalRecoveryVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_constantRecoveryVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_tau) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("tau"),
                        );
                    }
                    m_tau = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_damping) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("damping"),
                        );
                    }
                    m_damping = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(
                        &m_proportionalRecoveryVelocity,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "proportionalRecoveryVelocity",
                            ),
                        );
                    }
                    m_proportionalRecoveryVelocity = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_constantRecoveryVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "constantRecoveryVelocity",
                            ),
                        );
                    }
                    m_constantRecoveryVelocity = _serde::__private::Some(
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
        let m_tau = match m_tau {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tau"),
                );
            }
        };
        let m_damping = match m_damping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("damping"),
                );
            }
        };
        let m_proportionalRecoveryVelocity = match m_proportionalRecoveryVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "proportionalRecoveryVelocity",
                    ),
                );
            }
        };
        let m_constantRecoveryVelocity = match m_constantRecoveryVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "constantRecoveryVelocity",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpPositionConstraintMotor {
            __ptr,
            parent,
            m_tau,
            m_damping,
            m_proportionalRecoveryVelocity,
            m_constantRecoveryVelocity,
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
        let parent = __hkpLimitedForceConstraintMotorVisitor::visit_as_parent(
            &mut __map,
        )?;
        let mut m_tau: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_damping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_proportionalRecoveryVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_constantRecoveryVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..4usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_tau => {
                        if _serde::__private::Option::is_some(&m_tau) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("tau"),
                            );
                        }
                        m_tau = _serde::__private::Some(
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
                    __Field::m_damping => {
                        if _serde::__private::Option::is_some(&m_damping) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "damping",
                                ),
                            );
                        }
                        m_damping = _serde::__private::Some(
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
                    __Field::m_proportionalRecoveryVelocity => {
                        if _serde::__private::Option::is_some(
                            &m_proportionalRecoveryVelocity,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "proportionalRecoveryVelocity",
                                ),
                            );
                        }
                        m_proportionalRecoveryVelocity = _serde::__private::Some(
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
                    __Field::m_constantRecoveryVelocity => {
                        if _serde::__private::Option::is_some(
                            &m_constantRecoveryVelocity,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "constantRecoveryVelocity",
                                ),
                            );
                        }
                        m_constantRecoveryVelocity = _serde::__private::Some(
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
        let m_tau = match m_tau {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tau"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_damping = match m_damping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("damping"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_proportionalRecoveryVelocity = match m_proportionalRecoveryVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "proportionalRecoveryVelocity",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_constantRecoveryVelocity = match m_constantRecoveryVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "constantRecoveryVelocity",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpPositionConstraintMotor {
            __ptr,
            parent,
            m_tau,
            m_damping,
            m_proportionalRecoveryVelocity,
            m_constantRecoveryVelocity,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPositionConstraintMotor {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "tau",
                "damping",
                "proportionalRecoveryVelocity",
                "constantRecoveryVelocity",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPositionConstraintMotor",
                FIELDS,
                __hkpPositionConstraintMotorVisitor {
                    marker: _serde::__private::PhantomData::<hkpPositionConstraintMotor>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
