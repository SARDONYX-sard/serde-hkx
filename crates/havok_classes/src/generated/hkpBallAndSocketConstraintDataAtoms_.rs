use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpBallAndSocketConstraintDataAtoms`
/// - version: `1`
/// - signature: `0xc73dcaf9`
/// - size: ` 80`(x86)/` 80`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBallAndSocketConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `pivots`(ctype: `struct hkpSetLocalTranslationsConstraintAtom`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_pivots: hkpSetLocalTranslationsConstraintAtom,
    /// # C++ Info
    /// - name: `setupStabilization`(ctype: `struct hkpSetupStabilizationAtom`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_setupStabilization: hkpSetupStabilizationAtom,
    /// # C++ Info
    /// - name: `ballSocket`(ctype: `struct hkpBallSocketConstraintAtom`)
    /// - offset: ` 64`(x86)/` 64`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_ballSocket: hkpBallSocketConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpBallAndSocketConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBallAndSocketConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc73dcaf9)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_pivots.deps_indexes());
            v.extend(self.m_setupStabilization.deps_indexes());
            v.extend(self.m_ballSocket.deps_indexes());
            v
        }
    }
    impl _serde::Serialize for hkpBallAndSocketConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc73dcaf9)));
            let mut serializer = __serializer
                .serialize_struct("hkpBallAndSocketConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("pivots", &self.m_pivots)?;
            serializer
                .serialize_field("setupStabilization", &self.m_setupStabilization)?;
            serializer.serialize_field("ballSocket", &self.m_ballSocket)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpBallAndSocketConstraintDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_pivots,
                m_setupStabilization,
                m_ballSocket,
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
                        "pivots" => Ok(__Field::m_pivots),
                        "setupStabilization" => Ok(__Field::m_setupStabilization),
                        "ballSocket" => Ok(__Field::m_ballSocket),
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
            struct __hkpBallAndSocketConstraintDataAtomsVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpBallAndSocketConstraintDataAtoms,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpBallAndSocketConstraintDataAtomsVisitor<'de> {
                type Value = hkpBallAndSocketConstraintDataAtoms;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpBallAndSocketConstraintDataAtoms",
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
                    let mut m_pivots: _serde::__private::Option<
                        hkpSetLocalTranslationsConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_setupStabilization: _serde::__private::Option<
                        hkpSetupStabilizationAtom,
                    > = _serde::__private::None;
                    let mut m_ballSocket: _serde::__private::Option<
                        hkpBallSocketConstraintAtom,
                    > = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_pivots) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pivots"),
                                    );
                                }
                                m_pivots = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpSetLocalTranslationsConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_setupStabilization,
                                ) {
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
                            2usize => {
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
                    let m_pivots = match m_pivots {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pivots"),
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
                    let m_ballSocket = match m_ballSocket {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ballSocket",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpBallAndSocketConstraintDataAtoms {
                        __ptr,
                        m_pivots,
                        m_setupStabilization,
                        m_ballSocket,
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
                    let mut m_pivots: _serde::__private::Option<
                        hkpSetLocalTranslationsConstraintAtom,
                    > = _serde::__private::None;
                    let mut m_setupStabilization: _serde::__private::Option<
                        hkpSetupStabilizationAtom,
                    > = _serde::__private::None;
                    let mut m_ballSocket: _serde::__private::Option<
                        hkpBallSocketConstraintAtom,
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
                            __Field::m_pivots => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_pivots) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pivots"),
                                    );
                                }
                                m_pivots = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpSetLocalTranslationsConstraintAtom,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_setupStabilization => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_setupStabilization,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                            __Field::m_ballSocket => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ballSocket) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                    let m_pivots = match m_pivots {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pivots"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_setupStabilization = match m_setupStabilization {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "setupStabilization",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ballSocket = match m_ballSocket {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ballSocket",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpBallAndSocketConstraintDataAtoms {
                        __ptr,
                        m_pivots,
                        m_setupStabilization,
                        m_ballSocket,
                    })
                }
            }
            const FIELDS: &[&str] = &["pivots", "setupStabilization", "ballSocket"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpBallAndSocketConstraintDataAtoms",
                FIELDS,
                __hkpBallAndSocketConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpBallAndSocketConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
