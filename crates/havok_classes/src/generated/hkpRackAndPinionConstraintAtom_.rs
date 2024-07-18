use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpRackAndPinionConstraintAtom`
/// - version: `0`
/// - signature: `0x30cae006`
/// - size: ` 12`(x86)/` 12`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRackAndPinionConstraintAtom {
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
    /// - name: `pinionRadiusOrScrewPitch`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_pinionRadiusOrScrewPitch: f32,
    /// # C++ Info
    /// - name: `isScrew`(ctype: `hkBool`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isScrew: bool,
    /// # C++ Info
    /// - name: `memOffsetToInitialAngleOffset`(ctype: `hkInt8`)
    /// - offset: `  9`(x86)/`  9`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_memOffsetToInitialAngleOffset: i8,
    /// # C++ Info
    /// - name: `memOffsetToPrevAngle`(ctype: `hkInt8`)
    /// - offset: ` 10`(x86)/` 10`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_memOffsetToPrevAngle: i8,
    /// # C++ Info
    /// - name: `memOffsetToRevolutionCounter`(ctype: `hkInt8`)
    /// - offset: ` 11`(x86)/` 11`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_memOffsetToRevolutionCounter: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpRackAndPinionConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRackAndPinionConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x30cae006)
        }
    }
    impl _serde::Serialize for hkpRackAndPinionConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x30cae006)));
            let mut serializer = __serializer
                .serialize_struct("hkpRackAndPinionConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field(
                    "pinionRadiusOrScrewPitch",
                    &self.m_pinionRadiusOrScrewPitch,
                )?;
            serializer.serialize_field("isScrew", &self.m_isScrew)?;
            serializer
                .serialize_field(
                    "memOffsetToInitialAngleOffset",
                    &self.m_memOffsetToInitialAngleOffset,
                )?;
            serializer
                .serialize_field("memOffsetToPrevAngle", &self.m_memOffsetToPrevAngle)?;
            serializer
                .serialize_field(
                    "memOffsetToRevolutionCounter",
                    &self.m_memOffsetToRevolutionCounter,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_pinionRadiusOrScrewPitch,
    m_isScrew,
    m_memOffsetToInitialAngleOffset,
    m_memOffsetToPrevAngle,
    m_memOffsetToRevolutionCounter,
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
            "pinionRadiusOrScrewPitch" => Ok(__Field::m_pinionRadiusOrScrewPitch),
            "isScrew" => Ok(__Field::m_isScrew),
            "memOffsetToInitialAngleOffset" => {
                Ok(__Field::m_memOffsetToInitialAngleOffset)
            }
            "memOffsetToPrevAngle" => Ok(__Field::m_memOffsetToPrevAngle),
            "memOffsetToRevolutionCounter" => Ok(__Field::m_memOffsetToRevolutionCounter),
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
pub(super) struct __hkpRackAndPinionConstraintAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpRackAndPinionConstraintAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpRackAndPinionConstraintAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpRackAndPinionConstraintAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpRackAndPinionConstraintAtom>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpRackAndPinionConstraintAtomVisitor<'de> {
    type Value = hkpRackAndPinionConstraintAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpRackAndPinionConstraintAtom",
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
        let mut m_pinionRadiusOrScrewPitch: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_isScrew: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_memOffsetToInitialAngleOffset: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_memOffsetToPrevAngle: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_memOffsetToRevolutionCounter: _serde::__private::Option<i8> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_pinionRadiusOrScrewPitch) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pinionRadiusOrScrewPitch",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 2usize)?;
                    m_pinionRadiusOrScrewPitch = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_isScrew) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("isScrew"),
                        );
                    }
                    m_isScrew = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(
                        &m_memOffsetToInitialAngleOffset,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "memOffsetToInitialAngleOffset",
                            ),
                        );
                    }
                    m_memOffsetToInitialAngleOffset = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_memOffsetToPrevAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "memOffsetToPrevAngle",
                            ),
                        );
                    }
                    m_memOffsetToPrevAngle = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(
                        &m_memOffsetToRevolutionCounter,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "memOffsetToRevolutionCounter",
                            ),
                        );
                    }
                    m_memOffsetToRevolutionCounter = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
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
        let m_pinionRadiusOrScrewPitch = match m_pinionRadiusOrScrewPitch {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "pinionRadiusOrScrewPitch",
                    ),
                );
            }
        };
        let m_isScrew = match m_isScrew {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isScrew"),
                );
            }
        };
        let m_memOffsetToInitialAngleOffset = match m_memOffsetToInitialAngleOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "memOffsetToInitialAngleOffset",
                    ),
                );
            }
        };
        let m_memOffsetToPrevAngle = match m_memOffsetToPrevAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "memOffsetToPrevAngle",
                    ),
                );
            }
        };
        let m_memOffsetToRevolutionCounter = match m_memOffsetToRevolutionCounter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "memOffsetToRevolutionCounter",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpRackAndPinionConstraintAtom {
            __ptr,
            parent,
            m_pinionRadiusOrScrewPitch,
            m_isScrew,
            m_memOffsetToInitialAngleOffset,
            m_memOffsetToPrevAngle,
            m_memOffsetToRevolutionCounter,
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
        let mut m_pinionRadiusOrScrewPitch: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_isScrew: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_memOffsetToInitialAngleOffset: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_memOffsetToPrevAngle: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_memOffsetToRevolutionCounter: _serde::__private::Option<i8> = _serde::__private::None;
        for _ in 0..5usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_pinionRadiusOrScrewPitch => {
                        if _serde::__private::Option::is_some(
                            &m_pinionRadiusOrScrewPitch,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pinionRadiusOrScrewPitch",
                                ),
                            );
                        }
                        m_pinionRadiusOrScrewPitch = _serde::__private::Some(
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
                    __Field::m_isScrew => {
                        if _serde::__private::Option::is_some(&m_isScrew) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isScrew",
                                ),
                            );
                        }
                        m_isScrew = _serde::__private::Some(
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
                    __Field::m_memOffsetToInitialAngleOffset => {
                        if _serde::__private::Option::is_some(
                            &m_memOffsetToInitialAngleOffset,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "memOffsetToInitialAngleOffset",
                                ),
                            );
                        }
                        m_memOffsetToInitialAngleOffset = _serde::__private::Some(
                            match __A::next_value::<i8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_memOffsetToPrevAngle => {
                        if _serde::__private::Option::is_some(&m_memOffsetToPrevAngle) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "memOffsetToPrevAngle",
                                ),
                            );
                        }
                        m_memOffsetToPrevAngle = _serde::__private::Some(
                            match __A::next_value::<i8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_memOffsetToRevolutionCounter => {
                        if _serde::__private::Option::is_some(
                            &m_memOffsetToRevolutionCounter,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "memOffsetToRevolutionCounter",
                                ),
                            );
                        }
                        m_memOffsetToRevolutionCounter = _serde::__private::Some(
                            match __A::next_value::<i8>(&mut __map) {
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
        let m_pinionRadiusOrScrewPitch = match m_pinionRadiusOrScrewPitch {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "pinionRadiusOrScrewPitch",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_isScrew = match m_isScrew {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isScrew"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_memOffsetToInitialAngleOffset = match m_memOffsetToInitialAngleOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "memOffsetToInitialAngleOffset",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_memOffsetToPrevAngle = match m_memOffsetToPrevAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "memOffsetToPrevAngle",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_memOffsetToRevolutionCounter = match m_memOffsetToRevolutionCounter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "memOffsetToRevolutionCounter",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpRackAndPinionConstraintAtom {
            __ptr,
            parent,
            m_pinionRadiusOrScrewPitch,
            m_isScrew,
            m_memOffsetToInitialAngleOffset,
            m_memOffsetToPrevAngle,
            m_memOffsetToRevolutionCounter,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpRackAndPinionConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "pinionRadiusOrScrewPitch",
                "isScrew",
                "memOffsetToInitialAngleOffset",
                "memOffsetToPrevAngle",
                "memOffsetToRevolutionCounter",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpRackAndPinionConstraintAtom",
                FIELDS,
                __hkpRackAndPinionConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpRackAndPinionConstraintAtom,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
