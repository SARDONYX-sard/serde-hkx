use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaDeltaCompressedAnimation`
/// - version: `0`
/// - signature: `0x90a68d40`
/// - size: `120`(x86)/`144`(x86_64)
/// -  vtable: `true`
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
    /// - name: `numberOfPoses`(ctype: `hkInt32`)
    /// - offset: ` 40`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numberOfPoses: i32,
    /// # C++ Info
    /// - name: `blockSize`(ctype: `hkInt32`)
    /// - offset: ` 44`(x86)/` 60`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_blockSize: i32,
    /// # C++ Info
    /// - name: `qFormat`(ctype: `struct hkaDeltaCompressedAnimationQuantizationFormat`)
    /// - offset: ` 48`(x86)/` 64`(x86_64)
    /// - type_size: ` 20`(x86)/` 20`(x86_64)
    pub m_qFormat: hkaDeltaCompressedAnimationQuantizationFormat,
    /// # C++ Info
    /// - name: `quantizedDataIdx`(ctype: `hkUint32`)
    /// - offset: ` 68`(x86)/` 84`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_quantizedDataIdx: u32,
    /// # C++ Info
    /// - name: `quantizedDataSize`(ctype: `hkUint32`)
    /// - offset: ` 72`(x86)/` 88`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_quantizedDataSize: u32,
    /// # C++ Info
    /// - name: `staticMaskIdx`(ctype: `hkUint32`)
    /// - offset: ` 76`(x86)/` 92`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_staticMaskIdx: u32,
    /// # C++ Info
    /// - name: `staticMaskSize`(ctype: `hkUint32`)
    /// - offset: ` 80`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_staticMaskSize: u32,
    /// # C++ Info
    /// - name: `staticDOFsIdx`(ctype: `hkUint32`)
    /// - offset: ` 84`(x86)/`100`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_staticDOFsIdx: u32,
    /// # C++ Info
    /// - name: `staticDOFsSize`(ctype: `hkUint32`)
    /// - offset: ` 88`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_staticDOFsSize: u32,
    /// # C++ Info
    /// - name: `numStaticTransformDOFs`(ctype: `hkUint32`)
    /// - offset: ` 92`(x86)/`108`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numStaticTransformDOFs: u32,
    /// # C++ Info
    /// - name: `numDynamicTransformDOFs`(ctype: `hkUint32`)
    /// - offset: ` 96`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numDynamicTransformDOFs: u32,
    /// # C++ Info
    /// - name: `totalBlockSize`(ctype: `hkUint32`)
    /// - offset: `100`(x86)/`116`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_totalBlockSize: u32,
    /// # C++ Info
    /// - name: `lastBlockSize`(ctype: `hkUint32`)
    /// - offset: `104`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_lastBlockSize: u32,
    /// # C++ Info
    /// - name: `dataBuffer`(ctype: `hkArray<hkUint8>`)
    /// - offset: `108`(x86)/`128`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
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
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.m_extractedMotion.get());
            v.extend(
                self
                    .parent
                    .m_annotationTracks
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(self.m_qFormat.deps_indexes());
            v
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
                .serialize_struct(
                    "hkaDeltaCompressedAnimation",
                    class_meta,
                    (120u64, 144u64),
                )?;
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
                .serialize_array_field(
                    "annotationTracks",
                    &self.parent.m_annotationTracks,
                    TypeSize::Struct {
                        size_x86: 16u64,
                        size_x86_64: 24u64,
                    },
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
            serializer
                .serialize_array_field(
                    "dataBuffer",
                    &self.m_dataBuffer,
                    TypeSize::NonPtr,
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
    impl<'de> _serde::Deserialize<'de> for hkaDeltaCompressedAnimation<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_type,
                m_duration,
                m_numberOfTransformTracks,
                m_numberOfFloatTracks,
                m_extractedMotion,
                m_annotationTracks,
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
                        "duration" => Ok(__Field::m_duration),
                        "numberOfTransformTracks" => {
                            Ok(__Field::m_numberOfTransformTracks)
                        }
                        "numberOfFloatTracks" => Ok(__Field::m_numberOfFloatTracks),
                        "extractedMotion" => Ok(__Field::m_extractedMotion),
                        "annotationTracks" => Ok(__Field::m_annotationTracks),
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
                        "numDynamicTransformDOFs" => {
                            Ok(__Field::m_numDynamicTransformDOFs)
                        }
                        "totalBlockSize" => Ok(__Field::m_totalBlockSize),
                        "lastBlockSize" => Ok(__Field::m_lastBlockSize),
                        "dataBuffer" => Ok(__Field::m_dataBuffer),
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
            struct __hkaDeltaCompressedAnimationVisitor<'de> {
                marker: _serde::__private::PhantomData<hkaDeltaCompressedAnimation<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkaDeltaCompressedAnimationVisitor<'de> {
                type Value = hkaDeltaCompressedAnimation<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
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
                                if _serde::__private::Option::is_some(
                                    &m_quantizedDataSize,
                                ) {
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
                            10usize => {
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numberOfPoses",
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "quantizedDataIdx",
                                ),
                            );
                        }
                    };
                    let m_quantizedDataSize = match m_quantizedDataSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "quantizedDataSize",
                                ),
                            );
                        }
                    };
                    let m_staticMaskIdx = match m_staticMaskIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticMaskIdx",
                                ),
                            );
                        }
                    };
                    let m_staticMaskSize = match m_staticMaskSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticMaskSize",
                                ),
                            );
                        }
                    };
                    let m_staticDOFsIdx = match m_staticDOFsIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticDOFsIdx",
                                ),
                            );
                        }
                    };
                    let m_staticDOFsSize = match m_staticDOFsSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticDOFsSize",
                                ),
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "totalBlockSize",
                                ),
                            );
                        }
                    };
                    let m_lastBlockSize = match m_lastBlockSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastBlockSize",
                                ),
                            );
                        }
                    };
                    let m_dataBuffer = match m_dataBuffer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dataBuffer",
                                ),
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
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_type: _serde::__private::Option<AnimationType> = _serde::__private::None;
                    let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_numberOfTransformTracks: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_numberOfFloatTracks: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_extractedMotion: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_annotationTracks: _serde::__private::Option<
                        Vec<hkaAnnotationTrack<'de>>,
                    > = _serde::__private::None;
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
                                    match __A::next_value::<AnimationType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_duration => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_duration) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "duration",
                                        ),
                                    );
                                }
                                m_duration = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numberOfTransformTracks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numberOfTransformTracks,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numberOfTransformTracks",
                                        ),
                                    );
                                }
                                m_numberOfTransformTracks = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numberOfFloatTracks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numberOfFloatTracks,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numberOfFloatTracks",
                                        ),
                                    );
                                }
                                m_numberOfFloatTracks = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_extractedMotion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_extractedMotion) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extractedMotion",
                                        ),
                                    );
                                }
                                m_extractedMotion = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotationTracks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_annotationTracks) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotationTracks",
                                        ),
                                    );
                                }
                                m_annotationTracks = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkaAnnotationTrack<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numberOfPoses => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numberOfPoses) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_qFormat => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_qFormat) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_quantizedDataIdx) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_quantizedDataSize,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_staticMaskIdx) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_staticMaskSize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_staticDOFsIdx) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_staticDOFsSize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numStaticTransformDOFs,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numDynamicTransformDOFs,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_totalBlockSize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lastBlockSize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_dataBuffer) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                    let m_duration = match m_duration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("duration"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numberOfTransformTracks = match m_numberOfTransformTracks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numberOfTransformTracks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numberOfFloatTracks = match m_numberOfFloatTracks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numberOfFloatTracks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_extractedMotion = match m_extractedMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extractedMotion",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotationTracks = match m_annotationTracks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotationTracks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numberOfPoses = match m_numberOfPoses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numberOfPoses",
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
                    let m_qFormat = match m_qFormat {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("qFormat"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_quantizedDataIdx = match m_quantizedDataIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "quantizedDataIdx",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_quantizedDataSize = match m_quantizedDataSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "quantizedDataSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticMaskIdx = match m_staticMaskIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticMaskIdx",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticMaskSize = match m_staticMaskSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticMaskSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticDOFsIdx = match m_staticDOFsIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticDOFsIdx",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticDOFsSize = match m_staticDOFsSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticDOFsSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numStaticTransformDOFs = match m_numStaticTransformDOFs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numStaticTransformDOFs",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numDynamicTransformDOFs = match m_numDynamicTransformDOFs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numDynamicTransformDOFs",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_totalBlockSize = match m_totalBlockSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "totalBlockSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lastBlockSize = match m_lastBlockSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lastBlockSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dataBuffer = match m_dataBuffer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dataBuffer",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkaAnimation {
                        __ptr,
                        parent,
                        m_type,
                        m_duration,
                        m_numberOfTransformTracks,
                        m_numberOfFloatTracks,
                        m_extractedMotion,
                        m_annotationTracks,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
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
