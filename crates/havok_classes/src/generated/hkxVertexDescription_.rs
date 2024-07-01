use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxVertexDescription`
/// -         version: `1`
/// -       signature: `0x2df6313d`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxVertexDescription {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `decls`(ctype: `hkArray<struct hkxVertexDescriptionElementDecl>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_decls: Vec<hkxVertexDescriptionElementDecl>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxVertexDescription {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxVertexDescription"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(771109181u32)
        }
    }
    impl __serde::Serialize for hkxVertexDescription {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxVertexDescription", class_meta)?;
            serializer.serialize_array_meta_field("decls", &self.m_decls)?;
            serializer.serialize_array_field("decls", &self.m_decls)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT16`
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
pub enum DataType {
    #[default]
    HKX_DT_NONE = 0isize,
    HKX_DT_UINT8 = 1isize,
    HKX_DT_INT16 = 2isize,
    HKX_DT_UINT32 = 3isize,
    HKX_DT_FLOAT = 4isize,
}
///- size(C++): `TYPE_UINT16`
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
pub enum DataUsage {
    #[default]
    HKX_DU_NONE = 0isize,
    HKX_DU_POSITION = 1isize,
    HKX_DU_COLOR = 2isize,
    HKX_DU_NORMAL = 4isize,
    HKX_DU_TANGENT = 8isize,
    HKX_DU_BINORMAL = 16isize,
    HKX_DU_TEXCOORD = 32isize,
    HKX_DU_BLENDWEIGHTS = 64isize,
    HKX_DU_BLENDINDICES = 128isize,
    HKX_DU_USERDATA = 256isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for DataType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HKX_DT_NONE => __serializer.serialize_field("HKX_DT_NONE", &0u64),
                Self::HKX_DT_UINT8 => __serializer.serialize_field("HKX_DT_UINT8", &1u64),
                Self::HKX_DT_INT16 => __serializer.serialize_field("HKX_DT_INT16", &2u64),
                Self::HKX_DT_UINT32 => {
                    __serializer.serialize_field("HKX_DT_UINT32", &3u64)
                }
                Self::HKX_DT_FLOAT => __serializer.serialize_field("HKX_DT_FLOAT", &4u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u16()
                .ok_or(S::Error::custom("Failed enum DataType to_u16"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for DataUsage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HKX_DU_NONE => __serializer.serialize_field("HKX_DU_NONE", &0u64),
                Self::HKX_DU_POSITION => {
                    __serializer.serialize_field("HKX_DU_POSITION", &1u64)
                }
                Self::HKX_DU_COLOR => __serializer.serialize_field("HKX_DU_COLOR", &2u64),
                Self::HKX_DU_NORMAL => {
                    __serializer.serialize_field("HKX_DU_NORMAL", &4u64)
                }
                Self::HKX_DU_TANGENT => {
                    __serializer.serialize_field("HKX_DU_TANGENT", &8u64)
                }
                Self::HKX_DU_BINORMAL => {
                    __serializer.serialize_field("HKX_DU_BINORMAL", &16u64)
                }
                Self::HKX_DU_TEXCOORD => {
                    __serializer.serialize_field("HKX_DU_TEXCOORD", &32u64)
                }
                Self::HKX_DU_BLENDWEIGHTS => {
                    __serializer.serialize_field("HKX_DU_BLENDWEIGHTS", &64u64)
                }
                Self::HKX_DU_BLENDINDICES => {
                    __serializer.serialize_field("HKX_DU_BLENDINDICES", &128u64)
                }
                Self::HKX_DU_USERDATA => {
                    __serializer.serialize_field("HKX_DU_USERDATA", &256u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u16()
                .ok_or(S::Error::custom("Failed enum DataUsage to_u16"))?;
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
    impl<'de> _serde::Deserialize<'de> for DataType {
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
                fn visit_uint16<__E>(
                    self,
                    __value: u16,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u16 => _serde::__private::Ok(__Field::__field0),
                        1u16 => _serde::__private::Ok(__Field::__field1),
                        2u16 => _serde::__private::Ok(__Field::__field2),
                        3u16 => _serde::__private::Ok(__Field::__field3),
                        4u16 => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint16(__value),
                                    &"value(u16) of variant is one of 0, 1, 2, 3, 4",
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
                            v if v == "0" || v.eq_ignore_ascii_case("HKX_DT_NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("HKX_DT_UINT8") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("HKX_DT_INT16") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("HKX_DT_UINT32") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("HKX_DT_FLOAT") => {
                                _serde::__private::Ok(__Field::__field4)
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
                        _serde::de::ReadEnumSize::Uint16,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<DataType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DataType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum DataType")
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
                            _serde::__private::Ok(DataType::HKX_DT_NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataType::HKX_DT_UINT8)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataType::HKX_DT_INT16)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataType::HKX_DT_UINT32)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataType::HKX_DT_FLOAT)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HKX_DT_NONE",
                "HKX_DT_UINT8",
                "HKX_DT_INT16",
                "HKX_DT_UINT32",
                "HKX_DT_FLOAT",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "DataType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<DataType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DataUsage {
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
                fn visit_uint16<__E>(
                    self,
                    __value: u16,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u16 => _serde::__private::Ok(__Field::__field0),
                        1u16 => _serde::__private::Ok(__Field::__field1),
                        2u16 => _serde::__private::Ok(__Field::__field2),
                        4u16 => _serde::__private::Ok(__Field::__field3),
                        8u16 => _serde::__private::Ok(__Field::__field4),
                        16u16 => _serde::__private::Ok(__Field::__field5),
                        32u16 => _serde::__private::Ok(__Field::__field6),
                        64u16 => _serde::__private::Ok(__Field::__field7),
                        128u16 => _serde::__private::Ok(__Field::__field8),
                        256u16 => _serde::__private::Ok(__Field::__field9),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint16(__value),
                                    &"value(u16) of variant is one of 0, 1, 2, 4, 8, 16, 32, 64, 128, 256",
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
                            v if v == "0" || v.eq_ignore_ascii_case("HKX_DU_NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("HKX_DU_POSITION") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("HKX_DU_COLOR") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("HKX_DU_NORMAL") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("HKX_DU_TANGENT") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "16"
                                || v.eq_ignore_ascii_case("HKX_DU_BINORMAL") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "32"
                                || v.eq_ignore_ascii_case("HKX_DU_TEXCOORD") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "64"
                                || v.eq_ignore_ascii_case("HKX_DU_BLENDWEIGHTS") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "128"
                                || v.eq_ignore_ascii_case("HKX_DU_BLENDINDICES") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "256"
                                || v.eq_ignore_ascii_case("HKX_DU_USERDATA") => {
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
                        _serde::de::ReadEnumSize::Uint16,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<DataUsage>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DataUsage;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum DataUsage",
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
                            _serde::__private::Ok(DataUsage::HKX_DU_NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataUsage::HKX_DU_POSITION)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataUsage::HKX_DU_COLOR)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataUsage::HKX_DU_NORMAL)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataUsage::HKX_DU_TANGENT)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataUsage::HKX_DU_BINORMAL)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataUsage::HKX_DU_TEXCOORD)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataUsage::HKX_DU_BLENDWEIGHTS)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataUsage::HKX_DU_BLENDINDICES)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(DataUsage::HKX_DU_USERDATA)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HKX_DU_NONE",
                "HKX_DU_POSITION",
                "HKX_DU_COLOR",
                "HKX_DU_NORMAL",
                "HKX_DU_TANGENT",
                "HKX_DU_BINORMAL",
                "HKX_DU_TEXCOORD",
                "HKX_DU_BLENDWEIGHTS",
                "HKX_DU_BLENDINDICES",
                "HKX_DU_USERDATA",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "DataUsage",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<DataUsage>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
