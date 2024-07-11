use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkCustomAttributes`
/// -         version: `0`
/// -       signature: `0xbff19005`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkCustomAttributes<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `attributes`(ctype: `hkSimpleArray<struct hkCustomAttributesAttribute>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/ 12(x86_64)
    ///
    pub m_attributes: Vec<hkCustomAttributesAttribute<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkCustomAttributes<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkCustomAttributes"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xbff19005)
        }
    }
    impl<'a> _serde::Serialize for hkCustomAttributes<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xbff19005)));
            let mut serializer = __serializer
                .serialize_struct("hkCustomAttributes", class_meta)?;
            serializer.serialize_field("attributes", &self.m_attributes)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_attributes,
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
            "attributes" => Ok(__Field::m_attributes),
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
pub(super) struct __hkCustomAttributesVisitor<'de> {
    marker: core::marker::PhantomData<hkCustomAttributes<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkCustomAttributesVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkCustomAttributes<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkCustomAttributes<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkCustomAttributesVisitor<'de> {
    type Value = hkCustomAttributes<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkCustomAttributes")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_attributes: _serde::__private::Option<
            Vec<hkCustomAttributesAttribute<'de>>,
        > = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_attributes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attributes",
                            ),
                        );
                    }
                    m_attributes = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkCustomAttributesAttribute<'de>>,
                        >(&mut __map) {
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
        let m_attributes = match m_attributes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attributes"),
                );
            }
        };
        _serde::__private::Ok(hkCustomAttributes {
            __ptr: __A::class_ptr(&mut __map),
            m_attributes,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_attributes: _serde::__private::Option<
            Vec<hkCustomAttributesAttribute<'de>>,
        > = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_attributes => {
                        if _serde::__private::Option::is_some(&m_attributes) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "attributes",
                                ),
                            );
                        }
                        m_attributes = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkCustomAttributesAttribute<'de>>,
                            >(&mut __map) {
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
        let m_attributes = match m_attributes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attributes"),
                );
            }
        };
        _serde::__private::Ok(hkCustomAttributes {
            __ptr: __A::class_ptr(&mut __map),
            m_attributes,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkCustomAttributes<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["attributes"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkCustomAttributes",
                FIELDS,
                __hkCustomAttributesVisitor {
                    marker: _serde::__private::PhantomData::<hkCustomAttributes>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
