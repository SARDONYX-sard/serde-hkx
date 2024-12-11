use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkxMaterialShader`
/// - version: `1`
/// - signature: `0x28515eff`
/// - size: ` 40`(x86)/` 72`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMaterialShader<'a> {
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
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `type`(ctype: `enum ShaderType`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "type"))]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub m_type: ShaderType,
    /// # C++ Info
    /// - name: `vertexEntryName`(ctype: `hkStringPtr`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertexEntryName"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertexEntryName"))]
    pub m_vertexEntryName: StringPtr<'a>,
    /// # C++ Info
    /// - name: `geomEntryName`(ctype: `hkStringPtr`)
    /// - offset: ` 20`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "geomEntryName"))]
    #[cfg_attr(feature = "serde", serde(rename = "geomEntryName"))]
    pub m_geomEntryName: StringPtr<'a>,
    /// # C++ Info
    /// - name: `pixelEntryName`(ctype: `hkStringPtr`)
    /// - offset: ` 24`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pixelEntryName"))]
    #[cfg_attr(feature = "serde", serde(rename = "pixelEntryName"))]
    pub m_pixelEntryName: StringPtr<'a>,
    /// # C++ Info
    /// - name: `data`(ctype: `hkArray<hkUint8>`)
    /// - offset: ` 28`(x86)/` 56`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "data"))]
    #[cfg_attr(feature = "serde", serde(rename = "data"))]
    pub m_data: Vec<u8>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxMaterialShader<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxMaterialShader"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x28515eff)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkxMaterialShader<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x28515eff)));
            let mut serializer = __serializer
                .serialize_struct("hkxMaterialShader", class_meta, (40u64, 72u64))?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("vertexEntryName", &self.m_vertexEntryName)?;
            serializer.serialize_field("geomEntryName", &self.m_geomEntryName)?;
            serializer.serialize_field("pixelEntryName", &self.m_pixelEntryName)?;
            serializer.serialize_array_field("data", &self.m_data, TypeSize::NonPtr)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxMaterialShader<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_type,
                m_vertexEntryName,
                m_geomEntryName,
                m_pixelEntryName,
                m_data,
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
                        "name" => Ok(__Field::m_name),
                        "type" => Ok(__Field::m_type),
                        "vertexEntryName" => Ok(__Field::m_vertexEntryName),
                        "geomEntryName" => Ok(__Field::m_geomEntryName),
                        "pixelEntryName" => Ok(__Field::m_pixelEntryName),
                        "data" => Ok(__Field::m_data),
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
            struct __hkxMaterialShaderVisitor<'de> {
                marker: _serde::__private::PhantomData<hkxMaterialShader<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkxMaterialShaderVisitor<'de> {
                type Value = hkxMaterialShader<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkxMaterialShader",
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
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_type: _serde::__private::Option<ShaderType> = _serde::__private::None;
                    let mut m_vertexEntryName: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_geomEntryName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_pixelEntryName: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    for i in 0..6usize {
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
                                if _serde::__private::Option::is_some(&m_type) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<ShaderType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_vertexEntryName) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexEntryName",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 7usize)?;
                                m_vertexEntryName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_geomEntryName) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "geomEntryName",
                                        ),
                                    );
                                }
                                m_geomEntryName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_pixelEntryName) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pixelEntryName",
                                        ),
                                    );
                                }
                                m_pixelEntryName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_data) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
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
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                        }
                    };
                    let m_vertexEntryName = match m_vertexEntryName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexEntryName",
                                ),
                            );
                        }
                    };
                    let m_geomEntryName = match m_geomEntryName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "geomEntryName",
                                ),
                            );
                        }
                    };
                    let m_pixelEntryName = match m_pixelEntryName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pixelEntryName",
                                ),
                            );
                        }
                    };
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkxMaterialShader {
                        __ptr,
                        parent,
                        m_name,
                        m_type,
                        m_vertexEntryName,
                        m_geomEntryName,
                        m_pixelEntryName,
                        m_data,
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
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_type: _serde::__private::Option<ShaderType> = _serde::__private::None;
                    let mut m_vertexEntryName: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_geomEntryName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_pixelEntryName: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
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
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<ShaderType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_vertexEntryName => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertexEntryName) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexEntryName",
                                        ),
                                    );
                                }
                                m_vertexEntryName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_geomEntryName => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_geomEntryName) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "geomEntryName",
                                        ),
                                    );
                                }
                                m_geomEntryName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pixelEntryName => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_pixelEntryName) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pixelEntryName",
                                        ),
                                    );
                                }
                                m_pixelEntryName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_data => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_data) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
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
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_vertexEntryName = match m_vertexEntryName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexEntryName",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_geomEntryName = match m_geomEntryName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "geomEntryName",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pixelEntryName = match m_pixelEntryName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pixelEntryName",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkxMaterialShader {
                        __ptr,
                        parent,
                        m_name,
                        m_type,
                        m_vertexEntryName,
                        m_geomEntryName,
                        m_pixelEntryName,
                        m_data,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "name",
                "type",
                "vertexEntryName",
                "geomEntryName",
                "pixelEntryName",
                "data",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxMaterialShader",
                FIELDS,
                __hkxMaterialShaderVisitor {
                    marker: _serde::__private::PhantomData::<hkxMaterialShader>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `ShaderType`(ctype: `hkEnum<ShaderType, hkUint8>`)
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
pub enum ShaderType {
    #[default]
    EFFECT_TYPE_INVALID = 0isize,
    EFFECT_TYPE_UNKNOWN = 1isize,
    EFFECT_TYPE_HLSL_INLINE = 2isize,
    EFFECT_TYPE_CG_INLINE = 3isize,
    EFFECT_TYPE_HLSL_FILENAME = 4isize,
    EFFECT_TYPE_CG_FILENAME = 5isize,
    EFFECT_TYPE_MAX_ID = 6isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ShaderType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::EFFECT_TYPE_INVALID => {
                    __serializer.serialize_field("EFFECT_TYPE_INVALID", &0u64)
                }
                Self::EFFECT_TYPE_UNKNOWN => {
                    __serializer.serialize_field("EFFECT_TYPE_UNKNOWN", &1u64)
                }
                Self::EFFECT_TYPE_HLSL_INLINE => {
                    __serializer.serialize_field("EFFECT_TYPE_HLSL_INLINE", &2u64)
                }
                Self::EFFECT_TYPE_CG_INLINE => {
                    __serializer.serialize_field("EFFECT_TYPE_CG_INLINE", &3u64)
                }
                Self::EFFECT_TYPE_HLSL_FILENAME => {
                    __serializer.serialize_field("EFFECT_TYPE_HLSL_FILENAME", &4u64)
                }
                Self::EFFECT_TYPE_CG_FILENAME => {
                    __serializer.serialize_field("EFFECT_TYPE_CG_FILENAME", &5u64)
                }
                Self::EFFECT_TYPE_MAX_ID => {
                    __serializer.serialize_field("EFFECT_TYPE_MAX_ID", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ShaderType to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for ShaderType {
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
                fn visit_uint8<__E>(
                    self,
                    __value: u8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u8 => _serde::__private::Ok(__Field::__field0),
                        1u8 => _serde::__private::Ok(__Field::__field1),
                        2u8 => _serde::__private::Ok(__Field::__field2),
                        3u8 => _serde::__private::Ok(__Field::__field3),
                        4u8 => _serde::__private::Ok(__Field::__field4),
                        5u8 => _serde::__private::Ok(__Field::__field5),
                        6u8 => _serde::__private::Ok(__Field::__field6),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 5, 6",
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
                            v if v == "0"
                                || v.eq_ignore_ascii_case("EFFECT_TYPE_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("EFFECT_TYPE_UNKNOWN") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("EFFECT_TYPE_HLSL_INLINE") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("EFFECT_TYPE_CG_INLINE") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("EFFECT_TYPE_HLSL_FILENAME") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("EFFECT_TYPE_CG_FILENAME") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("EFFECT_TYPE_MAX_ID") => {
                                _serde::__private::Ok(__Field::__field6)
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
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<ShaderType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ShaderType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ShaderType",
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
                            _serde::__private::Ok(ShaderType::EFFECT_TYPE_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ShaderType::EFFECT_TYPE_UNKNOWN)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ShaderType::EFFECT_TYPE_HLSL_INLINE)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ShaderType::EFFECT_TYPE_CG_INLINE)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ShaderType::EFFECT_TYPE_HLSL_FILENAME)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ShaderType::EFFECT_TYPE_CG_FILENAME)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ShaderType::EFFECT_TYPE_MAX_ID)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "EFFECT_TYPE_INVALID",
                "EFFECT_TYPE_UNKNOWN",
                "EFFECT_TYPE_HLSL_INLINE",
                "EFFECT_TYPE_CG_INLINE",
                "EFFECT_TYPE_HLSL_FILENAME",
                "EFFECT_TYPE_CG_FILENAME",
                "EFFECT_TYPE_MAX_ID",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ShaderType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ShaderType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
