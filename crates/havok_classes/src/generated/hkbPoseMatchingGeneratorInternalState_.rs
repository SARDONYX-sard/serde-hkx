use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbPoseMatchingGeneratorInternalState`
/// - version: `0`
/// - signature: `0x552d9dd4`
/// - size: ` 28`(x86)/` 40`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbPoseMatchingGeneratorInternalState<'a> {
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
    pub parent: hkReferencedObject<'a>,
    /// # C++ Info
    /// - name: `currentMatch`(ctype: `hkInt32`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentMatch"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentMatch"))]
    pub m_currentMatch: I32<'a>,
    /// # C++ Info
    /// - name: `bestMatch`(ctype: `hkInt32`)
    /// - offset: ` 12`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "bestMatch"))]
    #[cfg_attr(feature = "serde", serde(rename = "bestMatch"))]
    pub m_bestMatch: I32<'a>,
    /// # C++ Info
    /// - name: `timeSinceBetterMatch`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeSinceBetterMatch"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeSinceBetterMatch"))]
    pub m_timeSinceBetterMatch: f32,
    /// # C++ Info
    /// - name: `error`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "error"))]
    #[cfg_attr(feature = "serde", serde(rename = "error"))]
    pub m_error: f32,
    /// # C++ Info
    /// - name: `resetCurrentMatchLocalTime`(ctype: `hkBool`)
    /// - offset: ` 24`(x86)/` 32`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "resetCurrentMatchLocalTime"))]
    #[cfg_attr(feature = "serde", serde(rename = "resetCurrentMatchLocalTime"))]
    pub m_resetCurrentMatchLocalTime: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbPoseMatchingGeneratorInternalState<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbPoseMatchingGeneratorInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x552d9dd4)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkbPoseMatchingGeneratorInternalState<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x552d9dd4)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbPoseMatchingGeneratorInternalState",
                    class_meta,
                    (28u64, 40u64),
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("currentMatch", &self.m_currentMatch)?;
            serializer.serialize_field("bestMatch", &self.m_bestMatch)?;
            serializer
                .serialize_field("timeSinceBetterMatch", &self.m_timeSinceBetterMatch)?;
            serializer.serialize_field("error", &self.m_error)?;
            serializer
                .serialize_field(
                    "resetCurrentMatchLocalTime",
                    &self.m_resetCurrentMatchLocalTime,
                )?;
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
    impl<'de> _serde::Deserialize<'de> for hkbPoseMatchingGeneratorInternalState<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_currentMatch,
                m_bestMatch,
                m_timeSinceBetterMatch,
                m_error,
                m_resetCurrentMatchLocalTime,
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
                        "currentMatch" => Ok(__Field::m_currentMatch),
                        "bestMatch" => Ok(__Field::m_bestMatch),
                        "timeSinceBetterMatch" => Ok(__Field::m_timeSinceBetterMatch),
                        "error" => Ok(__Field::m_error),
                        "resetCurrentMatchLocalTime" => {
                            Ok(__Field::m_resetCurrentMatchLocalTime)
                        }
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
            struct __hkbPoseMatchingGeneratorInternalStateVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbPoseMatchingGeneratorInternalState<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbPoseMatchingGeneratorInternalStateVisitor<'de> {
                type Value = hkbPoseMatchingGeneratorInternalState<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbPoseMatchingGeneratorInternalState",
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
                    let mut m_currentMatch: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_bestMatch: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_timeSinceBetterMatch: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_error: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_resetCurrentMatchLocalTime: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_currentMatch) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentMatch",
                                        ),
                                    );
                                }
                                m_currentMatch = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_bestMatch) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bestMatch",
                                        ),
                                    );
                                }
                                m_bestMatch = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_timeSinceBetterMatch,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeSinceBetterMatch",
                                        ),
                                    );
                                }
                                m_timeSinceBetterMatch = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_error) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("error"),
                                    );
                                }
                                m_error = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_resetCurrentMatchLocalTime,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "resetCurrentMatchLocalTime",
                                        ),
                                    );
                                }
                                m_resetCurrentMatchLocalTime = _serde::__private::Some(
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
                    let m_currentMatch = match m_currentMatch {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentMatch",
                                ),
                            );
                        }
                    };
                    let m_bestMatch = match m_bestMatch {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bestMatch",
                                ),
                            );
                        }
                    };
                    let m_timeSinceBetterMatch = match m_timeSinceBetterMatch {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeSinceBetterMatch",
                                ),
                            );
                        }
                    };
                    let m_error = match m_error {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("error"),
                            );
                        }
                    };
                    let m_resetCurrentMatchLocalTime = match m_resetCurrentMatchLocalTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "resetCurrentMatchLocalTime",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbPoseMatchingGeneratorInternalState {
                        __ptr,
                        parent,
                        m_currentMatch,
                        m_bestMatch,
                        m_timeSinceBetterMatch,
                        m_error,
                        m_resetCurrentMatchLocalTime,
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
                    let mut m_currentMatch: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_bestMatch: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_timeSinceBetterMatch: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_error: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_resetCurrentMatchLocalTime: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_currentMatch => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_currentMatch) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentMatch",
                                        ),
                                    );
                                }
                                m_currentMatch = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bestMatch => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bestMatch) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bestMatch",
                                        ),
                                    );
                                }
                                m_bestMatch = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_timeSinceBetterMatch => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_timeSinceBetterMatch,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeSinceBetterMatch",
                                        ),
                                    );
                                }
                                m_timeSinceBetterMatch = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_error => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_error) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("error"),
                                    );
                                }
                                m_error = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_resetCurrentMatchLocalTime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_resetCurrentMatchLocalTime,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "resetCurrentMatchLocalTime",
                                        ),
                                    );
                                }
                                m_resetCurrentMatchLocalTime = _serde::__private::Some(
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
                    let m_currentMatch = match m_currentMatch {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentMatch",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bestMatch = match m_bestMatch {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bestMatch",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_timeSinceBetterMatch = match m_timeSinceBetterMatch {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeSinceBetterMatch",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_error = match m_error {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("error"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_resetCurrentMatchLocalTime = match m_resetCurrentMatchLocalTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "resetCurrentMatchLocalTime",
                                ),
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbPoseMatchingGeneratorInternalState {
                        __ptr: __ptr.clone(),
                        parent,
                        m_currentMatch,
                        m_bestMatch,
                        m_timeSinceBetterMatch,
                        m_error,
                        m_resetCurrentMatchLocalTime,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "currentMatch",
                "bestMatch",
                "timeSinceBetterMatch",
                "error",
                "resetCurrentMatchLocalTime",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbPoseMatchingGeneratorInternalState",
                FIELDS,
                __hkbPoseMatchingGeneratorInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbPoseMatchingGeneratorInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
