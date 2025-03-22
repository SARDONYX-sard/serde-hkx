use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpAgent1nSector`
/// - version: `0`
/// - signature: `0x626e55a`
/// - size: `512`(x86)/`512`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAgent1nSector<'a> {
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
    /// # C++ Info
    /// - name: `bytesAllocated`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "bytesAllocated"))]
    #[cfg_attr(feature = "serde", serde(rename = "bytesAllocated"))]
    pub m_bytesAllocated: U32<'a>,
    /// # C++ Info
    /// - name: `pad0`(ctype: `hkUint32`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "pad0"))]
    #[cfg_attr(feature = "serde", serde(rename = "pad0"))]
    pub m_pad0: U32<'a>,
    /// # C++ Info
    /// - name: `pad1`(ctype: `hkUint32`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "pad1"))]
    #[cfg_attr(feature = "serde", serde(rename = "pad1"))]
    pub m_pad1: U32<'a>,
    /// # C++ Info
    /// - name: `pad2`(ctype: `hkUint32`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "pad2"))]
    #[cfg_attr(feature = "serde", serde(rename = "pad2"))]
    pub m_pad2: U32<'a>,
    /// # C++ Info
    /// - name: `data`(ctype: `hkUint8[496]`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `496`(x86)/`496`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(schema_with = "make_large_int_array_schema")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(with = "::serde_with::As::<[::serde_with::Same; 496]>")
    )]
    #[educe(Default(expression = core::array::from_fn(|_idx|Default::default())))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "data"))]
    #[cfg_attr(feature = "serde", serde(rename = "data"))]
    pub m_data: [U8<'a>; 496usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpAgent1nSector<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpAgent1nSector"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x626e55a)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkpAgent1nSector<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x626e55a)));
            let mut serializer = __serializer
                .serialize_struct("hkpAgent1nSector", class_meta, (512u64, 512u64))?;
            serializer.serialize_field("bytesAllocated", &self.m_bytesAllocated)?;
            serializer.serialize_field("pad0", &self.m_pad0)?;
            serializer.serialize_field("pad1", &self.m_pad1)?;
            serializer.serialize_field("pad2", &self.m_pad2)?;
            serializer
                .serialize_fixed_array_field(
                    "data",
                    self.m_data.as_slice(),
                    TypeSize::NonPtr,
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
    impl<'de> _serde::Deserialize<'de> for hkpAgent1nSector<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_bytesAllocated,
                m_pad0,
                m_pad1,
                m_pad2,
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
                        "bytesAllocated" => Ok(__Field::m_bytesAllocated),
                        "pad0" => Ok(__Field::m_pad0),
                        "pad1" => Ok(__Field::m_pad1),
                        "pad2" => Ok(__Field::m_pad2),
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
            struct __hkpAgent1nSectorVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpAgent1nSector<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpAgent1nSectorVisitor<'de> {
                type Value = hkpAgent1nSector<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpAgent1nSector",
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
                    let mut m_bytesAllocated: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_pad0: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_pad1: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_pad2: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<[U8<'de>; 496usize]> = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_bytesAllocated) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bytesAllocated",
                                        ),
                                    );
                                }
                                m_bytesAllocated = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_pad0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pad0"),
                                    );
                                }
                                m_pad0 = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_pad1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pad1"),
                                    );
                                }
                                m_pad1 = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_pad2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pad2"),
                                    );
                                }
                                m_pad2 = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_data) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<[U8<'de>; 496usize]>(&mut __map) {
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
                    let m_bytesAllocated = match m_bytesAllocated {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bytesAllocated",
                                ),
                            );
                        }
                    };
                    let m_pad0 = match m_pad0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pad0"),
                            );
                        }
                    };
                    let m_pad1 = match m_pad1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pad1"),
                            );
                        }
                    };
                    let m_pad2 = match m_pad2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pad2"),
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
                    _serde::__private::Ok(hkpAgent1nSector {
                        __ptr,
                        m_bytesAllocated,
                        m_pad0,
                        m_pad1,
                        m_pad2,
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
                    let mut m_bytesAllocated: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_pad0: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_pad1: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_pad2: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<[U8<'de>; 496usize]> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_bytesAllocated => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bytesAllocated) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bytesAllocated",
                                        ),
                                    );
                                }
                                m_bytesAllocated = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pad0 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_pad0) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pad0"),
                                    );
                                }
                                m_pad0 = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pad1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_pad1) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pad1"),
                                    );
                                }
                                m_pad1 = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pad2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_pad2) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pad2"),
                                    );
                                }
                                m_pad2 = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
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
                                    match __A::next_value::<[U8<'de>; 496usize]>(&mut __map) {
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
                    let m_bytesAllocated = match m_bytesAllocated {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bytesAllocated",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pad0 = match m_pad0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pad0"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pad1 = match m_pad1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pad1"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pad2 = match m_pad2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pad2"),
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
                            #[cfg(not(feature = "strict"))]
                            core::array::from_fn(|_idx| Default::default())
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpAgent1nSector {
                        __ptr: __ptr.clone(),
                        m_bytesAllocated,
                        m_pad0,
                        m_pad1,
                        m_pad2,
                        m_data,
                    })
                }
            }
            const FIELDS: &[&str] = &["bytesAllocated", "pad0", "pad1", "pad2", "data"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpAgent1nSector",
                FIELDS,
                __hkpAgent1nSectorVisitor {
                    marker: _serde::__private::PhantomData::<hkpAgent1nSector>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
