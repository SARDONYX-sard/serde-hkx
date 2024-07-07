use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSkeletonMapperData`
/// -         version: `1`
/// -       signature: `0x95687ea0`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSkeletonMapperData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `skeletonA`(ctype: `struct hkaSkeleton*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_skeletonA: Pointer,
    /// # C++ Info
    /// -          name: `skeletonB`(ctype: `struct hkaSkeleton*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_skeletonB: Pointer,
    /// # C++ Info
    /// -          name: `simpleMappings`(ctype: `hkArray<struct hkaSkeletonMapperDataSimpleMapping>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_simpleMappings: Vec<hkaSkeletonMapperDataSimpleMapping>,
    /// # C++ Info
    /// -          name: `chainMappings`(ctype: `hkArray<struct hkaSkeletonMapperDataChainMapping>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_chainMappings: Vec<hkaSkeletonMapperDataChainMapping>,
    /// # C++ Info
    /// -          name: `unmappedBones`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_unmappedBones: Vec<i16>,
    /// # C++ Info
    /// -          name: `extractedMotionMapping`(ctype: `hkQsTransform`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_extractedMotionMapping: QsTransform,
    /// # C++ Info
    /// -          name: `keepUnmappedLocal`(ctype: `hkBool`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_keepUnmappedLocal: bool,
    /// # C++ Info
    /// -          name: `mappingType`(ctype: `enum MappingType`)
    /// -        offset: 100(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_mappingType: MappingType,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaSkeletonMapperData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSkeletonMapperData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x95687ea0)
        }
    }
    impl _serde::Serialize for hkaSkeletonMapperData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x95687ea0)));
            let mut serializer = __serializer
                .serialize_struct("hkaSkeletonMapperData", class_meta)?;
            serializer.serialize_field("skeletonA", &self.m_skeletonA)?;
            serializer.serialize_field("skeletonB", &self.m_skeletonB)?;
            serializer
                .serialize_array_meta_field("simpleMappings", &self.m_simpleMappings)?;
            serializer
                .serialize_array_meta_field("chainMappings", &self.m_chainMappings)?;
            serializer
                .serialize_array_meta_field("unmappedBones", &self.m_unmappedBones)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field(
                    "extractedMotionMapping",
                    &self.m_extractedMotionMapping,
                )?;
            serializer.serialize_field("keepUnmappedLocal", &self.m_keepUnmappedLocal)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("mappingType", &self.m_mappingType)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_array_field("simpleMappings", &self.m_simpleMappings)?;
            serializer.serialize_array_field("chainMappings", &self.m_chainMappings)?;
            serializer.serialize_array_field("unmappedBones", &self.m_unmappedBones)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT32`
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
pub enum MappingType {
    #[default]
    HK_RAGDOLL_MAPPING = 0isize,
    HK_RETARGETING_MAPPING = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MappingType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HK_RAGDOLL_MAPPING => {
                    __serializer.serialize_field("HK_RAGDOLL_MAPPING", &0u64)
                }
                Self::HK_RETARGETING_MAPPING => {
                    __serializer.serialize_field("HK_RETARGETING_MAPPING", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i32()
                .ok_or(S::Error::custom("Failed enum MappingType to_i32"))?;
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
    impl<'de> _serde::Deserialize<'de> for MappingType {
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
                fn visit_int32<__E>(
                    self,
                    __value: i32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i32 => _serde::__private::Ok(__Field::__field0),
                        1i32 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int32(__value),
                                    &"value(i32) of variant is one of 0, 1",
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
                                || v.eq_ignore_ascii_case("HK_RAGDOLL_MAPPING") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("HK_RETARGETING_MAPPING") => {
                                _serde::__private::Ok(__Field::__field1)
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
                        _serde::de::ReadEnumSize::Int32,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MappingType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MappingType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum MappingType",
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
                            _serde::__private::Ok(MappingType::HK_RAGDOLL_MAPPING)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MappingType::HK_RETARGETING_MAPPING)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HK_RAGDOLL_MAPPING",
                "HK_RETARGETING_MAPPING",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MappingType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MappingType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
