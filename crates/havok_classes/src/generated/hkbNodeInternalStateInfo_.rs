use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbNodeInternalStateInfo`
/// -         version: `0`
/// -       signature: `0x7db9971d`
/// -          size: 100(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbNodeInternalStateInfo<'a> {
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
    /// -          name: `syncInfo`(ctype: `struct hkbGeneratorSyncInfo`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  80(x86)/ 80(x86_64)
    ///
    pub m_syncInfo: hkbGeneratorSyncInfo,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  88(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `internalState`(ctype: `struct hkReferencedObject*`)
    /// -        offset:  92(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_internalState: Pointer,
    /// # C++ Info
    /// -          name: `nodeId`(ctype: `hkInt16`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_nodeId: i16,
    /// # C++ Info
    /// -          name: `hasActivateBeenCalled`(ctype: `hkBool`)
    /// -        offset:  98(x86)/114(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hasActivateBeenCalled: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbNodeInternalStateInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbNodeInternalStateInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7db9971d)
        }
    }
    impl<'a> _serde::Serialize for hkbNodeInternalStateInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7db9971d)));
            let mut serializer = __serializer
                .serialize_struct("hkbNodeInternalStateInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("syncInfo", &self.m_syncInfo)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("internalState", &self.m_internalState)?;
            serializer.serialize_field("nodeId", &self.m_nodeId)?;
            serializer
                .serialize_field(
                    "hasActivateBeenCalled",
                    &self.m_hasActivateBeenCalled,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_syncInfo,
    m_name,
    m_internalState,
    m_nodeId,
    m_hasActivateBeenCalled,
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
            "syncInfo" => Ok(__Field::m_syncInfo),
            "name" => Ok(__Field::m_name),
            "internalState" => Ok(__Field::m_internalState),
            "nodeId" => Ok(__Field::m_nodeId),
            "hasActivateBeenCalled" => Ok(__Field::m_hasActivateBeenCalled),
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
pub(super) struct __hkbNodeInternalStateInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbNodeInternalStateInfo<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbNodeInternalStateInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbNodeInternalStateInfo<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbNodeInternalStateInfo<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbNodeInternalStateInfoVisitor<'de> {
    type Value = hkbNodeInternalStateInfo<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbNodeInternalStateInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_syncInfo: _serde::__private::Option<hkbGeneratorSyncInfo> = _serde::__private::None;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_internalState: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_nodeId: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_hasActivateBeenCalled: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_syncInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "syncInfo",
                            ),
                        );
                    }
                    m_syncInfo = _serde::__private::Some(
                        match __A::next_value::<hkbGeneratorSyncInfo>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_name) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                        );
                    }
                    m_name = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
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
                3usize => {
                    if _serde::__private::Option::is_some(&m_nodeId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("nodeId"),
                        );
                    }
                    m_nodeId = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_hasActivateBeenCalled) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hasActivateBeenCalled",
                            ),
                        );
                    }
                    m_hasActivateBeenCalled = _serde::__private::Some(
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
        let m_syncInfo = match m_syncInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("syncInfo"),
                );
            }
        };
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
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
        let m_nodeId = match m_nodeId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nodeId"),
                );
            }
        };
        let m_hasActivateBeenCalled = match m_hasActivateBeenCalled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hasActivateBeenCalled",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbNodeInternalStateInfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_syncInfo,
            m_name,
            m_internalState,
            m_nodeId,
            m_hasActivateBeenCalled,
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
        let mut m_syncInfo: _serde::__private::Option<hkbGeneratorSyncInfo> = _serde::__private::None;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_internalState: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_nodeId: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_hasActivateBeenCalled: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..5usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_syncInfo => {
                        if _serde::__private::Option::is_some(&m_syncInfo) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "syncInfo",
                                ),
                            );
                        }
                        m_syncInfo = _serde::__private::Some(
                            match __A::next_value::<hkbGeneratorSyncInfo>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_name => {
                        if _serde::__private::Option::is_some(&m_name) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                            );
                        }
                        m_name = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
                    __Field::m_nodeId => {
                        if _serde::__private::Option::is_some(&m_nodeId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("nodeId"),
                            );
                        }
                        m_nodeId = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_hasActivateBeenCalled => {
                        if _serde::__private::Option::is_some(&m_hasActivateBeenCalled) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "hasActivateBeenCalled",
                                ),
                            );
                        }
                        m_hasActivateBeenCalled = _serde::__private::Some(
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
        let m_syncInfo = match m_syncInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("syncInfo"),
                );
            }
        };
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
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
        let m_nodeId = match m_nodeId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nodeId"),
                );
            }
        };
        let m_hasActivateBeenCalled = match m_hasActivateBeenCalled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hasActivateBeenCalled",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbNodeInternalStateInfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_syncInfo,
            m_name,
            m_internalState,
            m_nodeId,
            m_hasActivateBeenCalled,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbNodeInternalStateInfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "syncInfo",
                "name",
                "internalState",
                "nodeId",
                "hasActivateBeenCalled",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbNodeInternalStateInfo",
                FIELDS,
                __hkbNodeInternalStateInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkbNodeInternalStateInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
