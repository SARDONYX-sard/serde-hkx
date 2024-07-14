use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaAnimationContainer`
/// -         version: `1`
/// -       signature: `0x8dc20333`
/// -          size:  68(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnimationContainer {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `skeletons`(ctype: `hkArray<hkaSkeleton*>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_skeletons: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `animations`(ctype: `hkArray<hkaAnimation*>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_animations: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `bindings`(ctype: `hkArray<hkaAnimationBinding*>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bindings: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `attachments`(ctype: `hkArray<hkaBoneAttachment*>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_attachments: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `skins`(ctype: `hkArray<hkaMeshBinding*>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_skins: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaAnimationContainer {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaAnimationContainer"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x8dc20333)
        }
    }
    impl _serde::Serialize for hkaAnimationContainer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x8dc20333)));
            let mut serializer = __serializer
                .serialize_struct("hkaAnimationContainer", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("skeletons", &self.m_skeletons)?;
            serializer.serialize_array_meta_field("animations", &self.m_animations)?;
            serializer.serialize_array_meta_field("bindings", &self.m_bindings)?;
            serializer.serialize_array_meta_field("attachments", &self.m_attachments)?;
            serializer.serialize_array_meta_field("skins", &self.m_skins)?;
            serializer.serialize_array_field("skeletons", &self.m_skeletons)?;
            serializer.serialize_array_field("animations", &self.m_animations)?;
            serializer.serialize_array_field("bindings", &self.m_bindings)?;
            serializer.serialize_array_field("attachments", &self.m_attachments)?;
            serializer.serialize_array_field("skins", &self.m_skins)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_skeletons,
    m_animations,
    m_bindings,
    m_attachments,
    m_skins,
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
            "skeletons" => Ok(__Field::m_skeletons),
            "animations" => Ok(__Field::m_animations),
            "bindings" => Ok(__Field::m_bindings),
            "attachments" => Ok(__Field::m_attachments),
            "skins" => Ok(__Field::m_skins),
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
pub(super) struct __hkaAnimationContainerVisitor<'de> {
    marker: core::marker::PhantomData<hkaAnimationContainer>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaAnimationContainerVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkaAnimationContainer, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkaAnimationContainer>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkaAnimationContainerVisitor<'de> {
    type Value = hkaAnimationContainer;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkaAnimationContainer")
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
        let mut m_skeletons: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_animations: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_bindings: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_attachments: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_skins: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_skeletons) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "skeletons",
                            ),
                        );
                    }
                    m_skeletons = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_animations) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "animations",
                            ),
                        );
                    }
                    m_animations = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_bindings) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bindings",
                            ),
                        );
                    }
                    m_bindings = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_attachments) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attachments",
                            ),
                        );
                    }
                    m_attachments = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_skins) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("skins"),
                        );
                    }
                    m_skins = _serde::__private::Some(
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
        let m_skeletons = match m_skeletons {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skeletons"),
                );
            }
        };
        let m_animations = match m_animations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("animations"),
                );
            }
        };
        let m_bindings = match m_bindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bindings"),
                );
            }
        };
        let m_attachments = match m_attachments {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attachments"),
                );
            }
        };
        let m_skins = match m_skins {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skins"),
                );
            }
        };
        _serde::__private::Ok(hkaAnimationContainer {
            __ptr,
            parent,
            m_skeletons,
            m_animations,
            m_bindings,
            m_attachments,
            m_skins,
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
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_skeletons: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_animations: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_bindings: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_attachments: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_skins: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for _ in 0..5usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_skeletons => {
                        if _serde::__private::Option::is_some(&m_skeletons) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "skeletons",
                                ),
                            );
                        }
                        m_skeletons = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_animations => {
                        if _serde::__private::Option::is_some(&m_animations) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "animations",
                                ),
                            );
                        }
                        m_animations = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bindings => {
                        if _serde::__private::Option::is_some(&m_bindings) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bindings",
                                ),
                            );
                        }
                        m_bindings = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_attachments => {
                        if _serde::__private::Option::is_some(&m_attachments) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "attachments",
                                ),
                            );
                        }
                        m_attachments = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_skins => {
                        if _serde::__private::Option::is_some(&m_skins) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("skins"),
                            );
                        }
                        m_skins = _serde::__private::Some(
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
        let m_skeletons = match m_skeletons {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skeletons"),
                );
            }
        };
        let m_animations = match m_animations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("animations"),
                );
            }
        };
        let m_bindings = match m_bindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bindings"),
                );
            }
        };
        let m_attachments = match m_attachments {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attachments"),
                );
            }
        };
        let m_skins = match m_skins {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skins"),
                );
            }
        };
        _serde::__private::Ok(hkaAnimationContainer {
            __ptr,
            parent,
            m_skeletons,
            m_animations,
            m_bindings,
            m_attachments,
            m_skins,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaAnimationContainer {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "skeletons",
                "animations",
                "bindings",
                "attachments",
                "skins",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaAnimationContainer",
                FIELDS,
                __hkaAnimationContainerVisitor {
                    marker: _serde::__private::PhantomData::<hkaAnimationContainer>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
