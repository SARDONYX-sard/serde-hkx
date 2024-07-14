use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSetBehaviorCommand`
/// -         version: `1`
/// -       signature: `0xe18b74b9`
/// -          size:  48(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSetBehaviorCommand {
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
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `behavior`(ctype: `struct hkbBehaviorGraph*`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behavior: Pointer,
    /// # C++ Info
    /// -          name: `rootGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rootGenerator: Pointer,
    /// # C++ Info
    /// -          name: `referencedBehaviors`(ctype: `hkArray<hkbBehaviorGraph*>`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_referencedBehaviors: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `startStateIndex`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_startStateIndex: i32,
    /// # C++ Info
    /// -          name: `randomizeSimulation`(ctype: `hkBool`)
    /// -        offset:  40(x86)/ 60(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_randomizeSimulation: bool,
    /// # C++ Info
    /// -          name: `padding`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_padding: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbSetBehaviorCommand {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSetBehaviorCommand"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe18b74b9)
        }
    }
    impl _serde::Serialize for hkbSetBehaviorCommand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe18b74b9)));
            let mut serializer = __serializer
                .serialize_struct("hkbSetBehaviorCommand", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("behavior", &self.m_behavior)?;
            serializer.serialize_field("rootGenerator", &self.m_rootGenerator)?;
            serializer
                .serialize_array_meta_field(
                    "referencedBehaviors",
                    &self.m_referencedBehaviors,
                )?;
            serializer.serialize_field("startStateIndex", &self.m_startStateIndex)?;
            serializer
                .serialize_field("randomizeSimulation", &self.m_randomizeSimulation)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("padding", &self.m_padding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "referencedBehaviors",
                    &self.m_referencedBehaviors,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_characterId,
    m_behavior,
    m_rootGenerator,
    m_referencedBehaviors,
    m_startStateIndex,
    m_randomizeSimulation,
    m_padding,
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
            "characterId" => Ok(__Field::m_characterId),
            "behavior" => Ok(__Field::m_behavior),
            "rootGenerator" => Ok(__Field::m_rootGenerator),
            "referencedBehaviors" => Ok(__Field::m_referencedBehaviors),
            "startStateIndex" => Ok(__Field::m_startStateIndex),
            "randomizeSimulation" => Ok(__Field::m_randomizeSimulation),
            "padding" => Ok(__Field::m_padding),
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
pub(super) struct __hkbSetBehaviorCommandVisitor<'de> {
    marker: core::marker::PhantomData<hkbSetBehaviorCommand>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbSetBehaviorCommandVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbSetBehaviorCommand, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbSetBehaviorCommand>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbSetBehaviorCommandVisitor<'de> {
    type Value = hkbSetBehaviorCommand;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbSetBehaviorCommand")
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
        let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_behavior: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_rootGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_referencedBehaviors: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_startStateIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_randomizeSimulation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_padding: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_characterId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "characterId",
                            ),
                        );
                    }
                    m_characterId = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
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
                    if _serde::__private::Option::is_some(&m_rootGenerator) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rootGenerator",
                            ),
                        );
                    }
                    m_rootGenerator = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_referencedBehaviors) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "referencedBehaviors",
                            ),
                        );
                    }
                    m_referencedBehaviors = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_startStateIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startStateIndex",
                            ),
                        );
                    }
                    m_startStateIndex = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_randomizeSimulation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "randomizeSimulation",
                            ),
                        );
                    }
                    m_randomizeSimulation = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_padding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("padding"),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_padding = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_characterId = match m_characterId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterId"),
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
        let m_rootGenerator = match m_rootGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rootGenerator"),
                );
            }
        };
        let m_referencedBehaviors = match m_referencedBehaviors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "referencedBehaviors",
                    ),
                );
            }
        };
        let m_startStateIndex = match m_startStateIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startStateIndex"),
                );
            }
        };
        let m_randomizeSimulation = match m_randomizeSimulation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "randomizeSimulation",
                    ),
                );
            }
        };
        let m_padding = match m_padding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("padding"),
                );
            }
        };
        _serde::__private::Ok(hkbSetBehaviorCommand {
            __ptr,
            parent,
            m_characterId,
            m_behavior,
            m_rootGenerator,
            m_referencedBehaviors,
            m_startStateIndex,
            m_randomizeSimulation,
            m_padding,
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
        let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_behavior: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_rootGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_referencedBehaviors: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_startStateIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_randomizeSimulation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_padding: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..7usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_characterId => {
                        if _serde::__private::Option::is_some(&m_characterId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "characterId",
                                ),
                            );
                        }
                        m_characterId = _serde::__private::Some(
                            match __A::next_value::<u64>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_behavior => {
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
                    __Field::m_rootGenerator => {
                        if _serde::__private::Option::is_some(&m_rootGenerator) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rootGenerator",
                                ),
                            );
                        }
                        m_rootGenerator = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_referencedBehaviors => {
                        if _serde::__private::Option::is_some(&m_referencedBehaviors) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "referencedBehaviors",
                                ),
                            );
                        }
                        m_referencedBehaviors = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_startStateIndex => {
                        if _serde::__private::Option::is_some(&m_startStateIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "startStateIndex",
                                ),
                            );
                        }
                        m_startStateIndex = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_randomizeSimulation => {
                        if _serde::__private::Option::is_some(&m_randomizeSimulation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "randomizeSimulation",
                                ),
                            );
                        }
                        m_randomizeSimulation = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_padding => {
                        if _serde::__private::Option::is_some(&m_padding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "padding",
                                ),
                            );
                        }
                        m_padding = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
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
        let m_characterId = match m_characterId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterId"),
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
        let m_rootGenerator = match m_rootGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rootGenerator"),
                );
            }
        };
        let m_referencedBehaviors = match m_referencedBehaviors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "referencedBehaviors",
                    ),
                );
            }
        };
        let m_startStateIndex = match m_startStateIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startStateIndex"),
                );
            }
        };
        let m_randomizeSimulation = match m_randomizeSimulation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "randomizeSimulation",
                    ),
                );
            }
        };
        let m_padding = match m_padding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("padding"),
                );
            }
        };
        _serde::__private::Ok(hkbSetBehaviorCommand {
            __ptr,
            parent,
            m_characterId,
            m_behavior,
            m_rootGenerator,
            m_referencedBehaviors,
            m_startStateIndex,
            m_randomizeSimulation,
            m_padding,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbSetBehaviorCommand {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "characterId",
                "behavior",
                "rootGenerator",
                "referencedBehaviors",
                "startStateIndex",
                "randomizeSimulation",
                "padding",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbSetBehaviorCommand",
                FIELDS,
                __hkbSetBehaviorCommandVisitor {
                    marker: _serde::__private::PhantomData::<hkbSetBehaviorCommand>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
