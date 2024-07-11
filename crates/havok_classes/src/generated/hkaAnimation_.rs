use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaAnimation`
/// -         version: `1`
/// -       signature: `0xa6fa7e88`
/// -          size:  40(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnimation<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum AnimationType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_type: AnimationType,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `numberOfTransformTracks`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numberOfTransformTracks: i32,
    /// # C++ Info
    /// -          name: `numberOfFloatTracks`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numberOfFloatTracks: i32,
    /// # C++ Info
    /// -          name: `extractedMotion`(ctype: `struct hkaAnimatedReferenceFrame*`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_extractedMotion: Pointer,
    /// # C++ Info
    /// -          name: `annotationTracks`(ctype: `hkArray<struct hkaAnnotationTrack>`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_annotationTracks: Vec<hkaAnnotationTrack<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaAnimation<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaAnimation"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa6fa7e88)
        }
    }
    impl<'a> _serde::Serialize for hkaAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa6fa7e88)));
            let mut serializer = __serializer
                .serialize_struct("hkaAnimation", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer
                .serialize_field(
                    "numberOfTransformTracks",
                    &self.m_numberOfTransformTracks,
                )?;
            serializer
                .serialize_field("numberOfFloatTracks", &self.m_numberOfFloatTracks)?;
            serializer.serialize_field("extractedMotion", &self.m_extractedMotion)?;
            serializer
                .serialize_array_meta_field(
                    "annotationTracks",
                    &self.m_annotationTracks,
                )?;
            serializer
                .serialize_array_field("annotationTracks", &self.m_annotationTracks)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_type,
    m_duration,
    m_numberOfTransformTracks,
    m_numberOfFloatTracks,
    m_extractedMotion,
    m_annotationTracks,
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
            "type" => Ok(__Field::m_type),
            "duration" => Ok(__Field::m_duration),
            "numberOfTransformTracks" => Ok(__Field::m_numberOfTransformTracks),
            "numberOfFloatTracks" => Ok(__Field::m_numberOfFloatTracks),
            "extractedMotion" => Ok(__Field::m_extractedMotion),
            "annotationTracks" => Ok(__Field::m_annotationTracks),
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
pub(super) struct __hkaAnimationVisitor<'de> {
    marker: core::marker::PhantomData<hkaAnimation<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaAnimationVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkaAnimation<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkaAnimation<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkaAnimationVisitor<'de> {
    type Value = hkaAnimation<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkaAnimation")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_type: _serde::__private::Option<AnimationType> = _serde::__private::None;
        let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_numberOfTransformTracks: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numberOfFloatTracks: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_extractedMotion: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_annotationTracks: _serde::__private::Option<
            Vec<hkaAnnotationTrack<'de>>,
        > = _serde::__private::None;
        for i in 0..6usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_type) {
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
                1usize => {
                    if _serde::__private::Option::is_some(&m_duration) {
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
                2usize => {
                    if _serde::__private::Option::is_some(&m_numberOfTransformTracks) {
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
                3usize => {
                    if _serde::__private::Option::is_some(&m_numberOfFloatTracks) {
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
                4usize => {
                    if _serde::__private::Option::is_some(&m_extractedMotion) {
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
                5usize => {
                    if _serde::__private::Option::is_some(&m_annotationTracks) {
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
                _ => {}
            }
        }
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_duration = match m_duration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("duration"),
                );
            }
        };
        let m_numberOfTransformTracks = match m_numberOfTransformTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numberOfTransformTracks",
                    ),
                );
            }
        };
        let m_numberOfFloatTracks = match m_numberOfFloatTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numberOfFloatTracks",
                    ),
                );
            }
        };
        let m_extractedMotion = match m_extractedMotion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("extractedMotion"),
                );
            }
        };
        let m_annotationTracks = match m_annotationTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("annotationTracks"),
                );
            }
        };
        _serde::__private::Ok(hkaAnimation {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_type,
            m_duration,
            m_numberOfTransformTracks,
            m_numberOfFloatTracks,
            m_extractedMotion,
            m_annotationTracks,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_type: _serde::__private::Option<AnimationType> = _serde::__private::None;
        let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_numberOfTransformTracks: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numberOfFloatTracks: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_extractedMotion: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_annotationTracks: _serde::__private::Option<
            Vec<hkaAnnotationTrack<'de>>,
        > = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_type => {
                        if _serde::__private::Option::is_some(&m_type) {
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
                        if _serde::__private::Option::is_some(&m_duration) {
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
                        if _serde::__private::Option::is_some(
                            &m_numberOfTransformTracks,
                        ) {
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
                        if _serde::__private::Option::is_some(&m_numberOfFloatTracks) {
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
                        if _serde::__private::Option::is_some(&m_extractedMotion) {
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
                        if _serde::__private::Option::is_some(&m_annotationTracks) {
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
                    _ => {}
                }
            }
        }
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_duration = match m_duration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("duration"),
                );
            }
        };
        let m_numberOfTransformTracks = match m_numberOfTransformTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numberOfTransformTracks",
                    ),
                );
            }
        };
        let m_numberOfFloatTracks = match m_numberOfFloatTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numberOfFloatTracks",
                    ),
                );
            }
        };
        let m_extractedMotion = match m_extractedMotion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("extractedMotion"),
                );
            }
        };
        let m_annotationTracks = match m_annotationTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("annotationTracks"),
                );
            }
        };
        _serde::__private::Ok(hkaAnimation {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_type,
            m_duration,
            m_numberOfTransformTracks,
            m_numberOfFloatTracks,
            m_extractedMotion,
            m_annotationTracks,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaAnimation<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "type",
                "duration",
                "numberOfTransformTracks",
                "numberOfFloatTracks",
                "extractedMotion",
                "annotationTracks",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaAnimation",
                FIELDS,
                __hkaAnimationVisitor {
                    marker: _serde::__private::PhantomData::<hkaAnimation>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT32`
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
pub enum AnimationType {
    #[default]
    HK_UNKNOWN_ANIMATION = 0isize,
    HK_INTERLEAVED_ANIMATION = 1isize,
    HK_DELTA_COMPRESSED_ANIMATION = 2isize,
    HK_WAVELET_COMPRESSED_ANIMATION = 3isize,
    HK_MIRRORED_ANIMATION = 4isize,
    HK_SPLINE_COMPRESSED_ANIMATION = 5isize,
    HK_QUANTIZED_COMPRESSED_ANIMATION = 6isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for AnimationType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HK_UNKNOWN_ANIMATION => {
                    __serializer.serialize_field("HK_UNKNOWN_ANIMATION", &0u64)
                }
                Self::HK_INTERLEAVED_ANIMATION => {
                    __serializer.serialize_field("HK_INTERLEAVED_ANIMATION", &1u64)
                }
                Self::HK_DELTA_COMPRESSED_ANIMATION => {
                    __serializer.serialize_field("HK_DELTA_COMPRESSED_ANIMATION", &2u64)
                }
                Self::HK_WAVELET_COMPRESSED_ANIMATION => {
                    __serializer
                        .serialize_field("HK_WAVELET_COMPRESSED_ANIMATION", &3u64)
                }
                Self::HK_MIRRORED_ANIMATION => {
                    __serializer.serialize_field("HK_MIRRORED_ANIMATION", &4u64)
                }
                Self::HK_SPLINE_COMPRESSED_ANIMATION => {
                    __serializer.serialize_field("HK_SPLINE_COMPRESSED_ANIMATION", &5u64)
                }
                Self::HK_QUANTIZED_COMPRESSED_ANIMATION => {
                    __serializer
                        .serialize_field("HK_QUANTIZED_COMPRESSED_ANIMATION", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i32()
                .ok_or(S::Error::custom("Failed enum AnimationType to_i32"))?;
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
    impl<'de> _serde::Deserialize<'de> for AnimationType {
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
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
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
                fn visit_int32<__E>(
                    self,
                    __value: i32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i32 => _serde::__private::Ok(__Field::__field0),
                        1i32 => _serde::__private::Ok(__Field::__field1),
                        2i32 => _serde::__private::Ok(__Field::__field2),
                        3i32 => _serde::__private::Ok(__Field::__field3),
                        4i32 => _serde::__private::Ok(__Field::__field4),
                        5i32 => _serde::__private::Ok(__Field::__field5),
                        6i32 => _serde::__private::Ok(__Field::__field6),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int32(__value),
                                    &"value(i32) of variant is one of 0, 1, 2, 3, 4, 5, 6",
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
                                || v.eq_ignore_ascii_case("HK_UNKNOWN_ANIMATION") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("HK_INTERLEAVED_ANIMATION") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case("HK_DELTA_COMPRESSED_ANIMATION") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case("HK_WAVELET_COMPRESSED_ANIMATION") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("HK_MIRRORED_ANIMATION") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v
                                    .eq_ignore_ascii_case("HK_SPLINE_COMPRESSED_ANIMATION") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v
                                    .eq_ignore_ascii_case(
                                        "HK_QUANTIZED_COMPRESSED_ANIMATION",
                                    ) => _serde::__private::Ok(__Field::__field6),
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
                        _serde::de::ReadEnumSize::Int32,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<AnimationType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AnimationType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum AnimationType",
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
                            _serde::__private::Ok(AnimationType::HK_UNKNOWN_ANIMATION)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AnimationType::HK_INTERLEAVED_ANIMATION,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AnimationType::HK_DELTA_COMPRESSED_ANIMATION,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AnimationType::HK_WAVELET_COMPRESSED_ANIMATION,
                            )
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AnimationType::HK_MIRRORED_ANIMATION)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AnimationType::HK_SPLINE_COMPRESSED_ANIMATION,
                            )
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AnimationType::HK_QUANTIZED_COMPRESSED_ANIMATION,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HK_UNKNOWN_ANIMATION",
                "HK_INTERLEAVED_ANIMATION",
                "HK_DELTA_COMPRESSED_ANIMATION",
                "HK_WAVELET_COMPRESSED_ANIMATION",
                "HK_MIRRORED_ANIMATION",
                "HK_SPLINE_COMPRESSED_ANIMATION",
                "HK_QUANTIZED_COMPRESSED_ANIMATION",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "AnimationType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AnimationType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
