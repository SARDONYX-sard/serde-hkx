use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEvaluateHandleModifier`
/// -         version: `2`
/// -       signature: `0x79757102`
/// -          size: 176(x86)/240(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEvaluateHandleModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `handle`(ctype: `struct hkbHandle*`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_handle: Pointer,
    /// # C++ Info
    /// -          name: `handlePositionOut`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_handlePositionOut: Vector4,
    /// # C++ Info
    /// -          name: `handleRotationOut`(ctype: `hkQuaternion`)
    /// -        offset:  64(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_handleRotationOut: Quaternion,
    /// # C++ Info
    /// -          name: `isValidOut`(ctype: `hkBool`)
    /// -        offset:  80(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isValidOut: bool,
    /// # C++ Info
    /// -          name: `extrapolationTimeStep`(ctype: `hkReal`)
    /// -        offset:  84(x86)/132(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_extrapolationTimeStep: f32,
    /// # C++ Info
    /// -          name: `handleChangeSpeed`(ctype: `hkReal`)
    /// -        offset:  88(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_handleChangeSpeed: f32,
    /// # C++ Info
    /// -          name: `handleChangeMode`(ctype: `enum HandleChangeMode`)
    /// -        offset:  92(x86)/140(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_handleChangeMode: HandleChangeMode,
    /// # C++ Info
    /// -          name: `oldHandle`(ctype: `struct hkbHandle`)
    /// -        offset:  96(x86)/144(x86_64)
    /// -     type_size:  24(x86)/ 48(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldHandle: hkbHandle,
    /// # C++ Info
    /// -          name: `oldHandlePosition`(ctype: `hkVector4`)
    /// -        offset: 128(x86)/192(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldHandlePosition: Vector4,
    /// # C++ Info
    /// -          name: `oldHandleRotation`(ctype: `hkQuaternion`)
    /// -        offset: 144(x86)/208(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldHandleRotation: Quaternion,
    /// # C++ Info
    /// -          name: `timeSinceLastModify`(ctype: `hkReal`)
    /// -        offset: 160(x86)/224(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeSinceLastModify: f32,
    /// # C++ Info
    /// -          name: `smoothlyChangingHandles`(ctype: `hkBool`)
    /// -        offset: 164(x86)/228(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_smoothlyChangingHandles: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbEvaluateHandleModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbEvaluateHandleModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x79757102)
        }
    }
    impl<'a> _serde::Serialize for hkbEvaluateHandleModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x79757102)));
            let mut serializer = __serializer
                .serialize_struct("hkbEvaluateHandleModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("handle", &self.m_handle)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("handlePositionOut", &self.m_handlePositionOut)?;
            serializer.serialize_field("handleRotationOut", &self.m_handleRotationOut)?;
            serializer.serialize_field("isValidOut", &self.m_isValidOut)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "extrapolationTimeStep",
                    &self.m_extrapolationTimeStep,
                )?;
            serializer.serialize_field("handleChangeSpeed", &self.m_handleChangeSpeed)?;
            serializer.serialize_field("handleChangeMode", &self.m_handleChangeMode)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("oldHandle", &self.m_oldHandle)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.skip_field("oldHandlePosition", &self.m_oldHandlePosition)?;
            serializer.skip_field("oldHandleRotation", &self.m_oldHandleRotation)?;
            serializer.skip_field("timeSinceLastModify", &self.m_timeSinceLastModify)?;
            serializer
                .skip_field("smoothlyChangingHandles", &self.m_smoothlyChangingHandles)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
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
pub enum HandleChangeMode {
    #[default]
    HANDLE_CHANGE_MODE_ABRUPT = 0isize,
    HANDLE_CHANGE_MODE_CONSTANT_VELOCITY = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for HandleChangeMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HANDLE_CHANGE_MODE_ABRUPT => {
                    __serializer.serialize_field("HANDLE_CHANGE_MODE_ABRUPT", &0u64)
                }
                Self::HANDLE_CHANGE_MODE_CONSTANT_VELOCITY => {
                    __serializer
                        .serialize_field("HANDLE_CHANGE_MODE_CONSTANT_VELOCITY", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum HandleChangeMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for HandleChangeMode {
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
                            v if v == "0"
                                || v.eq_ignore_ascii_case("HANDLE_CHANGE_MODE_ABRUPT") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY",
                                    ) => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<HandleChangeMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = HandleChangeMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum HandleChangeMode",
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
                                HandleChangeMode::HANDLE_CHANGE_MODE_ABRUPT,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                HandleChangeMode::HANDLE_CHANGE_MODE_CONSTANT_VELOCITY,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HANDLE_CHANGE_MODE_ABRUPT",
                "HANDLE_CHANGE_MODE_CONSTANT_VELOCITY",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "HandleChangeMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<HandleChangeMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
