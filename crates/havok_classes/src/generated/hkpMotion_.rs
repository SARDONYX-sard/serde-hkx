use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMotion`
/// -         version: `3`
/// -       signature: `0x98aadb4f`
/// -          size: 288(x86)/320(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMotion {
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
    /// -          name: `type`(ctype: `enum MotionType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: MotionType,
    /// # C++ Info
    /// -          name: `deactivationIntegrateCounter`(ctype: `hkUint8`)
    /// -        offset:   9(x86)/ 17(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_deactivationIntegrateCounter: u8,
    /// # C++ Info
    /// -          name: `deactivationNumInactiveFrames`(ctype: `hkUint16[2]`)
    /// -        offset:  10(x86)/ 18(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_deactivationNumInactiveFrames: [u16; 2usize],
    /// # C++ Info
    /// -          name: `motionState`(ctype: `struct hkMotionState`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size: 176(x86)/176(x86_64)
    ///
    pub m_motionState: hkMotionState,
    /// # C++ Info
    /// -          name: `inertiaAndMassInv`(ctype: `hkVector4`)
    /// -        offset: 192(x86)/208(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_inertiaAndMassInv: Vector4,
    /// # C++ Info
    /// -          name: `linearVelocity`(ctype: `hkVector4`)
    /// -        offset: 208(x86)/224(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_linearVelocity: Vector4,
    /// # C++ Info
    /// -          name: `angularVelocity`(ctype: `hkVector4`)
    /// -        offset: 224(x86)/240(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_angularVelocity: Vector4,
    /// # C++ Info
    /// -          name: `deactivationRefPosition`(ctype: `hkVector4[2]`)
    /// -        offset: 240(x86)/256(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_deactivationRefPosition: [Vector4; 2usize],
    /// # C++ Info
    /// -          name: `deactivationRefOrientation`(ctype: `hkUint32[2]`)
    /// -        offset: 272(x86)/288(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_deactivationRefOrientation: [u32; 2usize],
    /// # C++ Info
    /// -          name: `savedMotion`(ctype: `struct hkpMaxSizeMotion*`)
    /// -        offset: 280(x86)/296(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_savedMotion: Pointer,
    /// # C++ Info
    /// -          name: `savedQualityTypeIndex`(ctype: `hkUint16`)
    /// -        offset: 284(x86)/304(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_savedQualityTypeIndex: u16,
    /// # C++ Info
    /// -          name: `gravityFactor`(ctype: `hkHalf`)
    /// -        offset: 286(x86)/306(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_gravityFactor: f16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMotion {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMotion"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2561334095u32)
        }
    }
    impl _serde::Serialize for hkpMotion {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2561334095u32)));
            let mut serializer = __serializer.serialize_struct("hkpMotion", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer
                .serialize_field(
                    "deactivationIntegrateCounter",
                    &self.m_deactivationIntegrateCounter,
                )?;
            serializer
                .serialize_field(
                    "deactivationNumInactiveFrames",
                    &self.m_deactivationNumInactiveFrames.as_slice(),
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.serialize_field("motionState", &self.m_motionState)?;
            serializer.serialize_field("inertiaAndMassInv", &self.m_inertiaAndMassInv)?;
            serializer.serialize_field("linearVelocity", &self.m_linearVelocity)?;
            serializer.serialize_field("angularVelocity", &self.m_angularVelocity)?;
            serializer
                .serialize_field(
                    "deactivationRefPosition",
                    &self.m_deactivationRefPosition.as_slice(),
                )?;
            serializer
                .serialize_field(
                    "deactivationRefOrientation",
                    &self.m_deactivationRefOrientation.as_slice(),
                )?;
            serializer.serialize_field("savedMotion", &self.m_savedMotion)?;
            serializer
                .serialize_field(
                    "savedQualityTypeIndex",
                    &self.m_savedQualityTypeIndex,
                )?;
            serializer.serialize_field("gravityFactor", &self.m_gravityFactor)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
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
pub enum MotionType {
    #[default]
    MOTION_INVALID = 0isize,
    MOTION_DYNAMIC = 1isize,
    MOTION_SPHERE_INERTIA = 2isize,
    MOTION_BOX_INERTIA = 3isize,
    MOTION_KEYFRAMED = 4isize,
    MOTION_FIXED = 5isize,
    MOTION_THIN_BOX_INERTIA = 6isize,
    MOTION_CHARACTER = 7isize,
    MOTION_MAX_ID = 8isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MotionType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MOTION_INVALID => {
                    __serializer.serialize_field("MOTION_INVALID", &0u64)
                }
                Self::MOTION_DYNAMIC => {
                    __serializer.serialize_field("MOTION_DYNAMIC", &1u64)
                }
                Self::MOTION_SPHERE_INERTIA => {
                    __serializer.serialize_field("MOTION_SPHERE_INERTIA", &2u64)
                }
                Self::MOTION_BOX_INERTIA => {
                    __serializer.serialize_field("MOTION_BOX_INERTIA", &3u64)
                }
                Self::MOTION_KEYFRAMED => {
                    __serializer.serialize_field("MOTION_KEYFRAMED", &4u64)
                }
                Self::MOTION_FIXED => __serializer.serialize_field("MOTION_FIXED", &5u64),
                Self::MOTION_THIN_BOX_INERTIA => {
                    __serializer.serialize_field("MOTION_THIN_BOX_INERTIA", &6u64)
                }
                Self::MOTION_CHARACTER => {
                    __serializer.serialize_field("MOTION_CHARACTER", &7u64)
                }
                Self::MOTION_MAX_ID => {
                    __serializer.serialize_field("MOTION_MAX_ID", &8u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum MotionType to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for MotionType {
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
                        1u8 => _serde::__private::Ok(__Field::__field1),
                        2u8 => _serde::__private::Ok(__Field::__field2),
                        3u8 => _serde::__private::Ok(__Field::__field3),
                        4u8 => _serde::__private::Ok(__Field::__field4),
                        5u8 => _serde::__private::Ok(__Field::__field5),
                        6u8 => _serde::__private::Ok(__Field::__field6),
                        7u8 => _serde::__private::Ok(__Field::__field7),
                        8u8 => _serde::__private::Ok(__Field::__field8),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8",
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
                            v if v == "0" || v.eq_ignore_ascii_case("MOTION_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("MOTION_DYNAMIC") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("MOTION_SPHERE_INERTIA") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("MOTION_BOX_INERTIA") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("MOTION_KEYFRAMED") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5" || v.eq_ignore_ascii_case("MOTION_FIXED") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("MOTION_THIN_BOX_INERTIA") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("MOTION_CHARACTER") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("MOTION_MAX_ID") => {
                                _serde::__private::Ok(__Field::__field8)
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
                marker: _serde::__private::PhantomData<MotionType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MotionType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum MotionType",
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
                            _serde::__private::Ok(MotionType::MOTION_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_DYNAMIC)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_SPHERE_INERTIA)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_BOX_INERTIA)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_KEYFRAMED)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_FIXED)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_THIN_BOX_INERTIA)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_CHARACTER)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_MAX_ID)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "MOTION_INVALID",
                "MOTION_DYNAMIC",
                "MOTION_SPHERE_INERTIA",
                "MOTION_BOX_INERTIA",
                "MOTION_KEYFRAMED",
                "MOTION_FIXED",
                "MOTION_THIN_BOX_INERTIA",
                "MOTION_CHARACTER",
                "MOTION_MAX_ID",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MotionType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MotionType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
