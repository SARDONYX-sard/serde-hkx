use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpCompressedMeshShapeBigTriangle`
/// - version: `2`
/// - signature: `0xcbfc95a4`
/// - size: ` 16`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCompressedMeshShapeBigTriangle<'a> {
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
    /// - name: `a`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "a"))]
    #[cfg_attr(feature = "serde", serde(rename = "a"))]
    pub m_a: U16<'a>,
    /// # C++ Info
    /// - name: `b`(ctype: `hkUint16`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "b"))]
    #[cfg_attr(feature = "serde", serde(rename = "b"))]
    pub m_b: U16<'a>,
    /// # C++ Info
    /// - name: `c`(ctype: `hkUint16`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "c"))]
    #[cfg_attr(feature = "serde", serde(rename = "c"))]
    pub m_c: U16<'a>,
    /// # C++ Info
    /// - name: `material`(ctype: `hkUint32`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "material"))]
    #[cfg_attr(feature = "serde", serde(rename = "material"))]
    pub m_material: U32<'a>,
    /// # C++ Info
    /// - name: `weldingInfo`(ctype: `hkUint16`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "weldingInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "weldingInfo"))]
    pub m_weldingInfo: U16<'a>,
    /// # C++ Info
    /// - name: `transformIndex`(ctype: `hkUint16`)
    /// - offset: ` 14`(x86)/` 14`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "transformIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "transformIndex"))]
    pub m_transformIndex: U16<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpCompressedMeshShapeBigTriangle<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCompressedMeshShapeBigTriangle"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xcbfc95a4)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkpCompressedMeshShapeBigTriangle<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xcbfc95a4)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpCompressedMeshShapeBigTriangle",
                    class_meta,
                    (16u64, 16u64),
                )?;
            serializer.serialize_field("a", &self.m_a)?;
            serializer.serialize_field("b", &self.m_b)?;
            serializer.serialize_field("c", &self.m_c)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.serialize_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.serialize_field("transformIndex", &self.m_transformIndex)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCompressedMeshShapeBigTriangle<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_a,
                m_b,
                m_c,
                m_material,
                m_weldingInfo,
                m_transformIndex,
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
                        "a" => Ok(__Field::m_a),
                        "b" => Ok(__Field::m_b),
                        "c" => Ok(__Field::m_c),
                        "material" => Ok(__Field::m_material),
                        "weldingInfo" => Ok(__Field::m_weldingInfo),
                        "transformIndex" => Ok(__Field::m_transformIndex),
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
            struct __hkpCompressedMeshShapeBigTriangleVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpCompressedMeshShapeBigTriangle<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpCompressedMeshShapeBigTriangleVisitor<'de> {
                type Value = hkpCompressedMeshShapeBigTriangle<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpCompressedMeshShapeBigTriangle",
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
                    let mut m_a: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_b: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_c: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_material: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_weldingInfo: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_transformIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_a) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                    );
                                }
                                m_a = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_b) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                    );
                                }
                                m_b = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_c) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("c"),
                                    );
                                }
                                m_c = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_material) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "material",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_material = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_weldingInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weldingInfo",
                                        ),
                                    );
                                }
                                m_weldingInfo = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_transformIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformIndex",
                                        ),
                                    );
                                }
                                m_transformIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
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
                    let m_a = match m_a {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("a"),
                            );
                        }
                    };
                    let m_b = match m_b {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("b"),
                            );
                        }
                    };
                    let m_c = match m_c {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("c"),
                            );
                        }
                    };
                    let m_material = match m_material {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("material"),
                            );
                        }
                    };
                    let m_weldingInfo = match m_weldingInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "weldingInfo",
                                ),
                            );
                        }
                    };
                    let m_transformIndex = match m_transformIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformIndex",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpCompressedMeshShapeBigTriangle {
                        __ptr,
                        m_a,
                        m_b,
                        m_c,
                        m_material,
                        m_weldingInfo,
                        m_transformIndex,
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
                    let mut m_a: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_b: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_c: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_material: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_weldingInfo: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_transformIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_a => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_a) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                    );
                                }
                                m_a = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_b => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_b) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                    );
                                }
                                m_b = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_c => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_c) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("c"),
                                    );
                                }
                                m_c = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_material => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_material) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "material",
                                        ),
                                    );
                                }
                                m_material = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weldingInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weldingInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weldingInfo",
                                        ),
                                    );
                                }
                                m_weldingInfo = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transformIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transformIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformIndex",
                                        ),
                                    );
                                }
                                m_transformIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
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
                    let m_a = match m_a {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("a"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_b = match m_b {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("b"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_c = match m_c {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("c"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_material = match m_material {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("material"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weldingInfo = match m_weldingInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "weldingInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transformIndex = match m_transformIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpCompressedMeshShapeBigTriangle {
                        __ptr: __ptr.clone(),
                        m_a,
                        m_b,
                        m_c,
                        m_material,
                        m_weldingInfo,
                        m_transformIndex,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "a",
                "b",
                "c",
                "material",
                "weldingInfo",
                "transformIndex",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCompressedMeshShapeBigTriangle",
                FIELDS,
                __hkpCompressedMeshShapeBigTriangleVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpCompressedMeshShapeBigTriangle,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
