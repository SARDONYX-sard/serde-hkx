use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbContext`
/// -         version: `1`
/// -       signature: `0xe0c4d4a7`
/// -          size:  40(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbContext {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `character`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_character: Pointer,
    /// # C++ Info
    /// -          name: `behavior`(ctype: `void*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_behavior: Pointer,
    /// # C++ Info
    /// -          name: `nodeToIndexMap`(ctype: `void*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nodeToIndexMap: Pointer,
    /// # C++ Info
    /// -          name: `eventQueue`(ctype: `void*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_eventQueue: Pointer,
    /// # C++ Info
    /// -          name: `sharedEventQueue`(ctype: `void*`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_sharedEventQueue: Pointer,
    /// # C++ Info
    /// -          name: `generatorOutputListener`(ctype: `struct hkbGeneratorOutputListener*`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_generatorOutputListener: Pointer,
    /// # C++ Info
    /// -          name: `eventTriggeredTransition`(ctype: `hkBool`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_eventTriggeredTransition: bool,
    /// # C++ Info
    /// -          name: `world`(ctype: `void*`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_world: Pointer,
    /// # C++ Info
    /// -          name: `attachmentManager`(ctype: `void*`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_attachmentManager: Pointer,
    /// # C++ Info
    /// -          name: `animationCache`(ctype: `void*`)
    /// -        offset:  36(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_animationCache: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbContext {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbContext"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3770995879u32)
        }
    }
    impl _serde::Serialize for hkbContext {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3770995879u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbContext", class_meta)?;
            serializer.skip_field("character", &self.m_character)?;
            serializer.skip_field("behavior", &self.m_behavior)?;
            serializer.skip_field("nodeToIndexMap", &self.m_nodeToIndexMap)?;
            serializer.skip_field("eventQueue", &self.m_eventQueue)?;
            serializer.skip_field("sharedEventQueue", &self.m_sharedEventQueue)?;
            serializer
                .serialize_field(
                    "generatorOutputListener",
                    &self.m_generatorOutputListener,
                )?;
            serializer
                .skip_field(
                    "eventTriggeredTransition",
                    &self.m_eventTriggeredTransition,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_field("world", &self.m_world)?;
            serializer.skip_field("attachmentManager", &self.m_attachmentManager)?;
            serializer.skip_field("animationCache", &self.m_animationCache)?;
            serializer.end()
        }
    }
};
