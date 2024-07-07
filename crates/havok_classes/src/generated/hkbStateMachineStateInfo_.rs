use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineStateInfo`
/// -         version: `4`
/// -       signature: `0xed7f9d0`
/// -          size:  72(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineStateInfo<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbBindable,
    /// # C++ Info
    /// -          name: `listeners`(ctype: `hkArray<hkbStateListener*>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_listeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `enterNotifyEvents`(ctype: `struct hkbStateMachineEventPropertyArray*`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_enterNotifyEvents: Pointer,
    /// # C++ Info
    /// -          name: `exitNotifyEvents`(ctype: `struct hkbStateMachineEventPropertyArray*`)
    /// -        offset:  44(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_exitNotifyEvents: Pointer,
    /// # C++ Info
    /// -          name: `transitions`(ctype: `struct hkbStateMachineTransitionInfoArray*`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_transitions: Pointer,
    /// # C++ Info
    /// -          name: `generator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_generator: Pointer,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `stateId`(ctype: `hkInt32`)
    /// -        offset:  60(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_stateId: i32,
    /// # C++ Info
    /// -          name: `probability`(ctype: `hkReal`)
    /// -        offset:  64(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_probability: f32,
    /// # C++ Info
    /// -          name: `enable`(ctype: `hkBool`)
    /// -        offset:  68(x86)/112(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enable: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbStateMachineStateInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineStateInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xed7f9d0)
        }
    }
    impl<'a> _serde::Serialize for hkbStateMachineStateInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xed7f9d0)));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineStateInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field("areBindablesCached", &self.parent.m_areBindablesCached)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_meta_field("listeners", &self.m_listeners)?;
            serializer.serialize_field("enterNotifyEvents", &self.m_enterNotifyEvents)?;
            serializer.serialize_field("exitNotifyEvents", &self.m_exitNotifyEvents)?;
            serializer.serialize_field("transitions", &self.m_transitions)?;
            serializer.serialize_field("generator", &self.m_generator)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("stateId", &self.m_stateId)?;
            serializer.serialize_field("probability", &self.m_probability)?;
            serializer.serialize_field("enable", &self.m_enable)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                )?;
            serializer.serialize_array_field("listeners", &self.m_listeners)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
