use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryResourceHandleExternalLink`
/// -         version: `1`
/// -       signature: `0x3144d17c`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryResourceHandleExternalLink<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `memberName`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_memberName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `externalId`(ctype: `hkStringPtr`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_externalId: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkMemoryResourceHandleExternalLink<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMemoryResourceHandleExternalLink"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3144d17c)
        }
    }
    impl<'a> _serde::Serialize for hkMemoryResourceHandleExternalLink<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3144d17c)));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryResourceHandleExternalLink", class_meta)?;
            serializer.serialize_stringptr_meta_field("memberName", &self.m_memberName)?;
            serializer.serialize_stringptr_meta_field("externalId", &self.m_externalId)?;
            serializer.serialize_stringptr_field("memberName", &self.m_memberName)?;
            serializer.serialize_stringptr_field("externalId", &self.m_externalId)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_memberName,
    m_externalId,
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
            "memberName" => Ok(__Field::m_memberName),
            "externalId" => Ok(__Field::m_externalId),
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
pub(super) struct __hkMemoryResourceHandleExternalLinkVisitor<'de> {
    marker: core::marker::PhantomData<hkMemoryResourceHandleExternalLink<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkMemoryResourceHandleExternalLinkVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkMemoryResourceHandleExternalLink<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkMemoryResourceHandleExternalLink<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkMemoryResourceHandleExternalLinkVisitor<'de> {
    type Value = hkMemoryResourceHandleExternalLink<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkMemoryResourceHandleExternalLink",
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
        let mut m_memberName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_externalId: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_memberName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "memberName",
                            ),
                        );
                    }
                    m_memberName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_externalId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "externalId",
                            ),
                        );
                    }
                    m_externalId = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        let m_memberName = match m_memberName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("memberName"),
                );
            }
        };
        let m_externalId = match m_externalId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("externalId"),
                );
            }
        };
        _serde::__private::Ok(hkMemoryResourceHandleExternalLink {
            __ptr,
            m_memberName,
            m_externalId,
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
        let mut m_memberName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_externalId: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_memberName => {
                        if _serde::__private::Option::is_some(&m_memberName) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "memberName",
                                ),
                            );
                        }
                        m_memberName = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_externalId => {
                        if _serde::__private::Option::is_some(&m_externalId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "externalId",
                                ),
                            );
                        }
                        m_externalId = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        let m_memberName = match m_memberName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("memberName"),
                );
            }
        };
        let m_externalId = match m_externalId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("externalId"),
                );
            }
        };
        _serde::__private::Ok(hkMemoryResourceHandleExternalLink {
            __ptr,
            m_memberName,
            m_externalId,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkMemoryResourceHandleExternalLink<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["memberName", "externalId"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkMemoryResourceHandleExternalLink",
                FIELDS,
                __hkMemoryResourceHandleExternalLinkVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkMemoryResourceHandleExternalLink,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
