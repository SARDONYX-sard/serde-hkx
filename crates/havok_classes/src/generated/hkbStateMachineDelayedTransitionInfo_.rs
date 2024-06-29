use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineDelayedTransitionInfo`
/// -         version: `1`
/// -       signature: `0x26d5499`
/// -          size:  24(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineDelayedTransitionInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `delayedTransition`(ctype: `struct hkbStateMachineProspectiveTransitionInfo`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_delayedTransition: hkbStateMachineProspectiveTransitionInfo,
    /// # C++ Info
    /// -          name: `timeDelayed`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timeDelayed: f32,
    /// # C++ Info
    /// -          name: `isDelayedTransitionReturnToPreviousState`(ctype: `hkBool`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isDelayedTransitionReturnToPreviousState: bool,
    /// # C++ Info
    /// -          name: `wasInAbutRangeLastFrame`(ctype: `hkBool`)
    /// -        offset:  21(x86)/ 21(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_wasInAbutRangeLastFrame: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbStateMachineDelayedTransitionInfo {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbStateMachineDelayedTransitionInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(40719513u32)
        }
    }
    impl __serde::Serialize for hkbStateMachineDelayedTransitionInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineDelayedTransitionInfo", class_meta)?;
            serializer.serialize_field("delayedTransition", &self.m_delayedTransition)?;
            serializer.serialize_field("timeDelayed", &self.m_timeDelayed)?;
            serializer
                .serialize_field(
                    "isDelayedTransitionReturnToPreviousState",
                    &self.m_isDelayedTransitionReturnToPreviousState,
                )?;
            serializer
                .serialize_field(
                    "wasInAbutRangeLastFrame",
                    &self.m_wasInAbutRangeLastFrame,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.end()
        }
    }
};
