use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCollisionFilterList`
/// -         version: `0`
/// -       signature: `0x2603bf04`
/// -          size:  60(x86)/ 88(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCollisionFilterList {
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
    /// -          name: `collisionFilters`(ctype: `hkArray<hkpCollisionFilter*>`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_collisionFilters: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCollisionFilterList {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCollisionFilterList"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2603bf04)
        }
    }
    impl _serde::Serialize for hkpCollisionFilterList {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x2603bf04)));
            let mut serializer = __serializer
                .serialize_struct("hkpCollisionFilterList", class_meta)?;
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
                .serialize_array_meta_field(
                    "collisionFilters",
                    &self.m_collisionFilters,
                )?;
            serializer
                .serialize_array_field("collisionFilters", &self.m_collisionFilters)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_collisionFilters,
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
            "collisionFilters" => Ok(__Field::m_collisionFilters),
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
pub(super) struct __hkpCollisionFilterListVisitor<'de> {
    marker: core::marker::PhantomData<hkpCollisionFilterList>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpCollisionFilterListVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpCollisionFilterList, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpCollisionFilterList>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpCollisionFilterListVisitor<'de> {
    type Value = hkpCollisionFilterList;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpCollisionFilterList")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_collisionFilters: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_collisionFilters) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionFilters",
                            ),
                        );
                    }
                    m_collisionFilters = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_collisionFilters = match m_collisionFilters {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("collisionFilters"),
                );
            }
        };
        _serde::__private::Ok(hkpCollisionFilterList {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_collisionFilters,
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
        let mut m_collisionFilters: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_collisionFilters => {
                        if _serde::__private::Option::is_some(&m_collisionFilters) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "collisionFilters",
                                ),
                            );
                        }
                        m_collisionFilters = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_collisionFilters = match m_collisionFilters {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("collisionFilters"),
                );
            }
        };
        _serde::__private::Ok(hkpCollisionFilterList {
            __ptr,
            parent,
            m_collisionFilters,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCollisionFilterList {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["collisionFilters"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCollisionFilterList",
                FIELDS,
                __hkpCollisionFilterListVisitor {
                    marker: _serde::__private::PhantomData::<hkpCollisionFilterList>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
