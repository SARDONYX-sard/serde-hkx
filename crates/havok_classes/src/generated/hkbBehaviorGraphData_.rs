use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorGraphData`
/// -         version: `2`
/// -       signature: `0x95aca5d`
/// -          size:  88(x86)/128(x86_64)
/// -          vtable: true
///
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
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `attributeDefaults`(ctype: `hkArray<hkReal>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_attributeDefaults: Vec<f32>,
    /// # C++ Info
    /// -          name: `variableInfos`(ctype: `hkArray<struct hkbVariableInfo>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_variableInfos: Vec<hkbVariableInfo>,
    /// # C++ Info
    /// -          name: `characterPropertyInfos`(ctype: `hkArray<struct hkbVariableInfo>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_characterPropertyInfos: Vec<hkbVariableInfo>,
    /// # C++ Info
    /// -          name: `eventInfos`(ctype: `hkArray<struct hkbEventInfo>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_eventInfos: Vec<hkbEventInfo>,
    /// # C++ Info
    /// -          name: `wordMinVariableValues`(ctype: `hkArray<struct hkbVariableValue>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wordMinVariableValues: Vec<hkbVariableValue>,
    /// # C++ Info
    /// -          name: `wordMaxVariableValues`(ctype: `hkArray<struct hkbVariableValue>`)
    /// -        offset:  68(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wordMaxVariableValues: Vec<hkbVariableValue>,
    /// # C++ Info
    /// -          name: `variableInitialValues`(ctype: `struct hkbVariableValueSet*`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_variableInitialValues: Pointer,
    /// # C++ Info
    /// -          name: `stringData`(ctype: `struct hkbBehaviorGraphStringData*`)
    /// -        offset:  84(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
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
                .serialize_struct("hkbBehaviorGraphData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "attributeDefaults",
                    &self.m_attributeDefaults,
                )?;
            serializer
                .serialize_array_meta_field("variableInfos", &self.m_variableInfos)?;
            serializer
                .serialize_array_meta_field(
                    "characterPropertyInfos",
                    &self.m_characterPropertyInfos,
                )?;
            serializer.serialize_array_meta_field("eventInfos", &self.m_eventInfos)?;
            serializer
                .serialize_array_meta_field(
                    "wordMinVariableValues",
                    &self.m_wordMinVariableValues,
                )?;
            serializer
                .serialize_array_meta_field(
                    "wordMaxVariableValues",
                    &self.m_wordMaxVariableValues,
                )?;
            serializer
                .serialize_field(
                    "variableInitialValues",
                    &self.m_variableInitialValues,
                )?;
            serializer.serialize_field("stringData", &self.m_stringData)?;
            serializer
                .serialize_array_field("attributeDefaults", &self.m_attributeDefaults)?;
            serializer.serialize_array_field("variableInfos", &self.m_variableInfos)?;
            serializer
                .serialize_array_field(
                    "characterPropertyInfos",
                    &self.m_characterPropertyInfos,
                )?;
            serializer.serialize_array_field("eventInfos", &self.m_eventInfos)?;
            serializer
                .serialize_array_field(
                    "wordMinVariableValues",
                    &self.m_wordMinVariableValues,
                )?;
            serializer
                .serialize_array_field(
                    "wordMaxVariableValues",
                    &self.m_wordMaxVariableValues,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
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
    fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
    }
}
pub(super) struct __hkbBehaviorGraphDataVisitor<'de> {
    marker: core::marker::PhantomData<hkbBehaviorGraphData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbBehaviorGraphDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbBehaviorGraphData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbBehaviorGraphData>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbBehaviorGraphDataVisitor<'de> {
    type Value = hkbBehaviorGraphData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbBehaviorGraphData")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_attributeDefaults: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_variableInfos: _serde::__private::Option<Vec<hkbVariableInfo>> = _serde::__private::None;
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
        let mut m_variableInitialValues: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_stringData: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_attributeDefaults) {
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
                    if _serde::__private::Option::is_some(&m_characterPropertyInfos) {
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
                    if _serde::__private::Option::is_some(&m_wordMinVariableValues) {
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
                    if _serde::__private::Option::is_some(&m_wordMaxVariableValues) {
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
                    if _serde::__private::Option::is_some(&m_variableInitialValues) {
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
                    <__A::Error as _serde::de::Error>::missing_field("attributeDefaults"),
                );
            }
        };
        let m_variableInfos = match m_variableInfos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("variableInfos"),
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
                    <__A::Error as _serde::de::Error>::missing_field("eventInfos"),
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
                    <__A::Error as _serde::de::Error>::missing_field("stringData"),
                );
            }
        };
        _serde::__private::Ok(hkbBehaviorGraphData {
            __ptr: __A::class_ptr(&mut __map),
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
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_attributeDefaults: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_variableInfos: _serde::__private::Option<Vec<hkbVariableInfo>> = _serde::__private::None;
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
        let mut m_variableInitialValues: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_stringData: _serde::__private::Option<Pointer> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_attributeDefaults => {
                    if _serde::__private::Option::is_some(&m_attributeDefaults) {
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
                __Field::m_characterPropertyInfos => {
                    if _serde::__private::Option::is_some(&m_characterPropertyInfos) {
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
                __Field::m_wordMinVariableValues => {
                    if _serde::__private::Option::is_some(&m_wordMinVariableValues) {
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
                    if _serde::__private::Option::is_some(&m_wordMaxVariableValues) {
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
                    if _serde::__private::Option::is_some(&m_variableInitialValues) {
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
                    <__A::Error as _serde::de::Error>::missing_field("attributeDefaults"),
                );
            }
        };
        let m_variableInfos = match m_variableInfos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("variableInfos"),
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
                    <__A::Error as _serde::de::Error>::missing_field("eventInfos"),
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
                    <__A::Error as _serde::de::Error>::missing_field("stringData"),
                );
            }
        };
        _serde::__private::Ok(hkbBehaviorGraphData {
            __ptr: __A::class_ptr(&mut __map),
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbBehaviorGraphData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
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
