use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaWaveletCompressedAnimation`
/// -         version: `0`
/// -       signature: `0x77cf0962`
/// -          size: 112(x86)/136(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaWaveletCompressedAnimation<'a> {
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
    /// -          name: `qFormat`(ctype: `struct hkaWaveletCompressedAnimationQuantizationFormat`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_qFormat: hkaWaveletCompressedAnimationQuantizationFormat,
    /// # C++ Info
    /// -          name: `staticMaskIdx`(ctype: `hkUint32`)
    /// -        offset:  68(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticMaskIdx: u32,
    /// # C++ Info
    /// -          name: `staticDOFsIdx`(ctype: `hkUint32`)
    /// -        offset:  72(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticDOFsIdx: u32,
    /// # C++ Info
    /// -          name: `numStaticTransformDOFs`(ctype: `hkUint32`)
    /// -        offset:  76(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numStaticTransformDOFs: u32,
    /// # C++ Info
    /// -          name: `numDynamicTransformDOFs`(ctype: `hkUint32`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numDynamicTransformDOFs: u32,
    /// # C++ Info
    /// -          name: `blockIndexIdx`(ctype: `hkUint32`)
    /// -        offset:  84(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blockIndexIdx: u32,
    /// # C++ Info
    /// -          name: `blockIndexSize`(ctype: `hkUint32`)
    /// -        offset:  88(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blockIndexSize: u32,
    /// # C++ Info
    /// -          name: `quantizedDataIdx`(ctype: `hkUint32`)
    /// -        offset:  92(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_quantizedDataIdx: u32,
    /// # C++ Info
    /// -          name: `quantizedDataSize`(ctype: `hkUint32`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_quantizedDataSize: u32,
    /// # C++ Info
    /// -          name: `dataBuffer`(ctype: `hkArray<hkUint8>`)
    /// -        offset: 100(x86)/120(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_dataBuffer: Vec<u8>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaWaveletCompressedAnimation<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaWaveletCompressedAnimation"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x77cf0962)
        }
    }
    impl<'a> _serde::Serialize for hkaWaveletCompressedAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x77cf0962)));
            let mut serializer = __serializer
                .serialize_struct("hkaWaveletCompressedAnimation", class_meta)?;
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
            serializer.serialize_field("staticMaskIdx", &self.m_staticMaskIdx)?;
            serializer.serialize_field("staticDOFsIdx", &self.m_staticDOFsIdx)?;
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
            serializer.serialize_field("blockIndexIdx", &self.m_blockIndexIdx)?;
            serializer.serialize_field("blockIndexSize", &self.m_blockIndexSize)?;
            serializer.serialize_field("quantizedDataIdx", &self.m_quantizedDataIdx)?;
            serializer.serialize_field("quantizedDataSize", &self.m_quantizedDataSize)?;
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
    m_staticMaskIdx,
    m_staticDOFsIdx,
    m_numStaticTransformDOFs,
    m_numDynamicTransformDOFs,
    m_blockIndexIdx,
    m_blockIndexSize,
    m_quantizedDataIdx,
    m_quantizedDataSize,
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
            "staticMaskIdx" => Ok(__Field::m_staticMaskIdx),
            "staticDOFsIdx" => Ok(__Field::m_staticDOFsIdx),
            "numStaticTransformDOFs" => Ok(__Field::m_numStaticTransformDOFs),
            "numDynamicTransformDOFs" => Ok(__Field::m_numDynamicTransformDOFs),
            "blockIndexIdx" => Ok(__Field::m_blockIndexIdx),
            "blockIndexSize" => Ok(__Field::m_blockIndexSize),
            "quantizedDataIdx" => Ok(__Field::m_quantizedDataIdx),
            "quantizedDataSize" => Ok(__Field::m_quantizedDataSize),
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
pub(super) struct __hkaWaveletCompressedAnimationVisitor<'de> {
    marker: core::marker::PhantomData<hkaWaveletCompressedAnimation<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaWaveletCompressedAnimationVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkaWaveletCompressedAnimation<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkaWaveletCompressedAnimation<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkaWaveletCompressedAnimationVisitor<'de> {
    type Value = hkaWaveletCompressedAnimation<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkaWaveletCompressedAnimation",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_numberOfPoses: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_blockSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_qFormat: _serde::__private::Option<
            hkaWaveletCompressedAnimationQuantizationFormat,
        > = _serde::__private::None;
        let mut m_staticMaskIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticDOFsIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_numStaticTransformDOFs: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_numDynamicTransformDOFs: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_blockIndexIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_blockIndexSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_quantizedDataIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_quantizedDataSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_dataBuffer: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        for i in 0..12usize {
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
                            hkaWaveletCompressedAnimationQuantizationFormat,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
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
                4usize => {
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
                5usize => {
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
                6usize => {
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
                7usize => {
                    if _serde::__private::Option::is_some(&m_blockIndexIdx) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "blockIndexIdx",
                            ),
                        );
                    }
                    m_blockIndexIdx = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_blockIndexSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "blockIndexSize",
                            ),
                        );
                    }
                    m_blockIndexSize = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
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
                10usize => {
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
                11usize => {
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
        let m_staticMaskIdx = match m_staticMaskIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticMaskIdx"),
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
        let m_blockIndexIdx = match m_blockIndexIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockIndexIdx"),
                );
            }
        };
        let m_blockIndexSize = match m_blockIndexSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockIndexSize"),
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
        let m_dataBuffer = match m_dataBuffer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dataBuffer"),
                );
            }
        };
        _serde::__private::Ok(hkaWaveletCompressedAnimation {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_numberOfPoses,
            m_blockSize,
            m_qFormat,
            m_staticMaskIdx,
            m_staticDOFsIdx,
            m_numStaticTransformDOFs,
            m_numDynamicTransformDOFs,
            m_blockIndexIdx,
            m_blockIndexSize,
            m_quantizedDataIdx,
            m_quantizedDataSize,
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
        let parent = __hkaAnimationVisitor::visit_as_parent(&mut __map)?;
        let mut m_numberOfPoses: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_blockSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_qFormat: _serde::__private::Option<
            hkaWaveletCompressedAnimationQuantizationFormat,
        > = _serde::__private::None;
        let mut m_staticMaskIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_staticDOFsIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_numStaticTransformDOFs: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_numDynamicTransformDOFs: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_blockIndexIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_blockIndexSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_quantizedDataIdx: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_quantizedDataSize: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_dataBuffer: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        for _ in 0..12usize {
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
                                hkaWaveletCompressedAnimationQuantizationFormat,
                            >(&mut __map) {
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
                    __Field::m_blockIndexIdx => {
                        if _serde::__private::Option::is_some(&m_blockIndexIdx) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "blockIndexIdx",
                                ),
                            );
                        }
                        m_blockIndexIdx = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_blockIndexSize => {
                        if _serde::__private::Option::is_some(&m_blockIndexSize) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "blockIndexSize",
                                ),
                            );
                        }
                        m_blockIndexSize = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
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
        let m_staticMaskIdx = match m_staticMaskIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("staticMaskIdx"),
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
        let m_blockIndexIdx = match m_blockIndexIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockIndexIdx"),
                );
            }
        };
        let m_blockIndexSize = match m_blockIndexSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockIndexSize"),
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
        let m_dataBuffer = match m_dataBuffer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dataBuffer"),
                );
            }
        };
        _serde::__private::Ok(hkaWaveletCompressedAnimation {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_numberOfPoses,
            m_blockSize,
            m_qFormat,
            m_staticMaskIdx,
            m_staticDOFsIdx,
            m_numStaticTransformDOFs,
            m_numDynamicTransformDOFs,
            m_blockIndexIdx,
            m_blockIndexSize,
            m_quantizedDataIdx,
            m_quantizedDataSize,
            m_dataBuffer,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaWaveletCompressedAnimation<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "numberOfPoses",
                "blockSize",
                "qFormat",
                "staticMaskIdx",
                "staticDOFsIdx",
                "numStaticTransformDOFs",
                "numDynamicTransformDOFs",
                "blockIndexIdx",
                "blockIndexSize",
                "quantizedDataIdx",
                "quantizedDataSize",
                "dataBuffer",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaWaveletCompressedAnimation",
                FIELDS,
                __hkaWaveletCompressedAnimationVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaWaveletCompressedAnimation,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
