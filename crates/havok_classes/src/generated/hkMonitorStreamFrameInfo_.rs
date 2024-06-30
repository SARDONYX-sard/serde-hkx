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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for AbsoluteTimeCounter {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint32<__E>(
                    self,
                    __value: u32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u32 => _serde::__private::Ok(__Field::__field0),
                        1u32 => _serde::__private::Ok(__Field::__field1),
                        4294967295u32 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint32(__value),
                                    &"value(u32) of variant is one of 0, 1, -1",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("ABSOLUTE_TIME_TIMER_0") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("ABSOLUTE_TIME_TIMER_1") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "-1"
                                || v.eq_ignore_ascii_case("ABSOLUTE_TIME_NOT_TIMED") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<AbsoluteTimeCounter>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AbsoluteTimeCounter;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum AbsoluteTimeCounter",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AbsoluteTimeCounter::ABSOLUTE_TIME_TIMER_0,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AbsoluteTimeCounter::ABSOLUTE_TIME_TIMER_1,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AbsoluteTimeCounter::ABSOLUTE_TIME_NOT_TIMED,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "ABSOLUTE_TIME_TIMER_0",
                "ABSOLUTE_TIME_TIMER_1",
                "ABSOLUTE_TIME_NOT_TIMED",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "AbsoluteTimeCounter",
                VARIANTS,
                _serde::de::ReadEnumSize::Uint32,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AbsoluteTimeCounter>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
