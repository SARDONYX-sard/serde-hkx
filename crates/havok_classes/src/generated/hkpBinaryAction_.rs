use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBinaryAction`
/// -         version: `0`
/// -       signature: `0xc00f3403`
/// -          size:  32(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBinaryAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpAction<'a>,
    /// # C++ Info
    /// -          name: `entityA`(ctype: `struct hkpEntity*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_entityA: Pointer,
    /// # C++ Info
    /// -          name: `entityB`(ctype: `struct hkpEntity*`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_entityB: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpBinaryAction<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBinaryAction"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc00f3403)
        }
    }
    impl<'a> _serde::Serialize for hkpBinaryAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc00f3403)));
            let mut serializer = __serializer
                .serialize_struct("hkpBinaryAction", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.m_world)?;
            serializer.skip_field("island", &self.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("entityA", &self.m_entityA)?;
            serializer.serialize_field("entityB", &self.m_entityB)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_entityA,
    m_entityB,
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
            "entityA" => Ok(__Field::m_entityA),
            "entityB" => Ok(__Field::m_entityB),
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
pub(super) struct __hkpBinaryActionVisitor<'de> {
    marker: core::marker::PhantomData<hkpBinaryAction<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpBinaryActionVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpBinaryAction<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpBinaryAction<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpBinaryActionVisitor<'de> {
    type Value = hkpBinaryAction<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpBinaryAction")
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
        let mut m_entityA: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_entityB: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_entityA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("entityA"),
                        );
                    }
                    m_entityA = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_entityB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("entityB"),
                        );
                    }
                    m_entityB = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
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
        let m_entityA = match m_entityA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("entityA"),
                );
            }
        };
        let m_entityB = match m_entityB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("entityB"),
                );
            }
        };
        _serde::__private::Ok(hkpBinaryAction {
            __ptr,
            parent,
            m_entityA,
            m_entityB,
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
        let parent = __hkpActionVisitor::visit_as_parent(&mut __map)?;
        let mut m_entityA: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_entityB: _serde::__private::Option<Pointer> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_entityA => {
                        if _serde::__private::Option::is_some(&m_entityA) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "entityA",
                                ),
                            );
                        }
                        m_entityA = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_entityB => {
                        if _serde::__private::Option::is_some(&m_entityB) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "entityB",
                                ),
                            );
                        }
                        m_entityB = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
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
        let m_entityA = match m_entityA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("entityA"),
                );
            }
        };
        let m_entityB = match m_entityB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("entityB"),
                );
            }
        };
        _serde::__private::Ok(hkpBinaryAction {
            __ptr,
            parent,
            m_entityA,
            m_entityB,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpBinaryAction<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["entityA", "entityB"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpBinaryAction",
                FIELDS,
                __hkpBinaryActionVisitor {
                    marker: _serde::__private::PhantomData::<hkpBinaryAction>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
