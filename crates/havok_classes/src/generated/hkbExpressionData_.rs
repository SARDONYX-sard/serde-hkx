use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbExpressionData`
/// -         version: `0`
/// -       signature: `0x6740042a`
/// -          size:  16(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbExpressionData<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `expression`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_expression: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `assignmentVariableIndex`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_assignmentVariableIndex: i32,
    /// # C++ Info
    /// -          name: `assignmentEventIndex`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_assignmentEventIndex: i32,
    /// # C++ Info
    /// -          name: `eventMode`(ctype: `enum ExpressionEventMode`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_eventMode: ExpressionEventMode,
    /// # C++ Info
    /// -          name: `raisedEvent`(ctype: `hkBool`)
    /// -        offset:  13(x86)/ 17(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_raisedEvent: bool,
    /// # C++ Info
    /// -          name: `wasTrueInPreviousFrame`(ctype: `hkBool`)
    /// -        offset:  14(x86)/ 18(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_wasTrueInPreviousFrame: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbExpressionData<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbExpressionData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1732248618u32)
        }
    }
    impl<'a> __serde::Serialize for hkbExpressionData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbExpressionData", class_meta)?;
            serializer.serialize_stringptr_meta_field("expression", &self.m_expression)?;
            serializer
                .serialize_field(
                    "assignmentVariableIndex",
                    &self.m_assignmentVariableIndex,
                )?;
            serializer
                .serialize_field("assignmentEventIndex", &self.m_assignmentEventIndex)?;
            serializer.serialize_field("eventMode", &self.m_eventMode)?;
            serializer.skip_field("raisedEvent", &self.m_raisedEvent)?;
            serializer
                .skip_field("wasTrueInPreviousFrame", &self.m_wasTrueInPreviousFrame)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_stringptr_field("expression", &self.m_expression)?;
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
pub enum ExpressionEventMode {
    #[default]
    EVENT_MODE_SEND_ONCE = 0isize,
    EVENT_MODE_SEND_ON_TRUE = 1isize,
    EVENT_MODE_SEND_ON_FALSE_TO_TRUE = 2isize,
    EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ExpressionEventMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::EVENT_MODE_SEND_ONCE => {
                    __serializer.serialize_field("EVENT_MODE_SEND_ONCE", &0u64)
                }
                Self::EVENT_MODE_SEND_ON_TRUE => {
                    __serializer.serialize_field("EVENT_MODE_SEND_ON_TRUE", &1u64)
                }
                Self::EVENT_MODE_SEND_ON_FALSE_TO_TRUE => {
                    __serializer
                        .serialize_field("EVENT_MODE_SEND_ON_FALSE_TO_TRUE", &2u64)
                }
                Self::EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE => {
                    __serializer
                        .serialize_field("EVENT_MODE_SEND_EVERY_FRAME_ONCE_TRUE", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum ExpressionEventMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
