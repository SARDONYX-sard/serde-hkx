use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbBehaviorGraphStringData`
/// - version: `1`
/// - signature: `0xc713064e`
/// - size: ` 56`(x86)/` 80`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorGraphStringData<'a> {
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
    /// - name: `eventNames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "eventNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "eventNames"))]
    pub m_eventNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `attributeNames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "attributeNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "attributeNames"))]
    pub m_attributeNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `variableNames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "variableNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "variableNames"))]
    pub m_variableNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `characterPropertyNames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: ` 44`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterPropertyNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterPropertyNames"))]
    pub m_characterPropertyNames: Vec<StringPtr<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbBehaviorGraphStringData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBehaviorGraphStringData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc713064e)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkbBehaviorGraphStringData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc713064e)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbBehaviorGraphStringData",
                    class_meta,
                    (56u64, 80u64),
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "eventNames",
                    &self.m_eventNames,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "attributeNames",
                    &self.m_attributeNames,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "variableNames",
                    &self.m_variableNames,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "characterPropertyNames",
                    &self.m_characterPropertyNames,
                    TypeSize::String,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbBehaviorGraphStringData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_eventNames,
                m_attributeNames,
                m_variableNames,
                m_characterPropertyNames,
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
                        "eventNames" => Ok(__Field::m_eventNames),
                        "attributeNames" => Ok(__Field::m_attributeNames),
                        "variableNames" => Ok(__Field::m_variableNames),
                        "characterPropertyNames" => Ok(__Field::m_characterPropertyNames),
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
            struct __hkbBehaviorGraphStringDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbBehaviorGraphStringData<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbBehaviorGraphStringDataVisitor<'de> {
                type Value = hkbBehaviorGraphStringData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbBehaviorGraphStringData",
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
                    let mut m_eventNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_attributeNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_variableNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_characterPropertyNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_eventNames) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventNames",
                                        ),
                                    );
                                }
                                m_eventNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_attributeNames) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attributeNames",
                                        ),
                                    );
                                }
                                m_attributeNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_variableNames) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableNames",
                                        ),
                                    );
                                }
                                m_variableNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyNames,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyNames",
                                        ),
                                    );
                                }
                                m_characterPropertyNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
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
                    let m_eventNames = match m_eventNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventNames",
                                ),
                            );
                        }
                    };
                    let m_attributeNames = match m_attributeNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attributeNames",
                                ),
                            );
                        }
                    };
                    let m_variableNames = match m_variableNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableNames",
                                ),
                            );
                        }
                    };
                    let m_characterPropertyNames = match m_characterPropertyNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyNames",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbBehaviorGraphStringData {
                        __ptr,
                        parent,
                        m_eventNames,
                        m_attributeNames,
                        m_variableNames,
                        m_characterPropertyNames,
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
                    let mut m_eventNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_attributeNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_variableNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_characterPropertyNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_eventNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eventNames) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventNames",
                                        ),
                                    );
                                }
                                m_eventNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_attributeNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_attributeNames) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attributeNames",
                                        ),
                                    );
                                }
                                m_attributeNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_variableNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_variableNames) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableNames",
                                        ),
                                    );
                                }
                                m_variableNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterPropertyNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyNames,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyNames",
                                        ),
                                    );
                                }
                                m_characterPropertyNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
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
                    let m_eventNames = match m_eventNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_attributeNames = match m_attributeNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attributeNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_variableNames = match m_variableNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterPropertyNames = match m_characterPropertyNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyNames",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbBehaviorGraphStringData {
                        __ptr,
                        parent,
                        m_eventNames,
                        m_attributeNames,
                        m_variableNames,
                        m_characterPropertyNames,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "eventNames",
                "attributeNames",
                "variableNames",
                "characterPropertyNames",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBehaviorGraphStringData",
                FIELDS,
                __hkbBehaviorGraphStringDataVisitor {
                    marker: _serde::__private::PhantomData::<hkbBehaviorGraphStringData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
