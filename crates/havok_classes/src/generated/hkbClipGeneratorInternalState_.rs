use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbClipGeneratorInternalState`
/// -         version: `0`
/// -       signature: `0x26ce5bf3`
/// -          size: 112(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbClipGeneratorInternalState {
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
    /// -          name: `extractedMotion`(ctype: `hkQsTransform`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_extractedMotion: QsTransform,
    /// # C++ Info
    /// -          name: `echos`(ctype: `hkArray<struct hkbClipGeneratorEcho>`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_echos: Vec<hkbClipGeneratorEcho>,
    /// # C++ Info
    /// -          name: `localTime`(ctype: `hkReal`)
    /// -        offset:  76(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_localTime: f32,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset:  80(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_time: f32,
    /// # C++ Info
    /// -          name: `previousUserControlledTimeFraction`(ctype: `hkReal`)
    /// -        offset:  84(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_previousUserControlledTimeFraction: f32,
    /// # C++ Info
    /// -          name: `bufferSize`(ctype: `hkInt32`)
    /// -        offset:  88(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bufferSize: i32,
    /// # C++ Info
    /// -          name: `echoBufferSize`(ctype: `hkInt32`)
    /// -        offset:  92(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_echoBufferSize: i32,
    /// # C++ Info
    /// -          name: `atEnd`(ctype: `hkBool`)
    /// -        offset:  96(x86)/100(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_atEnd: bool,
    /// # C++ Info
    /// -          name: `ignoreStartTime`(ctype: `hkBool`)
    /// -        offset:  97(x86)/101(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_ignoreStartTime: bool,
    /// # C++ Info
    /// -          name: `pingPongBackward`(ctype: `hkBool`)
    /// -        offset:  98(x86)/102(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_pingPongBackward: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbClipGeneratorInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbClipGeneratorInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x26ce5bf3)
        }
    }
    impl _serde::Serialize for hkbClipGeneratorInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x26ce5bf3)));
            let mut serializer = __serializer
                .serialize_struct("hkbClipGeneratorInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("extractedMotion", &self.m_extractedMotion)?;
            serializer.serialize_array_meta_field("echos", &self.m_echos)?;
            serializer.serialize_field("localTime", &self.m_localTime)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer
                .serialize_field(
                    "previousUserControlledTimeFraction",
                    &self.m_previousUserControlledTimeFraction,
                )?;
            serializer.serialize_field("bufferSize", &self.m_bufferSize)?;
            serializer.serialize_field("echoBufferSize", &self.m_echoBufferSize)?;
            serializer.serialize_field("atEnd", &self.m_atEnd)?;
            serializer.serialize_field("ignoreStartTime", &self.m_ignoreStartTime)?;
            serializer.serialize_field("pingPongBackward", &self.m_pingPongBackward)?;
            serializer.pad_field([0u8; 13usize].as_slice(), [0u8; 9usize].as_slice())?;
            serializer.serialize_array_field("echos", &self.m_echos)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_extractedMotion,
    m_echos,
    m_localTime,
    m_time,
    m_previousUserControlledTimeFraction,
    m_bufferSize,
    m_echoBufferSize,
    m_atEnd,
    m_ignoreStartTime,
    m_pingPongBackward,
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
            "extractedMotion" => Ok(__Field::m_extractedMotion),
            "echos" => Ok(__Field::m_echos),
            "localTime" => Ok(__Field::m_localTime),
            "time" => Ok(__Field::m_time),
            "previousUserControlledTimeFraction" => {
                Ok(__Field::m_previousUserControlledTimeFraction)
            }
            "bufferSize" => Ok(__Field::m_bufferSize),
            "echoBufferSize" => Ok(__Field::m_echoBufferSize),
            "atEnd" => Ok(__Field::m_atEnd),
            "ignoreStartTime" => Ok(__Field::m_ignoreStartTime),
            "pingPongBackward" => Ok(__Field::m_pingPongBackward),
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
pub(super) struct __hkbClipGeneratorInternalStateVisitor<'de> {
    marker: core::marker::PhantomData<hkbClipGeneratorInternalState>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbClipGeneratorInternalStateVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbClipGeneratorInternalState, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbClipGeneratorInternalState>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbClipGeneratorInternalStateVisitor<'de> {
    type Value = hkbClipGeneratorInternalState;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbClipGeneratorInternalState",
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
        let parent = __A::next_value(&mut __map)?;
        let mut m_extractedMotion: _serde::__private::Option<QsTransform> = _serde::__private::None;
        let mut m_echos: _serde::__private::Option<Vec<hkbClipGeneratorEcho>> = _serde::__private::None;
        let mut m_localTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_time: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_previousUserControlledTimeFraction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_bufferSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_echoBufferSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_atEnd: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_ignoreStartTime: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_pingPongBackward: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..10usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_extractedMotion) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "extractedMotion",
                            ),
                        );
                    }
                    m_extractedMotion = _serde::__private::Some(
                        match __A::next_value::<QsTransform>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_echos) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("echos"),
                        );
                    }
                    m_echos = _serde::__private::Some(
                        match __A::next_value::<Vec<hkbClipGeneratorEcho>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_localTime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "localTime",
                            ),
                        );
                    }
                    m_localTime = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_time) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("time"),
                        );
                    }
                    m_time = _serde::__private::Some(
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
                        &m_previousUserControlledTimeFraction,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "previousUserControlledTimeFraction",
                            ),
                        );
                    }
                    m_previousUserControlledTimeFraction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_bufferSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bufferSize",
                            ),
                        );
                    }
                    m_bufferSize = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_echoBufferSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "echoBufferSize",
                            ),
                        );
                    }
                    m_echoBufferSize = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_atEnd) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("atEnd"),
                        );
                    }
                    m_atEnd = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_ignoreStartTime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ignoreStartTime",
                            ),
                        );
                    }
                    m_ignoreStartTime = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_pingPongBackward) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pingPongBackward",
                            ),
                        );
                    }
                    m_pingPongBackward = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
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
        __A::pad(&mut __map, 13usize, 9usize)?;
        let m_extractedMotion = match m_extractedMotion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("extractedMotion"),
                );
            }
        };
        let m_echos = match m_echos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("echos"),
                );
            }
        };
        let m_localTime = match m_localTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localTime"),
                );
            }
        };
        let m_time = match m_time {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("time"),
                );
            }
        };
        let m_previousUserControlledTimeFraction = match m_previousUserControlledTimeFraction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "previousUserControlledTimeFraction",
                    ),
                );
            }
        };
        let m_bufferSize = match m_bufferSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bufferSize"),
                );
            }
        };
        let m_echoBufferSize = match m_echoBufferSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("echoBufferSize"),
                );
            }
        };
        let m_atEnd = match m_atEnd {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("atEnd"),
                );
            }
        };
        let m_ignoreStartTime = match m_ignoreStartTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ignoreStartTime"),
                );
            }
        };
        let m_pingPongBackward = match m_pingPongBackward {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pingPongBackward"),
                );
            }
        };
        _serde::__private::Ok(hkbClipGeneratorInternalState {
            __ptr,
            parent,
            m_extractedMotion,
            m_echos,
            m_localTime,
            m_time,
            m_previousUserControlledTimeFraction,
            m_bufferSize,
            m_echoBufferSize,
            m_atEnd,
            m_ignoreStartTime,
            m_pingPongBackward,
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
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_extractedMotion: _serde::__private::Option<QsTransform> = _serde::__private::None;
        let mut m_echos: _serde::__private::Option<Vec<hkbClipGeneratorEcho>> = _serde::__private::None;
        let mut m_localTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_time: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_previousUserControlledTimeFraction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_bufferSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_echoBufferSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_atEnd: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_ignoreStartTime: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_pingPongBackward: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..10usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_extractedMotion => {
                        if _serde::__private::Option::is_some(&m_extractedMotion) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "extractedMotion",
                                ),
                            );
                        }
                        m_extractedMotion = _serde::__private::Some(
                            match __A::next_value::<QsTransform>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_echos => {
                        if _serde::__private::Option::is_some(&m_echos) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("echos"),
                            );
                        }
                        m_echos = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkbClipGeneratorEcho>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_localTime => {
                        if _serde::__private::Option::is_some(&m_localTime) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "localTime",
                                ),
                            );
                        }
                        m_localTime = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_time => {
                        if _serde::__private::Option::is_some(&m_time) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("time"),
                            );
                        }
                        m_time = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_previousUserControlledTimeFraction => {
                        if _serde::__private::Option::is_some(
                            &m_previousUserControlledTimeFraction,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "previousUserControlledTimeFraction",
                                ),
                            );
                        }
                        m_previousUserControlledTimeFraction = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bufferSize => {
                        if _serde::__private::Option::is_some(&m_bufferSize) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bufferSize",
                                ),
                            );
                        }
                        m_bufferSize = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_echoBufferSize => {
                        if _serde::__private::Option::is_some(&m_echoBufferSize) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "echoBufferSize",
                                ),
                            );
                        }
                        m_echoBufferSize = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_atEnd => {
                        if _serde::__private::Option::is_some(&m_atEnd) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("atEnd"),
                            );
                        }
                        m_atEnd = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_ignoreStartTime => {
                        if _serde::__private::Option::is_some(&m_ignoreStartTime) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ignoreStartTime",
                                ),
                            );
                        }
                        m_ignoreStartTime = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_pingPongBackward => {
                        if _serde::__private::Option::is_some(&m_pingPongBackward) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pingPongBackward",
                                ),
                            );
                        }
                        m_pingPongBackward = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
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
        let m_extractedMotion = match m_extractedMotion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("extractedMotion"),
                );
            }
        };
        let m_echos = match m_echos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("echos"),
                );
            }
        };
        let m_localTime = match m_localTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localTime"),
                );
            }
        };
        let m_time = match m_time {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("time"),
                );
            }
        };
        let m_previousUserControlledTimeFraction = match m_previousUserControlledTimeFraction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "previousUserControlledTimeFraction",
                    ),
                );
            }
        };
        let m_bufferSize = match m_bufferSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bufferSize"),
                );
            }
        };
        let m_echoBufferSize = match m_echoBufferSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("echoBufferSize"),
                );
            }
        };
        let m_atEnd = match m_atEnd {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("atEnd"),
                );
            }
        };
        let m_ignoreStartTime = match m_ignoreStartTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ignoreStartTime"),
                );
            }
        };
        let m_pingPongBackward = match m_pingPongBackward {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pingPongBackward"),
                );
            }
        };
        _serde::__private::Ok(hkbClipGeneratorInternalState {
            __ptr,
            parent,
            m_extractedMotion,
            m_echos,
            m_localTime,
            m_time,
            m_previousUserControlledTimeFraction,
            m_bufferSize,
            m_echoBufferSize,
            m_atEnd,
            m_ignoreStartTime,
            m_pingPongBackward,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbClipGeneratorInternalState {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "extractedMotion",
                "echos",
                "localTime",
                "time",
                "previousUserControlledTimeFraction",
                "bufferSize",
                "echoBufferSize",
                "atEnd",
                "ignoreStartTime",
                "pingPongBackward",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbClipGeneratorInternalState",
                FIELDS,
                __hkbClipGeneratorInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbClipGeneratorInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
