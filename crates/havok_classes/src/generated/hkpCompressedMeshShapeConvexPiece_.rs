use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpCompressedMeshShapeConvexPiece`
/// - version: `3`
/// - signature: `0x385bb842`
/// - size: ` 64`(x86)/` 80`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCompressedMeshShapeConvexPiece<'a> {
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
    /// - name: `offset`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "offset"))]
    #[cfg_attr(feature = "serde", serde(rename = "offset"))]
    pub m_offset: Vector4,
    /// # C++ Info
    /// - name: `vertices`(ctype: `hkArray<hkUint16>`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertices"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertices"))]
    pub m_vertices: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `faceVertices`(ctype: `hkArray<hkUint8>`)
    /// - offset: ` 28`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "faceVertices"))]
    #[cfg_attr(feature = "serde", serde(rename = "faceVertices"))]
    pub m_faceVertices: Vec<U8<'a>>,
    /// # C++ Info
    /// - name: `faceOffsets`(ctype: `hkArray<hkUint16>`)
    /// - offset: ` 40`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "faceOffsets"))]
    #[cfg_attr(feature = "serde", serde(rename = "faceOffsets"))]
    pub m_faceOffsets: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `reference`(ctype: `hkUint16`)
    /// - offset: ` 52`(x86)/` 64`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "reference"))]
    #[cfg_attr(feature = "serde", serde(rename = "reference"))]
    pub m_reference: U16<'a>,
    /// # C++ Info
    /// - name: `transformIndex`(ctype: `hkUint16`)
    /// - offset: ` 54`(x86)/` 66`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "transformIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "transformIndex"))]
    pub m_transformIndex: U16<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpCompressedMeshShapeConvexPiece<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCompressedMeshShapeConvexPiece"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x385bb842)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkpCompressedMeshShapeConvexPiece<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x385bb842)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpCompressedMeshShapeConvexPiece",
                    class_meta,
                    (64u64, 80u64),
                )?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer
                .serialize_array_field("vertices", &self.m_vertices, TypeSize::NonPtr)?;
            serializer
                .serialize_array_field(
                    "faceVertices",
                    &self.m_faceVertices,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "faceOffsets",
                    &self.m_faceOffsets,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("reference", &self.m_reference)?;
            serializer.serialize_field("transformIndex", &self.m_transformIndex)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCompressedMeshShapeConvexPiece<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_offset,
                m_vertices,
                m_faceVertices,
                m_faceOffsets,
                m_reference,
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
                        "offset" => Ok(__Field::m_offset),
                        "vertices" => Ok(__Field::m_vertices),
                        "faceVertices" => Ok(__Field::m_faceVertices),
                        "faceOffsets" => Ok(__Field::m_faceOffsets),
                        "reference" => Ok(__Field::m_reference),
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
            struct __hkpCompressedMeshShapeConvexPieceVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpCompressedMeshShapeConvexPiece<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpCompressedMeshShapeConvexPieceVisitor<'de> {
                type Value = hkpCompressedMeshShapeConvexPiece<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpCompressedMeshShapeConvexPiece",
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
                    let mut m_offset: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_vertices: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_faceVertices: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_faceOffsets: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_reference: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_transformIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_offset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                                    );
                                }
                                m_offset = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_vertices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertices",
                                        ),
                                    );
                                }
                                m_vertices = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_faceVertices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "faceVertices",
                                        ),
                                    );
                                }
                                m_faceVertices = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_faceOffsets) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "faceOffsets",
                                        ),
                                    );
                                }
                                m_faceOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_reference) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "reference",
                                        ),
                                    );
                                }
                                m_reference = _serde::__private::Some(
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
                    __A::pad(&mut __map, 8usize, 12usize)?;
                    let m_offset = match m_offset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("offset"),
                            );
                        }
                    };
                    let m_vertices = match m_vertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("vertices"),
                            );
                        }
                    };
                    let m_faceVertices = match m_faceVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "faceVertices",
                                ),
                            );
                        }
                    };
                    let m_faceOffsets = match m_faceOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "faceOffsets",
                                ),
                            );
                        }
                    };
                    let m_reference = match m_reference {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "reference",
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
                    _serde::__private::Ok(hkpCompressedMeshShapeConvexPiece {
                        __ptr,
                        m_offset,
                        m_vertices,
                        m_faceVertices,
                        m_faceOffsets,
                        m_reference,
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
                    let mut m_offset: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_vertices: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_faceVertices: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_faceOffsets: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_reference: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_transformIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_offset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_offset) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                                    );
                                }
                                m_offset = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_vertices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertices",
                                        ),
                                    );
                                }
                                m_vertices = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_faceVertices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_faceVertices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "faceVertices",
                                        ),
                                    );
                                }
                                m_faceVertices = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_faceOffsets => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_faceOffsets) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "faceOffsets",
                                        ),
                                    );
                                }
                                m_faceOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_reference => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_reference) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "reference",
                                        ),
                                    );
                                }
                                m_reference = _serde::__private::Some(
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
                    let m_offset = match m_offset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("offset"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_vertices = match m_vertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("vertices"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_faceVertices = match m_faceVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "faceVertices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_faceOffsets = match m_faceOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "faceOffsets",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_reference = match m_reference {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "reference",
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
                    _serde::__private::Ok(hkpCompressedMeshShapeConvexPiece {
                        __ptr: __ptr.clone(),
                        m_offset,
                        m_vertices,
                        m_faceVertices,
                        m_faceOffsets,
                        m_reference,
                        m_transformIndex,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "offset",
                "vertices",
                "faceVertices",
                "faceOffsets",
                "reference",
                "transformIndex",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCompressedMeshShapeConvexPiece",
                FIELDS,
                __hkpCompressedMeshShapeConvexPieceVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpCompressedMeshShapeConvexPiece,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
