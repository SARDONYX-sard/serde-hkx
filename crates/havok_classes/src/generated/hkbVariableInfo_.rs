use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbVariableInfo`
/// -         version: `1`
/// -       signature: `0x9e746ba2`
/// -          size:   6(x86)/  6(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbVariableInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `role`(ctype: `struct hkbRoleAttribute`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_role: hkbRoleAttribute,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum VariableType`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: VariableType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbVariableInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbVariableInfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2658429858u32)
        }
    }
    impl __serde::Serialize for hkbVariableInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2658429858u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbVariableInfo", class_meta)?;
            serializer.serialize_field("role", &self.m_role)?;
            serializer.serialize_field("type", &self.m_type)?;
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
pub enum VariableType {
    #[default]
    VARIABLE_TYPE_INVALID = -1isize,
    VARIABLE_TYPE_BOOL = 0isize,
    VARIABLE_TYPE_INT8 = 1isize,
    VARIABLE_TYPE_INT16 = 2isize,
    VARIABLE_TYPE_INT32 = 3isize,
    VARIABLE_TYPE_REAL = 4isize,
    VARIABLE_TYPE_POINTER = 5isize,
    VARIABLE_TYPE_VECTOR3 = 6isize,
    VARIABLE_TYPE_VECTOR4 = 7isize,
    VARIABLE_TYPE_QUATERNION = 8isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for VariableType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::VARIABLE_TYPE_INVALID => {
                    __serializer
                        .serialize_field(
                            "VARIABLE_TYPE_INVALID",
                            &18446744073709551615u64,
                        )
                }
                Self::VARIABLE_TYPE_BOOL => {
                    __serializer.serialize_field("VARIABLE_TYPE_BOOL", &0u64)
                }
                Self::VARIABLE_TYPE_INT8 => {
                    __serializer.serialize_field("VARIABLE_TYPE_INT8", &1u64)
                }
                Self::VARIABLE_TYPE_INT16 => {
                    __serializer.serialize_field("VARIABLE_TYPE_INT16", &2u64)
                }
                Self::VARIABLE_TYPE_INT32 => {
                    __serializer.serialize_field("VARIABLE_TYPE_INT32", &3u64)
                }
                Self::VARIABLE_TYPE_REAL => {
                    __serializer.serialize_field("VARIABLE_TYPE_REAL", &4u64)
                }
                Self::VARIABLE_TYPE_POINTER => {
                    __serializer.serialize_field("VARIABLE_TYPE_POINTER", &5u64)
                }
                Self::VARIABLE_TYPE_VECTOR3 => {
                    __serializer.serialize_field("VARIABLE_TYPE_VECTOR3", &6u64)
                }
                Self::VARIABLE_TYPE_VECTOR4 => {
                    __serializer.serialize_field("VARIABLE_TYPE_VECTOR4", &7u64)
                }
                Self::VARIABLE_TYPE_QUATERNION => {
                    __serializer.serialize_field("VARIABLE_TYPE_QUATERNION", &8u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum VariableType to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for VariableType {
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
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
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
                        255u8 => _serde::__private::Ok(__Field::__field0),
                        0u8 => _serde::__private::Ok(__Field::__field1),
                        1u8 => _serde::__private::Ok(__Field::__field2),
                        2u8 => _serde::__private::Ok(__Field::__field3),
                        3u8 => _serde::__private::Ok(__Field::__field4),
                        4u8 => _serde::__private::Ok(__Field::__field5),
                        5u8 => _serde::__private::Ok(__Field::__field6),
                        6u8 => _serde::__private::Ok(__Field::__field7),
                        7u8 => _serde::__private::Ok(__Field::__field8),
                        8u8 => _serde::__private::Ok(__Field::__field9),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of -1, 0, 1, 2, 3, 4, 5, 6, 7, 8",
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
                            v if v == "-1"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "0"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_BOOL") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_INT8") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_INT16") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_INT32") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_REAL") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_POINTER") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_VECTOR3") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_VECTOR4") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "8"
                                || v.eq_ignore_ascii_case("VARIABLE_TYPE_QUATERNION") => {
                                _serde::__private::Ok(__Field::__field9)
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
                marker: _serde::__private::PhantomData<VariableType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = VariableType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum VariableType",
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
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_BOOL)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_INT8)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_INT16)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_INT32)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_REAL)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_POINTER)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_VECTOR3)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_VECTOR4)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VariableType::VARIABLE_TYPE_QUATERNION)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "VARIABLE_TYPE_INVALID",
                "VARIABLE_TYPE_BOOL",
                "VARIABLE_TYPE_INT8",
                "VARIABLE_TYPE_INT16",
                "VARIABLE_TYPE_INT32",
                "VARIABLE_TYPE_REAL",
                "VARIABLE_TYPE_POINTER",
                "VARIABLE_TYPE_VECTOR3",
                "VARIABLE_TYPE_VECTOR4",
                "VARIABLE_TYPE_QUATERNION",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "VariableType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<VariableType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
