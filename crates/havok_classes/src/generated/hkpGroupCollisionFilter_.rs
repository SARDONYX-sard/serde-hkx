use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpGroupCollisionFilter`
/// -         version: `0`
/// -       signature: `0x5cc01561`
/// -          size: 180(x86)/208(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpGroupCollisionFilter {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpCollisionFilter,
    /// # C++ Info
    /// -          name: `noGroupCollisionEnabled`(ctype: `hkBool`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_noGroupCollisionEnabled: bool,
    /// # C++ Info
    /// -          name: `collisionGroups`(ctype: `hkUint32[32]`)
    /// -        offset:  52(x86)/ 76(x86_64)
    /// -     type_size: 128(x86)/128(x86_64)
    ///
    pub m_collisionGroups: [u32; 32usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpGroupCollisionFilter {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpGroupCollisionFilter"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5cc01561)
        }
    }
    impl _serde::Serialize for hkpGroupCollisionFilter {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5cc01561)));
            let mut serializer = __serializer
                .serialize_struct("hkpGroupCollisionFilter", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 16usize].as_slice(), [0u8; 32usize].as_slice())?;
            serializer.serialize_field("prepad", &self.parent.m_prepad.as_slice())?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("postpad", &self.parent.m_postpad.as_slice())?;
            serializer
                .serialize_field(
                    "noGroupCollisionEnabled",
                    &self.m_noGroupCollisionEnabled,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field("collisionGroups", &self.m_collisionGroups.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_noGroupCollisionEnabled,
    m_collisionGroups,
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
            "noGroupCollisionEnabled" => Ok(__Field::m_noGroupCollisionEnabled),
            "collisionGroups" => Ok(__Field::m_collisionGroups),
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
pub(super) struct __hkpGroupCollisionFilterVisitor<'de> {
    marker: core::marker::PhantomData<hkpGroupCollisionFilter>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpGroupCollisionFilterVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpGroupCollisionFilter, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpGroupCollisionFilter>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpGroupCollisionFilterVisitor<'de> {
    type Value = hkpGroupCollisionFilter;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpGroupCollisionFilter")
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
        let mut m_noGroupCollisionEnabled: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_collisionGroups: _serde::__private::Option<[u32; 32usize]> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_noGroupCollisionEnabled) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "noGroupCollisionEnabled",
                            ),
                        );
                    }
                    m_noGroupCollisionEnabled = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_collisionGroups) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionGroups",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_collisionGroups = _serde::__private::Some(
                        match __A::next_value::<[u32; 32usize]>(&mut __map) {
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
        let m_noGroupCollisionEnabled = match m_noGroupCollisionEnabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "noGroupCollisionEnabled",
                    ),
                );
            }
        };
        let m_collisionGroups = match m_collisionGroups {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("collisionGroups"),
                );
            }
        };
        _serde::__private::Ok(hkpGroupCollisionFilter {
            __ptr,
            parent,
            m_noGroupCollisionEnabled,
            m_collisionGroups,
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
        let parent = __hkpCollisionFilterVisitor::visit_as_parent(&mut __map)?;
        let mut m_noGroupCollisionEnabled: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_collisionGroups: _serde::__private::Option<[u32; 32usize]> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_noGroupCollisionEnabled => {
                        if _serde::__private::Option::is_some(
                            &m_noGroupCollisionEnabled,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "noGroupCollisionEnabled",
                                ),
                            );
                        }
                        m_noGroupCollisionEnabled = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_collisionGroups => {
                        if _serde::__private::Option::is_some(&m_collisionGroups) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "collisionGroups",
                                ),
                            );
                        }
                        m_collisionGroups = _serde::__private::Some(
                            match __A::next_value::<[u32; 32usize]>(&mut __map) {
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
        let m_noGroupCollisionEnabled = match m_noGroupCollisionEnabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "noGroupCollisionEnabled",
                    ),
                );
            }
        };
        let m_collisionGroups = match m_collisionGroups {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("collisionGroups"),
                );
            }
        };
        _serde::__private::Ok(hkpGroupCollisionFilter {
            __ptr,
            parent,
            m_noGroupCollisionEnabled,
            m_collisionGroups,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpGroupCollisionFilter {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["noGroupCollisionEnabled", "collisionGroups"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpGroupCollisionFilter",
                FIELDS,
                __hkpGroupCollisionFilterVisitor {
                    marker: _serde::__private::PhantomData::<hkpGroupCollisionFilter>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
