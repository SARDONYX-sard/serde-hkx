use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSetWordVariableCommand`
/// -         version: `0`
/// -       signature: `0xf3ae5fca`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSetWordVariableCommand {
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
    /// -          name: `quadValue`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_quadValue: Vector4,
    /// # C++ Info
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `variableId`(ctype: `hkInt32`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_variableId: i32,
    /// # C++ Info
    /// -          name: `value`(ctype: `struct hkbVariableValue`)
    /// -        offset:  44(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_value: hkbVariableValue,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum VariableType`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: VariableType,
    /// # C++ Info
    /// -          name: `global`(ctype: `hkBool`)
    /// -        offset:  49(x86)/ 49(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_global: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbSetWordVariableCommand {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSetWordVariableCommand"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf3ae5fca)
        }
    }
    impl _serde::Serialize for hkbSetWordVariableCommand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf3ae5fca)));
            let mut serializer = __serializer
                .serialize_struct("hkbSetWordVariableCommand", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("quadValue", &self.m_quadValue)?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("variableId", &self.m_variableId)?;
            serializer.serialize_field("value", &self.m_value)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("global", &self.m_global)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_quadValue,
    m_characterId,
    m_variableId,
    m_value,
    m_type,
    m_global,
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
            "quadValue" => Ok(__Field::m_quadValue),
            "characterId" => Ok(__Field::m_characterId),
            "variableId" => Ok(__Field::m_variableId),
            "value" => Ok(__Field::m_value),
            "type" => Ok(__Field::m_type),
            "global" => Ok(__Field::m_global),
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
pub(super) struct __hkbSetWordVariableCommandVisitor<'de> {
    marker: core::marker::PhantomData<hkbSetWordVariableCommand>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbSetWordVariableCommandVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbSetWordVariableCommand, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbSetWordVariableCommand>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbSetWordVariableCommandVisitor<'de> {
    type Value = hkbSetWordVariableCommand;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbSetWordVariableCommand")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_quadValue: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_variableId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_value: _serde::__private::Option<hkbVariableValue> = _serde::__private::None;
        let mut m_type: _serde::__private::Option<VariableType> = _serde::__private::None;
        let mut m_global: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..6usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_quadValue) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "quadValue",
                            ),
                        );
                    }
                    m_quadValue = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
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
                2usize => {
                    if _serde::__private::Option::is_some(&m_variableId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "variableId",
                            ),
                        );
                    }
                    m_variableId = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_value) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("value"),
                        );
                    }
                    m_value = _serde::__private::Some(
                        match __A::next_value::<hkbVariableValue>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_type) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("type"),
                        );
                    }
                    m_type = _serde::__private::Some(
                        match __A::next_value::<VariableType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_global) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("global"),
                        );
                    }
                    m_global = _serde::__private::Some(
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
        __A::pad(&mut __map, 14usize, 14usize)?;
        let m_quadValue = match m_quadValue {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("quadValue"),
                );
            }
        };
        let m_characterId = match m_characterId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterId"),
                );
            }
        };
        let m_variableId = match m_variableId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("variableId"),
                );
            }
        };
        let m_value = match m_value {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("value"),
                );
            }
        };
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_global = match m_global {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("global"),
                );
            }
        };
        _serde::__private::Ok(hkbSetWordVariableCommand {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_quadValue,
            m_characterId,
            m_variableId,
            m_value,
            m_type,
            m_global,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_quadValue: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_variableId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_value: _serde::__private::Option<hkbVariableValue> = _serde::__private::None;
        let mut m_type: _serde::__private::Option<VariableType> = _serde::__private::None;
        let mut m_global: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_quadValue => {
                        if _serde::__private::Option::is_some(&m_quadValue) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "quadValue",
                                ),
                            );
                        }
                        m_quadValue = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
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
                    __Field::m_variableId => {
                        if _serde::__private::Option::is_some(&m_variableId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "variableId",
                                ),
                            );
                        }
                        m_variableId = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_value => {
                        if _serde::__private::Option::is_some(&m_value) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("value"),
                            );
                        }
                        m_value = _serde::__private::Some(
                            match __A::next_value::<hkbVariableValue>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_type => {
                        if _serde::__private::Option::is_some(&m_type) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                            );
                        }
                        m_type = _serde::__private::Some(
                            match __A::next_value::<VariableType>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_global => {
                        if _serde::__private::Option::is_some(&m_global) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("global"),
                            );
                        }
                        m_global = _serde::__private::Some(
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
        let m_quadValue = match m_quadValue {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("quadValue"),
                );
            }
        };
        let m_characterId = match m_characterId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterId"),
                );
            }
        };
        let m_variableId = match m_variableId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("variableId"),
                );
            }
        };
        let m_value = match m_value {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("value"),
                );
            }
        };
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_global = match m_global {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("global"),
                );
            }
        };
        _serde::__private::Ok(hkbSetWordVariableCommand {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_quadValue,
            m_characterId,
            m_variableId,
            m_value,
            m_type,
            m_global,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbSetWordVariableCommand {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "quadValue",
                "characterId",
                "variableId",
                "value",
                "type",
                "global",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbSetWordVariableCommand",
                FIELDS,
                __hkbSetWordVariableCommandVisitor {
                    marker: _serde::__private::PhantomData::<hkbSetWordVariableCommand>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
