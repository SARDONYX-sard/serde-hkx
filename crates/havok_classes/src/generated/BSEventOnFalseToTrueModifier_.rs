use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSEventOnFalseToTrueModifier`
/// -         version: `1`
/// -       signature: `0x81d0777a`
/// -          size:  84(x86)/160(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSEventOnFalseToTrueModifier<'a> {
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
    /// -          name: `bEnableEvent1`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bEnableEvent1: bool,
    /// # C++ Info
    /// -          name: `bVariableToTest1`(ctype: `hkBool`)
    /// -        offset:  45(x86)/ 81(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bVariableToTest1: bool,
    /// # C++ Info
    /// -          name: `EventToSend1`(ctype: `struct hkbEventProperty`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_EventToSend1: hkbEventProperty,
    /// # C++ Info
    /// -          name: `bEnableEvent2`(ctype: `hkBool`)
    /// -        offset:  56(x86)/104(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bEnableEvent2: bool,
    /// # C++ Info
    /// -          name: `bVariableToTest2`(ctype: `hkBool`)
    /// -        offset:  57(x86)/105(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bVariableToTest2: bool,
    /// # C++ Info
    /// -          name: `EventToSend2`(ctype: `struct hkbEventProperty`)
    /// -        offset:  60(x86)/112(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_EventToSend2: hkbEventProperty,
    /// # C++ Info
    /// -          name: `bEnableEvent3`(ctype: `hkBool`)
    /// -        offset:  68(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bEnableEvent3: bool,
    /// # C++ Info
    /// -          name: `bVariableToTest3`(ctype: `hkBool`)
    /// -        offset:  69(x86)/129(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bVariableToTest3: bool,
    /// # C++ Info
    /// -          name: `EventToSend3`(ctype: `struct hkbEventProperty`)
    /// -        offset:  72(x86)/136(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_EventToSend3: hkbEventProperty,
    /// # C++ Info
    /// -          name: `bSlot1ActivatedLastFrame`(ctype: `hkBool`)
    /// -        offset:  80(x86)/152(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bSlot1ActivatedLastFrame: bool,
    /// # C++ Info
    /// -          name: `bSlot2ActivatedLastFrame`(ctype: `hkBool`)
    /// -        offset:  81(x86)/153(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bSlot2ActivatedLastFrame: bool,
    /// # C++ Info
    /// -          name: `bSlot3ActivatedLastFrame`(ctype: `hkBool`)
    /// -        offset:  82(x86)/154(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bSlot3ActivatedLastFrame: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSEventOnFalseToTrueModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSEventOnFalseToTrueModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x81d0777a)
        }
    }
    impl<'a> _serde::Serialize for BSEventOnFalseToTrueModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x81d0777a)));
            let mut serializer = __serializer
                .serialize_struct("BSEventOnFalseToTrueModifier", class_meta)?;
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
            serializer.serialize_field("bEnableEvent1", &self.m_bEnableEvent1)?;
            serializer.serialize_field("bVariableToTest1", &self.m_bVariableToTest1)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("EventToSend1", &self.m_EventToSend1)?;
            serializer.serialize_field("bEnableEvent2", &self.m_bEnableEvent2)?;
            serializer.serialize_field("bVariableToTest2", &self.m_bVariableToTest2)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("EventToSend2", &self.m_EventToSend2)?;
            serializer.serialize_field("bEnableEvent3", &self.m_bEnableEvent3)?;
            serializer.serialize_field("bVariableToTest3", &self.m_bVariableToTest3)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("EventToSend3", &self.m_EventToSend3)?;
            serializer
                .skip_field(
                    "bSlot1ActivatedLastFrame",
                    &self.m_bSlot1ActivatedLastFrame,
                )?;
            serializer
                .skip_field(
                    "bSlot2ActivatedLastFrame",
                    &self.m_bSlot2ActivatedLastFrame,
                )?;
            serializer
                .skip_field(
                    "bSlot3ActivatedLastFrame",
                    &self.m_bSlot3ActivatedLastFrame,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
