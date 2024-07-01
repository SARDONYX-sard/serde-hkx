use super::mock_requires::*;

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum EventMode {
    #[default]
    EventModeDefault = 0,
    EventModeProcessAll = 1,
    EventModeIgnoreFromGenerator = 2,
    EventModeIgnoreToGenerator = 3,
}

impl Serialize for EventMode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sv = serializer.serialize_enum_flags()?;

        // For XML
        match self {
            EventMode::EventModeDefault => sv.serialize_field("EVENT_MODE_DEFAULT", &0),
            EventMode::EventModeProcessAll => sv.serialize_field("EVENT_MODE_PROCESS_ALL", &1),
            EventMode::EventModeIgnoreFromGenerator => {
                sv.serialize_field("EVENT_MODE_IGNORE_FROM_GENERATOR", &2)
            }
            EventMode::EventModeIgnoreToGenerator => {
                sv.serialize_field("EVENT_MODE_IGNORE_TO_GENERATOR", &3)
            }
        }?;

        // For binary
        let n = self
            .to_i8()
            .ok_or(S::Error::custom("Failed enum to cast number"))?;

        use num_traits::ToPrimitive as _;
        sv.serialize_bits(&n)?;

        sv.end()
    }
}

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for EventMode {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_int8<__E>(self, __value: i8) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Int8(__value),
                            &"variant index 0 <= i < 4",
                        )),
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
                            v if v == "0" || v.eq_ignore_ascii_case("EVENT_MODE_DEFAULT") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("EVENT_MODE_PROCESSALL") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("EVENT_MODE_IGNORE_FROM_GENERATOR") =>
                            {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("EVENT_MODE_IGNORE_TO_GENERATOR") =>
                            {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                &__value, VARIANTS,
                            )),
                        }
                    } else {
                        _serde::__private::Err(_serde::de::Error::unknown_variant("None", VARIANTS))
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<EventMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = EventMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum EventMode")
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
                            _serde::__private::Ok(EventMode::EventModeDefault)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(EventMode::EventModeProcessAll)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(EventMode::EventModeIgnoreFromGenerator)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(EventMode::EventModeIgnoreToGenerator)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "EVENT_MODE_DEFAULT",
                "EVENT_MODE_PROCESS_ALL",
                "EVENT_MODE_IGNORE_FROM_GENERATOR",
                "EVENT_MODE_IGNORE_TO_GENERATOR",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "EventMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<EventMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
