use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbClipTrigger`
/// - version: `1`
/// - signature: `0x7eb45cea`
/// - size: ` 16`(x86)/` 32`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbClipTrigger {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `localTime`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_localTime: f32,
    /// # C++ Info
    /// - name: `event`(ctype: `struct hkbEventProperty`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    pub m_event: hkbEventProperty,
    /// # C++ Info
    /// - name: `relativeToEndOfClip`(ctype: `hkBool`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_relativeToEndOfClip: bool,
    /// # C++ Info
    /// - name: `acyclic`(ctype: `hkBool`)
    /// - offset: ` 13`(x86)/` 25`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_acyclic: bool,
    /// # C++ Info
    /// - name: `isAnnotation`(ctype: `hkBool`)
    /// - offset: ` 14`(x86)/` 26`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isAnnotation: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbClipTrigger {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbClipTrigger"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7eb45cea)
        }
    }
    impl _serde::Serialize for hkbClipTrigger {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7eb45cea)));
            let mut serializer = __serializer
                .serialize_struct("hkbClipTrigger", class_meta)?;
            serializer.serialize_field("localTime", &self.m_localTime)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("event", &self.m_event)?;
            serializer
                .serialize_field("relativeToEndOfClip", &self.m_relativeToEndOfClip)?;
            serializer.serialize_field("acyclic", &self.m_acyclic)?;
            serializer.serialize_field("isAnnotation", &self.m_isAnnotation)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_localTime,
    m_event,
    m_relativeToEndOfClip,
    m_acyclic,
    m_isAnnotation,
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
            "localTime" => Ok(__Field::m_localTime),
            "event" => Ok(__Field::m_event),
            "relativeToEndOfClip" => Ok(__Field::m_relativeToEndOfClip),
            "acyclic" => Ok(__Field::m_acyclic),
            "isAnnotation" => Ok(__Field::m_isAnnotation),
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
pub(super) struct __hkbClipTriggerVisitor<'de> {
    marker: core::marker::PhantomData<hkbClipTrigger>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbClipTriggerVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbClipTrigger, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbClipTrigger>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbClipTriggerVisitor<'de> {
    type Value = hkbClipTrigger;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbClipTrigger")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_localTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_event: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_relativeToEndOfClip: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_acyclic: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isAnnotation: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
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
                1usize => {
                    if _serde::__private::Option::is_some(&m_event) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("event"),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_event = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_relativeToEndOfClip) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "relativeToEndOfClip",
                            ),
                        );
                    }
                    m_relativeToEndOfClip = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_acyclic) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("acyclic"),
                        );
                    }
                    m_acyclic = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_isAnnotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isAnnotation",
                            ),
                        );
                    }
                    m_isAnnotation = _serde::__private::Some(
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
        __A::pad(&mut __map, 1usize, 5usize)?;
        let m_localTime = match m_localTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localTime"),
                );
            }
        };
        let m_event = match m_event {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("event"),
                );
            }
        };
        let m_relativeToEndOfClip = match m_relativeToEndOfClip {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "relativeToEndOfClip",
                    ),
                );
            }
        };
        let m_acyclic = match m_acyclic {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("acyclic"),
                );
            }
        };
        let m_isAnnotation = match m_isAnnotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isAnnotation"),
                );
            }
        };
        _serde::__private::Ok(hkbClipTrigger {
            __ptr,
            m_localTime,
            m_event,
            m_relativeToEndOfClip,
            m_acyclic,
            m_isAnnotation,
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
        let mut m_localTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_event: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_relativeToEndOfClip: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_acyclic: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isAnnotation: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..5usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
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
                    __Field::m_event => {
                        if _serde::__private::Option::is_some(&m_event) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("event"),
                            );
                        }
                        m_event = _serde::__private::Some(
                            match __A::next_value::<hkbEventProperty>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_relativeToEndOfClip => {
                        if _serde::__private::Option::is_some(&m_relativeToEndOfClip) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "relativeToEndOfClip",
                                ),
                            );
                        }
                        m_relativeToEndOfClip = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_acyclic => {
                        if _serde::__private::Option::is_some(&m_acyclic) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "acyclic",
                                ),
                            );
                        }
                        m_acyclic = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_isAnnotation => {
                        if _serde::__private::Option::is_some(&m_isAnnotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isAnnotation",
                                ),
                            );
                        }
                        m_isAnnotation = _serde::__private::Some(
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
        let m_localTime = match m_localTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localTime"),
                );
            }
        };
        let m_event = match m_event {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("event"),
                );
            }
        };
        let m_relativeToEndOfClip = match m_relativeToEndOfClip {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "relativeToEndOfClip",
                    ),
                );
            }
        };
        let m_acyclic = match m_acyclic {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("acyclic"),
                );
            }
        };
        let m_isAnnotation = match m_isAnnotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isAnnotation"),
                );
            }
        };
        _serde::__private::Ok(hkbClipTrigger {
            __ptr,
            m_localTime,
            m_event,
            m_relativeToEndOfClip,
            m_acyclic,
            m_isAnnotation,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbClipTrigger {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "localTime",
                "event",
                "relativeToEndOfClip",
                "acyclic",
                "isAnnotation",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbClipTrigger",
                FIELDS,
                __hkbClipTriggerVisitor {
                    marker: _serde::__private::PhantomData::<hkbClipTrigger>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
