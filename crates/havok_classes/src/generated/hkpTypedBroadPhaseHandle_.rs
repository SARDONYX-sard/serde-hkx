use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTypedBroadPhaseHandle`
/// -         version: `0`
/// -       signature: `0xf4b0f799`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTypedBroadPhaseHandle {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpBroadPhaseHandle,
    /// # C++ Info
    /// -          name: `type`(ctype: `hkInt8`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: i8,
    /// # C++ Info
    /// -          name: `ownerOffset`(ctype: `hkInt8`)
    /// -        offset:   5(x86)/  5(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_ownerOffset: i8,
    /// # C++ Info
    /// -          name: `objectQualityType`(ctype: `hkInt8`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_objectQualityType: i8,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpTypedBroadPhaseHandle {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTypedBroadPhaseHandle"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf4b0f799)
        }
    }
    impl _serde::Serialize for hkpTypedBroadPhaseHandle {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf4b0f799)));
            let mut serializer = __serializer
                .serialize_struct("hkpTypedBroadPhaseHandle", class_meta)?;
            serializer.skip_field("id", &self.parent.m_id)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.skip_field("ownerOffset", &self.m_ownerOffset)?;
            serializer.serialize_field("objectQualityType", &self.m_objectQualityType)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_type,
    m_ownerOffset,
    m_objectQualityType,
    m_collisionFilterInfo,
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
            "type" => Ok(__Field::m_type),
            "objectQualityType" => Ok(__Field::m_objectQualityType),
            "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
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
pub(super) struct __hkpTypedBroadPhaseHandleVisitor<'de> {
    marker: core::marker::PhantomData<hkpTypedBroadPhaseHandle>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpTypedBroadPhaseHandleVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpTypedBroadPhaseHandle, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpTypedBroadPhaseHandle>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpTypedBroadPhaseHandleVisitor<'de> {
    type Value = hkpTypedBroadPhaseHandle;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpTypedBroadPhaseHandle")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_type: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_ownerOffset: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_objectQualityType: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_type) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("type"),
                        );
                    }
                    m_type = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_ownerOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ownerOffset",
                            ),
                        );
                    }
                    m_ownerOffset = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_objectQualityType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "objectQualityType",
                            ),
                        );
                    }
                    m_objectQualityType = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_collisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionFilterInfo",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_collisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
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
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_ownerOffset = match m_ownerOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ownerOffset"),
                );
            }
        };
        let m_objectQualityType = match m_objectQualityType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("objectQualityType"),
                );
            }
        };
        let m_collisionFilterInfo = match m_collisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "collisionFilterInfo",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpTypedBroadPhaseHandle {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_type,
            m_ownerOffset,
            m_objectQualityType,
            m_collisionFilterInfo,
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
        let parent = __hkpBroadPhaseHandleVisitor::visit_as_parent(&mut __map)?;
        let mut m_type: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_objectQualityType: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_type => {
                        if _serde::__private::Option::is_some(&m_type) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                            );
                        }
                        m_type = _serde::__private::Some(
                            match __A::next_value::<i8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_objectQualityType => {
                        if _serde::__private::Option::is_some(&m_objectQualityType) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "objectQualityType",
                                ),
                            );
                        }
                        m_objectQualityType = _serde::__private::Some(
                            match __A::next_value::<i8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_collisionFilterInfo => {
                        if _serde::__private::Option::is_some(&m_collisionFilterInfo) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "collisionFilterInfo",
                                ),
                            );
                        }
                        m_collisionFilterInfo = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
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
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_objectQualityType = match m_objectQualityType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("objectQualityType"),
                );
            }
        };
        let m_collisionFilterInfo = match m_collisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "collisionFilterInfo",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpTypedBroadPhaseHandle {
            __ptr,
            parent,
            m_type,
            m_objectQualityType,
            m_collisionFilterInfo,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpTypedBroadPhaseHandle {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "type",
                "ownerOffset",
                "objectQualityType",
                "collisionFilterInfo",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpTypedBroadPhaseHandle",
                FIELDS,
                __hkpTypedBroadPhaseHandleVisitor {
                    marker: _serde::__private::PhantomData::<hkpTypedBroadPhaseHandle>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
