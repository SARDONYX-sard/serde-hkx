use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaSplineCompressedAnimation`
/// - version: `0`
/// - signature: `0x792ee0bb`
/// - size: `132`(x86)/`176`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSplineCompressedAnimation<'a> {
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
    /// - name: `numFrames`(ctype: `hkInt32`)
    /// - offset: ` 40`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numFrames: i32,
    /// # C++ Info
    /// - name: `numBlocks`(ctype: `hkInt32`)
    /// - offset: ` 44`(x86)/` 60`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numBlocks: i32,
    /// # C++ Info
    /// - name: `maxFramesPerBlock`(ctype: `hkInt32`)
    /// - offset: ` 48`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxFramesPerBlock: i32,
    /// # C++ Info
    /// - name: `maskAndQuantizationSize`(ctype: `hkInt32`)
    /// - offset: ` 52`(x86)/` 68`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maskAndQuantizationSize: i32,
    /// # C++ Info
    /// - name: `blockDuration`(ctype: `hkReal`)
    /// - offset: ` 56`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_blockDuration: f32,
    /// # C++ Info
    /// - name: `blockInverseDuration`(ctype: `hkReal`)
    /// - offset: ` 60`(x86)/` 76`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_blockInverseDuration: f32,
    /// # C++ Info
    /// - name: `frameDuration`(ctype: `hkReal`)
    /// - offset: ` 64`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_frameDuration: f32,
    /// # C++ Info
    /// - name: `blockOffsets`(ctype: `hkArray<hkUint32>`)
    /// - offset: ` 68`(x86)/` 88`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_blockOffsets: Vec<u32>,
    /// # C++ Info
    /// - name: `floatBlockOffsets`(ctype: `hkArray<hkUint32>`)
    /// - offset: ` 80`(x86)/`104`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_floatBlockOffsets: Vec<u32>,
    /// # C++ Info
    /// - name: `transformOffsets`(ctype: `hkArray<hkUint32>`)
    /// - offset: ` 92`(x86)/`120`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_transformOffsets: Vec<u32>,
    /// # C++ Info
    /// - name: `floatOffsets`(ctype: `hkArray<hkUint32>`)
    /// - offset: `104`(x86)/`136`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_floatOffsets: Vec<u32>,
    /// # C++ Info
    /// - name: `data`(ctype: `hkArray<hkUint8>`)
    /// - offset: `116`(x86)/`152`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_data: Vec<u8>,
    /// # C++ Info
    /// - name: `endian`(ctype: `hkInt32`)
    /// - offset: `128`(x86)/`168`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_endian: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaSplineCompressedAnimation<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSplineCompressedAnimation"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x792ee0bb)
        }
    }
    impl<'a> _serde::Serialize for hkaSplineCompressedAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x792ee0bb)));
            let mut serializer = __serializer
                .serialize_struct("hkaSplineCompressedAnimation", class_meta)?;
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
            serializer.serialize_field("numFrames", &self.m_numFrames)?;
            serializer.serialize_field("numBlocks", &self.m_numBlocks)?;
            serializer.serialize_field("maxFramesPerBlock", &self.m_maxFramesPerBlock)?;
            serializer
                .serialize_field(
                    "maskAndQuantizationSize",
                    &self.m_maskAndQuantizationSize,
                )?;
            serializer.serialize_field("blockDuration", &self.m_blockDuration)?;
            serializer
                .serialize_field("blockInverseDuration", &self.m_blockInverseDuration)?;
            serializer.serialize_field("frameDuration", &self.m_frameDuration)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("blockOffsets", &self.m_blockOffsets)?;
            serializer
                .serialize_array_meta_field(
                    "floatBlockOffsets",
                    &self.m_floatBlockOffsets,
                )?;
            serializer
                .serialize_array_meta_field(
                    "transformOffsets",
                    &self.m_transformOffsets,
                )?;
            serializer.serialize_array_meta_field("floatOffsets", &self.m_floatOffsets)?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_field("endian", &self.m_endian)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "annotationTracks",
                    &self.parent.m_annotationTracks,
                )?;
            serializer.serialize_array_field("blockOffsets", &self.m_blockOffsets)?;
            serializer
                .serialize_array_field("floatBlockOffsets", &self.m_floatBlockOffsets)?;
            serializer
                .serialize_array_field("transformOffsets", &self.m_transformOffsets)?;
            serializer.serialize_array_field("floatOffsets", &self.m_floatOffsets)?;
            serializer.serialize_array_field("data", &self.m_data)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_numFrames,
    m_numBlocks,
    m_maxFramesPerBlock,
    m_maskAndQuantizationSize,
    m_blockDuration,
    m_blockInverseDuration,
    m_frameDuration,
    m_blockOffsets,
    m_floatBlockOffsets,
    m_transformOffsets,
    m_floatOffsets,
    m_data,
    m_endian,
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
            "numFrames" => Ok(__Field::m_numFrames),
            "numBlocks" => Ok(__Field::m_numBlocks),
            "maxFramesPerBlock" => Ok(__Field::m_maxFramesPerBlock),
            "maskAndQuantizationSize" => Ok(__Field::m_maskAndQuantizationSize),
            "blockDuration" => Ok(__Field::m_blockDuration),
            "blockInverseDuration" => Ok(__Field::m_blockInverseDuration),
            "frameDuration" => Ok(__Field::m_frameDuration),
            "blockOffsets" => Ok(__Field::m_blockOffsets),
            "floatBlockOffsets" => Ok(__Field::m_floatBlockOffsets),
            "transformOffsets" => Ok(__Field::m_transformOffsets),
            "floatOffsets" => Ok(__Field::m_floatOffsets),
            "data" => Ok(__Field::m_data),
            "endian" => Ok(__Field::m_endian),
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
pub(super) struct __hkaSplineCompressedAnimationVisitor<'de> {
    marker: core::marker::PhantomData<hkaSplineCompressedAnimation<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaSplineCompressedAnimationVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkaSplineCompressedAnimation<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkaSplineCompressedAnimation<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkaSplineCompressedAnimationVisitor<'de> {
    type Value = hkaSplineCompressedAnimation<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkaSplineCompressedAnimation",
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
        let mut m_numFrames: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numBlocks: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_maxFramesPerBlock: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_maskAndQuantizationSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_blockDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_blockInverseDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_frameDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_blockOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_floatBlockOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_transformOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_floatOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_data: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_endian: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..13usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_numFrames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numFrames",
                            ),
                        );
                    }
                    m_numFrames = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_numBlocks) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numBlocks",
                            ),
                        );
                    }
                    m_numBlocks = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_maxFramesPerBlock) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxFramesPerBlock",
                            ),
                        );
                    }
                    m_maxFramesPerBlock = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_maskAndQuantizationSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maskAndQuantizationSize",
                            ),
                        );
                    }
                    m_maskAndQuantizationSize = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_blockDuration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "blockDuration",
                            ),
                        );
                    }
                    m_blockDuration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_blockInverseDuration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "blockInverseDuration",
                            ),
                        );
                    }
                    m_blockInverseDuration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_frameDuration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "frameDuration",
                            ),
                        );
                    }
                    m_frameDuration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_blockOffsets) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "blockOffsets",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_blockOffsets = _serde::__private::Some(
                        match __A::next_value::<Vec<u32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_floatBlockOffsets) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "floatBlockOffsets",
                            ),
                        );
                    }
                    m_floatBlockOffsets = _serde::__private::Some(
                        match __A::next_value::<Vec<u32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_transformOffsets) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformOffsets",
                            ),
                        );
                    }
                    m_transformOffsets = _serde::__private::Some(
                        match __A::next_value::<Vec<u32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_floatOffsets) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "floatOffsets",
                            ),
                        );
                    }
                    m_floatOffsets = _serde::__private::Some(
                        match __A::next_value::<Vec<u32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_data) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("data"),
                        );
                    }
                    m_data = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_endian) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("endian"),
                        );
                    }
                    m_endian = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_numFrames = match m_numFrames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numFrames"),
                );
            }
        };
        let m_numBlocks = match m_numBlocks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numBlocks"),
                );
            }
        };
        let m_maxFramesPerBlock = match m_maxFramesPerBlock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFramesPerBlock"),
                );
            }
        };
        let m_maskAndQuantizationSize = match m_maskAndQuantizationSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maskAndQuantizationSize",
                    ),
                );
            }
        };
        let m_blockDuration = match m_blockDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockDuration"),
                );
            }
        };
        let m_blockInverseDuration = match m_blockInverseDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "blockInverseDuration",
                    ),
                );
            }
        };
        let m_frameDuration = match m_frameDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("frameDuration"),
                );
            }
        };
        let m_blockOffsets = match m_blockOffsets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockOffsets"),
                );
            }
        };
        let m_floatBlockOffsets = match m_floatBlockOffsets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatBlockOffsets"),
                );
            }
        };
        let m_transformOffsets = match m_transformOffsets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformOffsets"),
                );
            }
        };
        let m_floatOffsets = match m_floatOffsets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatOffsets"),
                );
            }
        };
        let m_data = match m_data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("data"),
                );
            }
        };
        let m_endian = match m_endian {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endian"),
                );
            }
        };
        _serde::__private::Ok(hkaSplineCompressedAnimation {
            __ptr,
            parent,
            m_numFrames,
            m_numBlocks,
            m_maxFramesPerBlock,
            m_maskAndQuantizationSize,
            m_blockDuration,
            m_blockInverseDuration,
            m_frameDuration,
            m_blockOffsets,
            m_floatBlockOffsets,
            m_transformOffsets,
            m_floatOffsets,
            m_data,
            m_endian,
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
        let mut m_numFrames: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numBlocks: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_maxFramesPerBlock: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_maskAndQuantizationSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_blockDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_blockInverseDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_frameDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_blockOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_floatBlockOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_transformOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_floatOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_data: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_endian: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..13usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_numFrames => {
                        if _serde::__private::Option::is_some(&m_numFrames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numFrames",
                                ),
                            );
                        }
                        m_numFrames = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numBlocks => {
                        if _serde::__private::Option::is_some(&m_numBlocks) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numBlocks",
                                ),
                            );
                        }
                        m_numBlocks = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxFramesPerBlock => {
                        if _serde::__private::Option::is_some(&m_maxFramesPerBlock) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxFramesPerBlock",
                                ),
                            );
                        }
                        m_maxFramesPerBlock = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maskAndQuantizationSize => {
                        if _serde::__private::Option::is_some(
                            &m_maskAndQuantizationSize,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maskAndQuantizationSize",
                                ),
                            );
                        }
                        m_maskAndQuantizationSize = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_blockDuration => {
                        if _serde::__private::Option::is_some(&m_blockDuration) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "blockDuration",
                                ),
                            );
                        }
                        m_blockDuration = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_blockInverseDuration => {
                        if _serde::__private::Option::is_some(&m_blockInverseDuration) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "blockInverseDuration",
                                ),
                            );
                        }
                        m_blockInverseDuration = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_frameDuration => {
                        if _serde::__private::Option::is_some(&m_frameDuration) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "frameDuration",
                                ),
                            );
                        }
                        m_frameDuration = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_blockOffsets => {
                        if _serde::__private::Option::is_some(&m_blockOffsets) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "blockOffsets",
                                ),
                            );
                        }
                        m_blockOffsets = _serde::__private::Some(
                            match __A::next_value::<Vec<u32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_floatBlockOffsets => {
                        if _serde::__private::Option::is_some(&m_floatBlockOffsets) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "floatBlockOffsets",
                                ),
                            );
                        }
                        m_floatBlockOffsets = _serde::__private::Some(
                            match __A::next_value::<Vec<u32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_transformOffsets => {
                        if _serde::__private::Option::is_some(&m_transformOffsets) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "transformOffsets",
                                ),
                            );
                        }
                        m_transformOffsets = _serde::__private::Some(
                            match __A::next_value::<Vec<u32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_floatOffsets => {
                        if _serde::__private::Option::is_some(&m_floatOffsets) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "floatOffsets",
                                ),
                            );
                        }
                        m_floatOffsets = _serde::__private::Some(
                            match __A::next_value::<Vec<u32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_data => {
                        if _serde::__private::Option::is_some(&m_data) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("data"),
                            );
                        }
                        m_data = _serde::__private::Some(
                            match __A::next_value::<Vec<u8>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_endian => {
                        if _serde::__private::Option::is_some(&m_endian) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("endian"),
                            );
                        }
                        m_endian = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
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
        let m_numFrames = match m_numFrames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numFrames"),
                );
            }
        };
        let m_numBlocks = match m_numBlocks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numBlocks"),
                );
            }
        };
        let m_maxFramesPerBlock = match m_maxFramesPerBlock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFramesPerBlock"),
                );
            }
        };
        let m_maskAndQuantizationSize = match m_maskAndQuantizationSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maskAndQuantizationSize",
                    ),
                );
            }
        };
        let m_blockDuration = match m_blockDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockDuration"),
                );
            }
        };
        let m_blockInverseDuration = match m_blockInverseDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "blockInverseDuration",
                    ),
                );
            }
        };
        let m_frameDuration = match m_frameDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("frameDuration"),
                );
            }
        };
        let m_blockOffsets = match m_blockOffsets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blockOffsets"),
                );
            }
        };
        let m_floatBlockOffsets = match m_floatBlockOffsets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatBlockOffsets"),
                );
            }
        };
        let m_transformOffsets = match m_transformOffsets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformOffsets"),
                );
            }
        };
        let m_floatOffsets = match m_floatOffsets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatOffsets"),
                );
            }
        };
        let m_data = match m_data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("data"),
                );
            }
        };
        let m_endian = match m_endian {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endian"),
                );
            }
        };
        _serde::__private::Ok(hkaSplineCompressedAnimation {
            __ptr,
            parent,
            m_numFrames,
            m_numBlocks,
            m_maxFramesPerBlock,
            m_maskAndQuantizationSize,
            m_blockDuration,
            m_blockInverseDuration,
            m_frameDuration,
            m_blockOffsets,
            m_floatBlockOffsets,
            m_transformOffsets,
            m_floatOffsets,
            m_data,
            m_endian,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaSplineCompressedAnimation<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "numFrames",
                "numBlocks",
                "maxFramesPerBlock",
                "maskAndQuantizationSize",
                "blockDuration",
                "blockInverseDuration",
                "frameDuration",
                "blockOffsets",
                "floatBlockOffsets",
                "transformOffsets",
                "floatOffsets",
                "data",
                "endian",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaSplineCompressedAnimation",
                FIELDS,
                __hkaSplineCompressedAnimationVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaSplineCompressedAnimation,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
