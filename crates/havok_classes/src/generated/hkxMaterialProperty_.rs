use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxMaterialProperty`
/// -         version: `0`
/// -       signature: `0xd295234d`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMaterialProperty {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `key`(ctype: `hkUint32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_key: u32,
    /// # C++ Info
    /// -          name: `value`(ctype: `hkUint32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_value: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxMaterialProperty {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxMaterialProperty"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd295234d)
        }
    }
    impl _serde::Serialize for hkxMaterialProperty {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd295234d)));
            let mut serializer = __serializer
                .serialize_struct("hkxMaterialProperty", class_meta)?;
            serializer.serialize_field("key", &self.m_key)?;
            serializer.serialize_field("value", &self.m_value)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_key,
    m_value,
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
            "key" => Ok(__Field::m_key),
            "value" => Ok(__Field::m_value),
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
pub(super) struct __hkxMaterialPropertyVisitor<'de> {
    marker: core::marker::PhantomData<hkxMaterialProperty>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxMaterialPropertyVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxMaterialProperty, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxMaterialProperty>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxMaterialPropertyVisitor<'de> {
    type Value = hkxMaterialProperty;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxMaterialProperty")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_key: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_value: _serde::__private::Option<u32> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_key) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("key"),
                        );
                    }
                    m_key = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_value) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("value"),
                        );
                    }
                    m_value = _serde::__private::Some(
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
        let m_key = match m_key {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("key"),
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
        _serde::__private::Ok(hkxMaterialProperty {
            __ptr: __A::class_ptr(&mut __map),
            m_key,
            m_value,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_key: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_value: _serde::__private::Option<u32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_key => {
                    if _serde::__private::Option::is_some(&m_key) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("key"),
                        );
                    }
                    m_key = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
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
        let m_key = match m_key {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("key"),
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
        _serde::__private::Ok(hkxMaterialProperty {
            __ptr: __A::class_ptr(&mut __map),
            m_key,
            m_value,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxMaterialProperty {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["key", "value"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxMaterialProperty",
                FIELDS,
                __hkxMaterialPropertyVisitor {
                    marker: _serde::__private::PhantomData::<hkxMaterialProperty>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
