use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMonitorStreamFrameInfo`
/// -         version: `0`
/// -       signature: `0x7798b7db`
/// -          size:  36(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMonitorStreamFrameInfo<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `heading`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_heading: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `indexOfTimer0`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_indexOfTimer0: i32,
    /// # C++ Info
    /// -          name: `indexOfTimer1`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_indexOfTimer1: i32,
    /// # C++ Info
    /// -          name: `absoluteTimeCounter`(ctype: `enum AbsoluteTimeCounter`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_absoluteTimeCounter: AbsoluteTimeCounter,
    /// # C++ Info
    /// -          name: `timerFactor0`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timerFactor0: f32,
    /// # C++ Info
    /// -          name: `timerFactor1`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timerFactor1: f32,
    /// # C++ Info
    /// -          name: `threadId`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_threadId: i32,
    /// # C++ Info
    /// -          name: `frameStreamStart`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_frameStreamStart: i32,
    /// # C++ Info
    /// -          name: `frameStreamEnd`(ctype: `hkInt32`)
    /// -        offset:  32(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_frameStreamEnd: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkMonitorStreamFrameInfo<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMonitorStreamFrameInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2006497243u32)
        }
    }
    impl<'a> __serde::Serialize for hkMonitorStreamFrameInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMonitorStreamFrameInfo", class_meta)?;
            serializer.serialize_stringptr_meta_field("heading", &self.m_heading)?;
            serializer.serialize_field("indexOfTimer0", &self.m_indexOfTimer0)?;
            serializer.serialize_field("indexOfTimer1", &self.m_indexOfTimer1)?;
            serializer
                .serialize_field("absoluteTimeCounter", &self.m_absoluteTimeCounter)?;
            serializer.serialize_field("timerFactor0", &self.m_timerFactor0)?;
            serializer.serialize_field("timerFactor1", &self.m_timerFactor1)?;
            serializer.serialize_field("threadId", &self.m_threadId)?;
            serializer.serialize_field("frameStreamStart", &self.m_frameStreamStart)?;
            serializer.serialize_field("frameStreamEnd", &self.m_frameStreamEnd)?;
            serializer.serialize_stringptr_field("heading", &self.m_heading)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT32`
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
pub enum AbsoluteTimeCounter {
    #[default]
    ABSOLUTE_TIME_TIMER_0 = 0isize,
    ABSOLUTE_TIME_TIMER_1 = 1isize,
    ABSOLUTE_TIME_NOT_TIMED = -1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for AbsoluteTimeCounter {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ABSOLUTE_TIME_TIMER_0 => {
                    __serializer.serialize_field("ABSOLUTE_TIME_TIMER_0", &0u64)
                }
                Self::ABSOLUTE_TIME_TIMER_1 => {
                    __serializer.serialize_field("ABSOLUTE_TIME_TIMER_1", &1u64)
                }
                Self::ABSOLUTE_TIME_NOT_TIMED => {
                    __serializer
                        .serialize_field(
                            "ABSOLUTE_TIME_NOT_TIMED",
                            &18446744073709551615u64,
                        )
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u32()
                .ok_or(S::Error::custom("Failed enum AbsoluteTimeCounter to_u32"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
