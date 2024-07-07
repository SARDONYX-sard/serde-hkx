use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineInternalState`
/// -         version: `0`
/// -       signature: `0xbd1a7502`
/// -          size:  80(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineInternalState {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `activeTransitions`(ctype: `hkArray<struct hkbStateMachineActiveTransitionInfo>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_activeTransitions: Vec<hkbStateMachineActiveTransitionInfo>,
    /// # C++ Info
    /// -          name: `transitionFlags`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_transitionFlags: Vec<u8>,
    /// # C++ Info
    /// -          name: `wildcardTransitionFlags`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wildcardTransitionFlags: Vec<u8>,
    /// # C++ Info
    /// -          name: `delayedTransitions`(ctype: `hkArray<struct hkbStateMachineDelayedTransitionInfo>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_delayedTransitions: Vec<hkbStateMachineDelayedTransitionInfo>,
    /// # C++ Info
    /// -          name: `timeInState`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timeInState: f32,
    /// # C++ Info
    /// -          name: `lastLocalTime`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lastLocalTime: f32,
    /// # C++ Info
    /// -          name: `currentStateId`(ctype: `hkInt32`)
    /// -        offset:  64(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_currentStateId: i32,
    /// # C++ Info
    /// -          name: `previousStateId`(ctype: `hkInt32`)
    /// -        offset:  68(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_previousStateId: i32,
    /// # C++ Info
    /// -          name: `nextStartStateIndexOverride`(ctype: `hkInt32`)
    /// -        offset:  72(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_nextStartStateIndexOverride: i32,
    /// # C++ Info
    /// -          name: `stateOrTransitionChanged`(ctype: `hkBool`)
    /// -        offset:  76(x86)/100(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_stateOrTransitionChanged: bool,
    /// # C++ Info
    /// -          name: `echoNextUpdate`(ctype: `hkBool`)
    /// -        offset:  77(x86)/101(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_echoNextUpdate: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xbd1a7502)
        }
    }
    impl _serde::Serialize for hkbStateMachineInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xbd1a7502)));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "activeTransitions",
                    &self.m_activeTransitions,
                )?;
            serializer
                .serialize_array_meta_field("transitionFlags", &self.m_transitionFlags)?;
            serializer
                .serialize_array_meta_field(
                    "wildcardTransitionFlags",
                    &self.m_wildcardTransitionFlags,
                )?;
            serializer
                .serialize_array_meta_field(
                    "delayedTransitions",
                    &self.m_delayedTransitions,
                )?;
            serializer.serialize_field("timeInState", &self.m_timeInState)?;
            serializer.serialize_field("lastLocalTime", &self.m_lastLocalTime)?;
            serializer.serialize_field("currentStateId", &self.m_currentStateId)?;
            serializer.serialize_field("previousStateId", &self.m_previousStateId)?;
            serializer
                .serialize_field(
                    "nextStartStateIndexOverride",
                    &self.m_nextStartStateIndexOverride,
                )?;
            serializer
                .serialize_field(
                    "stateOrTransitionChanged",
                    &self.m_stateOrTransitionChanged,
                )?;
            serializer.serialize_field("echoNextUpdate", &self.m_echoNextUpdate)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_array_field("activeTransitions", &self.m_activeTransitions)?;
            serializer
                .serialize_array_field("transitionFlags", &self.m_transitionFlags)?;
            serializer
                .serialize_array_field(
                    "wildcardTransitionFlags",
                    &self.m_wildcardTransitionFlags,
                )?;
            serializer
                .serialize_array_field(
                    "delayedTransitions",
                    &self.m_delayedTransitions,
                )?;
            serializer.end()
        }
    }
};
