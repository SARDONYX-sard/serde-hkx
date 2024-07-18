use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaDeltaCompressedAnimationQuantizationFormat`
/// - version: `0`
/// - signature: `0x724a7561`
/// - size: ` 20`(x86)/` 20`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaDeltaCompressedAnimationQuantizationFormat {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `maxBitWidth`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_maxBitWidth: u8,
    /// # C++ Info
    /// - name: `preserved`(ctype: `hkUint8`)
    /// - offset: `  1`(x86)/`  1`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_preserved: u8,
    /// # C++ Info
    /// - name: `numD`(ctype: `hkUint32`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numD: u32,
    /// # C++ Info
    /// - name: `offsetIdx`(ctype: `hkUint32`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_offsetIdx: u32,
    /// # C++ Info
    /// - name: `scaleIdx`(ctype: `hkUint32`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_scaleIdx: u32,
    /// # C++ Info
    /// - name: `bitWidthIdx`(ctype: `hkUint32`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_bitWidthIdx: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaDeltaCompressedAnimationQuantizationFormat {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaDeltaCompressedAnimationQuantizationFormat"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x724a7561)
        }
    }
    impl _serde::Serialize for hkaDeltaCompressedAnimationQuantizationFormat {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x724a7561)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaDeltaCompressedAnimationQuantizationFormat",
                    class_meta,
                )?;
            serializer.serialize_field("maxBitWidth", &self.m_maxBitWidth)?;
            serializer.serialize_field("preserved", &self.m_preserved)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("numD", &self.m_numD)?;
            serializer.serialize_field("offsetIdx", &self.m_offsetIdx)?;
            serializer.serialize_field("scaleIdx", &self.m_scaleIdx)?;
            serializer.serialize_field("bitWidthIdx", &self.m_bitWidthIdx)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_maxBitWidth,
    m_preserved,
    m_numD,
    m_offsetIdx,
    m_scaleIdx,
    m_bitWidthIdx,
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
            "maxBitWidth" => Ok(__Field::m_maxBitWidth),
            "preserved" => Ok(__Field::m_preserved),
            "numD" => Ok(__Field::m_numD),
            "offsetIdx" => Ok(__Field::m_offsetIdx),
            "scaleIdx" => Ok(__Field::m_scaleIdx),
            "bitWidthIdx" => Ok(__Field::m_bitWidthIdx),
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
pub(super) struct __hkaDeltaCompressedAnimationQuantizationFormatVisitor<'de> {
    marker: core::marker::PhantomData<hkaDeltaCompressedAnimationQuantizationFormat>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaDeltaCompressedAnimationQuantizationFormatVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkaDeltaCompressedAnimationQuantizationFormat,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkaDeltaCompressedAnimationQuantizationFormat,
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
impl<'de> _serde::de::Visitor<'de>
for __hkaDeltaCompressedAnimationQuantizationFormatVisitor<'de> {
    type Value = hkaDeltaCompressedAnimationQuantizationFormat;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkaDeltaCompressedAnimationQuantizationFormat",
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
        let mut m_maxBitWidth: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_preserved: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_numD: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_offsetIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_scaleIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_bitWidthIdx: _serde::__private::Option<u32> = _serde::__private::None;
        for i in 0..6usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_maxBitWidth) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxBitWidth",
                            ),
                        );
                    }
                    m_maxBitWidth = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_preserved) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "preserved",
                            ),
                        );
                    }
                    m_preserved = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_numD) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("numD"),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 2usize)?;
                    m_numD = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_offsetIdx) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "offsetIdx",
                            ),
                        );
                    }
                    m_offsetIdx = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_scaleIdx) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "scaleIdx",
                            ),
                        );
                    }
                    m_scaleIdx = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_bitWidthIdx) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bitWidthIdx",
                            ),
                        );
                    }
                    m_bitWidthIdx = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
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
        let m_maxBitWidth = match m_maxBitWidth {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxBitWidth"),
                );
            }
        };
        let m_preserved = match m_preserved {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("preserved"),
                );
            }
        };
        let m_numD = match m_numD {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numD"),
                );
            }
        };
        let m_offsetIdx = match m_offsetIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offsetIdx"),
                );
            }
        };
        let m_scaleIdx = match m_scaleIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("scaleIdx"),
                );
            }
        };
        let m_bitWidthIdx = match m_bitWidthIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bitWidthIdx"),
                );
            }
        };
        _serde::__private::Ok(hkaDeltaCompressedAnimationQuantizationFormat {
            __ptr,
            m_maxBitWidth,
            m_preserved,
            m_numD,
            m_offsetIdx,
            m_scaleIdx,
            m_bitWidthIdx,
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
        let mut m_maxBitWidth: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_preserved: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_numD: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_offsetIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_scaleIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_bitWidthIdx: _serde::__private::Option<u32> = _serde::__private::None;
        for _ in 0..6usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_maxBitWidth => {
                        if _serde::__private::Option::is_some(&m_maxBitWidth) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxBitWidth",
                                ),
                            );
                        }
                        m_maxBitWidth = _serde::__private::Some(
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
                    __Field::m_preserved => {
                        if _serde::__private::Option::is_some(&m_preserved) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "preserved",
                                ),
                            );
                        }
                        m_preserved = _serde::__private::Some(
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
                    __Field::m_numD => {
                        if _serde::__private::Option::is_some(&m_numD) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("numD"),
                            );
                        }
                        m_numD = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_offsetIdx => {
                        if _serde::__private::Option::is_some(&m_offsetIdx) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "offsetIdx",
                                ),
                            );
                        }
                        m_offsetIdx = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_scaleIdx => {
                        if _serde::__private::Option::is_some(&m_scaleIdx) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "scaleIdx",
                                ),
                            );
                        }
                        m_scaleIdx = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_bitWidthIdx => {
                        if _serde::__private::Option::is_some(&m_bitWidthIdx) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bitWidthIdx",
                                ),
                            );
                        }
                        m_bitWidthIdx = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
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
        let m_maxBitWidth = match m_maxBitWidth {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxBitWidth"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_preserved = match m_preserved {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("preserved"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_numD = match m_numD {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numD"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_offsetIdx = match m_offsetIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offsetIdx"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_scaleIdx = match m_scaleIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("scaleIdx"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_bitWidthIdx = match m_bitWidthIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bitWidthIdx"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkaDeltaCompressedAnimationQuantizationFormat {
            __ptr,
            m_maxBitWidth,
            m_preserved,
            m_numD,
            m_offsetIdx,
            m_scaleIdx,
            m_bitWidthIdx,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkaDeltaCompressedAnimationQuantizationFormat {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "maxBitWidth",
                "preserved",
                "numD",
                "offsetIdx",
                "scaleIdx",
                "bitWidthIdx",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaDeltaCompressedAnimationQuantizationFormat",
                FIELDS,
                __hkaDeltaCompressedAnimationQuantizationFormatVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaDeltaCompressedAnimationQuantizationFormat,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
