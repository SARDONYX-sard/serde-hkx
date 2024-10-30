use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpCogWheelConstraintAtom`
/// - version: `0`
/// - signature: `0xf2b1f399`
/// - size: ` 16`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCogWheelConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// - name: `cogWheelRadiusA`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "cogWheelRadiusA"))]
    pub m_cogWheelRadiusA: f32,
    /// # C++ Info
    /// - name: `cogWheelRadiusB`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "cogWheelRadiusB"))]
    pub m_cogWheelRadiusB: f32,
    /// # C++ Info
    /// - name: `isScrew`(ctype: `hkBool`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "isScrew"))]
    pub m_isScrew: bool,
    /// # C++ Info
    /// - name: `memOffsetToInitialAngleOffset`(ctype: `hkInt8`)
    /// - offset: ` 13`(x86)/` 13`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "memOffsetToInitialAngleOffset"))]
    pub m_memOffsetToInitialAngleOffset: i8,
    /// # C++ Info
    /// - name: `memOffsetToPrevAngle`(ctype: `hkInt8`)
    /// - offset: ` 14`(x86)/` 14`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "memOffsetToPrevAngle"))]
    pub m_memOffsetToPrevAngle: i8,
    /// # C++ Info
    /// - name: `memOffsetToRevolutionCounter`(ctype: `hkInt8`)
    /// - offset: ` 15`(x86)/` 15`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "memOffsetToRevolutionCounter"))]
    pub m_memOffsetToRevolutionCounter: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCogWheelConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCogWheelConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf2b1f399)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkpCogWheelConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf2b1f399)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpCogWheelConstraintAtom",
                    class_meta,
                    (16u64, 16u64),
                )?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("cogWheelRadiusA", &self.m_cogWheelRadiusA)?;
            serializer.serialize_field("cogWheelRadiusB", &self.m_cogWheelRadiusB)?;
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCogWheelConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_type,
                m_cogWheelRadiusA,
                m_cogWheelRadiusB,
                m_isScrew,
                m_memOffsetToInitialAngleOffset,
                m_memOffsetToPrevAngle,
                m_memOffsetToRevolutionCounter,
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
                        "cogWheelRadiusA" => Ok(__Field::m_cogWheelRadiusA),
                        "cogWheelRadiusB" => Ok(__Field::m_cogWheelRadiusB),
                        "isScrew" => Ok(__Field::m_isScrew),
                        "memOffsetToInitialAngleOffset" => {
                            Ok(__Field::m_memOffsetToInitialAngleOffset)
                        }
                        "memOffsetToPrevAngle" => Ok(__Field::m_memOffsetToPrevAngle),
                        "memOffsetToRevolutionCounter" => {
                            Ok(__Field::m_memOffsetToRevolutionCounter)
                        }
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
            struct __hkpCogWheelConstraintAtomVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpCogWheelConstraintAtom>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpCogWheelConstraintAtomVisitor<'de> {
                type Value = hkpCogWheelConstraintAtom;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpCogWheelConstraintAtom",
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
                    let mut m_cogWheelRadiusA: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_cogWheelRadiusB: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_isScrew: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_memOffsetToInitialAngleOffset: _serde::__private::Option<
                        i8,
                    > = _serde::__private::None;
                    let mut m_memOffsetToPrevAngle: _serde::__private::Option<i8> = _serde::__private::None;
                    let mut m_memOffsetToRevolutionCounter: _serde::__private::Option<
                        i8,
                    > = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_cogWheelRadiusA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cogWheelRadiusA",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_cogWheelRadiusA = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_cogWheelRadiusB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cogWheelRadiusB",
                                        ),
                                    );
                                }
                                m_cogWheelRadiusB = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
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
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_memOffsetToPrevAngle,
                                ) {
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
                            5usize => {
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
                    let m_cogWheelRadiusA = match m_cogWheelRadiusA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cogWheelRadiusA",
                                ),
                            );
                        }
                    };
                    let m_cogWheelRadiusB = match m_cogWheelRadiusB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cogWheelRadiusB",
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
                    _serde::__private::Ok(hkpCogWheelConstraintAtom {
                        __ptr,
                        parent,
                        m_cogWheelRadiusA,
                        m_cogWheelRadiusB,
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
                    let mut m_type: _serde::__private::Option<AtomType> = _serde::__private::None;
                    let mut m_cogWheelRadiusA: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_cogWheelRadiusB: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_isScrew: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_memOffsetToInitialAngleOffset: _serde::__private::Option<
                        i8,
                    > = _serde::__private::None;
                    let mut m_memOffsetToPrevAngle: _serde::__private::Option<i8> = _serde::__private::None;
                    let mut m_memOffsetToRevolutionCounter: _serde::__private::Option<
                        i8,
                    > = _serde::__private::None;
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
                            __Field::m_cogWheelRadiusA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_cogWheelRadiusA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cogWheelRadiusA",
                                        ),
                                    );
                                }
                                m_cogWheelRadiusA = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_cogWheelRadiusB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_cogWheelRadiusB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cogWheelRadiusB",
                                        ),
                                    );
                                }
                                m_cogWheelRadiusB = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isScrew => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isScrew) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_memOffsetToInitialAngleOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_memOffsetToInitialAngleOffset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_memOffsetToPrevAngle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_memOffsetToPrevAngle,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_memOffsetToRevolutionCounter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_memOffsetToRevolutionCounter,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                    let m_cogWheelRadiusA = match m_cogWheelRadiusA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cogWheelRadiusA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_cogWheelRadiusB = match m_cogWheelRadiusB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cogWheelRadiusB",
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
                    let __ptr = None;
                    let parent = hkpConstraintAtom { __ptr, m_type };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpCogWheelConstraintAtom {
                        __ptr,
                        parent,
                        m_cogWheelRadiusA,
                        m_cogWheelRadiusB,
                        m_isScrew,
                        m_memOffsetToInitialAngleOffset,
                        m_memOffsetToPrevAngle,
                        m_memOffsetToRevolutionCounter,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "cogWheelRadiusA",
                "cogWheelRadiusB",
                "isScrew",
                "memOffsetToInitialAngleOffset",
                "memOffsetToPrevAngle",
                "memOffsetToRevolutionCounter",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCogWheelConstraintAtom",
                FIELDS,
                __hkpCogWheelConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<hkpCogWheelConstraintAtom>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
