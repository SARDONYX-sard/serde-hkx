use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbContext`
/// - version: `1`
/// - signature: `0xe0c4d4a7`
/// - size: ` 40`(x86)/` 80`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbContext {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `character`(ctype: `void*`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "character"))]
    pub m_character: Pointer,
    /// # C++ Info
    /// - name: `behavior`(ctype: `void*`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "behavior"))]
    pub m_behavior: Pointer,
    /// # C++ Info
    /// - name: `nodeToIndexMap`(ctype: `void*`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "nodeToIndexMap"))]
    pub m_nodeToIndexMap: Pointer,
    /// # C++ Info
    /// - name: `eventQueue`(ctype: `void*`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "eventQueue"))]
    pub m_eventQueue: Pointer,
    /// # C++ Info
    /// - name: `sharedEventQueue`(ctype: `void*`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "sharedEventQueue"))]
    pub m_sharedEventQueue: Pointer,
    /// # C++ Info
    /// - name: `generatorOutputListener`(ctype: `struct hkbGeneratorOutputListener*`)
    /// - offset: ` 20`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "generatorOutputListener"))]
    pub m_generatorOutputListener: Pointer,
    /// # C++ Info
    /// - name: `eventTriggeredTransition`(ctype: `hkBool`)
    /// - offset: ` 24`(x86)/` 48`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "eventTriggeredTransition"))]
    pub m_eventTriggeredTransition: bool,
    /// # C++ Info
    /// - name: `world`(ctype: `void*`)
    /// - offset: ` 28`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "world"))]
    pub m_world: Pointer,
    /// # C++ Info
    /// - name: `attachmentManager`(ctype: `void*`)
    /// - offset: ` 32`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "attachmentManager"))]
    pub m_attachmentManager: Pointer,
    /// # C++ Info
    /// - name: `animationCache`(ctype: `void*`)
    /// - offset: ` 36`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "animationCache"))]
    pub m_animationCache: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbContext {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbContext"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe0c4d4a7)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_character.get());
            v.push(self.m_behavior.get());
            v.push(self.m_nodeToIndexMap.get());
            v.push(self.m_eventQueue.get());
            v.push(self.m_sharedEventQueue.get());
            v.push(self.m_generatorOutputListener.get());
            v.push(self.m_world.get());
            v.push(self.m_attachmentManager.get());
            v.push(self.m_animationCache.get());
            v
        }
    }
    impl _serde::Serialize for hkbContext {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe0c4d4a7)));
            let mut serializer = __serializer
                .serialize_struct("hkbContext", class_meta, (40u64, 80u64))?;
            serializer.skip_field("character", &self.m_character)?;
            serializer.skip_field("behavior", &self.m_behavior)?;
            serializer.skip_field("nodeToIndexMap", &self.m_nodeToIndexMap)?;
            serializer.skip_field("eventQueue", &self.m_eventQueue)?;
            serializer.skip_field("sharedEventQueue", &self.m_sharedEventQueue)?;
            serializer
                .serialize_field(
                    "generatorOutputListener",
                    &self.m_generatorOutputListener,
                )?;
            serializer
                .skip_field(
                    "eventTriggeredTransition",
                    &self.m_eventTriggeredTransition,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_field("world", &self.m_world)?;
            serializer.skip_field("attachmentManager", &self.m_attachmentManager)?;
            serializer.skip_field("animationCache", &self.m_animationCache)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbContext {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_generatorOutputListener,
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
                        "generatorOutputListener" => {
                            Ok(__Field::m_generatorOutputListener)
                        }
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
            struct __hkbContextVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbContext>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbContextVisitor<'de> {
                type Value = hkbContext;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkbContext")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_character: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_behavior: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_nodeToIndexMap: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_eventQueue: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_sharedEventQueue: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_generatorOutputListener: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_eventTriggeredTransition: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_world: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_attachmentManager: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_animationCache: _serde::__private::Option<Pointer> = _serde::__private::None;
                    for i in 0..10usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_character) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "character",
                                        ),
                                    );
                                }
                                m_character = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_behavior) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behavior",
                                        ),
                                    );
                                }
                                m_behavior = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_nodeToIndexMap) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nodeToIndexMap",
                                        ),
                                    );
                                }
                                m_nodeToIndexMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_eventQueue) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventQueue",
                                        ),
                                    );
                                }
                                m_eventQueue = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_sharedEventQueue) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sharedEventQueue",
                                        ),
                                    );
                                }
                                m_sharedEventQueue = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_generatorOutputListener,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "generatorOutputListener",
                                        ),
                                    );
                                }
                                m_generatorOutputListener = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_eventTriggeredTransition,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventTriggeredTransition",
                                        ),
                                    );
                                }
                                m_eventTriggeredTransition = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_world) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("world"),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 7usize)?;
                                m_world = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_attachmentManager,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attachmentManager",
                                        ),
                                    );
                                }
                                m_attachmentManager = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_animationCache) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationCache",
                                        ),
                                    );
                                }
                                m_animationCache = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    let m_character = match m_character {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "character",
                                ),
                            );
                        }
                    };
                    let m_behavior = match m_behavior {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("behavior"),
                            );
                        }
                    };
                    let m_nodeToIndexMap = match m_nodeToIndexMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nodeToIndexMap",
                                ),
                            );
                        }
                    };
                    let m_eventQueue = match m_eventQueue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventQueue",
                                ),
                            );
                        }
                    };
                    let m_sharedEventQueue = match m_sharedEventQueue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sharedEventQueue",
                                ),
                            );
                        }
                    };
                    let m_generatorOutputListener = match m_generatorOutputListener {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "generatorOutputListener",
                                ),
                            );
                        }
                    };
                    let m_eventTriggeredTransition = match m_eventTriggeredTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventTriggeredTransition",
                                ),
                            );
                        }
                    };
                    let m_world = match m_world {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("world"),
                            );
                        }
                    };
                    let m_attachmentManager = match m_attachmentManager {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attachmentManager",
                                ),
                            );
                        }
                    };
                    let m_animationCache = match m_animationCache {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationCache",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbContext {
                        __ptr,
                        m_character,
                        m_behavior,
                        m_nodeToIndexMap,
                        m_eventQueue,
                        m_sharedEventQueue,
                        m_generatorOutputListener,
                        m_eventTriggeredTransition,
                        m_world,
                        m_attachmentManager,
                        m_animationCache,
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
                    let mut m_generatorOutputListener: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_generatorOutputListener => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_generatorOutputListener,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "generatorOutputListener",
                                        ),
                                    );
                                }
                                m_generatorOutputListener = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    let m_generatorOutputListener = match m_generatorOutputListener {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "generatorOutputListener",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbContext {
                        __ptr,
                        m_generatorOutputListener,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "character",
                "behavior",
                "nodeToIndexMap",
                "eventQueue",
                "sharedEventQueue",
                "generatorOutputListener",
                "eventTriggeredTransition",
                "world",
                "attachmentManager",
                "animationCache",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbContext",
                FIELDS,
                __hkbContextVisitor {
                    marker: _serde::__private::PhantomData::<hkbContext>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
