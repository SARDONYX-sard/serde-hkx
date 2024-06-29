use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineNestedStateMachineData`
/// -         version: `0`
/// -       signature: `0x7358f5da`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineNestedStateMachineData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `nestedStateMachine`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nestedStateMachine: Pointer,
    /// # C++ Info
    /// -          name: `eventIdMap`(ctype: `void*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_eventIdMap: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbStateMachineNestedStateMachineData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbStateMachineNestedStateMachineData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1935209946u32)
        }
    }
    impl __serde::Serialize for hkbStateMachineNestedStateMachineData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineNestedStateMachineData", class_meta)?;
            serializer.skip_field("nestedStateMachine", &self.m_nestedStateMachine)?;
            serializer.skip_field("eventIdMap", &self.m_eventIdMap)?;
            serializer.end()
        }
    }
};
