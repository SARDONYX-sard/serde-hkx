use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEventSequencedDataSequencedEvent`
/// -         version: `0`
/// -       signature: `0x9139b821`
/// -          size:  16(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEventSequencedDataSequencedEvent {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `event`(ctype: `struct hkbEvent`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_event: hkbEvent,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_time: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbEventSequencedDataSequencedEvent {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbEventSequencedDataSequencedEvent"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2436479009u32)
        }
    }
    impl __serde::Serialize for hkbEventSequencedDataSequencedEvent {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2436479009u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbEventSequencedDataSequencedEvent", class_meta)?;
            serializer.serialize_field("event", &self.m_event)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
