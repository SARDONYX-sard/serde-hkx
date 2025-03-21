use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbBlenderGenerator`
/// - version: `1`
/// - signature: `0x22df7147`
/// - size: `116`(x86)/`160`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// - name: `referencePoseWeightThreshold`(ctype: `hkReal`)
    /// - offset: ` 40`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "referencePoseWeightThreshold")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "referencePoseWeightThreshold"))]
    pub m_referencePoseWeightThreshold: f32,
    /// # C++ Info
    /// - name: `blendParameter`(ctype: `hkReal`)
    /// - offset: ` 44`(x86)/` 76`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "blendParameter"))]
    #[cfg_attr(feature = "serde", serde(rename = "blendParameter"))]
    pub m_blendParameter: f32,
    /// # C++ Info
    /// - name: `minCyclicBlendParameter`(ctype: `hkReal`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "minCyclicBlendParameter"))]
    #[cfg_attr(feature = "serde", serde(rename = "minCyclicBlendParameter"))]
    pub m_minCyclicBlendParameter: f32,
    /// # C++ Info
    /// - name: `maxCyclicBlendParameter`(ctype: `hkReal`)
    /// - offset: ` 52`(x86)/` 84`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxCyclicBlendParameter"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxCyclicBlendParameter"))]
    pub m_maxCyclicBlendParameter: f32,
    /// # C++ Info
    /// - name: `indexOfSyncMasterChild`(ctype: `hkInt16`)
    /// - offset: ` 56`(x86)/` 88`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "indexOfSyncMasterChild"))]
    #[cfg_attr(feature = "serde", serde(rename = "indexOfSyncMasterChild"))]
    pub m_indexOfSyncMasterChild: I16<'a>,
    /// # C++ Info
    /// - name: `flags`(ctype: `hkInt16`)
    /// - offset: ` 58`(x86)/` 90`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "flags"))]
    #[cfg_attr(feature = "serde", serde(rename = "flags"))]
    pub m_flags: I16<'a>,
    /// # C++ Info
    /// - name: `subtractLastChild`(ctype: `hkBool`)
    /// - offset: ` 60`(x86)/` 92`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "subtractLastChild"))]
    #[cfg_attr(feature = "serde", serde(rename = "subtractLastChild"))]
    pub m_subtractLastChild: bool,
    /// # C++ Info
    /// - name: `children`(ctype: `hkArray<hkbBlenderGeneratorChild*>`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "children"))]
    #[cfg_attr(feature = "serde", serde(rename = "children"))]
    pub m_children: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `childrenInternalStates`(ctype: `hkArray<void>`)
    /// - offset: ` 76`(x86)/`112`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "childrenInternalStates"))]
    #[cfg_attr(feature = "serde", serde(rename = "childrenInternalStates"))]
    pub m_childrenInternalStates: Vec<()>,
    /// # C++ Info
    /// - name: `sortedChildren`(ctype: `hkArray<void>`)
    /// - offset: ` 88`(x86)/`128`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "sortedChildren"))]
    #[cfg_attr(feature = "serde", serde(rename = "sortedChildren"))]
    pub m_sortedChildren: Vec<()>,
    /// # C++ Info
    /// - name: `endIntervalWeight`(ctype: `hkReal`)
    /// - offset: `100`(x86)/`144`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "endIntervalWeight"))]
    #[cfg_attr(feature = "serde", serde(rename = "endIntervalWeight"))]
    pub m_endIntervalWeight: f32,
    /// # C++ Info
    /// - name: `numActiveChildren`(ctype: `hkInt32`)
    /// - offset: `104`(x86)/`148`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "numActiveChildren"))]
    #[cfg_attr(feature = "serde", serde(rename = "numActiveChildren"))]
    pub m_numActiveChildren: I32<'a>,
    /// # C++ Info
    /// - name: `beginIntervalIndex`(ctype: `hkInt16`)
    /// - offset: `108`(x86)/`152`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "beginIntervalIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "beginIntervalIndex"))]
    pub m_beginIntervalIndex: I16<'a>,
    /// # C++ Info
    /// - name: `endIntervalIndex`(ctype: `hkInt16`)
    /// - offset: `110`(x86)/`154`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "endIntervalIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "endIntervalIndex"))]
    pub m_endIntervalIndex: I16<'a>,
    /// # C++ Info
    /// - name: `initSync`(ctype: `hkBool`)
    /// - offset: `112`(x86)/`156`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "initSync"))]
    #[cfg_attr(feature = "serde", serde(rename = "initSync"))]
    pub m_initSync: bool,
    /// # C++ Info
    /// - name: `doSubtractiveBlend`(ctype: `hkBool`)
    /// - offset: `113`(x86)/`157`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "doSubtractiveBlend"))]
    #[cfg_attr(feature = "serde", serde(rename = "doSubtractiveBlend"))]
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
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.m_variableBindingSet);
            v.extend(self.m_children.iter());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbBlenderGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x22df7147)));
            let mut serializer = __serializer
                .serialize_struct("hkbBlenderGenerator", class_meta, (116u64, 160u64))?;
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
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer
                .skip_fixed_array_field(
                    "padNode",
                    self.parent.parent.m_padNode.as_slice(),
                    TypeSize::NonPtr,
                )?;
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
            serializer
                .serialize_array_field("children", &self.m_children, TypeSize::NonPtr)?;
            serializer
                .skip_array_field(
                    "childrenInternalStates",
                    &self.m_childrenInternalStates,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "sortedChildren",
                    &self.m_sortedChildren,
                    TypeSize::NonPtr,
                )?;
            serializer.skip_field("endIntervalWeight", &self.m_endIntervalWeight)?;
            serializer.skip_field("numActiveChildren", &self.m_numActiveChildren)?;
            serializer.skip_field("beginIntervalIndex", &self.m_beginIntervalIndex)?;
            serializer.skip_field("endIntervalIndex", &self.m_endIntervalIndex)?;
            serializer.skip_field("initSync", &self.m_initSync)?;
            serializer.skip_field("doSubtractiveBlend", &self.m_doSubtractiveBlend)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbBlenderGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_referencePoseWeightThreshold,
                m_blendParameter,
                m_minCyclicBlendParameter,
                m_maxCyclicBlendParameter,
                m_indexOfSyncMasterChild,
                m_flags,
                m_subtractLastChild,
                m_children,
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
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "referencePoseWeightThreshold" => {
                            Ok(__Field::m_referencePoseWeightThreshold)
                        }
                        "blendParameter" => Ok(__Field::m_blendParameter),
                        "minCyclicBlendParameter" => {
                            Ok(__Field::m_minCyclicBlendParameter)
                        }
                        "maxCyclicBlendParameter" => {
                            Ok(__Field::m_maxCyclicBlendParameter)
                        }
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
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkbBlenderGeneratorVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbBlenderGenerator<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbBlenderGeneratorVisitor<'de> {
                type Value = hkbBlenderGenerator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbBlenderGenerator",
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
                    let mut m_referencePoseWeightThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_blendParameter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minCyclicBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxCyclicBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_indexOfSyncMasterChild: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_subtractLastChild: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_children: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_childrenInternalStates: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    let mut m_sortedChildren: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_endIntervalWeight: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_numActiveChildren: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_beginIntervalIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_endIntervalIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
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
                            3usize => {
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
                            4usize => {
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
                                    match __A::next_value::<I16<'de>>(&mut __map) {
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
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_subtractLastChild,
                                ) {
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
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_childrenInternalStates,
                                ) {
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
                                if _serde::__private::Option::is_some(
                                    &m_endIntervalWeight,
                                ) {
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
                                if _serde::__private::Option::is_some(
                                    &m_numActiveChildren,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numActiveChildren",
                                        ),
                                    );
                                }
                                m_numActiveChildren = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_beginIntervalIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "beginIntervalIndex",
                                        ),
                                    );
                                }
                                m_beginIntervalIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
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
                                    match __A::next_value::<I16<'de>>(&mut __map) {
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
                                if _serde::__private::Option::is_some(
                                    &m_doSubtractiveBlend,
                                ) {
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendParameter",
                                ),
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "subtractLastChild",
                                ),
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sortedChildren",
                                ),
                            );
                        }
                    };
                    let m_endIntervalWeight = match m_endIntervalWeight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endIntervalWeight",
                                ),
                            );
                        }
                    };
                    let m_numActiveChildren = match m_numActiveChildren {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numActiveChildren",
                                ),
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endIntervalIndex",
                                ),
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
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_variableBindingSet: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_referencePoseWeightThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_blendParameter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minCyclicBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxCyclicBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_indexOfSyncMasterChild: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_subtractLastChild: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_children: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_referencePoseWeightThreshold => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_referencePoseWeightThreshold,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blendParameter) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minCyclicBlendParameter,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxCyclicBlendParameter,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_indexOfSyncMasterChild,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexOfSyncMasterChild",
                                        ),
                                    );
                                }
                                m_indexOfSyncMasterChild = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_flags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_flags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_subtractLastChild => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_subtractLastChild,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_children) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "children",
                                        ),
                                    );
                                }
                                m_children = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
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
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_referencePoseWeightThreshold = match m_referencePoseWeightThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referencePoseWeightThreshold",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blendParameter = match m_blendParameter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendParameter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minCyclicBlendParameter = match m_minCyclicBlendParameter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minCyclicBlendParameter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxCyclicBlendParameter = match m_maxCyclicBlendParameter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCyclicBlendParameter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indexOfSyncMasterChild = match m_indexOfSyncMasterChild {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexOfSyncMasterChild",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_subtractLastChild = match m_subtractLastChild {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "subtractLastChild",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_children = match m_children {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("children"),
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
                    let parent = hkbBindable {
                        __ptr: __ptr.clone(),
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr: __ptr.clone(),
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbGenerator {
                        __ptr: __ptr.clone(),
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbBlenderGenerator {
                        __ptr: __ptr.clone(),
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
