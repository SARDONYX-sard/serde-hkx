use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMoppCode`
/// -         version: `0`
/// -       signature: `0x924c2661`
/// -          size:  48(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMoppCode {
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
    /// -          name: `info`(ctype: `struct hkpMoppCodeCodeInfo`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_info: hkpMoppCodeCodeInfo,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_data: Vec<u8>,
    /// # C++ Info
    /// -          name: `buildType`(ctype: `enum BuildType`)
    /// -        offset:  44(x86)/ 48(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_buildType: BuildType,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMoppCode {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMoppCode"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2454464097u32)
        }
    }
    impl _serde::Serialize for hkpMoppCode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2454464097u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpMoppCode", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("info", &self.m_info)?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_field("buildType", &self.m_buildType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer.serialize_array_field("data", &self.m_data)?;
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
pub enum BuildType {
    #[default]
    BUILT_WITH_CHUNK_SUBDIVISION = 0isize,
    BUILT_WITHOUT_CHUNK_SUBDIVISION = 1isize,
    BUILD_NOT_SET = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BuildType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BUILT_WITH_CHUNK_SUBDIVISION => {
                    __serializer.serialize_field("BUILT_WITH_CHUNK_SUBDIVISION", &0u64)
                }
                Self::BUILT_WITHOUT_CHUNK_SUBDIVISION => {
                    __serializer
                        .serialize_field("BUILT_WITHOUT_CHUNK_SUBDIVISION", &1u64)
                }
                Self::BUILD_NOT_SET => {
                    __serializer.serialize_field("BUILD_NOT_SET", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BuildType to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for BuildType {
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
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2",
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
                                || v
                                    .eq_ignore_ascii_case("BUILT_WITH_CHUNK_SUBDIVISION") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case("BUILT_WITHOUT_CHUNK_SUBDIVISION") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("BUILD_NOT_SET") => {
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BuildType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BuildType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum BuildType",
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
                                BuildType::BUILT_WITH_CHUNK_SUBDIVISION,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                BuildType::BUILT_WITHOUT_CHUNK_SUBDIVISION,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(BuildType::BUILD_NOT_SET)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "BUILT_WITH_CHUNK_SUBDIVISION",
                "BUILT_WITHOUT_CHUNK_SUBDIVISION",
                "BUILD_NOT_SET",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "BuildType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BuildType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
