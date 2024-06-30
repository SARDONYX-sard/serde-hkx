use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCollisionFilter`
/// -         version: `0`
/// -       signature: `0x60960336`
/// -          size:  48(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCollisionFilter {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `prepad`(ctype: `hkUint32[2]`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_prepad: [u32; 2usize],
    /// # C++ Info
    /// -          name: `type`(ctype: `enum hkpFilterType`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_type: hkpFilterType,
    /// # C++ Info
    /// -          name: `postpad`(ctype: `hkUint32[3]`)
    /// -        offset:  36(x86)/ 60(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_postpad: [u32; 3usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCollisionFilter {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpCollisionFilter"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1620443958u32)
        }
    }
    impl __serde::Serialize for hkpCollisionFilter {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpCollisionFilter", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 16usize].as_slice(), [0u8; 32usize].as_slice())?;
            serializer.serialize_field("prepad", &self.m_prepad.as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("postpad", &self.m_postpad.as_slice())?;
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
pub enum hkpFilterType {
    #[default]
    HK_FILTER_UNKNOWN = 0isize,
    HK_FILTER_NULL = 1isize,
    HK_FILTER_GROUP = 2isize,
    HK_FILTER_LIST = 3isize,
    HK_FILTER_CUSTOM = 4isize,
    HK_FILTER_PAIR = 5isize,
    HK_FILTER_CONSTRAINT = 6isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for hkpFilterType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HK_FILTER_UNKNOWN => {
                    __serializer.serialize_field("HK_FILTER_UNKNOWN", &0u64)
                }
                Self::HK_FILTER_NULL => {
                    __serializer.serialize_field("HK_FILTER_NULL", &1u64)
                }
                Self::HK_FILTER_GROUP => {
                    __serializer.serialize_field("HK_FILTER_GROUP", &2u64)
                }
                Self::HK_FILTER_LIST => {
                    __serializer.serialize_field("HK_FILTER_LIST", &3u64)
                }
                Self::HK_FILTER_CUSTOM => {
                    __serializer.serialize_field("HK_FILTER_CUSTOM", &4u64)
                }
                Self::HK_FILTER_PAIR => {
                    __serializer.serialize_field("HK_FILTER_PAIR", &5u64)
                }
                Self::HK_FILTER_CONSTRAINT => {
                    __serializer.serialize_field("HK_FILTER_CONSTRAINT", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u32()
                .ok_or(S::Error::custom("Failed enum hkpFilterType to_u32"))?;
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
    impl<'de> _serde::Deserialize<'de> for hkpFilterType {
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
                        2u32 => _serde::__private::Ok(__Field::__field2),
                        3u32 => _serde::__private::Ok(__Field::__field3),
                        4u32 => _serde::__private::Ok(__Field::__field4),
                        5u32 => _serde::__private::Ok(__Field::__field5),
                        6u32 => _serde::__private::Ok(__Field::__field6),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint32(__value),
                                    &"value(u32) of variant is one of 0, 1, 2, 3, 4, 5, 6",
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
                                || v.eq_ignore_ascii_case("HK_FILTER_UNKNOWN") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("HK_FILTER_NULL") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("HK_FILTER_GROUP") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("HK_FILTER_LIST") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("HK_FILTER_CUSTOM") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5" || v.eq_ignore_ascii_case("HK_FILTER_PAIR") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("HK_FILTER_CONSTRAINT") => {
                                _serde::__private::Ok(__Field::__field6)
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
                marker: _serde::__private::PhantomData<hkpFilterType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = hkpFilterType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum hkpFilterType",
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
                            _serde::__private::Ok(hkpFilterType::HK_FILTER_UNKNOWN)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(hkpFilterType::HK_FILTER_NULL)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(hkpFilterType::HK_FILTER_GROUP)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(hkpFilterType::HK_FILTER_LIST)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(hkpFilterType::HK_FILTER_CUSTOM)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(hkpFilterType::HK_FILTER_PAIR)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(hkpFilterType::HK_FILTER_CONSTRAINT)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HK_FILTER_UNKNOWN",
                "HK_FILTER_NULL",
                "HK_FILTER_GROUP",
                "HK_FILTER_LIST",
                "HK_FILTER_CUSTOM",
                "HK_FILTER_PAIR",
                "HK_FILTER_CONSTRAINT",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "hkpFilterType",
                VARIANTS,
                _serde::de::ReadEnumSize::Uint32,
                __Visitor {
                    marker: _serde::__private::PhantomData::<hkpFilterType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
