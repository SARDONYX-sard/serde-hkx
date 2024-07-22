use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbFootIkControlsModifierLeg`
/// - version: `0`
/// - signature: `0x9e17091a`
/// - size: ` 32`(x86)/` 48`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkControlsModifierLeg {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `groundPosition`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_groundPosition: Vector4,
    /// # C++ Info
    /// - name: `ungroundedEvent`(ctype: `struct hkbEventProperty`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    pub m_ungroundedEvent: hkbEventProperty,
    /// # C++ Info
    /// - name: `verticalError`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_verticalError: f32,
    /// # C++ Info
    /// - name: `hitSomething`(ctype: `hkBool`)
    /// - offset: ` 28`(x86)/` 36`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_hitSomething: bool,
    /// # C++ Info
    /// - name: `isPlantedMS`(ctype: `hkBool`)
    /// - offset: ` 29`(x86)/` 37`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isPlantedMS: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbFootIkControlsModifierLeg {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkControlsModifierLeg"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9e17091a)
        }
    }
    impl _serde::Serialize for hkbFootIkControlsModifierLeg {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9e17091a)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkControlsModifierLeg", class_meta)?;
            serializer.serialize_field("groundPosition", &self.m_groundPosition)?;
            serializer.serialize_field("ungroundedEvent", &self.m_ungroundedEvent)?;
            serializer.serialize_field("verticalError", &self.m_verticalError)?;
            serializer.serialize_field("hitSomething", &self.m_hitSomething)?;
            serializer.serialize_field("isPlantedMS", &self.m_isPlantedMS)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbFootIkControlsModifierLeg {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_isPlantedMS,
                m_hitSomething,
                m_verticalError,
                m_ungroundedEvent,
                m_groundPosition,
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
                        "isPlantedMS" => Ok(__Field::m_isPlantedMS),
                        "hitSomething" => Ok(__Field::m_hitSomething),
                        "verticalError" => Ok(__Field::m_verticalError),
                        "ungroundedEvent" => Ok(__Field::m_ungroundedEvent),
                        "groundPosition" => Ok(__Field::m_groundPosition),
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
            struct __hkbFootIkControlsModifierLegVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbFootIkControlsModifierLeg>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbFootIkControlsModifierLegVisitor<'de> {
                type Value = hkbFootIkControlsModifierLeg;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbFootIkControlsModifierLeg",
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
                    let mut m_groundPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ungroundedEvent: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_verticalError: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_hitSomething: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isPlantedMS: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_groundPosition) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "groundPosition",
                                        ),
                                    );
                                }
                                m_groundPosition = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_ungroundedEvent) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ungroundedEvent",
                                        ),
                                    );
                                }
                                m_ungroundedEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_verticalError) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalError",
                                        ),
                                    );
                                }
                                m_verticalError = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_hitSomething) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hitSomething",
                                        ),
                                    );
                                }
                                m_hitSomething = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_isPlantedMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isPlantedMS",
                                        ),
                                    );
                                }
                                m_isPlantedMS = _serde::__private::Some(
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
                    __A::pad(&mut __map, 2usize, 10usize)?;
                    let m_groundPosition = match m_groundPosition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "groundPosition",
                                ),
                            );
                        }
                    };
                    let m_ungroundedEvent = match m_ungroundedEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ungroundedEvent",
                                ),
                            );
                        }
                    };
                    let m_verticalError = match m_verticalError {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalError",
                                ),
                            );
                        }
                    };
                    let m_hitSomething = match m_hitSomething {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "hitSomething",
                                ),
                            );
                        }
                    };
                    let m_isPlantedMS = match m_isPlantedMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isPlantedMS",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbFootIkControlsModifierLeg {
                        __ptr,
                        m_groundPosition,
                        m_ungroundedEvent,
                        m_verticalError,
                        m_hitSomething,
                        m_isPlantedMS,
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
                    let mut m_isPlantedMS: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_hitSomething: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_verticalError: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_ungroundedEvent: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_groundPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_isPlantedMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isPlantedMS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isPlantedMS",
                                        ),
                                    );
                                }
                                m_isPlantedMS = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_hitSomething => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_hitSomething) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hitSomething",
                                        ),
                                    );
                                }
                                m_hitSomething = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_verticalError => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_verticalError) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalError",
                                        ),
                                    );
                                }
                                m_verticalError = _serde::__private::Some(
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
                            __Field::m_ungroundedEvent => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ungroundedEvent) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ungroundedEvent",
                                        ),
                                    );
                                }
                                m_ungroundedEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_groundPosition => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_groundPosition) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "groundPosition",
                                        ),
                                    );
                                }
                                m_groundPosition = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
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
                    let m_isPlantedMS = match m_isPlantedMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isPlantedMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_hitSomething = match m_hitSomething {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "hitSomething",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_verticalError = match m_verticalError {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalError",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ungroundedEvent = match m_ungroundedEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ungroundedEvent",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_groundPosition = match m_groundPosition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "groundPosition",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbFootIkControlsModifierLeg {
                        __ptr,
                        m_groundPosition,
                        m_ungroundedEvent,
                        m_verticalError,
                        m_hitSomething,
                        m_isPlantedMS,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "groundPosition",
                "ungroundedEvent",
                "verticalError",
                "hitSomething",
                "isPlantedMS",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbFootIkControlsModifierLeg",
                FIELDS,
                __hkbFootIkControlsModifierLegVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbFootIkControlsModifierLeg,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
