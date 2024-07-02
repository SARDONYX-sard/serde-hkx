use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineTransitionInfo`
/// -         version: `1`
/// -       signature: `0xcdec8025`
/// -          size:  60(x86)/ 72(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineTransitionInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `triggerInterval`(ctype: `struct hkbStateMachineTimeInterval`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_triggerInterval: hkbStateMachineTimeInterval,
    /// # C++ Info
    /// -          name: `initiateInterval`(ctype: `struct hkbStateMachineTimeInterval`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_initiateInterval: hkbStateMachineTimeInterval,
    /// # C++ Info
    /// -          name: `transition`(ctype: `struct hkbTransitionEffect*`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_transition: Pointer,
    /// # C++ Info
    /// -          name: `condition`(ctype: `struct hkbCondition*`)
    /// -        offset:  36(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_condition: Pointer,
    /// # C++ Info
    /// -          name: `eventId`(ctype: `hkInt32`)
    /// -        offset:  40(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_eventId: i32,
    /// # C++ Info
    /// -          name: `toStateId`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_toStateId: i32,
    /// # C++ Info
    /// -          name: `fromNestedStateId`(ctype: `hkInt32`)
    /// -        offset:  48(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fromNestedStateId: i32,
    /// # C++ Info
    /// -          name: `toNestedStateId`(ctype: `hkInt32`)
    /// -        offset:  52(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_toNestedStateId: i32,
    /// # C++ Info
    /// -          name: `priority`(ctype: `hkInt16`)
    /// -        offset:  56(x86)/ 64(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_priority: i16,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags TransitionFlags`)
    /// -        offset:  58(x86)/ 66(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_flags: TransitionFlags,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineTransitionInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineTransitionInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3454828581u32)
        }
    }
    impl _serde::Serialize for hkbStateMachineTransitionInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3454828581u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineTransitionInfo", class_meta)?;
            serializer.serialize_field("triggerInterval", &self.m_triggerInterval)?;
            serializer.serialize_field("initiateInterval", &self.m_initiateInterval)?;
            serializer.serialize_field("transition", &self.m_transition)?;
            serializer.serialize_field("condition", &self.m_condition)?;
            serializer.serialize_field("eventId", &self.m_eventId)?;
            serializer.serialize_field("toStateId", &self.m_toStateId)?;
            serializer.serialize_field("fromNestedStateId", &self.m_fromNestedStateId)?;
            serializer.serialize_field("toNestedStateId", &self.m_toNestedStateId)?;
            serializer.serialize_field("priority", &self.m_priority)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_INT16`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    TransitionFlags : i16 { #[doc = "1"] const FLAG_USE_TRIGGER_INTERVAL = 1i16; #[doc =
    "2"] const FLAG_USE_INITIATE_INTERVAL = 2i16; #[doc = "4"] const
    FLAG_UNINTERRUPTIBLE_WHILE_PLAYING = 4i16; #[doc = "8"] const
    FLAG_UNINTERRUPTIBLE_WHILE_DELAYED = 8i16; #[doc = "16"] const
    FLAG_DELAY_STATE_CHANGE = 16i16; #[doc = "32"] const FLAG_DISABLED = 32i16; #[doc =
    "64"] const FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE = 64i16; #[doc = "128"] const
    FLAG_DISALLOW_RANDOM_TRANSITION = 128i16; #[doc = "256"] const FLAG_DISABLE_CONDITION
    = 256i16; #[doc = "512"] const
    FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE = 512i16; #[doc = "1024"]
    const FLAG_IS_GLOBAL_WILDCARD = 1024i16; #[doc = "2048"] const FLAG_IS_LOCAL_WILDCARD
    = 2048i16; #[doc = "4096"] const FLAG_FROM_NESTED_STATE_ID_IS_VALID = 4096i16; #[doc
    = "8192"] const FLAG_TO_NESTED_STATE_ID_IS_VALID = 8192i16; #[doc = "16384"] const
    FLAG_ABUT_AT_END_OF_FROM_GENERATOR = 16384i16; }
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for TransitionFlags {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::FLAG_USE_TRIGGER_INTERVAL => {
                        __serializer
                            .serialize_field(
                                "FLAG_USE_TRIGGER_INTERVAL",
                                &Self::FLAG_USE_TRIGGER_INTERVAL,
                            )
                    }
                    Self::FLAG_USE_INITIATE_INTERVAL => {
                        __serializer
                            .serialize_field(
                                "FLAG_USE_INITIATE_INTERVAL",
                                &Self::FLAG_USE_INITIATE_INTERVAL,
                            )
                    }
                    Self::FLAG_UNINTERRUPTIBLE_WHILE_PLAYING => {
                        __serializer
                            .serialize_field(
                                "FLAG_UNINTERRUPTIBLE_WHILE_PLAYING",
                                &Self::FLAG_UNINTERRUPTIBLE_WHILE_PLAYING,
                            )
                    }
                    Self::FLAG_UNINTERRUPTIBLE_WHILE_DELAYED => {
                        __serializer
                            .serialize_field(
                                "FLAG_UNINTERRUPTIBLE_WHILE_DELAYED",
                                &Self::FLAG_UNINTERRUPTIBLE_WHILE_DELAYED,
                            )
                    }
                    Self::FLAG_DELAY_STATE_CHANGE => {
                        __serializer
                            .serialize_field(
                                "FLAG_DELAY_STATE_CHANGE",
                                &Self::FLAG_DELAY_STATE_CHANGE,
                            )
                    }
                    Self::FLAG_DISABLED => {
                        __serializer
                            .serialize_field("FLAG_DISABLED", &Self::FLAG_DISABLED)
                    }
                    Self::FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE => {
                        __serializer
                            .serialize_field(
                                "FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE",
                                &Self::FLAG_DISALLOW_RETURN_TO_PREVIOUS_STATE,
                            )
                    }
                    Self::FLAG_DISALLOW_RANDOM_TRANSITION => {
                        __serializer
                            .serialize_field(
                                "FLAG_DISALLOW_RANDOM_TRANSITION",
                                &Self::FLAG_DISALLOW_RANDOM_TRANSITION,
                            )
                    }
                    Self::FLAG_DISABLE_CONDITION => {
                        __serializer
                            .serialize_field(
                                "FLAG_DISABLE_CONDITION",
                                &Self::FLAG_DISABLE_CONDITION,
                            )
                    }
                    Self::FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE => {
                        __serializer
                            .serialize_field(
                                "FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE",
                                &Self::FLAG_ALLOW_SELF_TRANSITION_BY_TRANSITION_FROM_ANY_STATE,
                            )
                    }
                    Self::FLAG_IS_GLOBAL_WILDCARD => {
                        __serializer
                            .serialize_field(
                                "FLAG_IS_GLOBAL_WILDCARD",
                                &Self::FLAG_IS_GLOBAL_WILDCARD,
                            )
                    }
                    Self::FLAG_IS_LOCAL_WILDCARD => {
                        __serializer
                            .serialize_field(
                                "FLAG_IS_LOCAL_WILDCARD",
                                &Self::FLAG_IS_LOCAL_WILDCARD,
                            )
                    }
                    Self::FLAG_FROM_NESTED_STATE_ID_IS_VALID => {
                        __serializer
                            .serialize_field(
                                "FLAG_FROM_NESTED_STATE_ID_IS_VALID",
                                &Self::FLAG_FROM_NESTED_STATE_ID_IS_VALID,
                            )
                    }
                    Self::FLAG_TO_NESTED_STATE_ID_IS_VALID => {
                        __serializer
                            .serialize_field(
                                "FLAG_TO_NESTED_STATE_ID_IS_VALID",
                                &Self::FLAG_TO_NESTED_STATE_ID_IS_VALID,
                            )
                    }
                    Self::FLAG_ABUT_AT_END_OF_FROM_GENERATOR => {
                        __serializer
                            .serialize_field(
                                "FLAG_ABUT_AT_END_OF_FROM_GENERATOR",
                                &Self::FLAG_ABUT_AT_END_OF_FROM_GENERATOR,
                            )
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TransitionFlags {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TransitionFlags>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TransitionFlags;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct TransitionFlags(flags)",
                    )
                }
                #[inline]
                fn visit_int16<__E>(
                    self,
                    __value: i16,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    Ok(TransitionFlags::from_bits_retain(__value as _))
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match <TransitionFlags as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Int16,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TransitionFlags>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
