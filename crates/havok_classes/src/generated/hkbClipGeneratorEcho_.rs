use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbClipGeneratorEcho`
/// -         version: `0`
/// -       signature: `0x750edf40`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbClipGeneratorEcho {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `offsetLocalTime`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_offsetLocalTime: f32,
    /// # C++ Info
    /// -          name: `weight`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_weight: f32,
    /// # C++ Info
    /// -          name: `dwdt`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dwdt: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbClipGeneratorEcho {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbClipGeneratorEcho"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x750edf40)
        }
    }
    impl _serde::Serialize for hkbClipGeneratorEcho {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x750edf40)));
            let mut serializer = __serializer
                .serialize_struct("hkbClipGeneratorEcho", class_meta)?;
            serializer.serialize_field("offsetLocalTime", &self.m_offsetLocalTime)?;
            serializer.serialize_field("weight", &self.m_weight)?;
            serializer.serialize_field("dwdt", &self.m_dwdt)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_offsetLocalTime,
    m_weight,
    m_dwdt,
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
            "offsetLocalTime" => Ok(__Field::m_offsetLocalTime),
            "weight" => Ok(__Field::m_weight),
            "dwdt" => Ok(__Field::m_dwdt),
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
pub(super) struct __hkbClipGeneratorEchoVisitor<'de> {
    marker: core::marker::PhantomData<hkbClipGeneratorEcho>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbClipGeneratorEchoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbClipGeneratorEcho, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbClipGeneratorEcho>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbClipGeneratorEchoVisitor<'de> {
    type Value = hkbClipGeneratorEcho;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbClipGeneratorEcho")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_offsetLocalTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_weight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_dwdt: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_offsetLocalTime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "offsetLocalTime",
                            ),
                        );
                    }
                    m_offsetLocalTime = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_weight) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("weight"),
                        );
                    }
                    m_weight = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_dwdt) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("dwdt"),
                        );
                    }
                    m_dwdt = _serde::__private::Some(
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
        __A::pad(&mut __map, 4usize, 4usize)?;
        let m_offsetLocalTime = match m_offsetLocalTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offsetLocalTime"),
                );
            }
        };
        let m_weight = match m_weight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weight"),
                );
            }
        };
        let m_dwdt = match m_dwdt {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dwdt"),
                );
            }
        };
        _serde::__private::Ok(hkbClipGeneratorEcho {
            __ptr: __A::class_ptr(&mut __map),
            m_offsetLocalTime,
            m_weight,
            m_dwdt,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_offsetLocalTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_weight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_dwdt: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_offsetLocalTime => {
                        if _serde::__private::Option::is_some(&m_offsetLocalTime) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "offsetLocalTime",
                                ),
                            );
                        }
                        m_offsetLocalTime = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_weight => {
                        if _serde::__private::Option::is_some(&m_weight) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("weight"),
                            );
                        }
                        m_weight = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_dwdt => {
                        if _serde::__private::Option::is_some(&m_dwdt) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("dwdt"),
                            );
                        }
                        m_dwdt = _serde::__private::Some(
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
        }
        let m_offsetLocalTime = match m_offsetLocalTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offsetLocalTime"),
                );
            }
        };
        let m_weight = match m_weight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weight"),
                );
            }
        };
        let m_dwdt = match m_dwdt {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dwdt"),
                );
            }
        };
        _serde::__private::Ok(hkbClipGeneratorEcho {
            __ptr: __A::class_ptr(&mut __map),
            m_offsetLocalTime,
            m_weight,
            m_dwdt,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbClipGeneratorEcho {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["offsetLocalTime", "weight", "dwdt"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbClipGeneratorEcho",
                FIELDS,
                __hkbClipGeneratorEchoVisitor {
                    marker: _serde::__private::PhantomData::<hkbClipGeneratorEcho>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
