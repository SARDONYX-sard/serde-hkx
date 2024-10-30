use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaWaveletCompressedAnimationCompressionParams`
/// - version: `0`
/// - signature: `0x27c6cafa`
/// - size: ` 36`(x86)/` 36`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaWaveletCompressedAnimationCompressionParams {
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
    /// # C++ Info
    /// - name: `quantizationBits`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "quantizationBits"))]
    #[cfg_attr(feature = "serde", serde(rename = "quantizationBits"))]
    pub m_quantizationBits: u16,
    /// # C++ Info
    /// - name: `blockSize`(ctype: `hkUint16`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "blockSize"))]
    #[cfg_attr(feature = "serde", serde(rename = "blockSize"))]
    pub m_blockSize: u16,
    /// # C++ Info
    /// - name: `preserve`(ctype: `hkUint16`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "preserve"))]
    #[cfg_attr(feature = "serde", serde(rename = "preserve"))]
    pub m_preserve: u16,
    /// # C++ Info
    /// - name: `truncProp`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "truncProp"))]
    #[cfg_attr(feature = "serde", serde(rename = "truncProp"))]
    pub m_truncProp: f32,
    /// # C++ Info
    /// - name: `useOldStyleTruncation`(ctype: `hkBool`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "useOldStyleTruncation"))]
    #[cfg_attr(feature = "serde", serde(rename = "useOldStyleTruncation"))]
    pub m_useOldStyleTruncation: bool,
    /// # C++ Info
    /// - name: `absolutePositionTolerance`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "absolutePositionTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "absolutePositionTolerance"))]
    pub m_absolutePositionTolerance: f32,
    /// # C++ Info
    /// - name: `relativePositionTolerance`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "relativePositionTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "relativePositionTolerance"))]
    pub m_relativePositionTolerance: f32,
    /// # C++ Info
    /// - name: `rotationTolerance`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "rotationTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "rotationTolerance"))]
    pub m_rotationTolerance: f32,
    /// # C++ Info
    /// - name: `scaleTolerance`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "scaleTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "scaleTolerance"))]
    pub m_scaleTolerance: f32,
    /// # C++ Info
    /// - name: `absoluteFloatTolerance`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "absoluteFloatTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "absoluteFloatTolerance"))]
    pub m_absoluteFloatTolerance: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaWaveletCompressedAnimationCompressionParams {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaWaveletCompressedAnimationCompressionParams"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x27c6cafa)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkaWaveletCompressedAnimationCompressionParams {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x27c6cafa)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaWaveletCompressedAnimationCompressionParams",
                    class_meta,
                    (36u64, 36u64),
                )?;
            serializer.serialize_field("quantizationBits", &self.m_quantizationBits)?;
            serializer.serialize_field("blockSize", &self.m_blockSize)?;
            serializer.serialize_field("preserve", &self.m_preserve)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("truncProp", &self.m_truncProp)?;
            serializer
                .serialize_field(
                    "useOldStyleTruncation",
                    &self.m_useOldStyleTruncation,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "absolutePositionTolerance",
                    &self.m_absolutePositionTolerance,
                )?;
            serializer
                .serialize_field(
                    "relativePositionTolerance",
                    &self.m_relativePositionTolerance,
                )?;
            serializer.serialize_field("rotationTolerance", &self.m_rotationTolerance)?;
            serializer.serialize_field("scaleTolerance", &self.m_scaleTolerance)?;
            serializer
                .serialize_field(
                    "absoluteFloatTolerance",
                    &self.m_absoluteFloatTolerance,
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
    impl<'de> _serde::Deserialize<'de>
    for hkaWaveletCompressedAnimationCompressionParams {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_quantizationBits,
                m_blockSize,
                m_preserve,
                m_truncProp,
                m_useOldStyleTruncation,
                m_absolutePositionTolerance,
                m_relativePositionTolerance,
                m_rotationTolerance,
                m_scaleTolerance,
                m_absoluteFloatTolerance,
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
                        "quantizationBits" => Ok(__Field::m_quantizationBits),
                        "blockSize" => Ok(__Field::m_blockSize),
                        "preserve" => Ok(__Field::m_preserve),
                        "truncProp" => Ok(__Field::m_truncProp),
                        "useOldStyleTruncation" => Ok(__Field::m_useOldStyleTruncation),
                        "absolutePositionTolerance" => {
                            Ok(__Field::m_absolutePositionTolerance)
                        }
                        "relativePositionTolerance" => {
                            Ok(__Field::m_relativePositionTolerance)
                        }
                        "rotationTolerance" => Ok(__Field::m_rotationTolerance),
                        "scaleTolerance" => Ok(__Field::m_scaleTolerance),
                        "absoluteFloatTolerance" => Ok(__Field::m_absoluteFloatTolerance),
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
            struct __hkaWaveletCompressedAnimationCompressionParamsVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkaWaveletCompressedAnimationCompressionParams,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkaWaveletCompressedAnimationCompressionParamsVisitor<'de> {
                type Value = hkaWaveletCompressedAnimationCompressionParams;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkaWaveletCompressedAnimationCompressionParams",
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
                    let mut m_quantizationBits: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_blockSize: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_preserve: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_truncProp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useOldStyleTruncation: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_absolutePositionTolerance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_relativePositionTolerance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_rotationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_scaleTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_absoluteFloatTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..10usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_quantizationBits) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "quantizationBits",
                                        ),
                                    );
                                }
                                m_quantizationBits = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_blockSize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blockSize",
                                        ),
                                    );
                                }
                                m_blockSize = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_preserve) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "preserve",
                                        ),
                                    );
                                }
                                m_preserve = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_truncProp) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "truncProp",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_truncProp = _serde::__private::Some(
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
                                    &m_useOldStyleTruncation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useOldStyleTruncation",
                                        ),
                                    );
                                }
                                m_useOldStyleTruncation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_absolutePositionTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "absolutePositionTolerance",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_absolutePositionTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_relativePositionTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "relativePositionTolerance",
                                        ),
                                    );
                                }
                                m_relativePositionTolerance = _serde::__private::Some(
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
                                    &m_rotationTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotationTolerance",
                                        ),
                                    );
                                }
                                m_rotationTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_scaleTolerance) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "scaleTolerance",
                                        ),
                                    );
                                }
                                m_scaleTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_absoluteFloatTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "absoluteFloatTolerance",
                                        ),
                                    );
                                }
                                m_absoluteFloatTolerance = _serde::__private::Some(
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
                    let m_quantizationBits = match m_quantizationBits {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "quantizationBits",
                                ),
                            );
                        }
                    };
                    let m_blockSize = match m_blockSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blockSize",
                                ),
                            );
                        }
                    };
                    let m_preserve = match m_preserve {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("preserve"),
                            );
                        }
                    };
                    let m_truncProp = match m_truncProp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "truncProp",
                                ),
                            );
                        }
                    };
                    let m_useOldStyleTruncation = match m_useOldStyleTruncation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useOldStyleTruncation",
                                ),
                            );
                        }
                    };
                    let m_absolutePositionTolerance = match m_absolutePositionTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "absolutePositionTolerance",
                                ),
                            );
                        }
                    };
                    let m_relativePositionTolerance = match m_relativePositionTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "relativePositionTolerance",
                                ),
                            );
                        }
                    };
                    let m_rotationTolerance = match m_rotationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotationTolerance",
                                ),
                            );
                        }
                    };
                    let m_scaleTolerance = match m_scaleTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "scaleTolerance",
                                ),
                            );
                        }
                    };
                    let m_absoluteFloatTolerance = match m_absoluteFloatTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "absoluteFloatTolerance",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkaWaveletCompressedAnimationCompressionParams {
                        __ptr,
                        m_quantizationBits,
                        m_blockSize,
                        m_preserve,
                        m_truncProp,
                        m_useOldStyleTruncation,
                        m_absolutePositionTolerance,
                        m_relativePositionTolerance,
                        m_rotationTolerance,
                        m_scaleTolerance,
                        m_absoluteFloatTolerance,
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
                    let mut m_quantizationBits: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_blockSize: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_preserve: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_truncProp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useOldStyleTruncation: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_absolutePositionTolerance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_relativePositionTolerance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_rotationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_scaleTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_absoluteFloatTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_quantizationBits => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_quantizationBits) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "quantizationBits",
                                        ),
                                    );
                                }
                                m_quantizationBits = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_blockSize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blockSize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blockSize",
                                        ),
                                    );
                                }
                                m_blockSize = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_preserve => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_preserve) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "preserve",
                                        ),
                                    );
                                }
                                m_preserve = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_truncProp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_truncProp) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "truncProp",
                                        ),
                                    );
                                }
                                m_truncProp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_useOldStyleTruncation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_useOldStyleTruncation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useOldStyleTruncation",
                                        ),
                                    );
                                }
                                m_useOldStyleTruncation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_absolutePositionTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_absolutePositionTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "absolutePositionTolerance",
                                        ),
                                    );
                                }
                                m_absolutePositionTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_relativePositionTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_relativePositionTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "relativePositionTolerance",
                                        ),
                                    );
                                }
                                m_relativePositionTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rotationTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_rotationTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotationTolerance",
                                        ),
                                    );
                                }
                                m_rotationTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_scaleTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_scaleTolerance) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "scaleTolerance",
                                        ),
                                    );
                                }
                                m_scaleTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_absoluteFloatTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_absoluteFloatTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "absoluteFloatTolerance",
                                        ),
                                    );
                                }
                                m_absoluteFloatTolerance = _serde::__private::Some(
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
                    let m_quantizationBits = match m_quantizationBits {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "quantizationBits",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blockSize = match m_blockSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blockSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_preserve = match m_preserve {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("preserve"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_truncProp = match m_truncProp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "truncProp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useOldStyleTruncation = match m_useOldStyleTruncation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useOldStyleTruncation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_absolutePositionTolerance = match m_absolutePositionTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "absolutePositionTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_relativePositionTolerance = match m_relativePositionTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "relativePositionTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rotationTolerance = match m_rotationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotationTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_scaleTolerance = match m_scaleTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "scaleTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_absoluteFloatTolerance = match m_absoluteFloatTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "absoluteFloatTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkaWaveletCompressedAnimationCompressionParams {
                        __ptr,
                        m_quantizationBits,
                        m_blockSize,
                        m_preserve,
                        m_truncProp,
                        m_useOldStyleTruncation,
                        m_absolutePositionTolerance,
                        m_relativePositionTolerance,
                        m_rotationTolerance,
                        m_scaleTolerance,
                        m_absoluteFloatTolerance,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "quantizationBits",
                "blockSize",
                "preserve",
                "truncProp",
                "useOldStyleTruncation",
                "absolutePositionTolerance",
                "relativePositionTolerance",
                "rotationTolerance",
                "scaleTolerance",
                "absoluteFloatTolerance",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaWaveletCompressedAnimationCompressionParams",
                FIELDS,
                __hkaWaveletCompressedAnimationCompressionParamsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaWaveletCompressedAnimationCompressionParams,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
