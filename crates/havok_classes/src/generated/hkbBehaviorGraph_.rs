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
