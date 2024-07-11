use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSTimerModifier`
/// -         version: `0`
/// -       signature: `0x531f3292`
/// -          size:  64(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSTimerModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `alarmTimeSeconds`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_alarmTimeSeconds: f32,
    /// # C++ Info
    /// -          name: `alarmEvent`(ctype: `struct hkbEventProperty`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_alarmEvent: hkbEventProperty,
    /// # C++ Info
    /// -          name: `resetAlarm`(ctype: `hkBool`)
    /// -        offset:  56(x86)/104(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_resetAlarm: bool,
    /// # C++ Info
    /// -          name: `secondsElapsed`(ctype: `hkReal`)
    /// -        offset:  60(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_secondsElapsed: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSTimerModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSTimerModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x531f3292)
        }
    }
    impl<'a> _serde::Serialize for BSTimerModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x531f3292)));
            let mut serializer = __serializer
                .serialize_struct("BSTimerModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("alarmTimeSeconds", &self.m_alarmTimeSeconds)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("alarmEvent", &self.m_alarmEvent)?;
            serializer.serialize_field("resetAlarm", &self.m_resetAlarm)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("secondsElapsed", &self.m_secondsElapsed)?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_alarmTimeSeconds,
    m_alarmEvent,
    m_resetAlarm,
    m_secondsElapsed,
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
            "alarmTimeSeconds" => Ok(__Field::m_alarmTimeSeconds),
            "alarmEvent" => Ok(__Field::m_alarmEvent),
            "resetAlarm" => Ok(__Field::m_resetAlarm),
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
pub(super) struct __BSTimerModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSTimerModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSTimerModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSTimerModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSTimerModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSTimerModifierVisitor<'de> {
    type Value = BSTimerModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSTimerModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_alarmTimeSeconds: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_alarmEvent: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_resetAlarm: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_secondsElapsed: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_alarmTimeSeconds) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "alarmTimeSeconds",
                            ),
                        );
                    }
                    m_alarmTimeSeconds = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_alarmEvent) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "alarmEvent",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_alarmEvent = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_resetAlarm) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "resetAlarm",
                            ),
                        );
                    }
                    m_resetAlarm = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_secondsElapsed) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "secondsElapsed",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_secondsElapsed = _serde::__private::Some(
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
        let m_alarmTimeSeconds = match m_alarmTimeSeconds {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("alarmTimeSeconds"),
                );
            }
        };
        let m_alarmEvent = match m_alarmEvent {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("alarmEvent"),
                );
            }
        };
        let m_resetAlarm = match m_resetAlarm {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("resetAlarm"),
                );
            }
        };
        let m_secondsElapsed = match m_secondsElapsed {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("secondsElapsed"),
                );
            }
        };
        _serde::__private::Ok(BSTimerModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_alarmTimeSeconds,
            m_alarmEvent,
            m_resetAlarm,
            m_secondsElapsed,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_alarmTimeSeconds: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_alarmEvent: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_resetAlarm: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_alarmTimeSeconds => {
                        if _serde::__private::Option::is_some(&m_alarmTimeSeconds) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "alarmTimeSeconds",
                                ),
                            );
                        }
                        m_alarmTimeSeconds = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_alarmEvent => {
                        if _serde::__private::Option::is_some(&m_alarmEvent) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "alarmEvent",
                                ),
                            );
                        }
                        m_alarmEvent = _serde::__private::Some(
                            match __A::next_value::<hkbEventProperty>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_resetAlarm => {
                        if _serde::__private::Option::is_some(&m_resetAlarm) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "resetAlarm",
                                ),
                            );
                        }
                        m_resetAlarm = _serde::__private::Some(
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
        let m_alarmTimeSeconds = match m_alarmTimeSeconds {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("alarmTimeSeconds"),
                );
            }
        };
        let m_alarmEvent = match m_alarmEvent {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("alarmEvent"),
                );
            }
        };
        let m_resetAlarm = match m_resetAlarm {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("resetAlarm"),
                );
            }
        };
        _serde::__private::Ok(BSTimerModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_alarmTimeSeconds,
            m_alarmEvent,
            m_resetAlarm,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSTimerModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "alarmTimeSeconds",
                "alarmEvent",
                "resetAlarm",
                "secondsElapsed",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSTimerModifier",
                FIELDS,
                __BSTimerModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSTimerModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
