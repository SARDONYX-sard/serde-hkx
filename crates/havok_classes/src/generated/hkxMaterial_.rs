use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxMaterial`
/// -         version: `1`
/// -       signature: `0x2954537a`
/// -          size: 144(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMaterial<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkxAttributeHolder<'a>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `stages`(ctype: `hkArray<struct hkxMaterialTextureStage>`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_stages: Vec<hkxMaterialTextureStage>,
    /// # C++ Info
    /// -          name: `diffuseColor`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_diffuseColor: Vector4,
    /// # C++ Info
    /// -          name: `ambientColor`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_ambientColor: Vector4,
    /// # C++ Info
    /// -          name: `specularColor`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_specularColor: Vector4,
    /// # C++ Info
    /// -          name: `emissiveColor`(ctype: `hkVector4`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_emissiveColor: Vector4,
    /// # C++ Info
    /// -          name: `subMaterials`(ctype: `hkArray<hkxMaterial*>`)
    /// -        offset: 112(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_subMaterials: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `extraData`(ctype: `struct hkReferencedObject*`)
    /// -        offset: 124(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_extraData: Pointer,
    /// # C++ Info
    /// -          name: `properties`(ctype: `hkArray<struct hkxMaterialProperty>`)
    /// -        offset: 128(x86)/152(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_properties: Vec<hkxMaterialProperty>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkxMaterial<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxMaterial"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(693392250u32)
        }
    }
    impl<'a> __serde::Serialize for hkxMaterial<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(693392250u32)));
            let mut serializer = __serializer
                .serialize_struct("hkxMaterial", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "attributeGroups",
                    &self.parent.m_attributeGroups,
                )?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_array_meta_field("stages", &self.m_stages)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("diffuseColor", &self.m_diffuseColor)?;
            serializer.serialize_field("ambientColor", &self.m_ambientColor)?;
            serializer.serialize_field("specularColor", &self.m_specularColor)?;
            serializer.serialize_field("emissiveColor", &self.m_emissiveColor)?;
            serializer.serialize_array_meta_field("subMaterials", &self.m_subMaterials)?;
            serializer.serialize_field("extraData", &self.m_extraData)?;
            serializer.serialize_array_meta_field("properties", &self.m_properties)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "attributeGroups",
                    &self.parent.m_attributeGroups,
                )?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("stages", &self.m_stages)?;
            serializer.serialize_array_field("subMaterials", &self.m_subMaterials)?;
            serializer.serialize_array_field("properties", &self.m_properties)?;
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
pub enum TextureType {
    #[default]
    TEX_UNKNOWN = 0isize,
    TEX_DIFFUSE = 1isize,
    TEX_REFLECTION = 2isize,
    TEX_BUMP = 3isize,
    TEX_NORMAL = 4isize,
    TEX_DISPLACEMENT = 5isize,
    TEX_SPECULAR = 6isize,
    TEX_SPECULARANDGLOSS = 7isize,
    TEX_OPACITY = 8isize,
    TEX_EMISSIVE = 9isize,
    TEX_REFRACTION = 10isize,
    TEX_GLOSS = 11isize,
    TEX_NOTEXPORTED = 12isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for TextureType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TEX_UNKNOWN => __serializer.serialize_field("TEX_UNKNOWN", &0u64),
                Self::TEX_DIFFUSE => __serializer.serialize_field("TEX_DIFFUSE", &1u64),
                Self::TEX_REFLECTION => {
                    __serializer.serialize_field("TEX_REFLECTION", &2u64)
                }
                Self::TEX_BUMP => __serializer.serialize_field("TEX_BUMP", &3u64),
                Self::TEX_NORMAL => __serializer.serialize_field("TEX_NORMAL", &4u64),
                Self::TEX_DISPLACEMENT => {
                    __serializer.serialize_field("TEX_DISPLACEMENT", &5u64)
                }
                Self::TEX_SPECULAR => __serializer.serialize_field("TEX_SPECULAR", &6u64),
                Self::TEX_SPECULARANDGLOSS => {
                    __serializer.serialize_field("TEX_SPECULARANDGLOSS", &7u64)
                }
                Self::TEX_OPACITY => __serializer.serialize_field("TEX_OPACITY", &8u64),
                Self::TEX_EMISSIVE => __serializer.serialize_field("TEX_EMISSIVE", &9u64),
                Self::TEX_REFRACTION => {
                    __serializer.serialize_field("TEX_REFRACTION", &10u64)
                }
                Self::TEX_GLOSS => __serializer.serialize_field("TEX_GLOSS", &11u64),
                Self::TEX_NOTEXPORTED => {
                    __serializer.serialize_field("TEX_NOTEXPORTED", &12u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i32()
                .ok_or(S::Error::custom("Failed enum TextureType to_i32"))?;
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
    impl<'de> _serde::Deserialize<'de> for TextureType {
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
                __field10,
                __field11,
                __field12,
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
                        2i32 => _serde::__private::Ok(__Field::__field2),
                        3i32 => _serde::__private::Ok(__Field::__field3),
                        4i32 => _serde::__private::Ok(__Field::__field4),
                        5i32 => _serde::__private::Ok(__Field::__field5),
                        6i32 => _serde::__private::Ok(__Field::__field6),
                        7i32 => _serde::__private::Ok(__Field::__field7),
                        8i32 => _serde::__private::Ok(__Field::__field8),
                        9i32 => _serde::__private::Ok(__Field::__field9),
                        10i32 => _serde::__private::Ok(__Field::__field10),
                        11i32 => _serde::__private::Ok(__Field::__field11),
                        12i32 => _serde::__private::Ok(__Field::__field12),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int32(__value),
                                    &"value(i32) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12",
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
                            v if v == "0" || v.eq_ignore_ascii_case("TEX_UNKNOWN") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("TEX_DIFFUSE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("TEX_REFLECTION") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("TEX_BUMP") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("TEX_NORMAL") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("TEX_DISPLACEMENT") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6" || v.eq_ignore_ascii_case("TEX_SPECULAR") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("TEX_SPECULARANDGLOSS") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("TEX_OPACITY") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "9" || v.eq_ignore_ascii_case("TEX_EMISSIVE") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "10"
                                || v.eq_ignore_ascii_case("TEX_REFRACTION") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "11" || v.eq_ignore_ascii_case("TEX_GLOSS") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "12"
                                || v.eq_ignore_ascii_case("TEX_NOTEXPORTED") => {
                                _serde::__private::Ok(__Field::__field12)
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
                marker: _serde::__private::PhantomData<TextureType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TextureType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum TextureType",
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
                            _serde::__private::Ok(TextureType::TEX_UNKNOWN)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_DIFFUSE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_REFLECTION)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_BUMP)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_NORMAL)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_DISPLACEMENT)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_SPECULAR)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_SPECULARANDGLOSS)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_OPACITY)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_EMISSIVE)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_REFRACTION)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_GLOSS)
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TextureType::TEX_NOTEXPORTED)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "TEX_UNKNOWN",
                "TEX_DIFFUSE",
                "TEX_REFLECTION",
                "TEX_BUMP",
                "TEX_NORMAL",
                "TEX_DISPLACEMENT",
                "TEX_SPECULAR",
                "TEX_SPECULARANDGLOSS",
                "TEX_OPACITY",
                "TEX_EMISSIVE",
                "TEX_REFRACTION",
                "TEX_GLOSS",
                "TEX_NOTEXPORTED",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TextureType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TextureType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
