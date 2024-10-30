use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkIndexedTransformSet`
/// - version: `1`
/// - signature: `0x87fe6b5c`
/// - size: ` 72`(x86)/`104`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkIndexedTransformSet<'a> {
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
    /// - name: `matrices`(ctype: `hkArray<hkMatrix4>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "matrices"))]
    #[cfg_attr(feature = "serde", serde(rename = "matrices"))]
    pub m_matrices: Vec<Matrix4>,
    /// # C++ Info
    /// - name: `inverseMatrices`(ctype: `hkArray<hkMatrix4>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "inverseMatrices"))]
    #[cfg_attr(feature = "serde", serde(rename = "inverseMatrices"))]
    pub m_inverseMatrices: Vec<Matrix4>,
    /// # C++ Info
    /// - name: `matricesOrder`(ctype: `hkArray<hkInt16>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "matricesOrder"))]
    #[cfg_attr(feature = "serde", serde(rename = "matricesOrder"))]
    pub m_matricesOrder: Vec<i16>,
    /// # C++ Info
    /// - name: `matricesNames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: ` 44`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "matricesNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "matricesNames"))]
    pub m_matricesNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `indexMappings`(ctype: `hkArray<struct hkMeshBoneIndexMapping>`)
    /// - offset: ` 56`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "indexMappings"))]
    #[cfg_attr(feature = "serde", serde(rename = "indexMappings"))]
    pub m_indexMappings: Vec<hkMeshBoneIndexMapping>,
    /// # C++ Info
    /// - name: `allMatricesAreAffine`(ctype: `hkBool`)
    /// - offset: ` 68`(x86)/` 96`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "allMatricesAreAffine"))]
    #[cfg_attr(feature = "serde", serde(rename = "allMatricesAreAffine"))]
    pub m_allMatricesAreAffine: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkIndexedTransformSet<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkIndexedTransformSet"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x87fe6b5c)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_indexMappings
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkIndexedTransformSet<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x87fe6b5c)));
            let mut serializer = __serializer
                .serialize_struct("hkIndexedTransformSet", class_meta, (72u64, 104u64))?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field("matrices", &self.m_matrices, TypeSize::NonPtr)?;
            serializer
                .serialize_array_field(
                    "inverseMatrices",
                    &self.m_inverseMatrices,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "matricesOrder",
                    &self.m_matricesOrder,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "matricesNames",
                    &self.m_matricesNames,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "indexMappings",
                    &self.m_indexMappings,
                    TypeSize::Struct {
                        size_x86: 12u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer
                .serialize_field("allMatricesAreAffine", &self.m_allMatricesAreAffine)?;
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
    impl<'de> _serde::Deserialize<'de> for hkIndexedTransformSet<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_matrices,
                m_inverseMatrices,
                m_matricesOrder,
                m_matricesNames,
                m_indexMappings,
                m_allMatricesAreAffine,
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
                        "matrices" => Ok(__Field::m_matrices),
                        "inverseMatrices" => Ok(__Field::m_inverseMatrices),
                        "matricesOrder" => Ok(__Field::m_matricesOrder),
                        "matricesNames" => Ok(__Field::m_matricesNames),
                        "indexMappings" => Ok(__Field::m_indexMappings),
                        "allMatricesAreAffine" => Ok(__Field::m_allMatricesAreAffine),
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
            struct __hkIndexedTransformSetVisitor<'de> {
                marker: _serde::__private::PhantomData<hkIndexedTransformSet<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkIndexedTransformSetVisitor<'de> {
                type Value = hkIndexedTransformSet<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkIndexedTransformSet",
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
                    let mut m_matrices: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
                    let mut m_inverseMatrices: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
                    let mut m_matricesOrder: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_matricesNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_indexMappings: _serde::__private::Option<
                        Vec<hkMeshBoneIndexMapping>,
                    > = _serde::__private::None;
                    let mut m_allMatricesAreAffine: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_matrices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "matrices",
                                        ),
                                    );
                                }
                                m_matrices = _serde::__private::Some(
                                    match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_inverseMatrices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inverseMatrices",
                                        ),
                                    );
                                }
                                m_inverseMatrices = _serde::__private::Some(
                                    match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_matricesOrder) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "matricesOrder",
                                        ),
                                    );
                                }
                                m_matricesOrder = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_matricesNames) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "matricesNames",
                                        ),
                                    );
                                }
                                m_matricesNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_indexMappings) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexMappings",
                                        ),
                                    );
                                }
                                m_indexMappings = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMeshBoneIndexMapping>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_allMatricesAreAffine,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allMatricesAreAffine",
                                        ),
                                    );
                                }
                                m_allMatricesAreAffine = _serde::__private::Some(
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
                    let m_matrices = match m_matrices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("matrices"),
                            );
                        }
                    };
                    let m_inverseMatrices = match m_inverseMatrices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inverseMatrices",
                                ),
                            );
                        }
                    };
                    let m_matricesOrder = match m_matricesOrder {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "matricesOrder",
                                ),
                            );
                        }
                    };
                    let m_matricesNames = match m_matricesNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "matricesNames",
                                ),
                            );
                        }
                    };
                    let m_indexMappings = match m_indexMappings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexMappings",
                                ),
                            );
                        }
                    };
                    let m_allMatricesAreAffine = match m_allMatricesAreAffine {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allMatricesAreAffine",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkIndexedTransformSet {
                        __ptr,
                        parent,
                        m_matrices,
                        m_inverseMatrices,
                        m_matricesOrder,
                        m_matricesNames,
                        m_indexMappings,
                        m_allMatricesAreAffine,
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
                    let mut m_matrices: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
                    let mut m_inverseMatrices: _serde::__private::Option<Vec<Matrix4>> = _serde::__private::None;
                    let mut m_matricesOrder: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_matricesNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_indexMappings: _serde::__private::Option<
                        Vec<hkMeshBoneIndexMapping>,
                    > = _serde::__private::None;
                    let mut m_allMatricesAreAffine: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_matrices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_matrices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "matrices",
                                        ),
                                    );
                                }
                                m_matrices = _serde::__private::Some(
                                    match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_inverseMatrices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_inverseMatrices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inverseMatrices",
                                        ),
                                    );
                                }
                                m_inverseMatrices = _serde::__private::Some(
                                    match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_matricesOrder => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_matricesOrder) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "matricesOrder",
                                        ),
                                    );
                                }
                                m_matricesOrder = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_matricesNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_matricesNames) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "matricesNames",
                                        ),
                                    );
                                }
                                m_matricesNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_indexMappings => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_indexMappings) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexMappings",
                                        ),
                                    );
                                }
                                m_indexMappings = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMeshBoneIndexMapping>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_allMatricesAreAffine => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_allMatricesAreAffine,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allMatricesAreAffine",
                                        ),
                                    );
                                }
                                m_allMatricesAreAffine = _serde::__private::Some(
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
                    let m_matrices = match m_matrices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("matrices"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_inverseMatrices = match m_inverseMatrices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inverseMatrices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_matricesOrder = match m_matricesOrder {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "matricesOrder",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_matricesNames = match m_matricesNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "matricesNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indexMappings = match m_indexMappings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexMappings",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_allMatricesAreAffine = match m_allMatricesAreAffine {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allMatricesAreAffine",
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
                    _serde::__private::Ok(hkIndexedTransformSet {
                        __ptr,
                        parent,
                        m_matrices,
                        m_inverseMatrices,
                        m_matricesOrder,
                        m_matricesNames,
                        m_indexMappings,
                        m_allMatricesAreAffine,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "matrices",
                "inverseMatrices",
                "matricesOrder",
                "matricesNames",
                "indexMappings",
                "allMatricesAreAffine",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkIndexedTransformSet",
                FIELDS,
                __hkIndexedTransformSetVisitor {
                    marker: _serde::__private::PhantomData::<hkIndexedTransformSet>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
