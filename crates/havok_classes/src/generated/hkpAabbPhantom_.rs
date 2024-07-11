use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpAabbPhantom`
/// -         version: `0`
/// -       signature: `0x2c5189dd`
/// -          size: 224(x86)/304(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAabbPhantom<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpPhantom<'a>,
    /// # C++ Info
    /// -          name: `aabb`(ctype: `struct hkAabb`)
    /// -        offset: 176(x86)/240(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_aabb: hkAabb,
    /// # C++ Info
    /// -          name: `overlappingCollidables`(ctype: `hkArray<void*>`)
    /// -        offset: 208(x86)/272(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_overlappingCollidables: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `orderDirty`(ctype: `hkBool`)
    /// -        offset: 220(x86)/288(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_orderDirty: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpAabbPhantom<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpAabbPhantom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2c5189dd)
        }
    }
    impl<'a> _serde::Serialize for hkpAabbPhantom<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x2c5189dd)));
            let mut serializer = __serializer
                .serialize_struct("hkpAabbPhantom", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("collidable", &self.parent.parent.m_collidable)?;
            serializer
                .serialize_field(
                    "multiThreadCheck",
                    &self.parent.parent.m_multiThreadCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_meta_field(
                    "properties",
                    &self.parent.parent.m_properties,
                )?;
            serializer.skip_field("treeData", &self.parent.parent.m_treeData)?;
            serializer
                .skip_array_meta_field(
                    "overlapListeners",
                    &self.parent.m_overlapListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "phantomListeners",
                    &self.parent.m_phantomListeners,
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("aabb", &self.m_aabb)?;
            serializer
                .skip_array_meta_field(
                    "overlappingCollidables",
                    &self.m_overlappingCollidables,
                )?;
            serializer.skip_field("orderDirty", &self.m_orderDirty)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field("properties", &self.parent.parent.m_properties)?;
            serializer
                .serialize_array_field(
                    "overlapListeners",
                    &self.parent.m_overlapListeners,
                )?;
            serializer
                .serialize_array_field(
                    "phantomListeners",
                    &self.parent.m_phantomListeners,
                )?;
            serializer
                .serialize_array_field(
                    "overlappingCollidables",
                    &self.m_overlappingCollidables,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_aabb,
    m_overlappingCollidables,
    m_orderDirty,
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
            "aabb" => Ok(__Field::m_aabb),
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
pub(super) struct __hkpAabbPhantomVisitor<'de> {
    marker: core::marker::PhantomData<hkpAabbPhantom<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpAabbPhantomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpAabbPhantom<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpAabbPhantom<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpAabbPhantomVisitor<'de> {
    type Value = hkpAabbPhantom<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpAabbPhantom")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_aabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
        let mut m_overlappingCollidables: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_orderDirty: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_aabb) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("aabb"),
                        );
                    }
                    m_aabb = _serde::__private::Some(
                        match __A::next_value::<hkAabb>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_overlappingCollidables) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "overlappingCollidables",
                            ),
                        );
                    }
                    m_overlappingCollidables = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_orderDirty) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "orderDirty",
                            ),
                        );
                    }
                    m_orderDirty = _serde::__private::Some(
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
        __A::pad(&mut __map, 3usize, 15usize)?;
        let m_aabb = match m_aabb {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("aabb"),
                );
            }
        };
        let m_overlappingCollidables = match m_overlappingCollidables {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "overlappingCollidables",
                    ),
                );
            }
        };
        let m_orderDirty = match m_orderDirty {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("orderDirty"),
                );
            }
        };
        _serde::__private::Ok(hkpAabbPhantom {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_aabb,
            m_overlappingCollidables,
            m_orderDirty,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpPhantomVisitor::visit_as_parent(&mut __map)?;
        let mut m_aabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_aabb => {
                        if _serde::__private::Option::is_some(&m_aabb) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("aabb"),
                            );
                        }
                        m_aabb = _serde::__private::Some(
                            match __A::next_value::<hkAabb>(&mut __map) {
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
        let m_aabb = match m_aabb {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("aabb"),
                );
            }
        };
        _serde::__private::Ok(hkpAabbPhantom {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_aabb,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpAabbPhantom<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["aabb", "overlappingCollidables", "orderDirty"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpAabbPhantom",
                FIELDS,
                __hkpAabbPhantomVisitor {
                    marker: _serde::__private::PhantomData::<hkpAabbPhantom>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
