use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPhysicsSystemWithContacts`
/// -         version: `0`
/// -       signature: `0xd0fd4bbe`
/// -          size:  80(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPhysicsSystemWithContacts<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpPhysicsSystem<'a>,
    /// # C++ Info
    /// -          name: `contacts`(ctype: `hkArray<hkpSerializedAgentNnEntry*>`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_contacts: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpPhysicsSystemWithContacts<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPhysicsSystemWithContacts"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd0fd4bbe)
        }
    }
    impl<'a> _serde::Serialize for hkpPhysicsSystemWithContacts<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd0fd4bbe)));
            let mut serializer = __serializer
                .serialize_struct("hkpPhysicsSystemWithContacts", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("rigidBodies", &self.parent.m_rigidBodies)?;
            serializer
                .serialize_array_meta_field("constraints", &self.parent.m_constraints)?;
            serializer.serialize_array_meta_field("actions", &self.parent.m_actions)?;
            serializer.serialize_array_meta_field("phantoms", &self.parent.m_phantoms)?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("active", &self.parent.m_active)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_meta_field("contacts", &self.m_contacts)?;
            serializer.serialize_array_field("rigidBodies", &self.parent.m_rigidBodies)?;
            serializer.serialize_array_field("constraints", &self.parent.m_constraints)?;
            serializer.serialize_array_field("actions", &self.parent.m_actions)?;
            serializer.serialize_array_field("phantoms", &self.parent.m_phantoms)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("contacts", &self.m_contacts)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_contacts,
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
            "contacts" => Ok(__Field::m_contacts),
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
pub(super) struct __hkpPhysicsSystemWithContactsVisitor<'de> {
    marker: core::marker::PhantomData<hkpPhysicsSystemWithContacts<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPhysicsSystemWithContactsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpPhysicsSystemWithContacts<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpPhysicsSystemWithContacts<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkpPhysicsSystemWithContactsVisitor<'de> {
    type Value = hkpPhysicsSystemWithContacts<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpPhysicsSystemWithContacts",
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
        let mut m_contacts: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_contacts) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contacts",
                            ),
                        );
                    }
                    m_contacts = _serde::__private::Some(
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
        let m_contacts = match m_contacts {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contacts"),
                );
            }
        };
        _serde::__private::Ok(hkpPhysicsSystemWithContacts {
            __ptr,
            parent,
            m_contacts,
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
        let parent = __hkpPhysicsSystemVisitor::visit_as_parent(&mut __map)?;
        let mut m_contacts: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_contacts => {
                        if _serde::__private::Option::is_some(&m_contacts) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contacts",
                                ),
                            );
                        }
                        m_contacts = _serde::__private::Some(
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
        let m_contacts = match m_contacts {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contacts"),
                );
            }
        };
        _serde::__private::Ok(hkpPhysicsSystemWithContacts {
            __ptr,
            parent,
            m_contacts,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPhysicsSystemWithContacts<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["contacts"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPhysicsSystemWithContacts",
                FIELDS,
                __hkpPhysicsSystemWithContactsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpPhysicsSystemWithContacts,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
