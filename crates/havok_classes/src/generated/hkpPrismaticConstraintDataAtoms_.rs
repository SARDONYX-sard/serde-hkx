use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpPrismaticConstraintDataAtoms`
/// - version: `0`
/// - signature: `0x7f516137`
/// - size: `192`(x86)/`208`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPrismaticConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `transforms`(ctype: `struct hkpSetLocalTransformsConstraintAtom`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `144`(x86)/`144`(x86_64)
    pub m_transforms: hkpSetLocalTransformsConstraintAtom,
    /// # C++ Info
    /// - name: `motor`(ctype: `struct hkpLinMotorConstraintAtom`)
    /// - offset: `144`(x86)/`144`(x86_64)
    /// - type_size: ` 16`(x86)/` 24`(x86_64)
    pub m_motor: hkpLinMotorConstraintAtom,
    /// # C++ Info
    /// - name: `friction`(ctype: `struct hkpLinFrictionConstraintAtom`)
    /// - offset: `160`(x86)/`168`(x86_64)
    /// - type_size: `  8`(x86)/`  8`(x86_64)
    pub m_friction: hkpLinFrictionConstraintAtom,
    /// # C++ Info
    /// - name: `ang`(ctype: `struct hkpAngConstraintAtom`)
    /// - offset: `168`(x86)/`176`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_ang: hkpAngConstraintAtom,
    /// # C++ Info
    /// - name: `lin0`(ctype: `struct hkpLinConstraintAtom`)
    /// - offset: `172`(x86)/`180`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_lin0: hkpLinConstraintAtom,
    /// # C++ Info
    /// - name: `lin1`(ctype: `struct hkpLinConstraintAtom`)
    /// - offset: `176`(x86)/`184`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_lin1: hkpLinConstraintAtom,
    /// # C++ Info
    /// - name: `linLimit`(ctype: `struct hkpLinLimitConstraintAtom`)
    /// - offset: `180`(x86)/`188`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    pub m_linLimit: hkpLinLimitConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPrismaticConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPrismaticConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7f516137)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_transforms.deps_indexes());
            v.extend(self.m_motor.deps_indexes());
            v.extend(self.m_friction.deps_indexes());
            v.extend(self.m_ang.deps_indexes());
            v.extend(self.m_lin0.deps_indexes());
            v.extend(self.m_lin1.deps_indexes());
            v.extend(self.m_linLimit.deps_indexes());
            v
        }
    }
    impl _serde::Serialize for hkpPrismaticConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7f516137)));
            let mut serializer = __serializer
                .serialize_struct("hkpPrismaticConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer.serialize_field("motor", &self.m_motor)?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("ang", &self.m_ang)?;
            serializer.serialize_field("lin0", &self.m_lin0)?;
            serializer.serialize_field("lin1", &self.m_lin1)?;
            serializer.serialize_field("linLimit", &self.m_linLimit)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPrismaticConstraintDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_transforms,
                m_motor,
                m_friction,
                m_ang,
                m_lin0,
                m_lin1,
                m_linLimit,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "transforms" => Ok(__Field::m_transforms),
                        "motor" => Ok(__Field::m_motor),
                        "friction" => Ok(__Field::m_friction),
                        "ang" => Ok(__Field::m_ang),
                        "lin0" => Ok(__Field::m_lin0),
                        "lin1" => Ok(__Field::m_lin1),
                        "linLimit" => Ok(__Field::m_linLimit),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkpPrismaticConstraintDataAtomsVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpPrismaticConstraintDataAtoms>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpPrismaticConstraintDataAtomsVisitor<'de> {
                type Value = hkpPrismaticConstraintDataAtoms;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpPrismaticConstraintDataAtoms",
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
                    let mut m_transforms: _serde::__private::Option<
                        hkpSetLocalTransformsConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_motor: _serde::__private::Option<
                        hkpLinMotorConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_friction: _serde::__private::Option<
                        hkpLinFrictionConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_ang: _serde::__private::Option<hkpAngConstraintAtom> = _serde::__private::None;
                    let mut m_lin0: _serde::__private::Option<hkpLinConstraintAtom> = _serde::__private::None;
                    let mut m_lin1: _serde::__private::Option<hkpLinConstraintAtom> = _serde::__private::None;
                    let mut m_linLimit: _serde::__private::Option<
                        hkpLinLimitConstraintAtom,
                    > = _serde::__private::None;
                    for i in 0..7usize {
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
                                if _serde::__private::Option::is_some(&m_motor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("motor"),
                                    );
                                }
                                m_motor = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinMotorConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_friction) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "friction",
                                        ),
                                    );
                                }
                                m_friction = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinFrictionConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_ang) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("ang"),
                                    );
                                }
                                m_ang = _serde::__private::Some(
                                    match __A::next_value::<hkpAngConstraintAtom>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_lin0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("lin0"),
                                    );
                                }
                                m_lin0 = _serde::__private::Some(
                                    match __A::next_value::<hkpLinConstraintAtom>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
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
                            6usize => {
                                if _serde::__private::Option::is_some(&m_linLimit) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linLimit",
                                        ),
                                    );
                                }
                                m_linLimit = _serde::__private::Some(
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
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 0usize, 8usize)?;
                    let m_transforms = match m_transforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transforms",
                                ),
                            );
                        }
                    };
                    let m_motor = match m_motor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("motor"),
                            );
                        }
                    };
                    let m_friction = match m_friction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("friction"),
                            );
                        }
                    };
                    let m_ang = match m_ang {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("ang"),
                            );
                        }
                    };
                    let m_lin0 = match m_lin0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("lin0"),
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
                    let m_linLimit = match m_linLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("linLimit"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpPrismaticConstraintDataAtoms {
                        __ptr,
                        m_transforms,
                        m_motor,
                        m_friction,
                        m_ang,
                        m_lin0,
                        m_lin1,
                        m_linLimit,
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
                    let mut m_transforms: _serde::__private::Option<
                        hkpSetLocalTransformsConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_motor: _serde::__private::Option<
                        hkpLinMotorConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_friction: _serde::__private::Option<
                        hkpLinFrictionConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_ang: _serde::__private::Option<hkpAngConstraintAtom> = _serde::__private::None;
                    let mut m_lin0: _serde::__private::Option<hkpLinConstraintAtom> = _serde::__private::None;
                    let mut m_lin1: _serde::__private::Option<hkpLinConstraintAtom> = _serde::__private::None;
                    let mut m_linLimit: _serde::__private::Option<
                        hkpLinLimitConstraintAtom,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_transforms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transforms) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                            __Field::m_motor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_motor) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("motor"),
                                    );
                                }
                                m_motor = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinMotorConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_friction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_friction) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "friction",
                                        ),
                                    );
                                }
                                m_friction = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpLinFrictionConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ang => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ang) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("ang"),
                                    );
                                }
                                m_ang = _serde::__private::Some(
                                    match __A::next_value::<hkpAngConstraintAtom>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lin0 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lin0) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("lin0"),
                                    );
                                }
                                m_lin0 = _serde::__private::Some(
                                    match __A::next_value::<hkpLinConstraintAtom>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lin1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lin1) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                            __Field::m_linLimit => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_linLimit) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linLimit",
                                        ),
                                    );
                                }
                                m_linLimit = _serde::__private::Some(
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
                            _ => {}
                        }
                    }
                    let m_transforms = match m_transforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transforms",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motor = match m_motor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("motor"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_friction = match m_friction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("friction"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ang = match m_ang {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("ang"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lin0 = match m_lin0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("lin0"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lin1 = match m_lin1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("lin1"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_linLimit = match m_linLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("linLimit"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpPrismaticConstraintDataAtoms {
                        __ptr,
                        m_transforms,
                        m_motor,
                        m_friction,
                        m_ang,
                        m_lin0,
                        m_lin1,
                        m_linLimit,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transforms",
                "motor",
                "friction",
                "ang",
                "lin0",
                "lin1",
                "linLimit",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPrismaticConstraintDataAtoms",
                FIELDS,
                __hkpPrismaticConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpPrismaticConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
