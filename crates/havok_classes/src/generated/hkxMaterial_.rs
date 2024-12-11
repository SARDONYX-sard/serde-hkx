use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkxMaterial`
/// - version: `1`
/// - signature: `0x2954537a`
/// - size: `144`(x86)/`176`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMaterial<'a> {
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
    pub __ptr: Option<Pointer>,
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
    /// - name: `stages`(ctype: `hkArray<struct hkxMaterialTextureStage>`)
    /// - offset: ` 24`(x86)/` 40`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "stages"))]
    #[cfg_attr(feature = "serde", serde(rename = "stages"))]
    pub m_stages: Vec<hkxMaterialTextureStage>,
    /// # C++ Info
    /// - name: `diffuseColor`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 64`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "diffuseColor"))]
    #[cfg_attr(feature = "serde", serde(rename = "diffuseColor"))]
    pub m_diffuseColor: Vector4,
    /// # C++ Info
    /// - name: `ambientColor`(ctype: `hkVector4`)
    /// - offset: ` 64`(x86)/` 80`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "ambientColor"))]
    #[cfg_attr(feature = "serde", serde(rename = "ambientColor"))]
    pub m_ambientColor: Vector4,
    /// # C++ Info
    /// - name: `specularColor`(ctype: `hkVector4`)
    /// - offset: ` 80`(x86)/` 96`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "specularColor"))]
    #[cfg_attr(feature = "serde", serde(rename = "specularColor"))]
    pub m_specularColor: Vector4,
    /// # C++ Info
    /// - name: `emissiveColor`(ctype: `hkVector4`)
    /// - offset: ` 96`(x86)/`112`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "emissiveColor"))]
    #[cfg_attr(feature = "serde", serde(rename = "emissiveColor"))]
    pub m_emissiveColor: Vector4,
    /// # C++ Info
    /// - name: `subMaterials`(ctype: `hkArray<hkxMaterial*>`)
    /// - offset: `112`(x86)/`128`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "subMaterials"))]
    #[cfg_attr(feature = "serde", serde(rename = "subMaterials"))]
    pub m_subMaterials: Vec<Pointer>,
    /// # C++ Info
    /// - name: `extraData`(ctype: `struct hkReferencedObject*`)
    /// - offset: `124`(x86)/`144`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "extraData"))]
    #[cfg_attr(feature = "serde", serde(rename = "extraData"))]
    pub m_extraData: Pointer,
    /// # C++ Info
    /// - name: `properties`(ctype: `hkArray<struct hkxMaterialProperty>`)
    /// - offset: `128`(x86)/`152`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "properties"))]
    #[cfg_attr(feature = "serde", serde(rename = "properties"))]
    pub m_properties: Vec<hkxMaterialProperty>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxMaterial<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxMaterial"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2954537a)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(
                self
                    .parent
                    .m_attributeGroups
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_stages
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(self.m_subMaterials.iter().map(|ptr| ptr.get()));
            v.push(self.m_extraData.get());
            v.extend(
                self
                    .m_properties
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkxMaterial<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x2954537a)));
            let mut serializer = __serializer
                .serialize_struct("hkxMaterial", class_meta, (144u64, 176u64))?;
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
            serializer
                .serialize_array_field(
                    "stages",
                    &self.m_stages,
                    TypeSize::Struct {
                        size_x86: 12u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("diffuseColor", &self.m_diffuseColor)?;
            serializer.serialize_field("ambientColor", &self.m_ambientColor)?;
            serializer.serialize_field("specularColor", &self.m_specularColor)?;
            serializer.serialize_field("emissiveColor", &self.m_emissiveColor)?;
            serializer
                .serialize_array_field(
                    "subMaterials",
                    &self.m_subMaterials,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("extraData", &self.m_extraData)?;
            serializer
                .serialize_array_field(
                    "properties",
                    &self.m_properties,
                    TypeSize::Struct {
                        size_x86: 8u64,
                        size_x86_64: 8u64,
                    },
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxMaterial<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_attributeGroups,
                m_name,
                m_stages,
                m_diffuseColor,
                m_ambientColor,
                m_specularColor,
                m_emissiveColor,
                m_subMaterials,
                m_extraData,
                m_properties,
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
                        "stages" => Ok(__Field::m_stages),
                        "diffuseColor" => Ok(__Field::m_diffuseColor),
                        "ambientColor" => Ok(__Field::m_ambientColor),
                        "specularColor" => Ok(__Field::m_specularColor),
                        "emissiveColor" => Ok(__Field::m_emissiveColor),
                        "subMaterials" => Ok(__Field::m_subMaterials),
                        "extraData" => Ok(__Field::m_extraData),
                        "properties" => Ok(__Field::m_properties),
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
            struct __hkxMaterialVisitor<'de> {
                marker: _serde::__private::PhantomData<hkxMaterial<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkxMaterialVisitor<'de> {
                type Value = hkxMaterial<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkxMaterial")
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
                    let mut m_stages: _serde::__private::Option<
                        Vec<hkxMaterialTextureStage>,
                    > = _serde::__private::None;
                    let mut m_diffuseColor: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ambientColor: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_specularColor: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_emissiveColor: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_subMaterials: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_extraData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_properties: _serde::__private::Option<
                        Vec<hkxMaterialProperty>,
                    > = _serde::__private::None;
                    for i in 0..9usize {
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
                                if _serde::__private::Option::is_some(&m_stages) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("stages"),
                                    );
                                }
                                m_stages = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkxMaterialTextureStage>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_diffuseColor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "diffuseColor",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 12usize, 8usize)?;
                                m_diffuseColor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_ambientColor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ambientColor",
                                        ),
                                    );
                                }
                                m_ambientColor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_specularColor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "specularColor",
                                        ),
                                    );
                                }
                                m_specularColor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_emissiveColor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "emissiveColor",
                                        ),
                                    );
                                }
                                m_emissiveColor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_subMaterials) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "subMaterials",
                                        ),
                                    );
                                }
                                m_subMaterials = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_extraData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extraData",
                                        ),
                                    );
                                }
                                m_extraData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_properties) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "properties",
                                        ),
                                    );
                                }
                                m_properties = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkxMaterialProperty>,
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
                    __A::pad(&mut __map, 4usize, 8usize)?;
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_stages = match m_stages {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("stages"),
                            );
                        }
                    };
                    let m_diffuseColor = match m_diffuseColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "diffuseColor",
                                ),
                            );
                        }
                    };
                    let m_ambientColor = match m_ambientColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ambientColor",
                                ),
                            );
                        }
                    };
                    let m_specularColor = match m_specularColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "specularColor",
                                ),
                            );
                        }
                    };
                    let m_emissiveColor = match m_emissiveColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "emissiveColor",
                                ),
                            );
                        }
                    };
                    let m_subMaterials = match m_subMaterials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "subMaterials",
                                ),
                            );
                        }
                    };
                    let m_extraData = match m_extraData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extraData",
                                ),
                            );
                        }
                    };
                    let m_properties = match m_properties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "properties",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkxMaterial {
                        __ptr,
                        parent,
                        m_name,
                        m_stages,
                        m_diffuseColor,
                        m_ambientColor,
                        m_specularColor,
                        m_emissiveColor,
                        m_subMaterials,
                        m_extraData,
                        m_properties,
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
                    let mut m_stages: _serde::__private::Option<
                        Vec<hkxMaterialTextureStage>,
                    > = _serde::__private::None;
                    let mut m_diffuseColor: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ambientColor: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_specularColor: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_emissiveColor: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_subMaterials: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_extraData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_properties: _serde::__private::Option<
                        Vec<hkxMaterialProperty>,
                    > = _serde::__private::None;
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
                            __Field::m_stages => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_stages) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("stages"),
                                    );
                                }
                                m_stages = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkxMaterialTextureStage>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_diffuseColor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_diffuseColor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "diffuseColor",
                                        ),
                                    );
                                }
                                m_diffuseColor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ambientColor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ambientColor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ambientColor",
                                        ),
                                    );
                                }
                                m_ambientColor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_specularColor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_specularColor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "specularColor",
                                        ),
                                    );
                                }
                                m_specularColor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_emissiveColor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_emissiveColor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "emissiveColor",
                                        ),
                                    );
                                }
                                m_emissiveColor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_subMaterials => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_subMaterials) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "subMaterials",
                                        ),
                                    );
                                }
                                m_subMaterials = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_extraData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_extraData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extraData",
                                        ),
                                    );
                                }
                                m_extraData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_properties => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_properties) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "properties",
                                        ),
                                    );
                                }
                                m_properties = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkxMaterialProperty>,
                                    >(&mut __map) {
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
                    let m_stages = match m_stages {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("stages"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_diffuseColor = match m_diffuseColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "diffuseColor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ambientColor = match m_ambientColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ambientColor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_specularColor = match m_specularColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "specularColor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_emissiveColor = match m_emissiveColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "emissiveColor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_subMaterials = match m_subMaterials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "subMaterials",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_extraData = match m_extraData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extraData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_properties = match m_properties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "properties",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkxAttributeHolder {
                        __ptr,
                        parent,
                        m_attributeGroups,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkxMaterial {
                        __ptr,
                        parent,
                        m_name,
                        m_stages,
                        m_diffuseColor,
                        m_ambientColor,
                        m_specularColor,
                        m_emissiveColor,
                        m_subMaterials,
                        m_extraData,
                        m_properties,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "name",
                "stages",
                "diffuseColor",
                "ambientColor",
                "specularColor",
                "emissiveColor",
                "subMaterials",
                "extraData",
                "properties",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxMaterial",
                FIELDS,
                __hkxMaterialVisitor {
                    marker: _serde::__private::PhantomData::<hkxMaterial>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `TextureType`(ctype: `hkEnum<TextureType, hkInt32>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum TextureType {
    #[default]
    TEX_UNKNOWN = 0isize,
    TEX_DIFFUSE = 1isize,
    TEX_REFLECTION = 2isize,
    TEX_BUMP = 3isize,
    TEX_NORMAL = 4isize,
    TEX_DISPLACEMENT = 5isize,
    TEX_SPECULAR = 6isize,
    TEX_SPECULARANDGLOSS = 7isize,
    TEX_OPACITY = 8isize,
    TEX_EMISSIVE = 9isize,
    TEX_REFRACTION = 10isize,
    TEX_GLOSS = 11isize,
    TEX_NOTEXPORTED = 12isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for TextureType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TEX_UNKNOWN => __serializer.serialize_field("TEX_UNKNOWN", &0u64),
                Self::TEX_DIFFUSE => __serializer.serialize_field("TEX_DIFFUSE", &1u64),
                Self::TEX_REFLECTION => {
                    __serializer.serialize_field("TEX_REFLECTION", &2u64)
                }
                Self::TEX_BUMP => __serializer.serialize_field("TEX_BUMP", &3u64),
                Self::TEX_NORMAL => __serializer.serialize_field("TEX_NORMAL", &4u64),
                Self::TEX_DISPLACEMENT => {
                    __serializer.serialize_field("TEX_DISPLACEMENT", &5u64)
                }
                Self::TEX_SPECULAR => __serializer.serialize_field("TEX_SPECULAR", &6u64),
                Self::TEX_SPECULARANDGLOSS => {
                    __serializer.serialize_field("TEX_SPECULARANDGLOSS", &7u64)
                }
                Self::TEX_OPACITY => __serializer.serialize_field("TEX_OPACITY", &8u64),
                Self::TEX_EMISSIVE => __serializer.serialize_field("TEX_EMISSIVE", &9u64),
                Self::TEX_REFRACTION => {
                    __serializer.serialize_field("TEX_REFRACTION", &10u64)
                }
                Self::TEX_GLOSS => __serializer.serialize_field("TEX_GLOSS", &11u64),
                Self::TEX_NOTEXPORTED => {
                    __serializer.serialize_field("TEX_NOTEXPORTED", &12u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i32()
                .ok_or(S::Error::custom("Failed enum TextureType to_i32"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TextureType {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __field11,
                __field12,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int32<__E>(
                    self,
                    __value: i32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i32 => _serde::__private::Ok(__Field::__field0),
                        1i32 => _serde::__private::Ok(__Field::__field1),
                        2i32 => _serde::__private::Ok(__Field::__field2),
                        3i32 => _serde::__private::Ok(__Field::__field3),
                        4i32 => _serde::__private::Ok(__Field::__field4),
                        5i32 => _serde::__private::Ok(__Field::__field5),
                        6i32 => _serde::__private::Ok(__Field::__field6),
                        7i32 => _serde::__private::Ok(__Field::__field7),
                        8i32 => _serde::__private::Ok(__Field::__field8),
                        9i32 => _serde::__private::Ok(__Field::__field9),
                        10i32 => _serde::__private::Ok(__Field::__field10),
                        11i32 => _serde::__private::Ok(__Field::__field11),
                        12i32 => _serde::__private::Ok(__Field::__field12),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int32(__value),
                                    &"value(i32) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0" || v.eq_ignore_ascii_case("TEX_UNKNOWN") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("TEX_DIFFUSE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("TEX_REFLECTION") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("TEX_BUMP") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("TEX_NORMAL") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("TEX_DISPLACEMENT") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6" || v.eq_ignore_ascii_case("TEX_SPECULAR") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("TEX_SPECULARANDGLOSS") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("TEX_OPACITY") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "9" || v.eq_ignore_ascii_case("TEX_EMISSIVE") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "10"
                                || v.eq_ignore_ascii_case("TEX_REFRACTION") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "11" || v.eq_ignore_ascii_case("TEX_GLOSS") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "12"
                                || v.eq_ignore_ascii_case("TEX_NOTEXPORTED") => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int32,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TextureType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TextureType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum TextureType",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_UNKNOWN)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_DIFFUSE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_REFLECTION)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_BUMP)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_NORMAL)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_DISPLACEMENT)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_SPECULAR)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_SPECULARANDGLOSS)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_OPACITY)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_EMISSIVE)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_REFRACTION)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_GLOSS)
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_NOTEXPORTED)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "TEX_UNKNOWN",
                "TEX_DIFFUSE",
                "TEX_REFLECTION",
                "TEX_BUMP",
                "TEX_NORMAL",
                "TEX_DISPLACEMENT",
                "TEX_SPECULAR",
                "TEX_SPECULARANDGLOSS",
                "TEX_OPACITY",
                "TEX_EMISSIVE",
                "TEX_REFRACTION",
                "TEX_GLOSS",
                "TEX_NOTEXPORTED",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TextureType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TextureType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
