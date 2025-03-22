use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkxNode`
/// - version: `1`
/// - signature: `0x5a218502`
/// - size: ` 72`(x86)/`112`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hkxAttributeHolder<'a>,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `object`(ctype: `struct hkReferencedObject*`)
    /// - offset: ` 24`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "object"))]
    #[cfg_attr(feature = "serde", serde(rename = "object"))]
    pub m_object: Pointer<'a>,
    /// # C++ Info
    /// - name: `keyFrames`(ctype: `hkArray<hkMatrix4>`)
    /// - offset: ` 28`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "keyFrames"))]
    #[cfg_attr(feature = "serde", serde(rename = "keyFrames"))]
    pub m_keyFrames: Vec<Matrix4>,
    /// # C++ Info
    /// - name: `children`(ctype: `hkArray<hkxNode*>`)
    /// - offset: ` 40`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "children"))]
    #[cfg_attr(feature = "serde", serde(rename = "children"))]
    pub m_children: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `annotations`(ctype: `hkArray<struct hkxNodeAnnotationData>`)
    /// - offset: ` 52`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "annotations"))]
    #[cfg_attr(feature = "serde", serde(rename = "annotations"))]
    pub m_annotations: Vec<hkxNodeAnnotationData<'a>>,
    /// # C++ Info
    /// - name: `userProperties`(ctype: `hkStringPtr`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userProperties"))]
    #[cfg_attr(feature = "serde", serde(rename = "userProperties"))]
    pub m_userProperties: StringPtr<'a>,
    /// # C++ Info
    /// - name: `selected`(ctype: `hkBool`)
    /// - offset: ` 68`(x86)/`104`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "selected"))]
    #[cfg_attr(feature = "serde", serde(rename = "selected"))]
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
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .parent
                    .m_attributeGroups
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.push(&self.m_object);
            v.extend(self.m_children.iter());
            v.extend(
                self
                    .m_annotations
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkxNode<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x5a218502)));
            let mut serializer = __serializer
                .serialize_struct("hkxNode", class_meta, (72u64, 112u64))?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "attributeGroups",
                    &self.parent.m_attributeGroups,
                    TypeSize::Struct {
                        size_x86: 16u64,
                        size_x86_64: 24u64,
                    },
                )?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("object", &self.m_object)?;
            serializer
                .serialize_array_field(
                    "keyFrames",
                    &self.m_keyFrames,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field("children", &self.m_children, TypeSize::NonPtr)?;
            serializer
                .serialize_array_field(
                    "annotations",
                    &self.m_annotations,
                    TypeSize::Struct {
                        size_x86: 8u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer.serialize_field("userProperties", &self.m_userProperties)?;
            serializer.serialize_field("selected", &self.m_selected)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxNode<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_attributeGroups,
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
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "attributeGroups" => Ok(__Field::m_attributeGroups),
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
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkxNodeVisitor<'de> {
                marker: _serde::__private::PhantomData<hkxNode<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkxNodeVisitor<'de> {
                type Value = hkxNode<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
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
                    let mut m_object: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_keyFrames: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
                    let mut m_children: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_annotations: _serde::__private::Option<
                        Vec<hkxNodeAnnotationData<'de>>,
                    > = _serde::__private::None;
                    let mut m_userProperties: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
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
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keyFrames",
                                ),
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotations",
                                ),
                            );
                        }
                    };
                    let m_userProperties = match m_userProperties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userProperties",
                                ),
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
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_attributeGroups: _serde::__private::Option<
                        Vec<hkxAttributeGroup<'de>>,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_object: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_keyFrames: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
                    let mut m_children: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_annotations: _serde::__private::Option<
                        Vec<hkxNodeAnnotationData<'de>>,
                    > = _serde::__private::None;
                    let mut m_userProperties: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_selected: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_attributeGroups => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_attributeGroups) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attributeGroups",
                                        ),
                                    );
                                }
                                m_attributeGroups = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkxAttributeGroup<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_object) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("object"),
                                    );
                                }
                                m_object = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_keyFrames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_keyFrames) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_children) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "children",
                                        ),
                                    );
                                }
                                m_children = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_annotations) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userProperties) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_selected) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_attributeGroups = match m_attributeGroups {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attributeGroups",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_object = match m_object {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("object"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_keyFrames = match m_keyFrames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keyFrames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_children = match m_children {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("children"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotations = match m_annotations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userProperties = match m_userProperties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userProperties",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_selected = match m_selected {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("selected"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hkxAttributeHolder {
                        __ptr: __ptr.clone(),
                        parent,
                        m_attributeGroups,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkxNode {
                        __ptr: __ptr.clone(),
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
