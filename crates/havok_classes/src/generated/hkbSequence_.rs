use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSequence`
/// -         version: `0`
/// -       signature: `0x43182ca3`
/// -          size: 168(x86)/248(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSequence<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `eventSequencedData`(ctype: `hkArray<hkbEventSequencedData*>`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_eventSequencedData: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `realVariableSequencedData`(ctype: `hkArray<hkbRealVariableSequencedData*>`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_realVariableSequencedData: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `boolVariableSequencedData`(ctype: `hkArray<hkbBoolVariableSequencedData*>`)
    /// -        offset:  68(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_boolVariableSequencedData: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `intVariableSequencedData`(ctype: `hkArray<hkbIntVariableSequencedData*>`)
    /// -        offset:  80(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_intVariableSequencedData: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `enableEventId`(ctype: `hkInt32`)
    /// -        offset:  92(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_enableEventId: i32,
    /// # C++ Info
    /// -          name: `disableEventId`(ctype: `hkInt32`)
    /// -        offset:  96(x86)/148(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_disableEventId: i32,
    /// # C++ Info
    /// -          name: `stringData`(ctype: `struct hkbSequenceStringData*`)
    /// -        offset: 100(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_stringData: Pointer,
    /// # C++ Info
    /// -          name: `variableIdMap`(ctype: `void*`)
    /// -        offset: 104(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_variableIdMap: Pointer,
    /// # C++ Info
    /// -          name: `eventIdMap`(ctype: `void*`)
    /// -        offset: 108(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_eventIdMap: Pointer,
    /// # C++ Info
    /// -          name: `nextSampleEvents`(ctype: `hkArray<void>`)
    /// -        offset: 112(x86)/176(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nextSampleEvents: Vec<()>,
    /// # C++ Info
    /// -          name: `nextSampleReals`(ctype: `hkArray<void>`)
    /// -        offset: 124(x86)/192(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nextSampleReals: Vec<()>,
    /// # C++ Info
    /// -          name: `nextSampleBools`(ctype: `hkArray<void>`)
    /// -        offset: 136(x86)/208(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nextSampleBools: Vec<()>,
    /// # C++ Info
    /// -          name: `nextSampleInts`(ctype: `hkArray<void>`)
    /// -        offset: 148(x86)/224(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nextSampleInts: Vec<()>,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset: 160(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_time: f32,
    /// # C++ Info
    /// -          name: `isEnabled`(ctype: `hkBool`)
    /// -        offset: 164(x86)/244(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isEnabled: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbSequence<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSequence"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1125657763u32)
        }
    }
    impl<'a> __serde::Serialize for hkbSequence<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1125657763u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbSequence", class_meta)?;
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
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "eventSequencedData",
                    &self.m_eventSequencedData,
                )?;
            serializer
                .serialize_array_meta_field(
                    "realVariableSequencedData",
                    &self.m_realVariableSequencedData,
                )?;
            serializer
                .serialize_array_meta_field(
                    "boolVariableSequencedData",
                    &self.m_boolVariableSequencedData,
                )?;
            serializer
                .serialize_array_meta_field(
                    "intVariableSequencedData",
                    &self.m_intVariableSequencedData,
                )?;
            serializer.serialize_field("enableEventId", &self.m_enableEventId)?;
            serializer.serialize_field("disableEventId", &self.m_disableEventId)?;
            serializer.serialize_field("stringData", &self.m_stringData)?;
            serializer.skip_field("variableIdMap", &self.m_variableIdMap)?;
            serializer.skip_field("eventIdMap", &self.m_eventIdMap)?;
            serializer
                .skip_array_meta_field("nextSampleEvents", &self.m_nextSampleEvents)?;
            serializer
                .skip_array_meta_field("nextSampleReals", &self.m_nextSampleReals)?;
            serializer
                .skip_array_meta_field("nextSampleBools", &self.m_nextSampleBools)?;
            serializer.skip_array_meta_field("nextSampleInts", &self.m_nextSampleInts)?;
            serializer.skip_field("time", &self.m_time)?;
            serializer.skip_field("isEnabled", &self.m_isEnabled)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field(
                    "eventSequencedData",
                    &self.m_eventSequencedData,
                )?;
            serializer
                .serialize_array_field(
                    "realVariableSequencedData",
                    &self.m_realVariableSequencedData,
                )?;
            serializer
                .serialize_array_field(
                    "boolVariableSequencedData",
                    &self.m_boolVariableSequencedData,
                )?;
            serializer
                .serialize_array_field(
                    "intVariableSequencedData",
                    &self.m_intVariableSequencedData,
                )?;
            serializer
                .serialize_array_field("nextSampleEvents", &self.m_nextSampleEvents)?;
            serializer
                .serialize_array_field("nextSampleReals", &self.m_nextSampleReals)?;
            serializer
                .serialize_array_field("nextSampleBools", &self.m_nextSampleBools)?;
            serializer.serialize_array_field("nextSampleInts", &self.m_nextSampleInts)?;
            serializer.end()
        }
    }
};
