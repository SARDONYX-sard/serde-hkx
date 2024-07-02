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
            _serde::__private::Signature::new(585068871u32)
        }
    }
    impl<'a> _serde::Serialize for hkbBlenderGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(585068871u32)));
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
