use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaDeltaCompressedAnimation`
/// -         version: `0`
/// -       signature: `0x90a68d40`
/// -          size: 120(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaDeltaCompressedAnimation<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkaAnimation<'a>,
    /// # C++ Info
    /// -          name: `numberOfPoses`(ctype: `hkInt32`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numberOfPoses: i32,
    /// # C++ Info
    /// -          name: `blockSize`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blockSize: i32,
    /// # C++ Info
    /// -          name: `qFormat`(ctype: `struct hkaDeltaCompressedAnimationQuantizationFormat`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_qFormat: hkaDeltaCompressedAnimationQuantizationFormat,
    /// # C++ Info
    /// -          name: `quantizedDataIdx`(ctype: `hkUint32`)
    /// -        offset:  68(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_quantizedDataIdx: u32,
    /// # C++ Info
    /// -          name: `quantizedDataSize`(ctype: `hkUint32`)
    /// -        offset:  72(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_quantizedDataSize: u32,
    /// # C++ Info
    /// -          name: `staticMaskIdx`(ctype: `hkUint32`)
    /// -        offset:  76(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticMaskIdx: u32,
    /// # C++ Info
    /// -          name: `staticMaskSize`(ctype: `hkUint32`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticMaskSize: u32,
    /// # C++ Info
    /// -          name: `staticDOFsIdx`(ctype: `hkUint32`)
    /// -        offset:  84(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticDOFsIdx: u32,
    /// # C++ Info
    /// -          name: `staticDOFsSize`(ctype: `hkUint32`)
    /// -        offset:  88(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticDOFsSize: u32,
    /// # C++ Info
    /// -          name: `numStaticTransformDOFs`(ctype: `hkUint32`)
    /// -        offset:  92(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numStaticTransformDOFs: u32,
    /// # C++ Info
    /// -          name: `numDynamicTransformDOFs`(ctype: `hkUint32`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numDynamicTransformDOFs: u32,
    /// # C++ Info
    /// -          name: `totalBlockSize`(ctype: `hkUint32`)
    /// -        offset: 100(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_totalBlockSize: u32,
    /// # C++ Info
    /// -          name: `lastBlockSize`(ctype: `hkUint32`)
    /// -        offset: 104(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lastBlockSize: u32,
    /// # C++ Info
    /// -          name: `dataBuffer`(ctype: `hkArray<hkUint8>`)
    /// -        offset: 108(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_dataBuffer: Vec<u8>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaDeltaCompressedAnimation<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaDeltaCompressedAnimation"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x90a68d40)
        }
    }
    impl<'a> _serde::Serialize for hkaDeltaCompressedAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x90a68d40)));
            let mut serializer = __serializer
                .serialize_struct("hkaDeltaCompressedAnimation", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("duration", &self.parent.m_duration)?;
            serializer
                .serialize_field(
                    "numberOfTransformTracks",
                    &self.parent.m_numberOfTransformTracks,
                )?;
            serializer
                .serialize_field(
                    "numberOfFloatTracks",
                    &self.parent.m_numberOfFloatTracks,
                )?;
            serializer
                .serialize_field("extractedMotion", &self.parent.m_extractedMotion)?;
            serializer
                .serialize_array_meta_field(
                    "annotationTracks",
                    &self.parent.m_annotationTracks,
                )?;
            serializer.serialize_field("numberOfPoses", &self.m_numberOfPoses)?;
            serializer.serialize_field("blockSize", &self.m_blockSize)?;
            serializer.serialize_field("qFormat", &self.m_qFormat)?;
            serializer.serialize_field("quantizedDataIdx", &self.m_quantizedDataIdx)?;
            serializer.serialize_field("quantizedDataSize", &self.m_quantizedDataSize)?;
            serializer.serialize_field("staticMaskIdx", &self.m_staticMaskIdx)?;
            serializer.serialize_field("staticMaskSize", &self.m_staticMaskSize)?;
            serializer.serialize_field("staticDOFsIdx", &self.m_staticDOFsIdx)?;
            serializer.serialize_field("staticDOFsSize", &self.m_staticDOFsSize)?;
            serializer
                .serialize_field(
                    "numStaticTransformDOFs",
                    &self.m_numStaticTransformDOFs,
                )?;
            serializer
                .serialize_field(
                    "numDynamicTransformDOFs",
                    &self.m_numDynamicTransformDOFs,
                )?;
            serializer.serialize_field("totalBlockSize", &self.m_totalBlockSize)?;
            serializer.serialize_field("lastBlockSize", &self.m_lastBlockSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("dataBuffer", &self.m_dataBuffer)?;
            serializer
                .serialize_array_field(
                    "annotationTracks",
                    &self.parent.m_annotationTracks,
                )?;
            serializer.serialize_array_field("dataBuffer", &self.m_dataBuffer)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_numberOfPoses,
    m_blockSize,
    m_qFormat,
    m_quantizedDataIdx,
    m_quantizedDataSize,
    m_staticMaskIdx,
    m_staticMaskSize,
    m_staticDOFsIdx,
    m_staticDOFsSize,
    m_numStaticTransformDOFs,
    m_numDynamicTransformDOFs,
    m_totalBlockSize,
    m_lastBlockSize,
    m_dataBuffer,
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
            "numberOfPoses" => Ok(__Field::m_numberOfPoses),
            "blockSize" => Ok(__Field::m_blockSize),
            "qFormat" => Ok(__Field::m_qFormat),
            "quantizedDataIdx" => Ok(__Field::m_quantizedDataIdx),
            "quantizedDataSize" => Ok(__Field::m_quantizedDataSize),
            "staticMaskIdx" => Ok(__Field::m_staticMaskIdx),
            "staticMaskSize" => Ok(__Field::m_staticMaskSize),
            "staticDOFsIdx" => Ok(__Field::m_staticDOFsIdx),
            "staticDOFsSize" => Ok(__Field::m_staticDOFsSize),
            "numStaticTransformDOFs" => Ok(__Field::m_numStaticTransformDOFs),
            "numDynamicTransformDOFs" => Ok(__Field::m_numDynamicTransformDOFs),
            "totalBlockSize" => Ok(__Field::m_totalBlockSize),
            "lastBlockSize" => Ok(__Field::m_lastBlockSize),
            "dataBuffer" => Ok(__Field::m_dataBuffer),
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
pub(super) struct __hkaDeltaCompressedAnimationVisitor<'de> {
    marker: core::marker::PhantomData<hkaDeltaCompressedAnimation<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaDeltaCompressedAnimationVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkaDeltaCompressedAnimation<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkaDeltaCompressedAnimation<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkaDeltaCompressedAnimationVisitor<'de> {
    type Value = hkaDeltaCompressedAnimation<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkaDeltaCompressedAnimation",
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
        let mut m_numberOfPoses: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_blockSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_qFormat: _serde::__private::Option<
            hkaDeltaCompressedAnimationQuantizationFormat,
        > = _serde::__private::None;
        let mut m_quantizedDataIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_quantizedDataSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticMaskIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticMaskSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticDOFsIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticDOFsSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_numStaticTransformDOFs: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_numDynamicTransformDOFs: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_totalBlockSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_lastBlockSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_dataBuffer: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        for i in 0..14usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_numberOfPoses) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numberOfPoses",
                            ),
                        );
                    }
                    m_numberOfPoses = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_qFormat) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("qFormat"),
                        );
                    }
                    m_qFormat = _serde::__private::Some(
                        match __A::next_value::<
                            hkaDeltaCompressedAnimationQuantizationFormat,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_quantizedDataIdx) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "quantizedDataIdx",
                            ),
                        );
                    }
                    m_quantizedDataIdx = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_quantizedDataSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "quantizedDataSize",
                            ),
                        );
                    }
                    m_quantizedDataSize = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_staticMaskIdx) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "staticMaskIdx",
                            ),
                        );
                    }
                    m_staticMaskIdx = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_staticMaskSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "staticMaskSize",
                            ),
                        );
                    }
                    m_staticMaskSize = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_staticDOFsIdx) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "staticDOFsIdx",
                            ),
                        );
                    }
                    m_staticDOFsIdx = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_staticDOFsSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "staticDOFsSize",
                            ),
                        );
                    }
                    m_staticDOFsSize = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_numStaticTransformDOFs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numStaticTransformDOFs",
                            ),
                        );
                    }
                    m_numStaticTransformDOFs = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_numDynamicTransformDOFs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numDynamicTransformDOFs",
                            ),
                        );
                    }
                    m_numDynamicTransformDOFs = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_totalBlockSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "totalBlockSize",
                            ),
                        );
                    }
                    m_totalBlockSize = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_lastBlockSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "lastBlockSize",
                            ),
                        );
                    }
                    m_lastBlockSize = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_dataBuffer) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dataBuffer",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_dataBuffer = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
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
        let m_numberOfPoses = match m_numberOfPoses {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numberOfPoses"),
                );
            }
        };
        let m_blockSize = match m_blockSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockSize"),
                );
            }
        };
        let m_qFormat = match m_qFormat {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("qFormat"),
                );
            }
        };
        let m_quantizedDataIdx = match m_quantizedDataIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("quantizedDataIdx"),
                );
            }
        };
        let m_quantizedDataSize = match m_quantizedDataSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("quantizedDataSize"),
                );
            }
        };
        let m_staticMaskIdx = match m_staticMaskIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticMaskIdx"),
                );
            }
        };
        let m_staticMaskSize = match m_staticMaskSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticMaskSize"),
                );
            }
        };
        let m_staticDOFsIdx = match m_staticDOFsIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticDOFsIdx"),
                );
            }
        };
        let m_staticDOFsSize = match m_staticDOFsSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticDOFsSize"),
                );
            }
        };
        let m_numStaticTransformDOFs = match m_numStaticTransformDOFs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numStaticTransformDOFs",
                    ),
                );
            }
        };
        let m_numDynamicTransformDOFs = match m_numDynamicTransformDOFs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numDynamicTransformDOFs",
                    ),
                );
            }
        };
        let m_totalBlockSize = match m_totalBlockSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("totalBlockSize"),
                );
            }
        };
        let m_lastBlockSize = match m_lastBlockSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lastBlockSize"),
                );
            }
        };
        let m_dataBuffer = match m_dataBuffer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dataBuffer"),
                );
            }
        };
        _serde::__private::Ok(hkaDeltaCompressedAnimation {
            __ptr,
            parent,
            m_numberOfPoses,
            m_blockSize,
            m_qFormat,
            m_quantizedDataIdx,
            m_quantizedDataSize,
            m_staticMaskIdx,
            m_staticMaskSize,
            m_staticDOFsIdx,
            m_staticDOFsSize,
            m_numStaticTransformDOFs,
            m_numDynamicTransformDOFs,
            m_totalBlockSize,
            m_lastBlockSize,
            m_dataBuffer,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkaAnimationVisitor::visit_as_parent(&mut __map)?;
        let mut m_numberOfPoses: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_blockSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_qFormat: _serde::__private::Option<
            hkaDeltaCompressedAnimationQuantizationFormat,
        > = _serde::__private::None;
        let mut m_quantizedDataIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_quantizedDataSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticMaskIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticMaskSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticDOFsIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticDOFsSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_numStaticTransformDOFs: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_numDynamicTransformDOFs: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_totalBlockSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_lastBlockSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_dataBuffer: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        for _ in 0..14usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_numberOfPoses => {
                        if _serde::__private::Option::is_some(&m_numberOfPoses) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numberOfPoses",
                                ),
                            );
                        }
                        m_numberOfPoses = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_blockSize => {
                        if _serde::__private::Option::is_some(&m_blockSize) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "blockSize",
                                ),
                            );
                        }
                        m_blockSize = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_qFormat => {
                        if _serde::__private::Option::is_some(&m_qFormat) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "qFormat",
                                ),
                            );
                        }
                        m_qFormat = _serde::__private::Some(
                            match __A::next_value::<
                                hkaDeltaCompressedAnimationQuantizationFormat,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_quantizedDataIdx => {
                        if _serde::__private::Option::is_some(&m_quantizedDataIdx) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "quantizedDataIdx",
                                ),
                            );
                        }
                        m_quantizedDataIdx = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_quantizedDataSize => {
                        if _serde::__private::Option::is_some(&m_quantizedDataSize) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "quantizedDataSize",
                                ),
                            );
                        }
                        m_quantizedDataSize = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_staticMaskIdx => {
                        if _serde::__private::Option::is_some(&m_staticMaskIdx) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "staticMaskIdx",
                                ),
                            );
                        }
                        m_staticMaskIdx = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_staticMaskSize => {
                        if _serde::__private::Option::is_some(&m_staticMaskSize) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "staticMaskSize",
                                ),
                            );
                        }
                        m_staticMaskSize = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_staticDOFsIdx => {
                        if _serde::__private::Option::is_some(&m_staticDOFsIdx) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "staticDOFsIdx",
                                ),
                            );
                        }
                        m_staticDOFsIdx = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_staticDOFsSize => {
                        if _serde::__private::Option::is_some(&m_staticDOFsSize) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "staticDOFsSize",
                                ),
                            );
                        }
                        m_staticDOFsSize = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numStaticTransformDOFs => {
                        if _serde::__private::Option::is_some(
                            &m_numStaticTransformDOFs,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numStaticTransformDOFs",
                                ),
                            );
                        }
                        m_numStaticTransformDOFs = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numDynamicTransformDOFs => {
                        if _serde::__private::Option::is_some(
                            &m_numDynamicTransformDOFs,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numDynamicTransformDOFs",
                                ),
                            );
                        }
                        m_numDynamicTransformDOFs = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_totalBlockSize => {
                        if _serde::__private::Option::is_some(&m_totalBlockSize) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "totalBlockSize",
                                ),
                            );
                        }
                        m_totalBlockSize = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_lastBlockSize => {
                        if _serde::__private::Option::is_some(&m_lastBlockSize) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "lastBlockSize",
                                ),
                            );
                        }
                        m_lastBlockSize = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_dataBuffer => {
                        if _serde::__private::Option::is_some(&m_dataBuffer) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "dataBuffer",
                                ),
                            );
                        }
                        m_dataBuffer = _serde::__private::Some(
                            match __A::next_value::<Vec<u8>>(&mut __map) {
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
        }
        let m_numberOfPoses = match m_numberOfPoses {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numberOfPoses"),
                );
            }
        };
        let m_blockSize = match m_blockSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockSize"),
                );
            }
        };
        let m_qFormat = match m_qFormat {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("qFormat"),
                );
            }
        };
        let m_quantizedDataIdx = match m_quantizedDataIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("quantizedDataIdx"),
                );
            }
        };
        let m_quantizedDataSize = match m_quantizedDataSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("quantizedDataSize"),
                );
            }
        };
        let m_staticMaskIdx = match m_staticMaskIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticMaskIdx"),
                );
            }
        };
        let m_staticMaskSize = match m_staticMaskSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticMaskSize"),
                );
            }
        };
        let m_staticDOFsIdx = match m_staticDOFsIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticDOFsIdx"),
                );
            }
        };
        let m_staticDOFsSize = match m_staticDOFsSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticDOFsSize"),
                );
            }
        };
        let m_numStaticTransformDOFs = match m_numStaticTransformDOFs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numStaticTransformDOFs",
                    ),
                );
            }
        };
        let m_numDynamicTransformDOFs = match m_numDynamicTransformDOFs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numDynamicTransformDOFs",
                    ),
                );
            }
        };
        let m_totalBlockSize = match m_totalBlockSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("totalBlockSize"),
                );
            }
        };
        let m_lastBlockSize = match m_lastBlockSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lastBlockSize"),
                );
            }
        };
        let m_dataBuffer = match m_dataBuffer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dataBuffer"),
                );
            }
        };
        _serde::__private::Ok(hkaDeltaCompressedAnimation {
            __ptr,
            parent,
            m_numberOfPoses,
            m_blockSize,
            m_qFormat,
            m_quantizedDataIdx,
            m_quantizedDataSize,
            m_staticMaskIdx,
            m_staticMaskSize,
            m_staticDOFsIdx,
            m_staticDOFsSize,
            m_numStaticTransformDOFs,
            m_numDynamicTransformDOFs,
            m_totalBlockSize,
            m_lastBlockSize,
            m_dataBuffer,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaDeltaCompressedAnimation<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "numberOfPoses",
                "blockSize",
                "qFormat",
                "quantizedDataIdx",
                "quantizedDataSize",
                "staticMaskIdx",
                "staticMaskSize",
                "staticDOFsIdx",
                "staticDOFsSize",
                "numStaticTransformDOFs",
                "numDynamicTransformDOFs",
                "totalBlockSize",
                "lastBlockSize",
                "dataBuffer",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaDeltaCompressedAnimation",
                FIELDS,
                __hkaDeltaCompressedAnimationVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaDeltaCompressedAnimation,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
