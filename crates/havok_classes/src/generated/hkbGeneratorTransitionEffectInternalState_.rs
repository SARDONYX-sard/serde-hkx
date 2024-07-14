use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbGeneratorTransitionEffectInternalState`
/// -         version: `0`
/// -       signature: `0xd6692b5d`
/// -          size:  32(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbGeneratorTransitionEffectInternalState {
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
    /// -          name: `timeInTransition`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timeInTransition: f32,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `effectiveBlendInDuration`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_effectiveBlendInDuration: f32,
    /// # C++ Info
    /// -          name: `effectiveBlendOutDuration`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_effectiveBlendOutDuration: f32,
    /// # C++ Info
    /// -          name: `toGeneratorState`(ctype: `enum ToGeneratorState`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_toGeneratorState: ToGeneratorState,
    /// # C++ Info
    /// -          name: `echoTransitionGenerator`(ctype: `hkBool`)
    /// -        offset:  25(x86)/ 33(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_echoTransitionGenerator: bool,
    /// # C++ Info
    /// -          name: `echoToGenerator`(ctype: `hkBool`)
    /// -        offset:  26(x86)/ 34(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_echoToGenerator: bool,
    /// # C++ Info
    /// -          name: `justActivated`(ctype: `hkBool`)
    /// -        offset:  27(x86)/ 35(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_justActivated: bool,
    /// # C++ Info
    /// -          name: `updateActiveNodes`(ctype: `hkBool`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_updateActiveNodes: bool,
    /// # C++ Info
    /// -          name: `stage`(ctype: `enum Stage`)
    /// -        offset:  29(x86)/ 37(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_stage: Stage,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbGeneratorTransitionEffectInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbGeneratorTransitionEffectInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd6692b5d)
        }
    }
    impl _serde::Serialize for hkbGeneratorTransitionEffectInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd6692b5d)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbGeneratorTransitionEffectInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("timeInTransition", &self.m_timeInTransition)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer
                .serialize_field(
                    "effectiveBlendInDuration",
                    &self.m_effectiveBlendInDuration,
                )?;
            serializer
                .serialize_field(
                    "effectiveBlendOutDuration",
                    &self.m_effectiveBlendOutDuration,
                )?;
            serializer.serialize_field("toGeneratorState", &self.m_toGeneratorState)?;
            serializer
                .serialize_field(
                    "echoTransitionGenerator",
                    &self.m_echoTransitionGenerator,
                )?;
            serializer.serialize_field("echoToGenerator", &self.m_echoToGenerator)?;
            serializer.serialize_field("justActivated", &self.m_justActivated)?;
            serializer.serialize_field("updateActiveNodes", &self.m_updateActiveNodes)?;
            serializer.serialize_field("stage", &self.m_stage)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_timeInTransition,
    m_duration,
    m_effectiveBlendInDuration,
    m_effectiveBlendOutDuration,
    m_toGeneratorState,
    m_echoTransitionGenerator,
    m_echoToGenerator,
    m_justActivated,
    m_updateActiveNodes,
    m_stage,
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
            "timeInTransition" => Ok(__Field::m_timeInTransition),
            "duration" => Ok(__Field::m_duration),
            "effectiveBlendInDuration" => Ok(__Field::m_effectiveBlendInDuration),
            "effectiveBlendOutDuration" => Ok(__Field::m_effectiveBlendOutDuration),
            "toGeneratorState" => Ok(__Field::m_toGeneratorState),
            "echoTransitionGenerator" => Ok(__Field::m_echoTransitionGenerator),
            "echoToGenerator" => Ok(__Field::m_echoToGenerator),
            "justActivated" => Ok(__Field::m_justActivated),
            "updateActiveNodes" => Ok(__Field::m_updateActiveNodes),
            "stage" => Ok(__Field::m_stage),
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
pub(super) struct __hkbGeneratorTransitionEffectInternalStateVisitor<'de> {
    marker: core::marker::PhantomData<hkbGeneratorTransitionEffectInternalState>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbGeneratorTransitionEffectInternalStateVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbGeneratorTransitionEffectInternalState, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbGeneratorTransitionEffectInternalState,
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
impl<'de> _serde::de::Visitor<'de>
for __hkbGeneratorTransitionEffectInternalStateVisitor<'de> {
    type Value = hkbGeneratorTransitionEffectInternalState;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbGeneratorTransitionEffectInternalState",
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
        let mut m_timeInTransition: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_effectiveBlendInDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_effectiveBlendOutDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_toGeneratorState: _serde::__private::Option<ToGeneratorState> = _serde::__private::None;
        let mut m_echoTransitionGenerator: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_echoToGenerator: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_justActivated: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_updateActiveNodes: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_stage: _serde::__private::Option<Stage> = _serde::__private::None;
        for i in 0..10usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_timeInTransition) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "timeInTransition",
                            ),
                        );
                    }
                    m_timeInTransition = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
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
                    if _serde::__private::Option::is_some(&m_effectiveBlendInDuration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "effectiveBlendInDuration",
                            ),
                        );
                    }
                    m_effectiveBlendInDuration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_effectiveBlendOutDuration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "effectiveBlendOutDuration",
                            ),
                        );
                    }
                    m_effectiveBlendOutDuration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_toGeneratorState) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "toGeneratorState",
                            ),
                        );
                    }
                    m_toGeneratorState = _serde::__private::Some(
                        match __A::next_value::<ToGeneratorState>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_echoTransitionGenerator) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "echoTransitionGenerator",
                            ),
                        );
                    }
                    m_echoTransitionGenerator = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_echoToGenerator) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "echoToGenerator",
                            ),
                        );
                    }
                    m_echoToGenerator = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_justActivated) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "justActivated",
                            ),
                        );
                    }
                    m_justActivated = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_updateActiveNodes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "updateActiveNodes",
                            ),
                        );
                    }
                    m_updateActiveNodes = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_stage) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("stage"),
                        );
                    }
                    m_stage = _serde::__private::Some(
                        match __A::next_value::<Stage>(&mut __map) {
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
        __A::pad(&mut __map, 2usize, 2usize)?;
        let m_timeInTransition = match m_timeInTransition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("timeInTransition"),
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
        let m_effectiveBlendInDuration = match m_effectiveBlendInDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "effectiveBlendInDuration",
                    ),
                );
            }
        };
        let m_effectiveBlendOutDuration = match m_effectiveBlendOutDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "effectiveBlendOutDuration",
                    ),
                );
            }
        };
        let m_toGeneratorState = match m_toGeneratorState {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("toGeneratorState"),
                );
            }
        };
        let m_echoTransitionGenerator = match m_echoTransitionGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "echoTransitionGenerator",
                    ),
                );
            }
        };
        let m_echoToGenerator = match m_echoToGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("echoToGenerator"),
                );
            }
        };
        let m_justActivated = match m_justActivated {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("justActivated"),
                );
            }
        };
        let m_updateActiveNodes = match m_updateActiveNodes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("updateActiveNodes"),
                );
            }
        };
        let m_stage = match m_stage {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("stage"),
                );
            }
        };
        _serde::__private::Ok(hkbGeneratorTransitionEffectInternalState {
            __ptr,
            parent,
            m_timeInTransition,
            m_duration,
            m_effectiveBlendInDuration,
            m_effectiveBlendOutDuration,
            m_toGeneratorState,
            m_echoTransitionGenerator,
            m_echoToGenerator,
            m_justActivated,
            m_updateActiveNodes,
            m_stage,
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
        let mut m_timeInTransition: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_effectiveBlendInDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_effectiveBlendOutDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_toGeneratorState: _serde::__private::Option<ToGeneratorState> = _serde::__private::None;
        let mut m_echoTransitionGenerator: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_echoToGenerator: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_justActivated: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_updateActiveNodes: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_stage: _serde::__private::Option<Stage> = _serde::__private::None;
        for _ in 0..10usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_timeInTransition => {
                        if _serde::__private::Option::is_some(&m_timeInTransition) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "timeInTransition",
                                ),
                            );
                        }
                        m_timeInTransition = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
                    __Field::m_effectiveBlendInDuration => {
                        if _serde::__private::Option::is_some(
                            &m_effectiveBlendInDuration,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "effectiveBlendInDuration",
                                ),
                            );
                        }
                        m_effectiveBlendInDuration = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_effectiveBlendOutDuration => {
                        if _serde::__private::Option::is_some(
                            &m_effectiveBlendOutDuration,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "effectiveBlendOutDuration",
                                ),
                            );
                        }
                        m_effectiveBlendOutDuration = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_toGeneratorState => {
                        if _serde::__private::Option::is_some(&m_toGeneratorState) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "toGeneratorState",
                                ),
                            );
                        }
                        m_toGeneratorState = _serde::__private::Some(
                            match __A::next_value::<ToGeneratorState>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_echoTransitionGenerator => {
                        if _serde::__private::Option::is_some(
                            &m_echoTransitionGenerator,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "echoTransitionGenerator",
                                ),
                            );
                        }
                        m_echoTransitionGenerator = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_echoToGenerator => {
                        if _serde::__private::Option::is_some(&m_echoToGenerator) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "echoToGenerator",
                                ),
                            );
                        }
                        m_echoToGenerator = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_justActivated => {
                        if _serde::__private::Option::is_some(&m_justActivated) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "justActivated",
                                ),
                            );
                        }
                        m_justActivated = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_updateActiveNodes => {
                        if _serde::__private::Option::is_some(&m_updateActiveNodes) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "updateActiveNodes",
                                ),
                            );
                        }
                        m_updateActiveNodes = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_stage => {
                        if _serde::__private::Option::is_some(&m_stage) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("stage"),
                            );
                        }
                        m_stage = _serde::__private::Some(
                            match __A::next_value::<Stage>(&mut __map) {
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
        let m_timeInTransition = match m_timeInTransition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("timeInTransition"),
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
        let m_effectiveBlendInDuration = match m_effectiveBlendInDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "effectiveBlendInDuration",
                    ),
                );
            }
        };
        let m_effectiveBlendOutDuration = match m_effectiveBlendOutDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "effectiveBlendOutDuration",
                    ),
                );
            }
        };
        let m_toGeneratorState = match m_toGeneratorState {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("toGeneratorState"),
                );
            }
        };
        let m_echoTransitionGenerator = match m_echoTransitionGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "echoTransitionGenerator",
                    ),
                );
            }
        };
        let m_echoToGenerator = match m_echoToGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("echoToGenerator"),
                );
            }
        };
        let m_justActivated = match m_justActivated {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("justActivated"),
                );
            }
        };
        let m_updateActiveNodes = match m_updateActiveNodes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("updateActiveNodes"),
                );
            }
        };
        let m_stage = match m_stage {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("stage"),
                );
            }
        };
        _serde::__private::Ok(hkbGeneratorTransitionEffectInternalState {
            __ptr,
            parent,
            m_timeInTransition,
            m_duration,
            m_effectiveBlendInDuration,
            m_effectiveBlendOutDuration,
            m_toGeneratorState,
            m_echoTransitionGenerator,
            m_echoToGenerator,
            m_justActivated,
            m_updateActiveNodes,
            m_stage,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbGeneratorTransitionEffectInternalState {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "timeInTransition",
                "duration",
                "effectiveBlendInDuration",
                "effectiveBlendOutDuration",
                "toGeneratorState",
                "echoTransitionGenerator",
                "echoToGenerator",
                "justActivated",
                "updateActiveNodes",
                "stage",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbGeneratorTransitionEffectInternalState",
                FIELDS,
                __hkbGeneratorTransitionEffectInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbGeneratorTransitionEffectInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
