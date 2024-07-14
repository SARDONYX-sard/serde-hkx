use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxNode`
/// -         version: `1`
/// -       signature: `0x5a218502`
/// -          size:  72(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxNode<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkxAttributeHolder<'a>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `object`(ctype: `struct hkReferencedObject*`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_object: Pointer,
    /// # C++ Info
    /// -          name: `keyFrames`(ctype: `hkArray<hkMatrix4>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_keyFrames: Vec<Matrix4>,
    /// # C++ Info
    /// -          name: `children`(ctype: `hkArray<hkxNode*>`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_children: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `annotations`(ctype: `hkArray<struct hkxNodeAnnotationData>`)
    /// -        offset:  52(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_annotations: Vec<hkxNodeAnnotationData<'a>>,
    /// # C++ Info
    /// -          name: `userProperties`(ctype: `hkStringPtr`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userProperties: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `selected`(ctype: `hkBool`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_selected: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxNode<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxNode"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5a218502)
        }
    }
    impl<'a> _serde::Serialize for hkxNode<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5a218502)));
            let mut serializer = __serializer.serialize_struct("hkxNode", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "attributeGroups",
                    &self.parent.m_attributeGroups,
                )?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("object", &self.m_object)?;
            serializer.serialize_array_meta_field("keyFrames", &self.m_keyFrames)?;
            serializer.serialize_array_meta_field("children", &self.m_children)?;
            serializer.serialize_array_meta_field("annotations", &self.m_annotations)?;
            serializer
                .serialize_stringptr_meta_field(
                    "userProperties",
                    &self.m_userProperties,
                )?;
            serializer.serialize_field("selected", &self.m_selected)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "attributeGroups",
                    &self.parent.m_attributeGroups,
                )?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("keyFrames", &self.m_keyFrames)?;
            serializer.serialize_array_field("children", &self.m_children)?;
            serializer.serialize_array_field("annotations", &self.m_annotations)?;
            serializer
                .serialize_stringptr_field("userProperties", &self.m_userProperties)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_name,
    m_object,
    m_keyFrames,
    m_children,
    m_annotations,
    m_userProperties,
    m_selected,
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
            "name" => Ok(__Field::m_name),
            "object" => Ok(__Field::m_object),
            "keyFrames" => Ok(__Field::m_keyFrames),
            "children" => Ok(__Field::m_children),
            "annotations" => Ok(__Field::m_annotations),
            "userProperties" => Ok(__Field::m_userProperties),
            "selected" => Ok(__Field::m_selected),
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
pub(super) struct __hkxNodeVisitor<'de> {
    marker: core::marker::PhantomData<hkxNode<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxNodeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxNode<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxNode<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxNodeVisitor<'de> {
    type Value = hkxNode<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxNode")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::parent_value(&mut __map)?;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_object: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_keyFrames: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
        let mut m_children: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_annotations: _serde::__private::Option<
            Vec<hkxNodeAnnotationData<'de>>,
        > = _serde::__private::None;
        let mut m_userProperties: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_selected: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
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
                1usize => {
                    if _serde::__private::Option::is_some(&m_object) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("object"),
                        );
                    }
                    m_object = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_keyFrames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "keyFrames",
                            ),
                        );
                    }
                    m_keyFrames = _serde::__private::Some(
                        match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_children) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "children",
                            ),
                        );
                    }
                    m_children = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_annotations) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "annotations",
                            ),
                        );
                    }
                    m_annotations = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkxNodeAnnotationData<'de>>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_userProperties) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userProperties",
                            ),
                        );
                    }
                    m_userProperties = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_selected) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "selected",
                            ),
                        );
                    }
                    m_selected = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
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
        __A::pad(&mut __map, 3usize, 7usize)?;
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_object = match m_object {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("object"),
                );
            }
        };
        let m_keyFrames = match m_keyFrames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("keyFrames"),
                );
            }
        };
        let m_children = match m_children {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("children"),
                );
            }
        };
        let m_annotations = match m_annotations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("annotations"),
                );
            }
        };
        let m_userProperties = match m_userProperties {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userProperties"),
                );
            }
        };
        let m_selected = match m_selected {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("selected"),
                );
            }
        };
        _serde::__private::Ok(hkxNode {
            __ptr,
            parent,
            m_name,
            m_object,
            m_keyFrames,
            m_children,
            m_annotations,
            m_userProperties,
            m_selected,
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
        let parent = __hkxAttributeHolderVisitor::visit_as_parent(&mut __map)?;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_object: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_keyFrames: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
        let mut m_children: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_annotations: _serde::__private::Option<
            Vec<hkxNodeAnnotationData<'de>>,
        > = _serde::__private::None;
        let mut m_userProperties: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_selected: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..7usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
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
                    __Field::m_object => {
                        if _serde::__private::Option::is_some(&m_object) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("object"),
                            );
                        }
                        m_object = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_keyFrames => {
                        if _serde::__private::Option::is_some(&m_keyFrames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "keyFrames",
                                ),
                            );
                        }
                        m_keyFrames = _serde::__private::Some(
                            match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_children => {
                        if _serde::__private::Option::is_some(&m_children) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "children",
                                ),
                            );
                        }
                        m_children = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_annotations => {
                        if _serde::__private::Option::is_some(&m_annotations) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "annotations",
                                ),
                            );
                        }
                        m_annotations = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkxNodeAnnotationData<'de>>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_userProperties => {
                        if _serde::__private::Option::is_some(&m_userProperties) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "userProperties",
                                ),
                            );
                        }
                        m_userProperties = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_selected => {
                        if _serde::__private::Option::is_some(&m_selected) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "selected",
                                ),
                            );
                        }
                        m_selected = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
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
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_object = match m_object {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("object"),
                );
            }
        };
        let m_keyFrames = match m_keyFrames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("keyFrames"),
                );
            }
        };
        let m_children = match m_children {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("children"),
                );
            }
        };
        let m_annotations = match m_annotations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("annotations"),
                );
            }
        };
        let m_userProperties = match m_userProperties {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userProperties"),
                );
            }
        };
        let m_selected = match m_selected {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("selected"),
                );
            }
        };
        _serde::__private::Ok(hkxNode {
            __ptr,
            parent,
            m_name,
            m_object,
            m_keyFrames,
            m_children,
            m_annotations,
            m_userProperties,
            m_selected,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxNode<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "name",
                "object",
                "keyFrames",
                "children",
                "annotations",
                "userProperties",
                "selected",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxNode",
                FIELDS,
                __hkxNodeVisitor {
                    marker: _serde::__private::PhantomData::<hkxNode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
