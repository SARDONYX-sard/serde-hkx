use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineActiveTransitionInfo`
/// -         version: `1`
/// -       signature: `0xbb90d54f`
/// -          size:  32(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineActiveTransitionInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `transitionEffect`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_transitionEffect: Pointer,
    /// # C++ Info
    /// -          name: `transitionEffectInternalStateInfo`(ctype: `struct hkbNodeInternalStateInfo*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_transitionEffectInternalStateInfo: Pointer,
    /// # C++ Info
    /// -          name: `transitionInfoReference`(ctype: `struct hkbStateMachineTransitionInfoReference`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   6(x86)/  6(x86_64)
    ///
    pub m_transitionInfoReference: hkbStateMachineTransitionInfoReference,
    /// # C++ Info
    /// -          name: `transitionInfoReferenceForTE`(ctype: `struct hkbStateMachineTransitionInfoReference`)
    /// -        offset:  14(x86)/ 22(x86_64)
    /// -     type_size:   6(x86)/  6(x86_64)
    ///
    pub m_transitionInfoReferenceForTE: hkbStateMachineTransitionInfoReference,
    /// # C++ Info
    /// -          name: `fromStateId`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fromStateId: i32,
    /// # C++ Info
    /// -          name: `toStateId`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_toStateId: i32,
    /// # C++ Info
    /// -          name: `isReturnToPreviousState`(ctype: `hkBool`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isReturnToPreviousState: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineActiveTransitionInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineActiveTransitionInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3146831183u32)
        }
    }
    impl _serde::Serialize for hkbStateMachineActiveTransitionInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3146831183u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineActiveTransitionInfo", class_meta)?;
            serializer.skip_field("transitionEffect", &self.m_transitionEffect)?;
            serializer
                .serialize_field(
                    "transitionEffectInternalStateInfo",
                    &self.m_transitionEffectInternalStateInfo,
                )?;
            serializer
                .serialize_field(
                    "transitionInfoReference",
                    &self.m_transitionInfoReference,
                )?;
            serializer
                .serialize_field(
                    "transitionInfoReferenceForTE",
                    &self.m_transitionInfoReferenceForTE,
                )?;
            serializer.serialize_field("fromStateId", &self.m_fromStateId)?;
            serializer.serialize_field("toStateId", &self.m_toStateId)?;
            serializer
                .serialize_field(
                    "isReturnToPreviousState",
                    &self.m_isReturnToPreviousState,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
