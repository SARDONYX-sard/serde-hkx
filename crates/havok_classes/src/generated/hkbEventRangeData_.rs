use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEventRangeData`
/// -         version: `0`
/// -       signature: `0x6cb92c76`
/// -          size:  16(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEventRangeData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `upperBound`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_upperBound: f32,
    /// # C++ Info
    /// -          name: `event`(ctype: `struct hkbEventProperty`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_event: hkbEventProperty,
    /// # C++ Info
    /// -          name: `eventMode`(ctype: `enum EventRangeMode`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_eventMode: EventRangeMode,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbEventRangeData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbEventRangeData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1824074870u32)
        }
    }
    impl __serde::Serialize for hkbEventRangeData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbEventRangeData", class_meta)?;
            serializer.serialize_field("upperBound", &self.m_upperBound)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("event", &self.m_event)?;
            serializer.serialize_field("eventMode", &self.m_eventMode)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum EventRangeMode {
    #[default]
    EVENT_MODE_SEND_ON_ENTER_RANGE = 0isize,
    EVENT_MODE_SEND_WHEN_IN_RANGE = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for EventRangeMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::EVENT_MODE_SEND_ON_ENTER_RANGE => {
                    __serializer.serialize_field("EVENT_MODE_SEND_ON_ENTER_RANGE", &0u64)
                }
                Self::EVENT_MODE_SEND_WHEN_IN_RANGE => {
                    __serializer.serialize_field("EVENT_MODE_SEND_WHEN_IN_RANGE", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum EventRangeMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
