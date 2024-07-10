use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpGroupFilter`
/// -         version: `0`
/// -       signature: `0x65ee88e4`
/// -          size: 256(x86)/272(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpGroupFilter {
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
    /// -          name: `nextFreeSystemGroup`(ctype: `hkInt32`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_nextFreeSystemGroup: i32,
    /// # C++ Info
    /// -          name: `collisionLookupTable`(ctype: `hkUint32[32]`)
    /// -        offset:  52(x86)/ 76(x86_64)
    /// -     type_size: 128(x86)/128(x86_64)
    ///
    pub m_collisionLookupTable: [u32; 32usize],
    /// # C++ Info
    /// -          name: `pad256`(ctype: `hkVector4[4]`)
    /// -        offset: 192(x86)/208(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_pad256: [Vector4; 4usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpGroupFilter {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpGroupFilter"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x65ee88e4)
        }
    }
    impl _serde::Serialize for hkpGroupFilter {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x65ee88e4)));
            let mut serializer = __serializer
                .serialize_struct("hkpGroupFilter", class_meta)?;
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
                .serialize_field("nextFreeSystemGroup", &self.m_nextFreeSystemGroup)?;
            serializer
                .serialize_field(
                    "collisionLookupTable",
                    &self.m_collisionLookupTable.as_slice(),
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("pad256", &self.m_pad256.as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_nextFreeSystemGroup,
    m_collisionLookupTable,
    m_pad256,
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
            "nextFreeSystemGroup" => Ok(__Field::m_nextFreeSystemGroup),
            "collisionLookupTable" => Ok(__Field::m_collisionLookupTable),
            "pad256" => Ok(__Field::m_pad256),
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
pub(super) struct __hkpGroupFilterVisitor<'de> {
    marker: core::marker::PhantomData<hkpGroupFilter>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpGroupFilterVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpGroupFilter, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpGroupFilter>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpGroupFilterVisitor<'de> {
    type Value = hkpGroupFilter;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpGroupFilter")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_nextFreeSystemGroup: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_collisionLookupTable: _serde::__private::Option<[u32; 32usize]> = _serde::__private::None;
        let mut m_pad256: _serde::__private::Option<[Vector4; 4usize]> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_nextFreeSystemGroup) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nextFreeSystemGroup",
                            ),
                        );
                    }
                    m_nextFreeSystemGroup = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_collisionLookupTable) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionLookupTable",
                            ),
                        );
                    }
                    m_collisionLookupTable = _serde::__private::Some(
                        match __A::next_value::<[u32; 32usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_pad256) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pad256"),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 4usize)?;
                    m_pad256 = _serde::__private::Some(
                        match __A::next_value::<[Vector4; 4usize]>(&mut __map) {
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
        let m_nextFreeSystemGroup = match m_nextFreeSystemGroup {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "nextFreeSystemGroup",
                    ),
                );
            }
        };
        let m_collisionLookupTable = match m_collisionLookupTable {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "collisionLookupTable",
                    ),
                );
            }
        };
        let m_pad256 = match m_pad256 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad256"),
                );
            }
        };
        _serde::__private::Ok(hkpGroupFilter {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_nextFreeSystemGroup,
            m_collisionLookupTable,
            m_pad256,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpCollisionFilterVisitor::visit_as_parent(&mut __map)?;
        let mut m_nextFreeSystemGroup: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_collisionLookupTable: _serde::__private::Option<[u32; 32usize]> = _serde::__private::None;
        let mut m_pad256: _serde::__private::Option<[Vector4; 4usize]> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_nextFreeSystemGroup => {
                    if _serde::__private::Option::is_some(&m_nextFreeSystemGroup) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nextFreeSystemGroup",
                            ),
                        );
                    }
                    m_nextFreeSystemGroup = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_collisionLookupTable => {
                    if _serde::__private::Option::is_some(&m_collisionLookupTable) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionLookupTable",
                            ),
                        );
                    }
                    m_collisionLookupTable = _serde::__private::Some(
                        match __A::next_value::<[u32; 32usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_pad256 => {
                    if _serde::__private::Option::is_some(&m_pad256) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pad256"),
                        );
                    }
                    m_pad256 = _serde::__private::Some(
                        match __A::next_value::<[Vector4; 4usize]>(&mut __map) {
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
        let m_nextFreeSystemGroup = match m_nextFreeSystemGroup {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "nextFreeSystemGroup",
                    ),
                );
            }
        };
        let m_collisionLookupTable = match m_collisionLookupTable {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "collisionLookupTable",
                    ),
                );
            }
        };
        let m_pad256 = match m_pad256 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad256"),
                );
            }
        };
        _serde::__private::Ok(hkpGroupFilter {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_nextFreeSystemGroup,
            m_collisionLookupTable,
            m_pad256,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpGroupFilter {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "nextFreeSystemGroup",
                "collisionLookupTable",
                "pad256",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpGroupFilter",
                FIELDS,
                __hkpGroupFilterVisitor {
                    marker: _serde::__private::PhantomData::<hkpGroupFilter>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
