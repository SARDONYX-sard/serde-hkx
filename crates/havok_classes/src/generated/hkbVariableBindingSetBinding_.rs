use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbVariableBindingSetBinding`
/// - version: `1`
/// - signature: `0x4d592f72`
/// - size: ` 32`(x86)/` 40`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbVariableBindingSetBinding<'a> {
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
    /// # C++ Info
    /// - name: `memberPath`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "memberPath"))]
    pub m_memberPath: StringPtr<'a>,
    /// # C++ Info
    /// - name: `memberClass`(ctype: `void*`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "memberClass"))]
    pub m_memberClass: Pointer,
    /// # C++ Info
    /// - name: `offsetInObjectPlusOne`(ctype: `hkInt32`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "offsetInObjectPlusOne"))]
    pub m_offsetInObjectPlusOne: i32,
    /// # C++ Info
    /// - name: `offsetInArrayPlusOne`(ctype: `hkInt32`)
    /// - offset: ` 12`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "offsetInArrayPlusOne"))]
    pub m_offsetInArrayPlusOne: i32,
    /// # C++ Info
    /// - name: `rootVariableIndex`(ctype: `hkInt32`)
    /// - offset: ` 16`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "rootVariableIndex"))]
    pub m_rootVariableIndex: i32,
    /// # C++ Info
    /// - name: `variableIndex`(ctype: `hkInt32`)
    /// - offset: ` 20`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "variableIndex"))]
    pub m_variableIndex: i32,
    /// # C++ Info
    /// - name: `bitIndex`(ctype: `hkInt8`)
    /// - offset: ` 24`(x86)/` 32`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "bitIndex"))]
    pub m_bitIndex: i8,
    /// # C++ Info
    /// - name: `bindingType`(ctype: `enum BindingType`)
    /// - offset: ` 25`(x86)/` 33`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "bindingType"))]
    pub m_bindingType: BindingType,
    /// # C++ Info
    /// - name: `memberType`(ctype: `enum unknown`)
    /// - offset: ` 26`(x86)/` 34`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "memberType"))]
    pub m_memberType: u8,
    /// # C++ Info
    /// - name: `variableType`(ctype: `hkInt8`)
    /// - offset: ` 27`(x86)/` 35`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "variableType"))]
    pub m_variableType: i8,
    /// # C++ Info
    /// - name: `flags`(ctype: `flags unknown`)
    /// - offset: ` 28`(x86)/` 36`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(rename = "flags"))]
    pub m_flags: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbVariableBindingSetBinding<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbVariableBindingSetBinding"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x4d592f72)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_memberClass.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbVariableBindingSetBinding<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x4d592f72)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbVariableBindingSetBinding",
                    class_meta,
                    (32u64, 40u64),
                )?;
            serializer.serialize_field("memberPath", &self.m_memberPath)?;
            serializer.skip_field("memberClass", &self.m_memberClass)?;
            serializer
                .skip_field("offsetInObjectPlusOne", &self.m_offsetInObjectPlusOne)?;
            serializer.skip_field("offsetInArrayPlusOne", &self.m_offsetInArrayPlusOne)?;
            serializer.skip_field("rootVariableIndex", &self.m_rootVariableIndex)?;
            serializer.serialize_field("variableIndex", &self.m_variableIndex)?;
            serializer.serialize_field("bitIndex", &self.m_bitIndex)?;
            serializer.serialize_field("bindingType", &self.m_bindingType)?;
            serializer.skip_field("memberType", &self.m_memberType)?;
            serializer.skip_field("variableType", &self.m_variableType)?;
            serializer.skip_field("flags", &self.m_flags)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbVariableBindingSetBinding<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_memberPath,
                m_variableIndex,
                m_bitIndex,
                m_bindingType,
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
                        "memberPath" => Ok(__Field::m_memberPath),
                        "variableIndex" => Ok(__Field::m_variableIndex),
                        "bitIndex" => Ok(__Field::m_bitIndex),
                        "bindingType" => Ok(__Field::m_bindingType),
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
            struct __hkbVariableBindingSetBindingVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbVariableBindingSetBinding<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbVariableBindingSetBindingVisitor<'de> {
                type Value = hkbVariableBindingSetBinding<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbVariableBindingSetBinding",
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
                    let mut m_memberPath: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_memberClass: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_offsetInObjectPlusOne: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_offsetInArrayPlusOne: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_rootVariableIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_variableIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_bitIndex: _serde::__private::Option<i8> = _serde::__private::None;
                    let mut m_bindingType: _serde::__private::Option<BindingType> = _serde::__private::None;
                    let mut m_memberType: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_variableType: _serde::__private::Option<i8> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<i8> = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_memberPath) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "memberPath",
                                        ),
                                    );
                                }
                                m_memberPath = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_memberClass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "memberClass",
                                        ),
                                    );
                                }
                                m_memberClass = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_offsetInObjectPlusOne,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "offsetInObjectPlusOne",
                                        ),
                                    );
                                }
                                m_offsetInObjectPlusOne = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_offsetInArrayPlusOne,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "offsetInArrayPlusOne",
                                        ),
                                    );
                                }
                                m_offsetInArrayPlusOne = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_rootVariableIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rootVariableIndex",
                                        ),
                                    );
                                }
                                m_rootVariableIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_variableIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableIndex",
                                        ),
                                    );
                                }
                                m_variableIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_bitIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bitIndex",
                                        ),
                                    );
                                }
                                m_bitIndex = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_bindingType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bindingType",
                                        ),
                                    );
                                }
                                m_bindingType = _serde::__private::Some(
                                    match __A::next_value::<BindingType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_memberType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "memberType",
                                        ),
                                    );
                                }
                                m_memberType = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_variableType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableType",
                                        ),
                                    );
                                }
                                m_variableType = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_flags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
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
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    let m_memberPath = match m_memberPath {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "memberPath",
                                ),
                            );
                        }
                    };
                    let m_memberClass = match m_memberClass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "memberClass",
                                ),
                            );
                        }
                    };
                    let m_offsetInObjectPlusOne = match m_offsetInObjectPlusOne {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "offsetInObjectPlusOne",
                                ),
                            );
                        }
                    };
                    let m_offsetInArrayPlusOne = match m_offsetInArrayPlusOne {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "offsetInArrayPlusOne",
                                ),
                            );
                        }
                    };
                    let m_rootVariableIndex = match m_rootVariableIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rootVariableIndex",
                                ),
                            );
                        }
                    };
                    let m_variableIndex = match m_variableIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableIndex",
                                ),
                            );
                        }
                    };
                    let m_bitIndex = match m_bitIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bitIndex"),
                            );
                        }
                    };
                    let m_bindingType = match m_bindingType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bindingType",
                                ),
                            );
                        }
                    };
                    let m_memberType = match m_memberType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "memberType",
                                ),
                            );
                        }
                    };
                    let m_variableType = match m_variableType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableType",
                                ),
                            );
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbVariableBindingSetBinding {
                        __ptr,
                        m_memberPath,
                        m_memberClass,
                        m_offsetInObjectPlusOne,
                        m_offsetInArrayPlusOne,
                        m_rootVariableIndex,
                        m_variableIndex,
                        m_bitIndex,
                        m_bindingType,
                        m_memberType,
                        m_variableType,
                        m_flags,
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
                    let mut m_memberPath: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_variableIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_bitIndex: _serde::__private::Option<i8> = _serde::__private::None;
                    let mut m_bindingType: _serde::__private::Option<BindingType> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_memberPath => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_memberPath) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "memberPath",
                                        ),
                                    );
                                }
                                m_memberPath = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_variableIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_variableIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableIndex",
                                        ),
                                    );
                                }
                                m_variableIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bitIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bitIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bitIndex",
                                        ),
                                    );
                                }
                                m_bitIndex = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bindingType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bindingType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bindingType",
                                        ),
                                    );
                                }
                                m_bindingType = _serde::__private::Some(
                                    match __A::next_value::<BindingType>(&mut __map) {
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
                    let m_memberPath = match m_memberPath {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "memberPath",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_variableIndex = match m_variableIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bitIndex = match m_bitIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bitIndex"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bindingType = match m_bindingType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bindingType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbVariableBindingSetBinding {
                        __ptr,
                        m_memberPath,
                        m_variableIndex,
                        m_bitIndex,
                        m_bindingType,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "memberPath",
                "memberClass",
                "offsetInObjectPlusOne",
                "offsetInArrayPlusOne",
                "rootVariableIndex",
                "variableIndex",
                "bitIndex",
                "bindingType",
                "memberType",
                "variableType",
                "flags",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbVariableBindingSetBinding",
                FIELDS,
                __hkbVariableBindingSetBindingVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbVariableBindingSetBinding,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
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
pub enum BindingType {
    #[default]
    BINDING_TYPE_VARIABLE = 0isize,
    BINDING_TYPE_CHARACTER_PROPERTY = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BindingType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BINDING_TYPE_VARIABLE => {
                    __serializer.serialize_field("BINDING_TYPE_VARIABLE", &0u64)
                }
                Self::BINDING_TYPE_CHARACTER_PROPERTY => {
                    __serializer
                        .serialize_field("BINDING_TYPE_CHARACTER_PROPERTY", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BindingType to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for BindingType {
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
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
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
                                || v.eq_ignore_ascii_case("BINDING_TYPE_VARIABLE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case("BINDING_TYPE_CHARACTER_PROPERTY") => {
                                _serde::__private::Ok(__Field::__field1)
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BindingType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BindingType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum BindingType",
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
                            _serde::__private::Ok(BindingType::BINDING_TYPE_VARIABLE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                BindingType::BINDING_TYPE_CHARACTER_PROPERTY,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "BINDING_TYPE_VARIABLE",
                "BINDING_TYPE_CHARACTER_PROPERTY",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "BindingType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BindingType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
