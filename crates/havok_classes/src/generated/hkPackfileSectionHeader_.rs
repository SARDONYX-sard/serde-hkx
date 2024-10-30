use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkPackfileSectionHeader`
/// - version: `0`
/// - signature: `0xf2a92154`
/// - size: ` 48`(x86)/` 48`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkPackfileSectionHeader {
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
    /// - name: `sectionTag`(ctype: `hkChar[19]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 19`(x86)/` 19`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "sectionTag"))]
    pub m_sectionTag: [char; 19usize],
    /// # C++ Info
    /// - name: `nullByte`(ctype: `hkChar`)
    /// - offset: ` 19`(x86)/` 19`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "nullByte"))]
    pub m_nullByte: char,
    /// # C++ Info
    /// - name: `absoluteDataStart`(ctype: `hkInt32`)
    /// - offset: ` 20`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "absoluteDataStart"))]
    pub m_absoluteDataStart: i32,
    /// # C++ Info
    /// - name: `localFixupsOffset`(ctype: `hkInt32`)
    /// - offset: ` 24`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "localFixupsOffset"))]
    pub m_localFixupsOffset: i32,
    /// # C++ Info
    /// - name: `globalFixupsOffset`(ctype: `hkInt32`)
    /// - offset: ` 28`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "globalFixupsOffset"))]
    pub m_globalFixupsOffset: i32,
    /// # C++ Info
    /// - name: `virtualFixupsOffset`(ctype: `hkInt32`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "virtualFixupsOffset"))]
    pub m_virtualFixupsOffset: i32,
    /// # C++ Info
    /// - name: `exportsOffset`(ctype: `hkInt32`)
    /// - offset: ` 36`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "exportsOffset"))]
    pub m_exportsOffset: i32,
    /// # C++ Info
    /// - name: `importsOffset`(ctype: `hkInt32`)
    /// - offset: ` 40`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "importsOffset"))]
    pub m_importsOffset: i32,
    /// # C++ Info
    /// - name: `endOffset`(ctype: `hkInt32`)
    /// - offset: ` 44`(x86)/` 44`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde", serde(rename = "endOffset"))]
    pub m_endOffset: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkPackfileSectionHeader {
        #[inline]
        fn name(&self) -> &'static str {
            "hkPackfileSectionHeader"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf2a92154)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkPackfileSectionHeader {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf2a92154)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkPackfileSectionHeader",
                    class_meta,
                    (48u64, 48u64),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "sectionTag",
                    self.m_sectionTag.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("nullByte", &self.m_nullByte)?;
            serializer.serialize_field("absoluteDataStart", &self.m_absoluteDataStart)?;
            serializer.serialize_field("localFixupsOffset", &self.m_localFixupsOffset)?;
            serializer
                .serialize_field("globalFixupsOffset", &self.m_globalFixupsOffset)?;
            serializer
                .serialize_field("virtualFixupsOffset", &self.m_virtualFixupsOffset)?;
            serializer.serialize_field("exportsOffset", &self.m_exportsOffset)?;
            serializer.serialize_field("importsOffset", &self.m_importsOffset)?;
            serializer.serialize_field("endOffset", &self.m_endOffset)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkPackfileSectionHeader {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_sectionTag,
                m_nullByte,
                m_absoluteDataStart,
                m_localFixupsOffset,
                m_globalFixupsOffset,
                m_virtualFixupsOffset,
                m_exportsOffset,
                m_importsOffset,
                m_endOffset,
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
                        "sectionTag" => Ok(__Field::m_sectionTag),
                        "nullByte" => Ok(__Field::m_nullByte),
                        "absoluteDataStart" => Ok(__Field::m_absoluteDataStart),
                        "localFixupsOffset" => Ok(__Field::m_localFixupsOffset),
                        "globalFixupsOffset" => Ok(__Field::m_globalFixupsOffset),
                        "virtualFixupsOffset" => Ok(__Field::m_virtualFixupsOffset),
                        "exportsOffset" => Ok(__Field::m_exportsOffset),
                        "importsOffset" => Ok(__Field::m_importsOffset),
                        "endOffset" => Ok(__Field::m_endOffset),
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
            struct __hkPackfileSectionHeaderVisitor<'de> {
                marker: _serde::__private::PhantomData<hkPackfileSectionHeader>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkPackfileSectionHeaderVisitor<'de> {
                type Value = hkPackfileSectionHeader;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkPackfileSectionHeader",
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
                    let mut m_sectionTag: _serde::__private::Option<[char; 19usize]> = _serde::__private::None;
                    let mut m_nullByte: _serde::__private::Option<char> = _serde::__private::None;
                    let mut m_absoluteDataStart: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_localFixupsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_globalFixupsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_virtualFixupsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_exportsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_importsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_endOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    for i in 0..9usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_sectionTag) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sectionTag",
                                        ),
                                    );
                                }
                                m_sectionTag = _serde::__private::Some(
                                    match __A::next_value::<[char; 19usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_nullByte) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nullByte",
                                        ),
                                    );
                                }
                                m_nullByte = _serde::__private::Some(
                                    match __A::next_value::<char>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_absoluteDataStart,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "absoluteDataStart",
                                        ),
                                    );
                                }
                                m_absoluteDataStart = _serde::__private::Some(
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
                                    &m_localFixupsOffset,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localFixupsOffset",
                                        ),
                                    );
                                }
                                m_localFixupsOffset = _serde::__private::Some(
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
                                    &m_globalFixupsOffset,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "globalFixupsOffset",
                                        ),
                                    );
                                }
                                m_globalFixupsOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_virtualFixupsOffset,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "virtualFixupsOffset",
                                        ),
                                    );
                                }
                                m_virtualFixupsOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_exportsOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "exportsOffset",
                                        ),
                                    );
                                }
                                m_exportsOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_importsOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "importsOffset",
                                        ),
                                    );
                                }
                                m_importsOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_endOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endOffset",
                                        ),
                                    );
                                }
                                m_endOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
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
                    let m_sectionTag = match m_sectionTag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sectionTag",
                                ),
                            );
                        }
                    };
                    let m_nullByte = match m_nullByte {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("nullByte"),
                            );
                        }
                    };
                    let m_absoluteDataStart = match m_absoluteDataStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "absoluteDataStart",
                                ),
                            );
                        }
                    };
                    let m_localFixupsOffset = match m_localFixupsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localFixupsOffset",
                                ),
                            );
                        }
                    };
                    let m_globalFixupsOffset = match m_globalFixupsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "globalFixupsOffset",
                                ),
                            );
                        }
                    };
                    let m_virtualFixupsOffset = match m_virtualFixupsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "virtualFixupsOffset",
                                ),
                            );
                        }
                    };
                    let m_exportsOffset = match m_exportsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "exportsOffset",
                                ),
                            );
                        }
                    };
                    let m_importsOffset = match m_importsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "importsOffset",
                                ),
                            );
                        }
                    };
                    let m_endOffset = match m_endOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endOffset",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkPackfileSectionHeader {
                        __ptr,
                        m_sectionTag,
                        m_nullByte,
                        m_absoluteDataStart,
                        m_localFixupsOffset,
                        m_globalFixupsOffset,
                        m_virtualFixupsOffset,
                        m_exportsOffset,
                        m_importsOffset,
                        m_endOffset,
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
                    let mut m_sectionTag: _serde::__private::Option<[char; 19usize]> = _serde::__private::None;
                    let mut m_nullByte: _serde::__private::Option<char> = _serde::__private::None;
                    let mut m_absoluteDataStart: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_localFixupsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_globalFixupsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_virtualFixupsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_exportsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_importsOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_endOffset: _serde::__private::Option<i32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_sectionTag => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_sectionTag) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sectionTag",
                                        ),
                                    );
                                }
                                m_sectionTag = _serde::__private::Some(
                                    match __A::next_value::<[char; 19usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nullByte => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nullByte) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nullByte",
                                        ),
                                    );
                                }
                                m_nullByte = _serde::__private::Some(
                                    match __A::next_value::<char>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_absoluteDataStart => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_absoluteDataStart,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "absoluteDataStart",
                                        ),
                                    );
                                }
                                m_absoluteDataStart = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_localFixupsOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_localFixupsOffset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localFixupsOffset",
                                        ),
                                    );
                                }
                                m_localFixupsOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_globalFixupsOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_globalFixupsOffset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "globalFixupsOffset",
                                        ),
                                    );
                                }
                                m_globalFixupsOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_virtualFixupsOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_virtualFixupsOffset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "virtualFixupsOffset",
                                        ),
                                    );
                                }
                                m_virtualFixupsOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_exportsOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_exportsOffset) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "exportsOffset",
                                        ),
                                    );
                                }
                                m_exportsOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_importsOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_importsOffset) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "importsOffset",
                                        ),
                                    );
                                }
                                m_importsOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_endOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_endOffset) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endOffset",
                                        ),
                                    );
                                }
                                m_endOffset = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
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
                    let m_sectionTag = match m_sectionTag {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sectionTag",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nullByte = match m_nullByte {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("nullByte"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_absoluteDataStart = match m_absoluteDataStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "absoluteDataStart",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_localFixupsOffset = match m_localFixupsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localFixupsOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_globalFixupsOffset = match m_globalFixupsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "globalFixupsOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_virtualFixupsOffset = match m_virtualFixupsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "virtualFixupsOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_exportsOffset = match m_exportsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "exportsOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_importsOffset = match m_importsOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "importsOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_endOffset = match m_endOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkPackfileSectionHeader {
                        __ptr,
                        m_sectionTag,
                        m_nullByte,
                        m_absoluteDataStart,
                        m_localFixupsOffset,
                        m_globalFixupsOffset,
                        m_virtualFixupsOffset,
                        m_exportsOffset,
                        m_importsOffset,
                        m_endOffset,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "sectionTag",
                "nullByte",
                "absoluteDataStart",
                "localFixupsOffset",
                "globalFixupsOffset",
                "virtualFixupsOffset",
                "exportsOffset",
                "importsOffset",
                "endOffset",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkPackfileSectionHeader",
                FIELDS,
                __hkPackfileSectionHeaderVisitor {
                    marker: _serde::__private::PhantomData::<hkPackfileSectionHeader>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
