use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbBehaviorGraphData`
/// - version: `2`
/// - signature: `0x95aca5d`
/// - size: ` 88`(x86)/`128`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorGraphData {
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
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// - name: `attributeDefaults`(ctype: `hkArray<hkReal>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "attributeDefaults"))]
    pub m_attributeDefaults: Vec<f32>,
    /// # C++ Info
    /// - name: `variableInfos`(ctype: `hkArray<struct hkbVariableInfo>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "variableInfos"))]
    pub m_variableInfos: Vec<hkbVariableInfo>,
    /// # C++ Info
    /// - name: `characterPropertyInfos`(ctype: `hkArray<struct hkbVariableInfo>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "characterPropertyInfos"))]
    pub m_characterPropertyInfos: Vec<hkbVariableInfo>,
    /// # C++ Info
    /// - name: `eventInfos`(ctype: `hkArray<struct hkbEventInfo>`)
    /// - offset: ` 44`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "eventInfos"))]
    pub m_eventInfos: Vec<hkbEventInfo>,
    /// # C++ Info
    /// - name: `wordMinVariableValues`(ctype: `hkArray<struct hkbVariableValue>`)
    /// - offset: ` 56`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "wordMinVariableValues"))]
    pub m_wordMinVariableValues: Vec<hkbVariableValue>,
    /// # C++ Info
    /// - name: `wordMaxVariableValues`(ctype: `hkArray<struct hkbVariableValue>`)
    /// - offset: ` 68`(x86)/` 96`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "wordMaxVariableValues"))]
    pub m_wordMaxVariableValues: Vec<hkbVariableValue>,
    /// # C++ Info
    /// - name: `variableInitialValues`(ctype: `struct hkbVariableValueSet*`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "variableInitialValues"))]
    pub m_variableInitialValues: Pointer,
    /// # C++ Info
    /// - name: `stringData`(ctype: `struct hkbBehaviorGraphStringData*`)
    /// - offset: ` 84`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "stringData"))]
    pub m_stringData: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBehaviorGraphData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBehaviorGraphData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x95aca5d)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_variableInfos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_characterPropertyInfos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_eventInfos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_wordMinVariableValues
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_wordMaxVariableValues
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.push(self.m_variableInitialValues.get());
            v.push(self.m_stringData.get());
            v
        }
    }
    impl _serde::Serialize for hkbBehaviorGraphData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x95aca5d)));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorGraphData", class_meta, (88u64, 128u64))?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "attributeDefaults",
                    &self.m_attributeDefaults,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "variableInfos",
                    &self.m_variableInfos,
                    TypeSize::Struct {
                        size_x86: 6u64,
                        size_x86_64: 6u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "characterPropertyInfos",
                    &self.m_characterPropertyInfos,
                    TypeSize::Struct {
                        size_x86: 6u64,
                        size_x86_64: 6u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "eventInfos",
                    &self.m_eventInfos,
                    TypeSize::Struct {
                        size_x86: 4u64,
                        size_x86_64: 4u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "wordMinVariableValues",
                    &self.m_wordMinVariableValues,
                    TypeSize::Struct {
                        size_x86: 4u64,
                        size_x86_64: 4u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "wordMaxVariableValues",
                    &self.m_wordMaxVariableValues,
                    TypeSize::Struct {
                        size_x86: 4u64,
                        size_x86_64: 4u64,
                    },
                )?;
            serializer
                .serialize_field(
                    "variableInitialValues",
                    &self.m_variableInitialValues,
                )?;
            serializer.serialize_field("stringData", &self.m_stringData)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbBehaviorGraphData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_attributeDefaults,
                m_variableInfos,
                m_characterPropertyInfos,
                m_eventInfos,
                m_wordMinVariableValues,
                m_wordMaxVariableValues,
                m_variableInitialValues,
                m_stringData,
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
                        "attributeDefaults" => Ok(__Field::m_attributeDefaults),
                        "variableInfos" => Ok(__Field::m_variableInfos),
                        "characterPropertyInfos" => Ok(__Field::m_characterPropertyInfos),
                        "eventInfos" => Ok(__Field::m_eventInfos),
                        "wordMinVariableValues" => Ok(__Field::m_wordMinVariableValues),
                        "wordMaxVariableValues" => Ok(__Field::m_wordMaxVariableValues),
                        "variableInitialValues" => Ok(__Field::m_variableInitialValues),
                        "stringData" => Ok(__Field::m_stringData),
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
            struct __hkbBehaviorGraphDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbBehaviorGraphData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbBehaviorGraphDataVisitor<'de> {
                type Value = hkbBehaviorGraphData;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbBehaviorGraphData",
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
                    let mut m_attributeDefaults: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_variableInfos: _serde::__private::Option<
                        Vec<hkbVariableInfo>,
                    > = _serde::__private::None;
                    let mut m_characterPropertyInfos: _serde::__private::Option<
                        Vec<hkbVariableInfo>,
                    > = _serde::__private::None;
                    let mut m_eventInfos: _serde::__private::Option<Vec<hkbEventInfo>> = _serde::__private::None;
                    let mut m_wordMinVariableValues: _serde::__private::Option<
                        Vec<hkbVariableValue>,
                    > = _serde::__private::None;
                    let mut m_wordMaxVariableValues: _serde::__private::Option<
                        Vec<hkbVariableValue>,
                    > = _serde::__private::None;
                    let mut m_variableInitialValues: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_stringData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    for i in 0..8usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_attributeDefaults,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attributeDefaults",
                                        ),
                                    );
                                }
                                m_attributeDefaults = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_variableInfos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableInfos",
                                        ),
                                    );
                                }
                                m_variableInfos = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableInfo>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyInfos,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyInfos",
                                        ),
                                    );
                                }
                                m_characterPropertyInfos = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableInfo>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_eventInfos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventInfos",
                                        ),
                                    );
                                }
                                m_eventInfos = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbEventInfo>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wordMinVariableValues,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wordMinVariableValues",
                                        ),
                                    );
                                }
                                m_wordMinVariableValues = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableValue>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wordMaxVariableValues,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wordMaxVariableValues",
                                        ),
                                    );
                                }
                                m_wordMaxVariableValues = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableValue>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_variableInitialValues,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableInitialValues",
                                        ),
                                    );
                                }
                                m_variableInitialValues = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_stringData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stringData",
                                        ),
                                    );
                                }
                                m_stringData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    let m_attributeDefaults = match m_attributeDefaults {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attributeDefaults",
                                ),
                            );
                        }
                    };
                    let m_variableInfos = match m_variableInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableInfos",
                                ),
                            );
                        }
                    };
                    let m_characterPropertyInfos = match m_characterPropertyInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyInfos",
                                ),
                            );
                        }
                    };
                    let m_eventInfos = match m_eventInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventInfos",
                                ),
                            );
                        }
                    };
                    let m_wordMinVariableValues = match m_wordMinVariableValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wordMinVariableValues",
                                ),
                            );
                        }
                    };
                    let m_wordMaxVariableValues = match m_wordMaxVariableValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wordMaxVariableValues",
                                ),
                            );
                        }
                    };
                    let m_variableInitialValues = match m_variableInitialValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableInitialValues",
                                ),
                            );
                        }
                    };
                    let m_stringData = match m_stringData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stringData",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbBehaviorGraphData {
                        __ptr,
                        parent,
                        m_attributeDefaults,
                        m_variableInfos,
                        m_characterPropertyInfos,
                        m_eventInfos,
                        m_wordMinVariableValues,
                        m_wordMaxVariableValues,
                        m_variableInitialValues,
                        m_stringData,
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
                    let mut m_attributeDefaults: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_variableInfos: _serde::__private::Option<
                        Vec<hkbVariableInfo>,
                    > = _serde::__private::None;
                    let mut m_characterPropertyInfos: _serde::__private::Option<
                        Vec<hkbVariableInfo>,
                    > = _serde::__private::None;
                    let mut m_eventInfos: _serde::__private::Option<Vec<hkbEventInfo>> = _serde::__private::None;
                    let mut m_wordMinVariableValues: _serde::__private::Option<
                        Vec<hkbVariableValue>,
                    > = _serde::__private::None;
                    let mut m_wordMaxVariableValues: _serde::__private::Option<
                        Vec<hkbVariableValue>,
                    > = _serde::__private::None;
                    let mut m_variableInitialValues: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_stringData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_attributeDefaults => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_attributeDefaults,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attributeDefaults",
                                        ),
                                    );
                                }
                                m_attributeDefaults = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_variableInfos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_variableInfos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableInfos",
                                        ),
                                    );
                                }
                                m_variableInfos = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableInfo>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterPropertyInfos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyInfos,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyInfos",
                                        ),
                                    );
                                }
                                m_characterPropertyInfos = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableInfo>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_eventInfos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eventInfos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventInfos",
                                        ),
                                    );
                                }
                                m_eventInfos = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbEventInfo>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wordMinVariableValues => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wordMinVariableValues,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wordMinVariableValues",
                                        ),
                                    );
                                }
                                m_wordMinVariableValues = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableValue>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wordMaxVariableValues => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wordMaxVariableValues,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wordMaxVariableValues",
                                        ),
                                    );
                                }
                                m_wordMaxVariableValues = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableValue>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_variableInitialValues => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableInitialValues,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableInitialValues",
                                        ),
                                    );
                                }
                                m_variableInitialValues = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stringData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_stringData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stringData",
                                        ),
                                    );
                                }
                                m_stringData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    let m_attributeDefaults = match m_attributeDefaults {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attributeDefaults",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_variableInfos = match m_variableInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableInfos",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterPropertyInfos = match m_characterPropertyInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyInfos",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_eventInfos = match m_eventInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventInfos",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wordMinVariableValues = match m_wordMinVariableValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wordMinVariableValues",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wordMaxVariableValues = match m_wordMaxVariableValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wordMaxVariableValues",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_variableInitialValues = match m_variableInitialValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableInitialValues",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stringData = match m_stringData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stringData",
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
                    _serde::__private::Ok(hkbBehaviorGraphData {
                        __ptr,
                        parent,
                        m_attributeDefaults,
                        m_variableInfos,
                        m_characterPropertyInfos,
                        m_eventInfos,
                        m_wordMinVariableValues,
                        m_wordMaxVariableValues,
                        m_variableInitialValues,
                        m_stringData,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "attributeDefaults",
                "variableInfos",
                "characterPropertyInfos",
                "eventInfos",
                "wordMinVariableValues",
                "wordMaxVariableValues",
                "variableInitialValues",
                "stringData",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBehaviorGraphData",
                FIELDS,
                __hkbBehaviorGraphDataVisitor {
                    marker: _serde::__private::PhantomData::<hkbBehaviorGraphData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
