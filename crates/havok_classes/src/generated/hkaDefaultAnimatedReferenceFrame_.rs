use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaDefaultAnimatedReferenceFrame`
/// -         version: `0`
/// -       signature: `0x6d85e445`
/// -          size:  64(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaDefaultAnimatedReferenceFrame {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkaAnimatedReferenceFrame,
    /// # C++ Info
    /// -          name: `up`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_up: Vector4,
    /// # C++ Info
    /// -          name: `forward`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_forward: Vector4,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `referenceFrameSamples`(ctype: `hkArray<hkVector4>`)
    /// -        offset:  52(x86)/ 56(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_referenceFrameSamples: Vec<Vector4>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaDefaultAnimatedReferenceFrame {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaDefaultAnimatedReferenceFrame"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6d85e445)
        }
    }
    impl _serde::Serialize for hkaDefaultAnimatedReferenceFrame {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6d85e445)));
            let mut serializer = __serializer
                .serialize_struct("hkaDefaultAnimatedReferenceFrame", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer.serialize_field("forward", &self.m_forward)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "referenceFrameSamples",
                    &self.m_referenceFrameSamples,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "referenceFrameSamples",
                    &self.m_referenceFrameSamples,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_up,
    m_forward,
    m_duration,
    m_referenceFrameSamples,
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
            "up" => Ok(__Field::m_up),
            "forward" => Ok(__Field::m_forward),
            "duration" => Ok(__Field::m_duration),
            "referenceFrameSamples" => Ok(__Field::m_referenceFrameSamples),
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
pub(super) struct __hkaDefaultAnimatedReferenceFrameVisitor<'de> {
    marker: core::marker::PhantomData<hkaDefaultAnimatedReferenceFrame>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaDefaultAnimatedReferenceFrameVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkaDefaultAnimatedReferenceFrame, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkaDefaultAnimatedReferenceFrame,
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
impl<'de> _serde::de::Visitor<'de> for __hkaDefaultAnimatedReferenceFrameVisitor<'de> {
    type Value = hkaDefaultAnimatedReferenceFrame;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkaDefaultAnimatedReferenceFrame",
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
        let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_forward: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_referenceFrameSamples: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_up) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("up"),
                        );
                    }
                    m_up = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_forward) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("forward"),
                        );
                    }
                    m_forward = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
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
                3usize => {
                    if _serde::__private::Option::is_some(&m_referenceFrameSamples) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "referenceFrameSamples",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_referenceFrameSamples = _serde::__private::Some(
                        match __A::next_value::<Vec<Vector4>>(&mut __map) {
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
        __A::pad(&mut __map, 0usize, 8usize)?;
        let m_up = match m_up {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("up"),
                );
            }
        };
        let m_forward = match m_forward {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("forward"),
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
        let m_referenceFrameSamples = match m_referenceFrameSamples {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "referenceFrameSamples",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkaDefaultAnimatedReferenceFrame {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_up,
            m_forward,
            m_duration,
            m_referenceFrameSamples,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkaAnimatedReferenceFrameVisitor::visit_as_parent(&mut __map)?;
        let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_forward: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_referenceFrameSamples: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_up => {
                        if _serde::__private::Option::is_some(&m_up) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("up"),
                            );
                        }
                        m_up = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_forward => {
                        if _serde::__private::Option::is_some(&m_forward) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "forward",
                                ),
                            );
                        }
                        m_forward = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
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
                    __Field::m_referenceFrameSamples => {
                        if _serde::__private::Option::is_some(&m_referenceFrameSamples) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "referenceFrameSamples",
                                ),
                            );
                        }
                        m_referenceFrameSamples = _serde::__private::Some(
                            match __A::next_value::<Vec<Vector4>>(&mut __map) {
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
        let m_up = match m_up {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("up"),
                );
            }
        };
        let m_forward = match m_forward {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("forward"),
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
        let m_referenceFrameSamples = match m_referenceFrameSamples {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "referenceFrameSamples",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkaDefaultAnimatedReferenceFrame {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_up,
            m_forward,
            m_duration,
            m_referenceFrameSamples,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaDefaultAnimatedReferenceFrame {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "up",
                "forward",
                "duration",
                "referenceFrameSamples",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaDefaultAnimatedReferenceFrame",
                FIELDS,
                __hkaDefaultAnimatedReferenceFrameVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaDefaultAnimatedReferenceFrame,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
