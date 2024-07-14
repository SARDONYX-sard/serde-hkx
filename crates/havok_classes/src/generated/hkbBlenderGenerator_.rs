use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBlenderGenerator`
/// -         version: `1`
/// -       signature: `0x22df7147`
/// -          size: 116(x86)/160(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlenderGenerator<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// -          name: `referencePoseWeightThreshold`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_referencePoseWeightThreshold: f32,
    /// # C++ Info
    /// -          name: `blendParameter`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blendParameter: f32,
    /// # C++ Info
    /// -          name: `minCyclicBlendParameter`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minCyclicBlendParameter: f32,
    /// # C++ Info
    /// -          name: `maxCyclicBlendParameter`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxCyclicBlendParameter: f32,
    /// # C++ Info
    /// -          name: `indexOfSyncMasterChild`(ctype: `hkInt16`)
    /// -        offset:  56(x86)/ 88(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_indexOfSyncMasterChild: i16,
    /// # C++ Info
    /// -          name: `flags`(ctype: `hkInt16`)
    /// -        offset:  58(x86)/ 90(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_flags: i16,
    /// # C++ Info
    /// -          name: `subtractLastChild`(ctype: `hkBool`)
    /// -        offset:  60(x86)/ 92(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_subtractLastChild: bool,
    /// # C++ Info
    /// -          name: `children`(ctype: `hkArray<hkbBlenderGeneratorChild*>`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_children: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `childrenInternalStates`(ctype: `hkArray<void>`)
    /// -        offset:  76(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_childrenInternalStates: Vec<()>,
    /// # C++ Info
    /// -          name: `sortedChildren`(ctype: `hkArray<void>`)
    /// -        offset:  88(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_sortedChildren: Vec<()>,
    /// # C++ Info
    /// -          name: `endIntervalWeight`(ctype: `hkReal`)
    /// -        offset: 100(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_endIntervalWeight: f32,
    /// # C++ Info
    /// -          name: `numActiveChildren`(ctype: `hkInt32`)
    /// -        offset: 104(x86)/148(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numActiveChildren: i32,
    /// # C++ Info
    /// -          name: `beginIntervalIndex`(ctype: `hkInt16`)
    /// -        offset: 108(x86)/152(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_beginIntervalIndex: i16,
    /// # C++ Info
    /// -          name: `endIntervalIndex`(ctype: `hkInt16`)
    /// -        offset: 110(x86)/154(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_endIntervalIndex: i16,
    /// # C++ Info
    /// -          name: `initSync`(ctype: `hkBool`)
    /// -        offset: 112(x86)/156(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_initSync: bool,
    /// # C++ Info
    /// -          name: `doSubtractiveBlend`(ctype: `hkBool`)
    /// -        offset: 113(x86)/157(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_doSubtractiveBlend: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbBlenderGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBlenderGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x22df7147)
        }
    }
    impl<'a> _serde::Serialize for hkbBlenderGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x22df7147)));
            let mut serializer = __serializer
                .serialize_struct("hkbBlenderGenerator", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "referencePoseWeightThreshold",
                    &self.m_referencePoseWeightThreshold,
                )?;
            serializer.serialize_field("blendParameter", &self.m_blendParameter)?;
            serializer
                .serialize_field(
                    "minCyclicBlendParameter",
                    &self.m_minCyclicBlendParameter,
                )?;
            serializer
                .serialize_field(
                    "maxCyclicBlendParameter",
                    &self.m_maxCyclicBlendParameter,
                )?;
            serializer
                .serialize_field(
                    "indexOfSyncMasterChild",
                    &self.m_indexOfSyncMasterChild,
                )?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("subtractLastChild", &self.m_subtractLastChild)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_array_meta_field("children", &self.m_children)?;
            serializer
                .skip_array_meta_field(
                    "childrenInternalStates",
                    &self.m_childrenInternalStates,
                )?;
            serializer.skip_array_meta_field("sortedChildren", &self.m_sortedChildren)?;
            serializer.skip_field("endIntervalWeight", &self.m_endIntervalWeight)?;
            serializer.skip_field("numActiveChildren", &self.m_numActiveChildren)?;
            serializer.skip_field("beginIntervalIndex", &self.m_beginIntervalIndex)?;
            serializer.skip_field("endIntervalIndex", &self.m_endIntervalIndex)?;
            serializer.skip_field("initSync", &self.m_initSync)?;
            serializer.skip_field("doSubtractiveBlend", &self.m_doSubtractiveBlend)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("children", &self.m_children)?;
            serializer
                .serialize_array_field(
                    "childrenInternalStates",
                    &self.m_childrenInternalStates,
                )?;
            serializer.serialize_array_field("sortedChildren", &self.m_sortedChildren)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_referencePoseWeightThreshold,
    m_blendParameter,
    m_minCyclicBlendParameter,
    m_maxCyclicBlendParameter,
    m_indexOfSyncMasterChild,
    m_flags,
    m_subtractLastChild,
    m_children,
    m_childrenInternalStates,
    m_sortedChildren,
    m_endIntervalWeight,
    m_numActiveChildren,
    m_beginIntervalIndex,
    m_endIntervalIndex,
    m_initSync,
    m_doSubtractiveBlend,
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
            "referencePoseWeightThreshold" => Ok(__Field::m_referencePoseWeightThreshold),
            "blendParameter" => Ok(__Field::m_blendParameter),
            "minCyclicBlendParameter" => Ok(__Field::m_minCyclicBlendParameter),
            "maxCyclicBlendParameter" => Ok(__Field::m_maxCyclicBlendParameter),
            "indexOfSyncMasterChild" => Ok(__Field::m_indexOfSyncMasterChild),
            "flags" => Ok(__Field::m_flags),
            "subtractLastChild" => Ok(__Field::m_subtractLastChild),
            "children" => Ok(__Field::m_children),
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
pub(super) struct __hkbBlenderGeneratorVisitor<'de> {
    marker: core::marker::PhantomData<hkbBlenderGenerator<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbBlenderGeneratorVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbBlenderGenerator<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbBlenderGenerator<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbBlenderGeneratorVisitor<'de> {
    type Value = hkbBlenderGenerator<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbBlenderGenerator")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::next_value(&mut __map)?;
        let mut m_referencePoseWeightThreshold: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_blendParameter: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minCyclicBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxCyclicBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_indexOfSyncMasterChild: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_flags: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_subtractLastChild: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_children: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_childrenInternalStates: _serde::__private::Option<Vec<()>> = _serde::__private::None;
        let mut m_sortedChildren: _serde::__private::Option<Vec<()>> = _serde::__private::None;
        let mut m_endIntervalWeight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_numActiveChildren: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_beginIntervalIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endIntervalIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_initSync: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_doSubtractiveBlend: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..16usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(
                        &m_referencePoseWeightThreshold,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "referencePoseWeightThreshold",
                            ),
                        );
                    }
                    m_referencePoseWeightThreshold = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_blendParameter) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "blendParameter",
                            ),
                        );
                    }
                    m_blendParameter = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_minCyclicBlendParameter) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minCyclicBlendParameter",
                            ),
                        );
                    }
                    m_minCyclicBlendParameter = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_maxCyclicBlendParameter) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxCyclicBlendParameter",
                            ),
                        );
                    }
                    m_maxCyclicBlendParameter = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_indexOfSyncMasterChild) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "indexOfSyncMasterChild",
                            ),
                        );
                    }
                    m_indexOfSyncMasterChild = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_flags) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                        );
                    }
                    m_flags = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_subtractLastChild) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "subtractLastChild",
                            ),
                        );
                    }
                    m_subtractLastChild = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_children) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "children",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_children = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_childrenInternalStates) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childrenInternalStates",
                            ),
                        );
                    }
                    m_childrenInternalStates = _serde::__private::Some(
                        match __A::next_value::<Vec<()>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_sortedChildren) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sortedChildren",
                            ),
                        );
                    }
                    m_sortedChildren = _serde::__private::Some(
                        match __A::next_value::<Vec<()>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_endIntervalWeight) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endIntervalWeight",
                            ),
                        );
                    }
                    m_endIntervalWeight = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_numActiveChildren) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numActiveChildren",
                            ),
                        );
                    }
                    m_numActiveChildren = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_beginIntervalIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "beginIntervalIndex",
                            ),
                        );
                    }
                    m_beginIntervalIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_endIntervalIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endIntervalIndex",
                            ),
                        );
                    }
                    m_endIntervalIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                14usize => {
                    if _serde::__private::Option::is_some(&m_initSync) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "initSync",
                            ),
                        );
                    }
                    m_initSync = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_doSubtractiveBlend) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "doSubtractiveBlend",
                            ),
                        );
                    }
                    m_doSubtractiveBlend = _serde::__private::Some(
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
        __A::pad(&mut __map, 2usize, 2usize)?;
        let m_referencePoseWeightThreshold = match m_referencePoseWeightThreshold {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "referencePoseWeightThreshold",
                    ),
                );
            }
        };
        let m_blendParameter = match m_blendParameter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blendParameter"),
                );
            }
        };
        let m_minCyclicBlendParameter = match m_minCyclicBlendParameter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minCyclicBlendParameter",
                    ),
                );
            }
        };
        let m_maxCyclicBlendParameter = match m_maxCyclicBlendParameter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxCyclicBlendParameter",
                    ),
                );
            }
        };
        let m_indexOfSyncMasterChild = match m_indexOfSyncMasterChild {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "indexOfSyncMasterChild",
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
        let m_subtractLastChild = match m_subtractLastChild {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("subtractLastChild"),
                );
            }
        };
        let m_children = match m_children {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("children"),
                );
            }
        };
        let m_childrenInternalStates = match m_childrenInternalStates {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "childrenInternalStates",
                    ),
                );
            }
        };
        let m_sortedChildren = match m_sortedChildren {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sortedChildren"),
                );
            }
        };
        let m_endIntervalWeight = match m_endIntervalWeight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endIntervalWeight"),
                );
            }
        };
        let m_numActiveChildren = match m_numActiveChildren {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numActiveChildren"),
                );
            }
        };
        let m_beginIntervalIndex = match m_beginIntervalIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "beginIntervalIndex",
                    ),
                );
            }
        };
        let m_endIntervalIndex = match m_endIntervalIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endIntervalIndex"),
                );
            }
        };
        let m_initSync = match m_initSync {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initSync"),
                );
            }
        };
        let m_doSubtractiveBlend = match m_doSubtractiveBlend {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "doSubtractiveBlend",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbBlenderGenerator {
            __ptr,
            parent,
            m_referencePoseWeightThreshold,
            m_blendParameter,
            m_minCyclicBlendParameter,
            m_maxCyclicBlendParameter,
            m_indexOfSyncMasterChild,
            m_flags,
            m_subtractLastChild,
            m_children,
            m_childrenInternalStates,
            m_sortedChildren,
            m_endIntervalWeight,
            m_numActiveChildren,
            m_beginIntervalIndex,
            m_endIntervalIndex,
            m_initSync,
            m_doSubtractiveBlend,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkbGeneratorVisitor::visit_as_parent(&mut __map)?;
        let mut m_referencePoseWeightThreshold: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_blendParameter: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minCyclicBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxCyclicBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_indexOfSyncMasterChild: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_flags: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_subtractLastChild: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_children: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for _ in 0..8usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_referencePoseWeightThreshold => {
                        if _serde::__private::Option::is_some(
                            &m_referencePoseWeightThreshold,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "referencePoseWeightThreshold",
                                ),
                            );
                        }
                        m_referencePoseWeightThreshold = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_blendParameter => {
                        if _serde::__private::Option::is_some(&m_blendParameter) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "blendParameter",
                                ),
                            );
                        }
                        m_blendParameter = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_minCyclicBlendParameter => {
                        if _serde::__private::Option::is_some(
                            &m_minCyclicBlendParameter,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "minCyclicBlendParameter",
                                ),
                            );
                        }
                        m_minCyclicBlendParameter = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxCyclicBlendParameter => {
                        if _serde::__private::Option::is_some(
                            &m_maxCyclicBlendParameter,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxCyclicBlendParameter",
                                ),
                            );
                        }
                        m_maxCyclicBlendParameter = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_indexOfSyncMasterChild => {
                        if _serde::__private::Option::is_some(
                            &m_indexOfSyncMasterChild,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "indexOfSyncMasterChild",
                                ),
                            );
                        }
                        m_indexOfSyncMasterChild = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_flags => {
                        if _serde::__private::Option::is_some(&m_flags) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                            );
                        }
                        m_flags = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_subtractLastChild => {
                        if _serde::__private::Option::is_some(&m_subtractLastChild) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "subtractLastChild",
                                ),
                            );
                        }
                        m_subtractLastChild = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_children => {
                        if _serde::__private::Option::is_some(&m_children) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "children",
                                ),
                            );
                        }
                        m_children = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        }
        let m_referencePoseWeightThreshold = match m_referencePoseWeightThreshold {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "referencePoseWeightThreshold",
                    ),
                );
            }
        };
        let m_blendParameter = match m_blendParameter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blendParameter"),
                );
            }
        };
        let m_minCyclicBlendParameter = match m_minCyclicBlendParameter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minCyclicBlendParameter",
                    ),
                );
            }
        };
        let m_maxCyclicBlendParameter = match m_maxCyclicBlendParameter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxCyclicBlendParameter",
                    ),
                );
            }
        };
        let m_indexOfSyncMasterChild = match m_indexOfSyncMasterChild {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "indexOfSyncMasterChild",
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
        let m_subtractLastChild = match m_subtractLastChild {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("subtractLastChild"),
                );
            }
        };
        let m_children = match m_children {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("children"),
                );
            }
        };
        _serde::__private::Ok(hkbBlenderGenerator {
            __ptr,
            parent,
            m_referencePoseWeightThreshold,
            m_blendParameter,
            m_minCyclicBlendParameter,
            m_maxCyclicBlendParameter,
            m_indexOfSyncMasterChild,
            m_flags,
            m_subtractLastChild,
            m_children,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbBlenderGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "referencePoseWeightThreshold",
                "blendParameter",
                "minCyclicBlendParameter",
                "maxCyclicBlendParameter",
                "indexOfSyncMasterChild",
                "flags",
                "subtractLastChild",
                "children",
                "childrenInternalStates",
                "sortedChildren",
                "endIntervalWeight",
                "numActiveChildren",
                "beginIntervalIndex",
                "endIntervalIndex",
                "initSync",
                "doSubtractiveBlend",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBlenderGenerator",
                FIELDS,
                __hkbBlenderGeneratorVisitor {
                    marker: _serde::__private::PhantomData::<hkbBlenderGenerator>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
