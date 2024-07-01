use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpWeldingUtility`
/// -         version: `0`
/// -       signature: `0xb2b41feb`
/// -          size:   1(x86)/  1(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpWeldingUtility {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpWeldingUtility {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpWeldingUtility"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2998149099u32)
        }
    }
    impl __serde::Serialize for hkpWeldingUtility {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpWeldingUtility", class_meta)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT8`
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
pub enum WeldingType {
    #[default]
    WELDING_TYPE_ANTICLOCKWISE = 0isize,
    WELDING_TYPE_CLOCKWISE = 4isize,
    WELDING_TYPE_TWO_SIDED = 5isize,
    WELDING_TYPE_NONE = 6isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for WeldingType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::WELDING_TYPE_ANTICLOCKWISE => {
                    __serializer.serialize_field("WELDING_TYPE_ANTICLOCKWISE", &0u64)
                }
                Self::WELDING_TYPE_CLOCKWISE => {
                    __serializer.serialize_field("WELDING_TYPE_CLOCKWISE", &4u64)
                }
                Self::WELDING_TYPE_TWO_SIDED => {
                    __serializer.serialize_field("WELDING_TYPE_TWO_SIDED", &5u64)
                }
                Self::WELDING_TYPE_NONE => {
                    __serializer.serialize_field("WELDING_TYPE_NONE", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum WeldingType to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for WeldingType {
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
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint8<__E>(
                    self,
                    __value: u8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u8 => _serde::__private::Ok(__Field::__field0),
                        4u8 => _serde::__private::Ok(__Field::__field1),
                        5u8 => _serde::__private::Ok(__Field::__field2),
                        6u8 => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 4, 5, 6",
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
                                || v.eq_ignore_ascii_case("WELDING_TYPE_ANTICLOCKWISE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("WELDING_TYPE_CLOCKWISE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("WELDING_TYPE_TWO_SIDED") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("WELDING_TYPE_NONE") => {
                                _serde::__private::Ok(__Field::__field3)
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
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<WeldingType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = WeldingType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum WeldingType",
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
                                WeldingType::WELDING_TYPE_ANTICLOCKWISE,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(WeldingType::WELDING_TYPE_CLOCKWISE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(WeldingType::WELDING_TYPE_TWO_SIDED)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(WeldingType::WELDING_TYPE_NONE)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "WELDING_TYPE_ANTICLOCKWISE",
                "WELDING_TYPE_CLOCKWISE",
                "WELDING_TYPE_TWO_SIDED",
                "WELDING_TYPE_NONE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "WeldingType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<WeldingType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
