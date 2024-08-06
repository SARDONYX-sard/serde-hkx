use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpConeLimitConstraintAtom`
/// - version: `0`
/// - signature: `0xf19443c8`
/// - size: ` 20`(x86)/` 20`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConeLimitConstraintAtom {
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
    /// - name: `isEnabled`(ctype: `hkUint8`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isEnabled: u8,
    /// # C++ Info
    /// - name: `twistAxisInA`(ctype: `hkUint8`)
    /// - offset: `  3`(x86)/`  3`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_twistAxisInA: u8,
    /// # C++ Info
    /// - name: `refAxisInB`(ctype: `hkUint8`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_refAxisInB: u8,
    /// # C++ Info
    /// - name: `angleMeasurementMode`(ctype: `enum MeasurementMode`)
    /// - offset: `  5`(x86)/`  5`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_angleMeasurementMode: MeasurementMode,
    /// # C++ Info
    /// - name: `memOffsetToAngleOffset`(ctype: `hkUint8`)
    /// - offset: `  6`(x86)/`  6`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_memOffsetToAngleOffset: u8,
    /// # C++ Info
    /// - name: `minAngle`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minAngle: f32,
    /// # C++ Info
    /// - name: `maxAngle`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxAngle: f32,
    /// # C++ Info
    /// - name: `angularLimitsTauFactor`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_angularLimitsTauFactor: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConeLimitConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConeLimitConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf19443c8)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkpConeLimitConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf19443c8)));
            let mut serializer = __serializer
                .serialize_struct("hkpConeLimitConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.serialize_field("twistAxisInA", &self.m_twistAxisInA)?;
            serializer.serialize_field("refAxisInB", &self.m_refAxisInB)?;
            serializer
                .serialize_field("angleMeasurementMode", &self.m_angleMeasurementMode)?;
            serializer
                .serialize_field(
                    "memOffsetToAngleOffset",
                    &self.m_memOffsetToAngleOffset,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("minAngle", &self.m_minAngle)?;
            serializer.serialize_field("maxAngle", &self.m_maxAngle)?;
            serializer
                .serialize_field(
                    "angularLimitsTauFactor",
                    &self.m_angularLimitsTauFactor,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConeLimitConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_type,
                m_isEnabled,
                m_twistAxisInA,
                m_refAxisInB,
                m_angleMeasurementMode,
                m_memOffsetToAngleOffset,
                m_minAngle,
                m_maxAngle,
                m_angularLimitsTauFactor,
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
                        "type" => Ok(__Field::m_type),
                        "isEnabled" => Ok(__Field::m_isEnabled),
                        "twistAxisInA" => Ok(__Field::m_twistAxisInA),
                        "refAxisInB" => Ok(__Field::m_refAxisInB),
                        "angleMeasurementMode" => Ok(__Field::m_angleMeasurementMode),
                        "memOffsetToAngleOffset" => Ok(__Field::m_memOffsetToAngleOffset),
                        "minAngle" => Ok(__Field::m_minAngle),
                        "maxAngle" => Ok(__Field::m_maxAngle),
                        "angularLimitsTauFactor" => Ok(__Field::m_angularLimitsTauFactor),
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
            struct __hkpConeLimitConstraintAtomVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpConeLimitConstraintAtom>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpConeLimitConstraintAtomVisitor<'de> {
                type Value = hkpConeLimitConstraintAtom;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpConeLimitConstraintAtom",
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
                    let mut m_twistAxisInA: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_refAxisInB: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_angleMeasurementMode: _serde::__private::Option<
                        MeasurementMode,
                    > = _serde::__private::None;
                    let mut m_memOffsetToAngleOffset: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_minAngle: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAngle: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_angularLimitsTauFactor: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..8usize {
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
                                if _serde::__private::Option::is_some(&m_twistAxisInA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "twistAxisInA",
                                        ),
                                    );
                                }
                                m_twistAxisInA = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_refAxisInB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "refAxisInB",
                                        ),
                                    );
                                }
                                m_refAxisInB = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_angleMeasurementMode,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angleMeasurementMode",
                                        ),
                                    );
                                }
                                m_angleMeasurementMode = _serde::__private::Some(
                                    match __A::next_value::<MeasurementMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_memOffsetToAngleOffset,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "memOffsetToAngleOffset",
                                        ),
                                    );
                                }
                                m_memOffsetToAngleOffset = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_minAngle) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minAngle",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 1usize, 1usize)?;
                                m_minAngle = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_maxAngle) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAngle",
                                        ),
                                    );
                                }
                                m_maxAngle = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_angularLimitsTauFactor,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularLimitsTauFactor",
                                        ),
                                    );
                                }
                                m_angularLimitsTauFactor = _serde::__private::Some(
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isEnabled",
                                ),
                            );
                        }
                    };
                    let m_twistAxisInA = match m_twistAxisInA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "twistAxisInA",
                                ),
                            );
                        }
                    };
                    let m_refAxisInB = match m_refAxisInB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "refAxisInB",
                                ),
                            );
                        }
                    };
                    let m_angleMeasurementMode = match m_angleMeasurementMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angleMeasurementMode",
                                ),
                            );
                        }
                    };
                    let m_memOffsetToAngleOffset = match m_memOffsetToAngleOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "memOffsetToAngleOffset",
                                ),
                            );
                        }
                    };
                    let m_minAngle = match m_minAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("minAngle"),
                            );
                        }
                    };
                    let m_maxAngle = match m_maxAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("maxAngle"),
                            );
                        }
                    };
                    let m_angularLimitsTauFactor = match m_angularLimitsTauFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularLimitsTauFactor",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpConeLimitConstraintAtom {
                        __ptr,
                        parent,
                        m_isEnabled,
                        m_twistAxisInA,
                        m_refAxisInB,
                        m_angleMeasurementMode,
                        m_memOffsetToAngleOffset,
                        m_minAngle,
                        m_maxAngle,
                        m_angularLimitsTauFactor,
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
                    let mut m_type: _serde::__private::Option<AtomType> = _serde::__private::None;
                    let mut m_isEnabled: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_twistAxisInA: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_refAxisInB: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_angleMeasurementMode: _serde::__private::Option<
                        MeasurementMode,
                    > = _serde::__private::None;
                    let mut m_memOffsetToAngleOffset: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_minAngle: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAngle: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_angularLimitsTauFactor: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<AtomType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isEnabled => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isEnabled) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_twistAxisInA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_twistAxisInA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "twistAxisInA",
                                        ),
                                    );
                                }
                                m_twistAxisInA = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_refAxisInB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_refAxisInB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "refAxisInB",
                                        ),
                                    );
                                }
                                m_refAxisInB = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_angleMeasurementMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_angleMeasurementMode,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angleMeasurementMode",
                                        ),
                                    );
                                }
                                m_angleMeasurementMode = _serde::__private::Some(
                                    match __A::next_value::<MeasurementMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_memOffsetToAngleOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_memOffsetToAngleOffset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "memOffsetToAngleOffset",
                                        ),
                                    );
                                }
                                m_memOffsetToAngleOffset = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minAngle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_minAngle) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minAngle",
                                        ),
                                    );
                                }
                                m_minAngle = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxAngle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxAngle) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAngle",
                                        ),
                                    );
                                }
                                m_maxAngle = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_angularLimitsTauFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_angularLimitsTauFactor,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularLimitsTauFactor",
                                        ),
                                    );
                                }
                                m_angularLimitsTauFactor = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isEnabled = match m_isEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isEnabled",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_twistAxisInA = match m_twistAxisInA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "twistAxisInA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_refAxisInB = match m_refAxisInB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "refAxisInB",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_angleMeasurementMode = match m_angleMeasurementMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angleMeasurementMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_memOffsetToAngleOffset = match m_memOffsetToAngleOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "memOffsetToAngleOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minAngle = match m_minAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("minAngle"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxAngle = match m_maxAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("maxAngle"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_angularLimitsTauFactor = match m_angularLimitsTauFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularLimitsTauFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkpConstraintAtom { __ptr, m_type };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpConeLimitConstraintAtom {
                        __ptr,
                        parent,
                        m_isEnabled,
                        m_twistAxisInA,
                        m_refAxisInB,
                        m_angleMeasurementMode,
                        m_memOffsetToAngleOffset,
                        m_minAngle,
                        m_maxAngle,
                        m_angularLimitsTauFactor,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "isEnabled",
                "twistAxisInA",
                "refAxisInB",
                "angleMeasurementMode",
                "memOffsetToAngleOffset",
                "minAngle",
                "maxAngle",
                "angularLimitsTauFactor",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConeLimitConstraintAtom",
                FIELDS,
                __hkpConeLimitConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<hkpConeLimitConstraintAtom>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_UINT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum MeasurementMode {
    #[default]
    ZERO_WHEN_VECTORS_ALIGNED = 0isize,
    ZERO_WHEN_VECTORS_PERPENDICULAR = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MeasurementMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ZERO_WHEN_VECTORS_ALIGNED => {
                    __serializer.serialize_field("ZERO_WHEN_VECTORS_ALIGNED", &0u64)
                }
                Self::ZERO_WHEN_VECTORS_PERPENDICULAR => {
                    __serializer
                        .serialize_field("ZERO_WHEN_VECTORS_PERPENDICULAR", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum MeasurementMode to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for MeasurementMode {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint8<__E>(
                    self,
                    __value: u8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u8 => _serde::__private::Ok(__Field::__field0),
                        1u8 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("ZERO_WHEN_VECTORS_ALIGNED") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case("ZERO_WHEN_VECTORS_PERPENDICULAR") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MeasurementMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MeasurementMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum MeasurementMode",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MeasurementMode::ZERO_WHEN_VECTORS_ALIGNED,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MeasurementMode::ZERO_WHEN_VECTORS_PERPENDICULAR,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "ZERO_WHEN_VECTORS_ALIGNED",
                "ZERO_WHEN_VECTORS_PERPENDICULAR",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MeasurementMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MeasurementMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
