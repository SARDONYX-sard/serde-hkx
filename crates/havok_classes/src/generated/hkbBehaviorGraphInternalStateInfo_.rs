use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorGraphInternalStateInfo`
/// -         version: `1`
/// -       signature: `0x645f898b`
/// -          size:  56(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorGraphInternalStateInfo {
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
    /// -          name: `internalState`(ctype: `struct hkbBehaviorGraphInternalState*`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_internalState: Pointer,
    /// # C++ Info
    /// -          name: `auxiliaryNodeInfo`(ctype: `hkArray<hkbAuxiliaryNodeInfo*>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_auxiliaryNodeInfo: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `activeEventIds`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_activeEventIds: Vec<i16>,
    /// # C++ Info
    /// -          name: `activeVariableIds`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_activeVariableIds: Vec<i16>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBehaviorGraphInternalStateInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBehaviorGraphInternalStateInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x645f898b)
        }
    }
    impl _serde::Serialize for hkbBehaviorGraphInternalStateInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x645f898b)));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorGraphInternalStateInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("internalState", &self.m_internalState)?;
            serializer
                .serialize_array_meta_field(
                    "auxiliaryNodeInfo",
                    &self.m_auxiliaryNodeInfo,
                )?;
            serializer
                .serialize_array_meta_field("activeEventIds", &self.m_activeEventIds)?;
            serializer
                .serialize_array_meta_field(
                    "activeVariableIds",
                    &self.m_activeVariableIds,
                )?;
            serializer
                .serialize_array_field("auxiliaryNodeInfo", &self.m_auxiliaryNodeInfo)?;
            serializer.serialize_array_field("activeEventIds", &self.m_activeEventIds)?;
            serializer
                .serialize_array_field("activeVariableIds", &self.m_activeVariableIds)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_characterId,
    m_internalState,
    m_auxiliaryNodeInfo,
    m_activeEventIds,
    m_activeVariableIds,
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
            "internalState" => Ok(__Field::m_internalState),
            "auxiliaryNodeInfo" => Ok(__Field::m_auxiliaryNodeInfo),
            "activeEventIds" => Ok(__Field::m_activeEventIds),
            "activeVariableIds" => Ok(__Field::m_activeVariableIds),
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
pub(super) struct __hkbBehaviorGraphInternalStateInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbBehaviorGraphInternalStateInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbBehaviorGraphInternalStateInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbBehaviorGraphInternalStateInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbBehaviorGraphInternalStateInfo,
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
impl<'de> _serde::de::Visitor<'de> for __hkbBehaviorGraphInternalStateInfoVisitor<'de> {
    type Value = hkbBehaviorGraphInternalStateInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbBehaviorGraphInternalStateInfo",
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
        let parent = __A::next_value(&mut __map)?;
        let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_internalState: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_auxiliaryNodeInfo: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_activeEventIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
        let mut m_activeVariableIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
        for i in 0..5usize {
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
                    if _serde::__private::Option::is_some(&m_internalState) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "internalState",
                            ),
                        );
                    }
                    m_internalState = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_auxiliaryNodeInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "auxiliaryNodeInfo",
                            ),
                        );
                    }
                    m_auxiliaryNodeInfo = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_activeEventIds) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "activeEventIds",
                            ),
                        );
                    }
                    m_activeEventIds = _serde::__private::Some(
                        match __A::next_value::<Vec<i16>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_activeVariableIds) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "activeVariableIds",
                            ),
                        );
                    }
                    m_activeVariableIds = _serde::__private::Some(
                        match __A::next_value::<Vec<i16>>(&mut __map) {
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
        let m_characterId = match m_characterId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterId"),
                );
            }
        };
        let m_internalState = match m_internalState {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("internalState"),
                );
            }
        };
        let m_auxiliaryNodeInfo = match m_auxiliaryNodeInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("auxiliaryNodeInfo"),
                );
            }
        };
        let m_activeEventIds = match m_activeEventIds {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("activeEventIds"),
                );
            }
        };
        let m_activeVariableIds = match m_activeVariableIds {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("activeVariableIds"),
                );
            }
        };
        _serde::__private::Ok(hkbBehaviorGraphInternalStateInfo {
            __ptr,
            parent,
            m_characterId,
            m_internalState,
            m_auxiliaryNodeInfo,
            m_activeEventIds,
            m_activeVariableIds,
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
        let mut m_internalState: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_auxiliaryNodeInfo: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_activeEventIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
        let mut m_activeVariableIds: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
        for _ in 0..5usize {
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
                    __Field::m_internalState => {
                        if _serde::__private::Option::is_some(&m_internalState) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "internalState",
                                ),
                            );
                        }
                        m_internalState = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_auxiliaryNodeInfo => {
                        if _serde::__private::Option::is_some(&m_auxiliaryNodeInfo) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "auxiliaryNodeInfo",
                                ),
                            );
                        }
                        m_auxiliaryNodeInfo = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_activeEventIds => {
                        if _serde::__private::Option::is_some(&m_activeEventIds) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "activeEventIds",
                                ),
                            );
                        }
                        m_activeEventIds = _serde::__private::Some(
                            match __A::next_value::<Vec<i16>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_activeVariableIds => {
                        if _serde::__private::Option::is_some(&m_activeVariableIds) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "activeVariableIds",
                                ),
                            );
                        }
                        m_activeVariableIds = _serde::__private::Some(
                            match __A::next_value::<Vec<i16>>(&mut __map) {
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
        let m_internalState = match m_internalState {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("internalState"),
                );
            }
        };
        let m_auxiliaryNodeInfo = match m_auxiliaryNodeInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("auxiliaryNodeInfo"),
                );
            }
        };
        let m_activeEventIds = match m_activeEventIds {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("activeEventIds"),
                );
            }
        };
        let m_activeVariableIds = match m_activeVariableIds {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("activeVariableIds"),
                );
            }
        };
        _serde::__private::Ok(hkbBehaviorGraphInternalStateInfo {
            __ptr,
            parent,
            m_characterId,
            m_internalState,
            m_auxiliaryNodeInfo,
            m_activeEventIds,
            m_activeVariableIds,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbBehaviorGraphInternalStateInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "characterId",
                "internalState",
                "auxiliaryNodeInfo",
                "activeEventIds",
                "activeVariableIds",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBehaviorGraphInternalStateInfo",
                FIELDS,
                __hkbBehaviorGraphInternalStateInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbBehaviorGraphInternalStateInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
