use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaSkeletonMapperData`
/// - version: `1`
/// - signature: `0x95687ea0`
/// - size: `112`(x86)/`128`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSkeletonMapperData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `skeletonA`(ctype: `struct hkaSkeleton*`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_skeletonA: Pointer,
    /// # C++ Info
    /// - name: `skeletonB`(ctype: `struct hkaSkeleton*`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_skeletonB: Pointer,
    /// # C++ Info
    /// - name: `simpleMappings`(ctype: `hkArray<struct hkaSkeletonMapperDataSimpleMapping>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_simpleMappings: Vec<hkaSkeletonMapperDataSimpleMapping>,
    /// # C++ Info
    /// - name: `chainMappings`(ctype: `hkArray<struct hkaSkeletonMapperDataChainMapping>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_chainMappings: Vec<hkaSkeletonMapperDataChainMapping>,
    /// # C++ Info
    /// - name: `unmappedBones`(ctype: `hkArray<hkInt16>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_unmappedBones: Vec<i16>,
    /// # C++ Info
    /// - name: `extractedMotionMapping`(ctype: `hkQsTransform`)
    /// - offset: ` 48`(x86)/` 64`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_extractedMotionMapping: QsTransform,
    /// # C++ Info
    /// - name: `keepUnmappedLocal`(ctype: `hkBool`)
    /// - offset: ` 96`(x86)/`112`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_keepUnmappedLocal: bool,
    /// # C++ Info
    /// - name: `mappingType`(ctype: `enum MappingType`)
    /// - offset: `100`(x86)/`116`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_mappingType: MappingType,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaSkeletonMapperData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSkeletonMapperData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x95687ea0)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_skeletonA.get());
            v.push(self.m_skeletonB.get());
            v.extend(
                self
                    .m_simpleMappings
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_chainMappings
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v
        }
    }
    impl _serde::Serialize for hkaSkeletonMapperData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x95687ea0)));
            let mut serializer = __serializer
                .serialize_struct("hkaSkeletonMapperData", class_meta)?;
            serializer.serialize_field("skeletonA", &self.m_skeletonA)?;
            serializer.serialize_field("skeletonB", &self.m_skeletonB)?;
            serializer
                .serialize_array_meta_field("simpleMappings", &self.m_simpleMappings)?;
            serializer
                .serialize_array_meta_field("chainMappings", &self.m_chainMappings)?;
            serializer
                .serialize_array_meta_field("unmappedBones", &self.m_unmappedBones)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field(
                    "extractedMotionMapping",
                    &self.m_extractedMotionMapping,
                )?;
            serializer.serialize_field("keepUnmappedLocal", &self.m_keepUnmappedLocal)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("mappingType", &self.m_mappingType)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_array_field("simpleMappings", &self.m_simpleMappings)?;
            serializer.serialize_array_field("chainMappings", &self.m_chainMappings)?;
            serializer.serialize_array_field("unmappedBones", &self.m_unmappedBones)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaSkeletonMapperData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_skeletonA,
                m_skeletonB,
                m_simpleMappings,
                m_chainMappings,
                m_unmappedBones,
                m_extractedMotionMapping,
                m_keepUnmappedLocal,
                m_mappingType,
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
                        "skeletonA" => Ok(__Field::m_skeletonA),
                        "skeletonB" => Ok(__Field::m_skeletonB),
                        "simpleMappings" => Ok(__Field::m_simpleMappings),
                        "chainMappings" => Ok(__Field::m_chainMappings),
                        "unmappedBones" => Ok(__Field::m_unmappedBones),
                        "extractedMotionMapping" => Ok(__Field::m_extractedMotionMapping),
                        "keepUnmappedLocal" => Ok(__Field::m_keepUnmappedLocal),
                        "mappingType" => Ok(__Field::m_mappingType),
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
            struct __hkaSkeletonMapperDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkaSkeletonMapperData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkaSkeletonMapperDataVisitor<'de> {
                type Value = hkaSkeletonMapperData;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkaSkeletonMapperData",
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
                    let mut m_skeletonA: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_skeletonB: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_simpleMappings: _serde::__private::Option<
                        Vec<hkaSkeletonMapperDataSimpleMapping>,
                    > = _serde::__private::None;
                    let mut m_chainMappings: _serde::__private::Option<
                        Vec<hkaSkeletonMapperDataChainMapping>,
                    > = _serde::__private::None;
                    let mut m_unmappedBones: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_extractedMotionMapping: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_keepUnmappedLocal: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_mappingType: _serde::__private::Option<MappingType> = _serde::__private::None;
                    for i in 0..8usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_skeletonA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skeletonA",
                                        ),
                                    );
                                }
                                m_skeletonA = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_skeletonB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skeletonB",
                                        ),
                                    );
                                }
                                m_skeletonB = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_simpleMappings) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleMappings",
                                        ),
                                    );
                                }
                                m_simpleMappings = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkaSkeletonMapperDataSimpleMapping>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_chainMappings) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "chainMappings",
                                        ),
                                    );
                                }
                                m_chainMappings = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkaSkeletonMapperDataChainMapping>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_unmappedBones) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "unmappedBones",
                                        ),
                                    );
                                }
                                m_unmappedBones = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_extractedMotionMapping,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extractedMotionMapping",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 4usize, 0usize)?;
                                m_extractedMotionMapping = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_keepUnmappedLocal,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "keepUnmappedLocal",
                                        ),
                                    );
                                }
                                m_keepUnmappedLocal = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_mappingType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mappingType",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_mappingType = _serde::__private::Some(
                                    match __A::next_value::<MappingType>(&mut __map) {
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
                    __A::pad(&mut __map, 8usize, 8usize)?;
                    let m_skeletonA = match m_skeletonA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skeletonA",
                                ),
                            );
                        }
                    };
                    let m_skeletonB = match m_skeletonB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skeletonB",
                                ),
                            );
                        }
                    };
                    let m_simpleMappings = match m_simpleMappings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleMappings",
                                ),
                            );
                        }
                    };
                    let m_chainMappings = match m_chainMappings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "chainMappings",
                                ),
                            );
                        }
                    };
                    let m_unmappedBones = match m_unmappedBones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "unmappedBones",
                                ),
                            );
                        }
                    };
                    let m_extractedMotionMapping = match m_extractedMotionMapping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extractedMotionMapping",
                                ),
                            );
                        }
                    };
                    let m_keepUnmappedLocal = match m_keepUnmappedLocal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keepUnmappedLocal",
                                ),
                            );
                        }
                    };
                    let m_mappingType = match m_mappingType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mappingType",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkaSkeletonMapperData {
                        __ptr,
                        m_skeletonA,
                        m_skeletonB,
                        m_simpleMappings,
                        m_chainMappings,
                        m_unmappedBones,
                        m_extractedMotionMapping,
                        m_keepUnmappedLocal,
                        m_mappingType,
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
                    let mut m_skeletonA: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_skeletonB: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_simpleMappings: _serde::__private::Option<
                        Vec<hkaSkeletonMapperDataSimpleMapping>,
                    > = _serde::__private::None;
                    let mut m_chainMappings: _serde::__private::Option<
                        Vec<hkaSkeletonMapperDataChainMapping>,
                    > = _serde::__private::None;
                    let mut m_unmappedBones: _serde::__private::Option<Vec<i16>> = _serde::__private::None;
                    let mut m_extractedMotionMapping: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_keepUnmappedLocal: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_mappingType: _serde::__private::Option<MappingType> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_skeletonA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_skeletonA) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skeletonA",
                                        ),
                                    );
                                }
                                m_skeletonA = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_skeletonB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_skeletonB) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skeletonB",
                                        ),
                                    );
                                }
                                m_skeletonB = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleMappings => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simpleMappings) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleMappings",
                                        ),
                                    );
                                }
                                m_simpleMappings = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkaSkeletonMapperDataSimpleMapping>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_chainMappings => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_chainMappings) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "chainMappings",
                                        ),
                                    );
                                }
                                m_chainMappings = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkaSkeletonMapperDataChainMapping>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_unmappedBones => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_unmappedBones) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "unmappedBones",
                                        ),
                                    );
                                }
                                m_unmappedBones = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_extractedMotionMapping => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_extractedMotionMapping,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extractedMotionMapping",
                                        ),
                                    );
                                }
                                m_extractedMotionMapping = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_keepUnmappedLocal => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_keepUnmappedLocal,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "keepUnmappedLocal",
                                        ),
                                    );
                                }
                                m_keepUnmappedLocal = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mappingType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_mappingType) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mappingType",
                                        ),
                                    );
                                }
                                m_mappingType = _serde::__private::Some(
                                    match __A::next_value::<MappingType>(&mut __map) {
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
                    let m_skeletonA = match m_skeletonA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skeletonA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_skeletonB = match m_skeletonB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skeletonB",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleMappings = match m_simpleMappings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleMappings",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_chainMappings = match m_chainMappings {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "chainMappings",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_unmappedBones = match m_unmappedBones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "unmappedBones",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_extractedMotionMapping = match m_extractedMotionMapping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extractedMotionMapping",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_keepUnmappedLocal = match m_keepUnmappedLocal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keepUnmappedLocal",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mappingType = match m_mappingType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mappingType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkaSkeletonMapperData {
                        __ptr,
                        m_skeletonA,
                        m_skeletonB,
                        m_simpleMappings,
                        m_chainMappings,
                        m_unmappedBones,
                        m_extractedMotionMapping,
                        m_keepUnmappedLocal,
                        m_mappingType,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "skeletonA",
                "skeletonB",
                "simpleMappings",
                "chainMappings",
                "unmappedBones",
                "extractedMotionMapping",
                "keepUnmappedLocal",
                "mappingType",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaSkeletonMapperData",
                FIELDS,
                __hkaSkeletonMapperDataVisitor {
                    marker: _serde::__private::PhantomData::<hkaSkeletonMapperData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT32`
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
pub enum MappingType {
    #[default]
    HK_RAGDOLL_MAPPING = 0isize,
    HK_RETARGETING_MAPPING = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MappingType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HK_RAGDOLL_MAPPING => {
                    __serializer.serialize_field("HK_RAGDOLL_MAPPING", &0u64)
                }
                Self::HK_RETARGETING_MAPPING => {
                    __serializer.serialize_field("HK_RETARGETING_MAPPING", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i32()
                .ok_or(S::Error::custom("Failed enum MappingType to_i32"))?;
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
    impl<'de> _serde::Deserialize<'de> for MappingType {
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
                fn visit_int32<__E>(
                    self,
                    __value: i32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i32 => _serde::__private::Ok(__Field::__field0),
                        1i32 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int32(__value),
                                    &"value(i32) of variant is one of 0, 1",
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
                                || v.eq_ignore_ascii_case("HK_RAGDOLL_MAPPING") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("HK_RETARGETING_MAPPING") => {
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
                        _serde::de::ReadEnumSize::Int32,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MappingType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MappingType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum MappingType",
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
                            _serde::__private::Ok(MappingType::HK_RAGDOLL_MAPPING)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MappingType::HK_RETARGETING_MAPPING)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HK_RAGDOLL_MAPPING",
                "HK_RETARGETING_MAPPING",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MappingType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MappingType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
