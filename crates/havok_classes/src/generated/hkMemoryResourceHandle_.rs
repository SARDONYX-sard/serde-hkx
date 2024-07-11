use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryResourceHandle`
/// -         version: `2`
/// -       signature: `0xbffac086`
/// -          size:  28(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryResourceHandle<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkResourceHandle,
    /// # C++ Info
    /// -          name: `variant`(ctype: `struct hkReferencedObject*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_variant: Pointer,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `references`(ctype: `hkArray<struct hkMemoryResourceHandleExternalLink>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_references: Vec<hkMemoryResourceHandleExternalLink<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkMemoryResourceHandle<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMemoryResourceHandle"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xbffac086)
        }
    }
    impl<'a> _serde::Serialize for hkMemoryResourceHandle<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xbffac086)));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryResourceHandle", class_meta)?;
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
            serializer.serialize_field("variant", &self.m_variant)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_array_meta_field("references", &self.m_references)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("references", &self.m_references)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_variant,
    m_name,
    m_references,
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
            "variant" => Ok(__Field::m_variant),
            "name" => Ok(__Field::m_name),
            "references" => Ok(__Field::m_references),
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
pub(super) struct __hkMemoryResourceHandleVisitor<'de> {
    marker: core::marker::PhantomData<hkMemoryResourceHandle<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkMemoryResourceHandleVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkMemoryResourceHandle<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkMemoryResourceHandle<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkMemoryResourceHandleVisitor<'de> {
    type Value = hkMemoryResourceHandle<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkMemoryResourceHandle")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_variant: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_references: _serde::__private::Option<
            Vec<hkMemoryResourceHandleExternalLink<'de>>,
        > = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_variant) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("variant"),
                        );
                    }
                    m_variant = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_name) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                        );
                    }
                    m_name = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_references) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "references",
                            ),
                        );
                    }
                    m_references = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkMemoryResourceHandleExternalLink<'de>>,
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
        let m_variant = match m_variant {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("variant"),
                );
            }
        };
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_references = match m_references {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("references"),
                );
            }
        };
        _serde::__private::Ok(hkMemoryResourceHandle {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_variant,
            m_name,
            m_references,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkResourceHandleVisitor::visit_as_parent(&mut __map)?;
        let mut m_variant: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_references: _serde::__private::Option<
            Vec<hkMemoryResourceHandleExternalLink<'de>>,
        > = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_variant => {
                        if _serde::__private::Option::is_some(&m_variant) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "variant",
                                ),
                            );
                        }
                        m_variant = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_name => {
                        if _serde::__private::Option::is_some(&m_name) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                            );
                        }
                        m_name = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_references => {
                        if _serde::__private::Option::is_some(&m_references) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "references",
                                ),
                            );
                        }
                        m_references = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkMemoryResourceHandleExternalLink<'de>>,
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
        let m_variant = match m_variant {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("variant"),
                );
            }
        };
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_references = match m_references {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("references"),
                );
            }
        };
        _serde::__private::Ok(hkMemoryResourceHandle {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_variant,
            m_name,
            m_references,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkMemoryResourceHandle<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["variant", "name", "references"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkMemoryResourceHandle",
                FIELDS,
                __hkMemoryResourceHandleVisitor {
                    marker: _serde::__private::PhantomData::<hkMemoryResourceHandle>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
