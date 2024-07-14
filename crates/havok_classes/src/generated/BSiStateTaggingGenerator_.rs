use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSiStateTaggingGenerator`
/// -         version: `1`
/// -       signature: `0xf0826fc1`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSiStateTaggingGenerator<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// -          name: `pDefaultGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_pDefaultGenerator: Pointer,
    /// # C++ Info
    /// -          name: `iStateToSetAs`(ctype: `hkInt32`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_iStateToSetAs: i32,
    /// # C++ Info
    /// -          name: `iPriority`(ctype: `hkInt32`)
    /// -        offset:  56(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_iPriority: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSiStateTaggingGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSiStateTaggingGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf0826fc1)
        }
    }
    impl<'a> _serde::Serialize for BSiStateTaggingGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf0826fc1)));
            let mut serializer = __serializer
                .serialize_struct("BSiStateTaggingGenerator", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("pDefaultGenerator", &self.m_pDefaultGenerator)?;
            serializer.serialize_field("iStateToSetAs", &self.m_iStateToSetAs)?;
            serializer.serialize_field("iPriority", &self.m_iPriority)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_pDefaultGenerator,
    m_iStateToSetAs,
    m_iPriority,
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
            "pDefaultGenerator" => Ok(__Field::m_pDefaultGenerator),
            "iStateToSetAs" => Ok(__Field::m_iStateToSetAs),
            "iPriority" => Ok(__Field::m_iPriority),
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
pub(super) struct __BSiStateTaggingGeneratorVisitor<'de> {
    marker: core::marker::PhantomData<BSiStateTaggingGenerator<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSiStateTaggingGeneratorVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSiStateTaggingGenerator<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSiStateTaggingGenerator<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSiStateTaggingGeneratorVisitor<'de> {
    type Value = BSiStateTaggingGenerator<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSiStateTaggingGenerator")
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
        let mut m_pDefaultGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_iStateToSetAs: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_iPriority: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_pDefaultGenerator) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pDefaultGenerator",
                            ),
                        );
                    }
                    m_pDefaultGenerator = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_iStateToSetAs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "iStateToSetAs",
                            ),
                        );
                    }
                    m_iStateToSetAs = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_iPriority) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "iPriority",
                            ),
                        );
                    }
                    m_iPriority = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        __A::pad(&mut __map, 4usize, 0usize)?;
        let m_pDefaultGenerator = match m_pDefaultGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pDefaultGenerator"),
                );
            }
        };
        let m_iStateToSetAs = match m_iStateToSetAs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("iStateToSetAs"),
                );
            }
        };
        let m_iPriority = match m_iPriority {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("iPriority"),
                );
            }
        };
        _serde::__private::Ok(BSiStateTaggingGenerator {
            __ptr,
            parent,
            m_pDefaultGenerator,
            m_iStateToSetAs,
            m_iPriority,
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
        let parent = __hkbGeneratorVisitor::visit_as_parent(&mut __map)?;
        let mut m_pDefaultGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_iStateToSetAs: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_iPriority: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_pDefaultGenerator => {
                        if _serde::__private::Option::is_some(&m_pDefaultGenerator) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pDefaultGenerator",
                                ),
                            );
                        }
                        m_pDefaultGenerator = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_iStateToSetAs => {
                        if _serde::__private::Option::is_some(&m_iStateToSetAs) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "iStateToSetAs",
                                ),
                            );
                        }
                        m_iStateToSetAs = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_iPriority => {
                        if _serde::__private::Option::is_some(&m_iPriority) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "iPriority",
                                ),
                            );
                        }
                        m_iPriority = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
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
        let m_pDefaultGenerator = match m_pDefaultGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pDefaultGenerator"),
                );
            }
        };
        let m_iStateToSetAs = match m_iStateToSetAs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("iStateToSetAs"),
                );
            }
        };
        let m_iPriority = match m_iPriority {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("iPriority"),
                );
            }
        };
        _serde::__private::Ok(BSiStateTaggingGenerator {
            __ptr,
            parent,
            m_pDefaultGenerator,
            m_iStateToSetAs,
            m_iPriority,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSiStateTaggingGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["pDefaultGenerator", "iStateToSetAs", "iPriority"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSiStateTaggingGenerator",
                FIELDS,
                __BSiStateTaggingGeneratorVisitor {
                    marker: _serde::__private::PhantomData::<BSiStateTaggingGenerator>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
