use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineProspectiveTransitionInfo`
/// -         version: `2`
/// -       signature: `0x3ab09a2e`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineProspectiveTransitionInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `transitionInfoReference`(ctype: `struct hkbStateMachineTransitionInfoReference`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   6(x86)/  6(x86_64)
    ///
    pub m_transitionInfoReference: hkbStateMachineTransitionInfoReference,
    /// # C++ Info
    /// -          name: `transitionInfoReferenceForTE`(ctype: `struct hkbStateMachineTransitionInfoReference`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   6(x86)/  6(x86_64)
    ///
    pub m_transitionInfoReferenceForTE: hkbStateMachineTransitionInfoReference,
    /// # C++ Info
    /// -          name: `toStateId`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_toStateId: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbStateMachineProspectiveTransitionInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineProspectiveTransitionInfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(984652334u32)
        }
    }
    impl __serde::Serialize for hkbStateMachineProspectiveTransitionInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(984652334u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbStateMachineProspectiveTransitionInfo",
                    class_meta,
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
            serializer.serialize_field("toStateId", &self.m_toStateId)?;
            serializer.end()
        }
    }
};
