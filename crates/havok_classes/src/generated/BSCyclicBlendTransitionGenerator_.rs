use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSCyclicBlendTransitionGenerator`
/// - version: `1`
/// - signature: `0x5119eb06`
/// - size: `112`(x86)/`176`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSCyclicBlendTransitionGenerator<'a> {
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
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// - name: `pBlenderGenerator`(ctype: `struct hkbGenerator*`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `ALIGN_16`
    #[cfg_attr(feature = "json_schema", schemars(rename = "pBlenderGenerator"))]
    #[cfg_attr(feature = "serde", serde(rename = "pBlenderGenerator"))]
    pub m_pBlenderGenerator: Pointer,
    /// # C++ Info
    /// - name: `EventToFreezeBlendValue`(ctype: `struct hkbEventProperty`)
    /// - offset: ` 52`(x86)/` 88`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "EventToFreezeBlendValue"))]
    #[cfg_attr(feature = "serde", serde(rename = "EventToFreezeBlendValue"))]
    pub m_EventToFreezeBlendValue: hkbEventProperty,
    /// # C++ Info
    /// - name: `EventToCrossBlend`(ctype: `struct hkbEventProperty`)
    /// - offset: ` 60`(x86)/`104`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "EventToCrossBlend"))]
    #[cfg_attr(feature = "serde", serde(rename = "EventToCrossBlend"))]
    pub m_EventToCrossBlend: hkbEventProperty,
    /// # C++ Info
    /// - name: `fBlendParameter`(ctype: `hkReal`)
    /// - offset: ` 68`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "fBlendParameter"))]
    #[cfg_attr(feature = "serde", serde(rename = "fBlendParameter"))]
    pub m_fBlendParameter: f32,
    /// # C++ Info
    /// - name: `fTransitionDuration`(ctype: `hkReal`)
    /// - offset: ` 72`(x86)/`124`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "fTransitionDuration"))]
    #[cfg_attr(feature = "serde", serde(rename = "fTransitionDuration"))]
    pub m_fTransitionDuration: f32,
    /// # C++ Info
    /// - name: `eBlendCurve`(ctype: `enum BlendCurve`)
    /// - offset: ` 76`(x86)/`128`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "eBlendCurve"))]
    #[cfg_attr(feature = "serde", serde(rename = "eBlendCurve"))]
    pub m_eBlendCurve: BlendCurve,
    /// # C++ Info
    /// - name: `pTransitionBlenderGenerator`(ctype: `void*`)
    /// - offset: ` 80`(x86)/`144`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `ALIGN_16|SERIALIZE_IGNORED`
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "pTransitionBlenderGenerator")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "pTransitionBlenderGenerator"))]
    pub m_pTransitionBlenderGenerator: Pointer,
    /// # C++ Info
    /// - name: `pTransitionEffect`(ctype: `void*`)
    /// - offset: ` 96`(x86)/`160`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `ALIGN_16|SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "pTransitionEffect"))]
    #[cfg_attr(feature = "serde", serde(rename = "pTransitionEffect"))]
    pub m_pTransitionEffect: Pointer,
    /// # C++ Info
    /// - name: `currentMode`(ctype: `enum unknown`)
    /// - offset: `100`(x86)/`168`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentMode"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentMode"))]
    pub m_currentMode: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSCyclicBlendTransitionGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSCyclicBlendTransitionGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5119eb06)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v.push(self.m_pBlenderGenerator.get());
            v.extend(self.m_EventToFreezeBlendValue.deps_indexes());
            v.extend(self.m_EventToCrossBlend.deps_indexes());
            v.push(self.m_pTransitionBlenderGenerator.get());
            v.push(self.m_pTransitionEffect.get());
            v
        }
    }
    impl<'a> _serde::Serialize for BSCyclicBlendTransitionGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5119eb06)));
            let mut serializer = __serializer
                .serialize_struct(
                    "BSCyclicBlendTransitionGenerator",
                    class_meta,
                    (112u64, 176u64),
                )?;
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
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("pBlenderGenerator", &self.m_pBlenderGenerator)?;
            serializer
                .serialize_field(
                    "EventToFreezeBlendValue",
                    &self.m_EventToFreezeBlendValue,
                )?;
            serializer.serialize_field("EventToCrossBlend", &self.m_EventToCrossBlend)?;
            serializer.serialize_field("fBlendParameter", &self.m_fBlendParameter)?;
            serializer
                .serialize_field("fTransitionDuration", &self.m_fTransitionDuration)?;
            serializer.serialize_field("eBlendCurve", &self.m_eBlendCurve)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer
                .skip_field(
                    "pTransitionBlenderGenerator",
                    &self.m_pTransitionBlenderGenerator,
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("pTransitionEffect", &self.m_pTransitionEffect)?;
            serializer.skip_field("currentMode", &self.m_currentMode)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSCyclicBlendTransitionGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_pBlenderGenerator,
                m_EventToFreezeBlendValue,
                m_EventToCrossBlend,
                m_fBlendParameter,
                m_fTransitionDuration,
                m_eBlendCurve,
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
                        "pBlenderGenerator" => Ok(__Field::m_pBlenderGenerator),
                        "EventToFreezeBlendValue" => {
                            Ok(__Field::m_EventToFreezeBlendValue)
                        }
                        "EventToCrossBlend" => Ok(__Field::m_EventToCrossBlend),
                        "fBlendParameter" => Ok(__Field::m_fBlendParameter),
                        "fTransitionDuration" => Ok(__Field::m_fTransitionDuration),
                        "eBlendCurve" => Ok(__Field::m_eBlendCurve),
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
            struct __BSCyclicBlendTransitionGeneratorVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    BSCyclicBlendTransitionGenerator<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __BSCyclicBlendTransitionGeneratorVisitor<'de> {
                type Value = BSCyclicBlendTransitionGenerator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct BSCyclicBlendTransitionGenerator",
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
                    let mut m_pBlenderGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_EventToFreezeBlendValue: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_EventToCrossBlend: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_fBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fTransitionDuration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_eBlendCurve: _serde::__private::Option<BlendCurve> = _serde::__private::None;
                    let mut m_pTransitionBlenderGenerator: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_pTransitionEffect: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_currentMode: _serde::__private::Option<i8> = _serde::__private::None;
                    for i in 0..9usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pBlenderGenerator,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pBlenderGenerator",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 8usize)?;
                                m_pBlenderGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_EventToFreezeBlendValue,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "EventToFreezeBlendValue",
                                        ),
                                    );
                                }
                                m_EventToFreezeBlendValue = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_EventToCrossBlend,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "EventToCrossBlend",
                                        ),
                                    );
                                }
                                m_EventToCrossBlend = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_fBlendParameter) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fBlendParameter",
                                        ),
                                    );
                                }
                                m_fBlendParameter = _serde::__private::Some(
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
                                    &m_fTransitionDuration,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fTransitionDuration",
                                        ),
                                    );
                                }
                                m_fTransitionDuration = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_eBlendCurve) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eBlendCurve",
                                        ),
                                    );
                                }
                                m_eBlendCurve = _serde::__private::Some(
                                    match __A::next_value::<BlendCurve>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pTransitionBlenderGenerator,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pTransitionBlenderGenerator",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 15usize)?;
                                m_pTransitionBlenderGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pTransitionEffect,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pTransitionEffect",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 12usize, 8usize)?;
                                m_pTransitionEffect = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_currentMode) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentMode",
                                        ),
                                    );
                                }
                                m_currentMode = _serde::__private::Some(
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
                    __A::pad(&mut __map, 11usize, 7usize)?;
                    let m_pBlenderGenerator = match m_pBlenderGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pBlenderGenerator",
                                ),
                            );
                        }
                    };
                    let m_EventToFreezeBlendValue = match m_EventToFreezeBlendValue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "EventToFreezeBlendValue",
                                ),
                            );
                        }
                    };
                    let m_EventToCrossBlend = match m_EventToCrossBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "EventToCrossBlend",
                                ),
                            );
                        }
                    };
                    let m_fBlendParameter = match m_fBlendParameter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fBlendParameter",
                                ),
                            );
                        }
                    };
                    let m_fTransitionDuration = match m_fTransitionDuration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fTransitionDuration",
                                ),
                            );
                        }
                    };
                    let m_eBlendCurve = match m_eBlendCurve {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eBlendCurve",
                                ),
                            );
                        }
                    };
                    let m_pTransitionBlenderGenerator = match m_pTransitionBlenderGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pTransitionBlenderGenerator",
                                ),
                            );
                        }
                    };
                    let m_pTransitionEffect = match m_pTransitionEffect {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pTransitionEffect",
                                ),
                            );
                        }
                    };
                    let m_currentMode = match m_currentMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentMode",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BSCyclicBlendTransitionGenerator {
                        __ptr,
                        parent,
                        m_pBlenderGenerator,
                        m_EventToFreezeBlendValue,
                        m_EventToCrossBlend,
                        m_fBlendParameter,
                        m_fTransitionDuration,
                        m_eBlendCurve,
                        m_pTransitionBlenderGenerator,
                        m_pTransitionEffect,
                        m_currentMode,
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
                    let mut m_variableBindingSet: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_pBlenderGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_EventToFreezeBlendValue: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_EventToCrossBlend: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_fBlendParameter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fTransitionDuration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_eBlendCurve: _serde::__private::Option<BlendCurve> = _serde::__private::None;
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
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                            __Field::m_pBlenderGenerator => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pBlenderGenerator,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pBlenderGenerator",
                                        ),
                                    );
                                }
                                m_pBlenderGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_EventToFreezeBlendValue => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_EventToFreezeBlendValue,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "EventToFreezeBlendValue",
                                        ),
                                    );
                                }
                                m_EventToFreezeBlendValue = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_EventToCrossBlend => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_EventToCrossBlend,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "EventToCrossBlend",
                                        ),
                                    );
                                }
                                m_EventToCrossBlend = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fBlendParameter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fBlendParameter) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fBlendParameter",
                                        ),
                                    );
                                }
                                m_fBlendParameter = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fTransitionDuration => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_fTransitionDuration,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fTransitionDuration",
                                        ),
                                    );
                                }
                                m_fTransitionDuration = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_eBlendCurve => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eBlendCurve) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eBlendCurve",
                                        ),
                                    );
                                }
                                m_eBlendCurve = _serde::__private::Some(
                                    match __A::next_value::<BlendCurve>(&mut __map) {
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
                    let m_pBlenderGenerator = match m_pBlenderGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pBlenderGenerator",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_EventToFreezeBlendValue = match m_EventToFreezeBlendValue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "EventToFreezeBlendValue",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_EventToCrossBlend = match m_EventToCrossBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "EventToCrossBlend",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fBlendParameter = match m_fBlendParameter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fBlendParameter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fTransitionDuration = match m_fTransitionDuration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fTransitionDuration",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_eBlendCurve = match m_eBlendCurve {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eBlendCurve",
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
                    let parent = hkbBindable {
                        __ptr,
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr,
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbGenerator { __ptr, parent };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(BSCyclicBlendTransitionGenerator {
                        __ptr,
                        parent,
                        m_pBlenderGenerator,
                        m_EventToFreezeBlendValue,
                        m_EventToCrossBlend,
                        m_fBlendParameter,
                        m_fTransitionDuration,
                        m_eBlendCurve,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "pBlenderGenerator",
                "EventToFreezeBlendValue",
                "EventToCrossBlend",
                "fBlendParameter",
                "fTransitionDuration",
                "eBlendCurve",
                "pTransitionBlenderGenerator",
                "pTransitionEffect",
                "currentMode",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSCyclicBlendTransitionGenerator",
                FIELDS,
                __BSCyclicBlendTransitionGeneratorVisitor {
                    marker: _serde::__private::PhantomData::<
                        BSCyclicBlendTransitionGenerator,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
