use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSEventOnFalseToTrueModifier`
/// -         version: `1`
/// -       signature: `0x81d0777a`
/// -          size:  84(x86)/160(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSEventOnFalseToTrueModifier<'a> {
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
    /// -          name: `bEnableEvent1`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bEnableEvent1: bool,
    /// # C++ Info
    /// -          name: `bVariableToTest1`(ctype: `hkBool`)
    /// -        offset:  45(x86)/ 81(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bVariableToTest1: bool,
    /// # C++ Info
    /// -          name: `EventToSend1`(ctype: `struct hkbEventProperty`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_EventToSend1: hkbEventProperty,
    /// # C++ Info
    /// -          name: `bEnableEvent2`(ctype: `hkBool`)
    /// -        offset:  56(x86)/104(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bEnableEvent2: bool,
    /// # C++ Info
    /// -          name: `bVariableToTest2`(ctype: `hkBool`)
    /// -        offset:  57(x86)/105(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bVariableToTest2: bool,
    /// # C++ Info
    /// -          name: `EventToSend2`(ctype: `struct hkbEventProperty`)
    /// -        offset:  60(x86)/112(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_EventToSend2: hkbEventProperty,
    /// # C++ Info
    /// -          name: `bEnableEvent3`(ctype: `hkBool`)
    /// -        offset:  68(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bEnableEvent3: bool,
    /// # C++ Info
    /// -          name: `bVariableToTest3`(ctype: `hkBool`)
    /// -        offset:  69(x86)/129(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bVariableToTest3: bool,
    /// # C++ Info
    /// -          name: `EventToSend3`(ctype: `struct hkbEventProperty`)
    /// -        offset:  72(x86)/136(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_EventToSend3: hkbEventProperty,
    /// # C++ Info
    /// -          name: `bSlot1ActivatedLastFrame`(ctype: `hkBool`)
    /// -        offset:  80(x86)/152(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bSlot1ActivatedLastFrame: bool,
    /// # C++ Info
    /// -          name: `bSlot2ActivatedLastFrame`(ctype: `hkBool`)
    /// -        offset:  81(x86)/153(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bSlot2ActivatedLastFrame: bool,
    /// # C++ Info
    /// -          name: `bSlot3ActivatedLastFrame`(ctype: `hkBool`)
    /// -        offset:  82(x86)/154(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bSlot3ActivatedLastFrame: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSEventOnFalseToTrueModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSEventOnFalseToTrueModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x81d0777a)
        }
    }
    impl<'a> _serde::Serialize for BSEventOnFalseToTrueModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x81d0777a)));
            let mut serializer = __serializer
                .serialize_struct("BSEventOnFalseToTrueModifier", class_meta)?;
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
            serializer.serialize_field("bEnableEvent1", &self.m_bEnableEvent1)?;
            serializer.serialize_field("bVariableToTest1", &self.m_bVariableToTest1)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("EventToSend1", &self.m_EventToSend1)?;
            serializer.serialize_field("bEnableEvent2", &self.m_bEnableEvent2)?;
            serializer.serialize_field("bVariableToTest2", &self.m_bVariableToTest2)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("EventToSend2", &self.m_EventToSend2)?;
            serializer.serialize_field("bEnableEvent3", &self.m_bEnableEvent3)?;
            serializer.serialize_field("bVariableToTest3", &self.m_bVariableToTest3)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("EventToSend3", &self.m_EventToSend3)?;
            serializer
                .skip_field(
                    "bSlot1ActivatedLastFrame",
                    &self.m_bSlot1ActivatedLastFrame,
                )?;
            serializer
                .skip_field(
                    "bSlot2ActivatedLastFrame",
                    &self.m_bSlot2ActivatedLastFrame,
                )?;
            serializer
                .skip_field(
                    "bSlot3ActivatedLastFrame",
                    &self.m_bSlot3ActivatedLastFrame,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
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
    m_bEnableEvent1,
    m_bVariableToTest1,
    m_EventToSend1,
    m_bEnableEvent2,
    m_bVariableToTest2,
    m_EventToSend2,
    m_bEnableEvent3,
    m_bVariableToTest3,
    m_EventToSend3,
    m_bSlot1ActivatedLastFrame,
    m_bSlot2ActivatedLastFrame,
    m_bSlot3ActivatedLastFrame,
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
            "bEnableEvent1" => Ok(__Field::m_bEnableEvent1),
            "bVariableToTest1" => Ok(__Field::m_bVariableToTest1),
            "EventToSend1" => Ok(__Field::m_EventToSend1),
            "bEnableEvent2" => Ok(__Field::m_bEnableEvent2),
            "bVariableToTest2" => Ok(__Field::m_bVariableToTest2),
            "EventToSend2" => Ok(__Field::m_EventToSend2),
            "bEnableEvent3" => Ok(__Field::m_bEnableEvent3),
            "bVariableToTest3" => Ok(__Field::m_bVariableToTest3),
            "EventToSend3" => Ok(__Field::m_EventToSend3),
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
pub(super) struct __BSEventOnFalseToTrueModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSEventOnFalseToTrueModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSEventOnFalseToTrueModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSEventOnFalseToTrueModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    BSEventOnFalseToTrueModifier<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __BSEventOnFalseToTrueModifierVisitor<'de> {
    type Value = BSEventOnFalseToTrueModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct BSEventOnFalseToTrueModifier",
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
        let mut m_bEnableEvent1: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bVariableToTest1: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_EventToSend1: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_bEnableEvent2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bVariableToTest2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_EventToSend2: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_bEnableEvent3: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bVariableToTest3: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_EventToSend3: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_bSlot1ActivatedLastFrame: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bSlot2ActivatedLastFrame: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bSlot3ActivatedLastFrame: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..12usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_bEnableEvent1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bEnableEvent1",
                            ),
                        );
                    }
                    m_bEnableEvent1 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_bVariableToTest1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bVariableToTest1",
                            ),
                        );
                    }
                    m_bVariableToTest1 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_EventToSend1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "EventToSend1",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 6usize)?;
                    m_EventToSend1 = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_bEnableEvent2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bEnableEvent2",
                            ),
                        );
                    }
                    m_bEnableEvent2 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_bVariableToTest2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bVariableToTest2",
                            ),
                        );
                    }
                    m_bVariableToTest2 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_EventToSend2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "EventToSend2",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 6usize)?;
                    m_EventToSend2 = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_bEnableEvent3) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bEnableEvent3",
                            ),
                        );
                    }
                    m_bEnableEvent3 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_bVariableToTest3) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bVariableToTest3",
                            ),
                        );
                    }
                    m_bVariableToTest3 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_EventToSend3) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "EventToSend3",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 6usize)?;
                    m_EventToSend3 = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_bSlot1ActivatedLastFrame) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bSlot1ActivatedLastFrame",
                            ),
                        );
                    }
                    m_bSlot1ActivatedLastFrame = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_bSlot2ActivatedLastFrame) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bSlot2ActivatedLastFrame",
                            ),
                        );
                    }
                    m_bSlot2ActivatedLastFrame = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_bSlot3ActivatedLastFrame) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bSlot3ActivatedLastFrame",
                            ),
                        );
                    }
                    m_bSlot3ActivatedLastFrame = _serde::__private::Some(
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
        let m_bEnableEvent1 = match m_bEnableEvent1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bEnableEvent1"),
                );
            }
        };
        let m_bVariableToTest1 = match m_bVariableToTest1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bVariableToTest1"),
                );
            }
        };
        let m_EventToSend1 = match m_EventToSend1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("EventToSend1"),
                );
            }
        };
        let m_bEnableEvent2 = match m_bEnableEvent2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bEnableEvent2"),
                );
            }
        };
        let m_bVariableToTest2 = match m_bVariableToTest2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bVariableToTest2"),
                );
            }
        };
        let m_EventToSend2 = match m_EventToSend2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("EventToSend2"),
                );
            }
        };
        let m_bEnableEvent3 = match m_bEnableEvent3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bEnableEvent3"),
                );
            }
        };
        let m_bVariableToTest3 = match m_bVariableToTest3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bVariableToTest3"),
                );
            }
        };
        let m_EventToSend3 = match m_EventToSend3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("EventToSend3"),
                );
            }
        };
        let m_bSlot1ActivatedLastFrame = match m_bSlot1ActivatedLastFrame {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "bSlot1ActivatedLastFrame",
                    ),
                );
            }
        };
        let m_bSlot2ActivatedLastFrame = match m_bSlot2ActivatedLastFrame {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "bSlot2ActivatedLastFrame",
                    ),
                );
            }
        };
        let m_bSlot3ActivatedLastFrame = match m_bSlot3ActivatedLastFrame {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "bSlot3ActivatedLastFrame",
                    ),
                );
            }
        };
        _serde::__private::Ok(BSEventOnFalseToTrueModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_bEnableEvent1,
            m_bVariableToTest1,
            m_EventToSend1,
            m_bEnableEvent2,
            m_bVariableToTest2,
            m_EventToSend2,
            m_bEnableEvent3,
            m_bVariableToTest3,
            m_EventToSend3,
            m_bSlot1ActivatedLastFrame,
            m_bSlot2ActivatedLastFrame,
            m_bSlot3ActivatedLastFrame,
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
        let mut m_bEnableEvent1: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bVariableToTest1: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_EventToSend1: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_bEnableEvent2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bVariableToTest2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_EventToSend2: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_bEnableEvent3: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bVariableToTest3: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_EventToSend3: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_bEnableEvent1 => {
                    if _serde::__private::Option::is_some(&m_bEnableEvent1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bEnableEvent1",
                            ),
                        );
                    }
                    m_bEnableEvent1 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_bVariableToTest1 => {
                    if _serde::__private::Option::is_some(&m_bVariableToTest1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bVariableToTest1",
                            ),
                        );
                    }
                    m_bVariableToTest1 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_EventToSend1 => {
                    if _serde::__private::Option::is_some(&m_EventToSend1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "EventToSend1",
                            ),
                        );
                    }
                    m_EventToSend1 = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_bEnableEvent2 => {
                    if _serde::__private::Option::is_some(&m_bEnableEvent2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bEnableEvent2",
                            ),
                        );
                    }
                    m_bEnableEvent2 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_bVariableToTest2 => {
                    if _serde::__private::Option::is_some(&m_bVariableToTest2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bVariableToTest2",
                            ),
                        );
                    }
                    m_bVariableToTest2 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_EventToSend2 => {
                    if _serde::__private::Option::is_some(&m_EventToSend2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "EventToSend2",
                            ),
                        );
                    }
                    m_EventToSend2 = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_bEnableEvent3 => {
                    if _serde::__private::Option::is_some(&m_bEnableEvent3) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bEnableEvent3",
                            ),
                        );
                    }
                    m_bEnableEvent3 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_bVariableToTest3 => {
                    if _serde::__private::Option::is_some(&m_bVariableToTest3) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bVariableToTest3",
                            ),
                        );
                    }
                    m_bVariableToTest3 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_EventToSend3 => {
                    if _serde::__private::Option::is_some(&m_EventToSend3) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "EventToSend3",
                            ),
                        );
                    }
                    m_EventToSend3 = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
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
        let m_bEnableEvent1 = match m_bEnableEvent1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bEnableEvent1"),
                );
            }
        };
        let m_bVariableToTest1 = match m_bVariableToTest1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bVariableToTest1"),
                );
            }
        };
        let m_EventToSend1 = match m_EventToSend1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("EventToSend1"),
                );
            }
        };
        let m_bEnableEvent2 = match m_bEnableEvent2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bEnableEvent2"),
                );
            }
        };
        let m_bVariableToTest2 = match m_bVariableToTest2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bVariableToTest2"),
                );
            }
        };
        let m_EventToSend2 = match m_EventToSend2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("EventToSend2"),
                );
            }
        };
        let m_bEnableEvent3 = match m_bEnableEvent3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bEnableEvent3"),
                );
            }
        };
        let m_bVariableToTest3 = match m_bVariableToTest3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bVariableToTest3"),
                );
            }
        };
        let m_EventToSend3 = match m_EventToSend3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("EventToSend3"),
                );
            }
        };
        _serde::__private::Ok(BSEventOnFalseToTrueModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_bEnableEvent1,
            m_bVariableToTest1,
            m_EventToSend1,
            m_bEnableEvent2,
            m_bVariableToTest2,
            m_EventToSend2,
            m_bEnableEvent3,
            m_bVariableToTest3,
            m_EventToSend3,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSEventOnFalseToTrueModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "bEnableEvent1",
                "bVariableToTest1",
                "EventToSend1",
                "bEnableEvent2",
                "bVariableToTest2",
                "EventToSend2",
                "bEnableEvent3",
                "bVariableToTest3",
                "EventToSend3",
                "bSlot1ActivatedLastFrame",
                "bSlot2ActivatedLastFrame",
                "bSlot3ActivatedLastFrame",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSEventOnFalseToTrueModifier",
                FIELDS,
                __BSEventOnFalseToTrueModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        BSEventOnFalseToTrueModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
