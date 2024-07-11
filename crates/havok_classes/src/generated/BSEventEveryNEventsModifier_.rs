use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSEventEveryNEventsModifier`
/// -         version: `1`
/// -       signature: `0x6030970c`
/// -          size:  72(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSEventEveryNEventsModifier<'a> {
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
    /// -          name: `eventToCheckFor`(ctype: `struct hkbEventProperty`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_eventToCheckFor: hkbEventProperty,
    /// # C++ Info
    /// -          name: `eventToSend`(ctype: `struct hkbEventProperty`)
    /// -        offset:  52(x86)/ 96(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_eventToSend: hkbEventProperty,
    /// # C++ Info
    /// -          name: `numberOfEventsBeforeSend`(ctype: `hkInt8`)
    /// -        offset:  60(x86)/112(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numberOfEventsBeforeSend: i8,
    /// # C++ Info
    /// -          name: `minimumNumberOfEventsBeforeSend`(ctype: `hkInt8`)
    /// -        offset:  61(x86)/113(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_minimumNumberOfEventsBeforeSend: i8,
    /// # C++ Info
    /// -          name: `randomizeNumberOfEvents`(ctype: `hkBool`)
    /// -        offset:  62(x86)/114(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_randomizeNumberOfEvents: bool,
    /// # C++ Info
    /// -          name: `numberOfEventsSeen`(ctype: `hkInt32`)
    /// -        offset:  64(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numberOfEventsSeen: i32,
    /// # C++ Info
    /// -          name: `calculatedNumberOfEventsBeforeSend`(ctype: `hkInt8`)
    /// -        offset:  68(x86)/120(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_calculatedNumberOfEventsBeforeSend: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSEventEveryNEventsModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSEventEveryNEventsModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6030970c)
        }
    }
    impl<'a> _serde::Serialize for BSEventEveryNEventsModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6030970c)));
            let mut serializer = __serializer
                .serialize_struct("BSEventEveryNEventsModifier", class_meta)?;
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
            serializer.serialize_field("eventToCheckFor", &self.m_eventToCheckFor)?;
            serializer.serialize_field("eventToSend", &self.m_eventToSend)?;
            serializer
                .serialize_field(
                    "numberOfEventsBeforeSend",
                    &self.m_numberOfEventsBeforeSend,
                )?;
            serializer
                .serialize_field(
                    "minimumNumberOfEventsBeforeSend",
                    &self.m_minimumNumberOfEventsBeforeSend,
                )?;
            serializer
                .serialize_field(
                    "randomizeNumberOfEvents",
                    &self.m_randomizeNumberOfEvents,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.skip_field("numberOfEventsSeen", &self.m_numberOfEventsSeen)?;
            serializer
                .skip_field(
                    "calculatedNumberOfEventsBeforeSend",
                    &self.m_calculatedNumberOfEventsBeforeSend,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
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
    m_eventToCheckFor,
    m_eventToSend,
    m_numberOfEventsBeforeSend,
    m_minimumNumberOfEventsBeforeSend,
    m_randomizeNumberOfEvents,
    m_numberOfEventsSeen,
    m_calculatedNumberOfEventsBeforeSend,
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
            "eventToCheckFor" => Ok(__Field::m_eventToCheckFor),
            "eventToSend" => Ok(__Field::m_eventToSend),
            "numberOfEventsBeforeSend" => Ok(__Field::m_numberOfEventsBeforeSend),
            "minimumNumberOfEventsBeforeSend" => {
                Ok(__Field::m_minimumNumberOfEventsBeforeSend)
            }
            "randomizeNumberOfEvents" => Ok(__Field::m_randomizeNumberOfEvents),
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
pub(super) struct __BSEventEveryNEventsModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSEventEveryNEventsModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSEventEveryNEventsModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSEventEveryNEventsModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    BSEventEveryNEventsModifier<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __BSEventEveryNEventsModifierVisitor<'de> {
    type Value = BSEventEveryNEventsModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct BSEventEveryNEventsModifier",
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
        let mut m_eventToCheckFor: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_eventToSend: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_numberOfEventsBeforeSend: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_minimumNumberOfEventsBeforeSend: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_randomizeNumberOfEvents: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_numberOfEventsSeen: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_calculatedNumberOfEventsBeforeSend: _serde::__private::Option<i8> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_eventToCheckFor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "eventToCheckFor",
                            ),
                        );
                    }
                    m_eventToCheckFor = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_eventToSend) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "eventToSend",
                            ),
                        );
                    }
                    m_eventToSend = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_numberOfEventsBeforeSend) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numberOfEventsBeforeSend",
                            ),
                        );
                    }
                    m_numberOfEventsBeforeSend = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(
                        &m_minimumNumberOfEventsBeforeSend,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minimumNumberOfEventsBeforeSend",
                            ),
                        );
                    }
                    m_minimumNumberOfEventsBeforeSend = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_randomizeNumberOfEvents) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "randomizeNumberOfEvents",
                            ),
                        );
                    }
                    m_randomizeNumberOfEvents = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_numberOfEventsSeen) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numberOfEventsSeen",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_numberOfEventsSeen = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(
                        &m_calculatedNumberOfEventsBeforeSend,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "calculatedNumberOfEventsBeforeSend",
                            ),
                        );
                    }
                    m_calculatedNumberOfEventsBeforeSend = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
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
        __A::pad(&mut __map, 3usize, 7usize)?;
        let m_eventToCheckFor = match m_eventToCheckFor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("eventToCheckFor"),
                );
            }
        };
        let m_eventToSend = match m_eventToSend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("eventToSend"),
                );
            }
        };
        let m_numberOfEventsBeforeSend = match m_numberOfEventsBeforeSend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numberOfEventsBeforeSend",
                    ),
                );
            }
        };
        let m_minimumNumberOfEventsBeforeSend = match m_minimumNumberOfEventsBeforeSend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minimumNumberOfEventsBeforeSend",
                    ),
                );
            }
        };
        let m_randomizeNumberOfEvents = match m_randomizeNumberOfEvents {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "randomizeNumberOfEvents",
                    ),
                );
            }
        };
        let m_numberOfEventsSeen = match m_numberOfEventsSeen {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numberOfEventsSeen",
                    ),
                );
            }
        };
        let m_calculatedNumberOfEventsBeforeSend = match m_calculatedNumberOfEventsBeforeSend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "calculatedNumberOfEventsBeforeSend",
                    ),
                );
            }
        };
        _serde::__private::Ok(BSEventEveryNEventsModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_eventToCheckFor,
            m_eventToSend,
            m_numberOfEventsBeforeSend,
            m_minimumNumberOfEventsBeforeSend,
            m_randomizeNumberOfEvents,
            m_numberOfEventsSeen,
            m_calculatedNumberOfEventsBeforeSend,
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
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_eventToCheckFor: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_eventToSend: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_numberOfEventsBeforeSend: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_minimumNumberOfEventsBeforeSend: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_randomizeNumberOfEvents: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..5usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_eventToCheckFor => {
                        if _serde::__private::Option::is_some(&m_eventToCheckFor) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "eventToCheckFor",
                                ),
                            );
                        }
                        m_eventToCheckFor = _serde::__private::Some(
                            match __A::next_value::<hkbEventProperty>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_eventToSend => {
                        if _serde::__private::Option::is_some(&m_eventToSend) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "eventToSend",
                                ),
                            );
                        }
                        m_eventToSend = _serde::__private::Some(
                            match __A::next_value::<hkbEventProperty>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numberOfEventsBeforeSend => {
                        if _serde::__private::Option::is_some(
                            &m_numberOfEventsBeforeSend,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numberOfEventsBeforeSend",
                                ),
                            );
                        }
                        m_numberOfEventsBeforeSend = _serde::__private::Some(
                            match __A::next_value::<i8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_minimumNumberOfEventsBeforeSend => {
                        if _serde::__private::Option::is_some(
                            &m_minimumNumberOfEventsBeforeSend,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "minimumNumberOfEventsBeforeSend",
                                ),
                            );
                        }
                        m_minimumNumberOfEventsBeforeSend = _serde::__private::Some(
                            match __A::next_value::<i8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_randomizeNumberOfEvents => {
                        if _serde::__private::Option::is_some(
                            &m_randomizeNumberOfEvents,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "randomizeNumberOfEvents",
                                ),
                            );
                        }
                        m_randomizeNumberOfEvents = _serde::__private::Some(
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
        let m_eventToCheckFor = match m_eventToCheckFor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("eventToCheckFor"),
                );
            }
        };
        let m_eventToSend = match m_eventToSend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("eventToSend"),
                );
            }
        };
        let m_numberOfEventsBeforeSend = match m_numberOfEventsBeforeSend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numberOfEventsBeforeSend",
                    ),
                );
            }
        };
        let m_minimumNumberOfEventsBeforeSend = match m_minimumNumberOfEventsBeforeSend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minimumNumberOfEventsBeforeSend",
                    ),
                );
            }
        };
        let m_randomizeNumberOfEvents = match m_randomizeNumberOfEvents {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "randomizeNumberOfEvents",
                    ),
                );
            }
        };
        _serde::__private::Ok(BSEventEveryNEventsModifier {
            __ptr,
            parent,
            m_eventToCheckFor,
            m_eventToSend,
            m_numberOfEventsBeforeSend,
            m_minimumNumberOfEventsBeforeSend,
            m_randomizeNumberOfEvents,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSEventEveryNEventsModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "eventToCheckFor",
                "eventToSend",
                "numberOfEventsBeforeSend",
                "minimumNumberOfEventsBeforeSend",
                "randomizeNumberOfEvents",
                "numberOfEventsSeen",
                "calculatedNumberOfEventsBeforeSend",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSEventEveryNEventsModifier",
                FIELDS,
                __BSEventEveryNEventsModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        BSEventEveryNEventsModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
