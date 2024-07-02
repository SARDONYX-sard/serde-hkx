use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaAnimationBinding`
/// -         version: `1`
/// -       signature: `0x66eac971`
/// -          size:  44(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnimationBinding<'a> {
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
    /// -          name: `originalSkeletonName`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_originalSkeletonName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `animation`(ctype: `struct hkaAnimation*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_animation: Pointer,
    /// # C++ Info
    /// -          name: `transformTrackToBoneIndices`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_transformTrackToBoneIndices: Vec<i16>,
    /// # C++ Info
    /// -          name: `floatTrackToFloatSlotIndices`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_floatTrackToFloatSlotIndices: Vec<i16>,
    /// # C++ Info
    /// -          name: `blendHint`(ctype: `enum BlendHint`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_blendHint: BlendHint,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaAnimationBinding<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaAnimationBinding"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1726663025u32)
        }
    }
    impl<'a> _serde::Serialize for hkaAnimationBinding<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1726663025u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaAnimationBinding", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer.serialize_field("animation", &self.m_animation)?;
            serializer
                .serialize_array_meta_field(
                    "transformTrackToBoneIndices",
                    &self.m_transformTrackToBoneIndices,
                )?;
            serializer
                .serialize_array_meta_field(
                    "floatTrackToFloatSlotIndices",
                    &self.m_floatTrackToFloatSlotIndices,
                )?;
            serializer.serialize_field("blendHint", &self.m_blendHint)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_stringptr_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer
                .serialize_array_field(
                    "transformTrackToBoneIndices",
                    &self.m_transformTrackToBoneIndices,
                )?;
            serializer
                .serialize_array_field(
                    "floatTrackToFloatSlotIndices",
                    &self.m_floatTrackToFloatSlotIndices,
                )?;
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
pub enum BlendHint {
    #[default]
    NORMAL = 0isize,
    ADDITIVE = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BlendHint {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NORMAL => __serializer.serialize_field("NORMAL", &0u64),
                Self::ADDITIVE => __serializer.serialize_field("ADDITIVE", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BlendHint to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for BlendHint {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
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
                            v if v == "0" || v.eq_ignore_ascii_case("NORMAL") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("ADDITIVE") => {
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BlendHint>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BlendHint;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum BlendHint",
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
                            _serde::__private::Ok(BlendHint::NORMAL)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(BlendHint::ADDITIVE)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["NORMAL", "ADDITIVE"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "BlendHint",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BlendHint>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
