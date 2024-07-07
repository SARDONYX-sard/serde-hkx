use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSEventEveryNEventsModifier`
/// -         version: `1`
/// -       signature: `0x6030970c`
/// -          size:  72(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSEventEveryNEventsModifier<'a> {
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
    /// -          name: `eventToCheckFor`(ctype: `struct hkbEventProperty`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_eventToCheckFor: hkbEventProperty,
    /// # C++ Info
    /// -          name: `eventToSend`(ctype: `struct hkbEventProperty`)
    /// -        offset:  52(x86)/ 96(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_eventToSend: hkbEventProperty,
    /// # C++ Info
    /// -          name: `numberOfEventsBeforeSend`(ctype: `hkInt8`)
    /// -        offset:  60(x86)/112(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numberOfEventsBeforeSend: i8,
    /// # C++ Info
    /// -          name: `minimumNumberOfEventsBeforeSend`(ctype: `hkInt8`)
    /// -        offset:  61(x86)/113(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_minimumNumberOfEventsBeforeSend: i8,
    /// # C++ Info
    /// -          name: `randomizeNumberOfEvents`(ctype: `hkBool`)
    /// -        offset:  62(x86)/114(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_randomizeNumberOfEvents: bool,
    /// # C++ Info
    /// -          name: `numberOfEventsSeen`(ctype: `hkInt32`)
    /// -        offset:  64(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numberOfEventsSeen: i32,
    /// # C++ Info
    /// -          name: `calculatedNumberOfEventsBeforeSend`(ctype: `hkInt8`)
    /// -        offset:  68(x86)/120(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_calculatedNumberOfEventsBeforeSend: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSEventEveryNEventsModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSEventEveryNEventsModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6030970c)
        }
    }
    impl<'a> _serde::Serialize for BSEventEveryNEventsModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6030970c)));
            let mut serializer = __serializer
                .serialize_struct("BSEventEveryNEventsModifier", class_meta)?;
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
            serializer.serialize_field("eventToCheckFor", &self.m_eventToCheckFor)?;
            serializer.serialize_field("eventToSend", &self.m_eventToSend)?;
            serializer
                .serialize_field(
                    "numberOfEventsBeforeSend",
                    &self.m_numberOfEventsBeforeSend,
                )?;
            serializer
                .serialize_field(
                    "minimumNumberOfEventsBeforeSend",
                    &self.m_minimumNumberOfEventsBeforeSend,
                )?;
            serializer
                .serialize_field(
                    "randomizeNumberOfEvents",
                    &self.m_randomizeNumberOfEvents,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.skip_field("numberOfEventsSeen", &self.m_numberOfEventsSeen)?;
            serializer
                .skip_field(
                    "calculatedNumberOfEventsBeforeSend",
                    &self.m_calculatedNumberOfEventsBeforeSend,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
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
