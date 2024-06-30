use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorGraph`
/// -         version: `1`
/// -       signature: `0xb1218f86`
/// -          size: 176(x86)/304(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorGraph<'a> {
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
    /// -          name: `variableMode`(ctype: `enum VariableMode`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_variableMode: VariableMode,
    /// # C++ Info
    /// -          name: `uniqueIdPool`(ctype: `hkArray<void>`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_uniqueIdPool: Vec<()>,
    /// # C++ Info
    /// -          name: `idToStateMachineTemplateMap`(ctype: `void*`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_idToStateMachineTemplateMap: Pointer,
    /// # C++ Info
    /// -          name: `mirroredExternalIdMap`(ctype: `hkArray<void>`)
    /// -        offset:  60(x86)/104(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_mirroredExternalIdMap: Vec<()>,
    /// # C++ Info
    /// -          name: `pseudoRandomGenerator`(ctype: `void*`)
    /// -        offset:  72(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pseudoRandomGenerator: Pointer,
    /// # C++ Info
    /// -          name: `rootGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  76(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rootGenerator: Pointer,
    /// # C++ Info
    /// -          name: `data`(ctype: `struct hkbBehaviorGraphData*`)
    /// -        offset:  80(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_data: Pointer,
    /// # C++ Info
    /// -          name: `rootGeneratorClone`(ctype: `void*`)
    /// -        offset:  84(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_rootGeneratorClone: Pointer,
    /// # C++ Info
    /// -          name: `activeNodes`(ctype: `void*`)
    /// -        offset:  88(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_activeNodes: Pointer,
    /// # C++ Info
    /// -          name: `activeNodeTemplateToIndexMap`(ctype: `void*`)
    /// -        offset:  92(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_activeNodeTemplateToIndexMap: Pointer,
    /// # C++ Info
    /// -          name: `activeNodesChildrenIndices`(ctype: `void*`)
    /// -        offset:  96(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_activeNodesChildrenIndices: Pointer,
    /// # C++ Info
    /// -          name: `globalTransitionData`(ctype: `void*`)
    /// -        offset: 100(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_globalTransitionData: Pointer,
    /// # C++ Info
    /// -          name: `eventIdMap`(ctype: `void*`)
    /// -        offset: 104(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_eventIdMap: Pointer,
    /// # C++ Info
    /// -          name: `attributeIdMap`(ctype: `void*`)
    /// -        offset: 108(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_attributeIdMap: Pointer,
    /// # C++ Info
    /// -          name: `variableIdMap`(ctype: `void*`)
    /// -        offset: 112(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_variableIdMap: Pointer,
    /// # C++ Info
    /// -          name: `characterPropertyIdMap`(ctype: `void*`)
    /// -        offset: 116(x86)/208(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_characterPropertyIdMap: Pointer,
    /// # C++ Info
    /// -          name: `variableValueSet`(ctype: `void*`)
    /// -        offset: 120(x86)/216(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_variableValueSet: Pointer,
    /// # C++ Info
    /// -          name: `nodeTemplateToCloneMap`(ctype: `void*`)
    /// -        offset: 124(x86)/224(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nodeTemplateToCloneMap: Pointer,
    /// # C++ Info
    /// -          name: `nodeCloneToTemplateMap`(ctype: `void*`)
    /// -        offset: 128(x86)/232(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nodeCloneToTemplateMap: Pointer,
    /// # C++ Info
    /// -          name: `stateListenerTemplateToCloneMap`(ctype: `void*`)
    /// -        offset: 132(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_stateListenerTemplateToCloneMap: Pointer,
    /// # C++ Info
    /// -          name: `nodePartitionInfo`(ctype: `void*`)
    /// -        offset: 136(x86)/248(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nodePartitionInfo: Pointer,
    /// # C++ Info
    /// -          name: `numIntermediateOutputs`(ctype: `hkInt32`)
    /// -        offset: 140(x86)/256(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numIntermediateOutputs: i32,
    /// # C++ Info
    /// -          name: `jobs`(ctype: `hkArray<void*>`)
    /// -        offset: 144(x86)/264(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_jobs: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `allPartitionMemory`(ctype: `hkArray<void*>`)
    /// -        offset: 156(x86)/280(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_allPartitionMemory: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `numStaticNodes`(ctype: `hkInt16`)
    /// -        offset: 168(x86)/296(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numStaticNodes: i16,
    /// # C++ Info
    /// -          name: `nextUniqueId`(ctype: `hkInt16`)
    /// -        offset: 170(x86)/298(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nextUniqueId: i16,
    /// # C++ Info
    /// -          name: `isActive`(ctype: `hkBool`)
    /// -        offset: 172(x86)/300(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isActive: bool,
    /// # C++ Info
    /// -          name: `isLinked`(ctype: `hkBool`)
    /// -        offset: 173(x86)/301(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isLinked: bool,
    /// # C++ Info
    /// -          name: `updateActiveNodes`(ctype: `hkBool`)
    /// -        offset: 174(x86)/302(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_updateActiveNodes: bool,
    /// # C++ Info
    /// -          name: `stateOrTransitionChanged`(ctype: `hkBool`)
    /// -        offset: 175(x86)/303(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_stateOrTransitionChanged: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbBehaviorGraph<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbBehaviorGraph"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2971766662u32)
        }
    }
    impl<'a> __serde::Serialize for hkbBehaviorGraph<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorGraph", class_meta)?;
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
            serializer.serialize_field("variableMode", &self.m_variableMode)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_array_meta_field("uniqueIdPool", &self.m_uniqueIdPool)?;
            serializer
                .skip_field(
                    "idToStateMachineTemplateMap",
                    &self.m_idToStateMachineTemplateMap,
                )?;
            serializer
                .skip_array_meta_field(
                    "mirroredExternalIdMap",
                    &self.m_mirroredExternalIdMap,
                )?;
            serializer
                .skip_field("pseudoRandomGenerator", &self.m_pseudoRandomGenerator)?;
            serializer.serialize_field("rootGenerator", &self.m_rootGenerator)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer.skip_field("rootGeneratorClone", &self.m_rootGeneratorClone)?;
            serializer.skip_field("activeNodes", &self.m_activeNodes)?;
            serializer
                .skip_field(
                    "activeNodeTemplateToIndexMap",
                    &self.m_activeNodeTemplateToIndexMap,
                )?;
            serializer
                .skip_field(
                    "activeNodesChildrenIndices",
                    &self.m_activeNodesChildrenIndices,
                )?;
            serializer.skip_field("globalTransitionData", &self.m_globalTransitionData)?;
            serializer.skip_field("eventIdMap", &self.m_eventIdMap)?;
            serializer.skip_field("attributeIdMap", &self.m_attributeIdMap)?;
            serializer.skip_field("variableIdMap", &self.m_variableIdMap)?;
            serializer
                .skip_field("characterPropertyIdMap", &self.m_characterPropertyIdMap)?;
            serializer.skip_field("variableValueSet", &self.m_variableValueSet)?;
            serializer
                .skip_field("nodeTemplateToCloneMap", &self.m_nodeTemplateToCloneMap)?;
            serializer
                .skip_field("nodeCloneToTemplateMap", &self.m_nodeCloneToTemplateMap)?;
            serializer
                .skip_field(
                    "stateListenerTemplateToCloneMap",
                    &self.m_stateListenerTemplateToCloneMap,
                )?;
            serializer.skip_field("nodePartitionInfo", &self.m_nodePartitionInfo)?;
            serializer
                .skip_field("numIntermediateOutputs", &self.m_numIntermediateOutputs)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_array_meta_field("jobs", &self.m_jobs)?;
            serializer
                .skip_array_meta_field(
                    "allPartitionMemory",
                    &self.m_allPartitionMemory,
                )?;
            serializer.skip_field("numStaticNodes", &self.m_numStaticNodes)?;
            serializer.skip_field("nextUniqueId", &self.m_nextUniqueId)?;
            serializer.skip_field("isActive", &self.m_isActive)?;
            serializer.skip_field("isLinked", &self.m_isLinked)?;
            serializer.skip_field("updateActiveNodes", &self.m_updateActiveNodes)?;
            serializer
                .skip_field(
                    "stateOrTransitionChanged",
                    &self.m_stateOrTransitionChanged,
                )?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("uniqueIdPool", &self.m_uniqueIdPool)?;
            serializer
                .serialize_array_field(
                    "mirroredExternalIdMap",
                    &self.m_mirroredExternalIdMap,
                )?;
            serializer.serialize_array_field("jobs", &self.m_jobs)?;
            serializer
                .serialize_array_field(
                    "allPartitionMemory",
                    &self.m_allPartitionMemory,
                )?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
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
pub enum VariableMode {
    #[default]
    VARIABLE_MODE_DISCARD_WHEN_INACTIVE = 0isize,
    VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for VariableMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::VARIABLE_MODE_DISCARD_WHEN_INACTIVE => {
                    __serializer
                        .serialize_field("VARIABLE_MODE_DISCARD_WHEN_INACTIVE", &0u64)
                }
                Self::VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE => {
                    __serializer
                        .serialize_field(
                            "VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE",
                            &1u64,
                        )
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum VariableMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for VariableMode {
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
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
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
                                || v
                                    .eq_ignore_ascii_case(
                                        "VARIABLE_MODE_DISCARD_WHEN_INACTIVE",
                                    ) => _serde::__private::Ok(__Field::__field0),
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE",
                                    ) => _serde::__private::Ok(__Field::__field1),
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
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<VariableMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = VariableMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum VariableMode",
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
                            _serde::__private::Ok(
                                VariableMode::VARIABLE_MODE_DISCARD_WHEN_INACTIVE,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                VariableMode::VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "VARIABLE_MODE_DISCARD_WHEN_INACTIVE",
                "VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "VariableMode",
                VARIANTS,
                _serde::de::ReadEnumSize::Int8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<VariableMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
