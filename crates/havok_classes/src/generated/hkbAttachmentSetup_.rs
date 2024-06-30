use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbAttachmentSetup`
/// -         version: `1`
/// -       signature: `0x774632b`
/// -          size:  40(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbAttachmentSetup {
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
    /// -          name: `blendInTime`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blendInTime: f32,
    /// # C++ Info
    /// -          name: `moveAttacherFraction`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_moveAttacherFraction: f32,
    /// # C++ Info
    /// -          name: `gain`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_gain: f32,
    /// # C++ Info
    /// -          name: `extrapolationTimeStep`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_extrapolationTimeStep: f32,
    /// # C++ Info
    /// -          name: `fixUpGain`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fixUpGain: f32,
    /// # C++ Info
    /// -          name: `maxLinearDistance`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxLinearDistance: f32,
    /// # C++ Info
    /// -          name: `maxAngularDistance`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAngularDistance: f32,
    /// # C++ Info
    /// -          name: `attachmentType`(ctype: `enum AttachmentType`)
    /// -        offset:  36(x86)/ 44(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_attachmentType: AttachmentType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbAttachmentSetup {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbAttachmentSetup"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(125068075u32)
        }
    }
    impl __serde::Serialize for hkbAttachmentSetup {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbAttachmentSetup", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("blendInTime", &self.m_blendInTime)?;
            serializer
                .serialize_field("moveAttacherFraction", &self.m_moveAttacherFraction)?;
            serializer.serialize_field("gain", &self.m_gain)?;
            serializer
                .serialize_field(
                    "extrapolationTimeStep",
                    &self.m_extrapolationTimeStep,
                )?;
            serializer.serialize_field("fixUpGain", &self.m_fixUpGain)?;
            serializer.serialize_field("maxLinearDistance", &self.m_maxLinearDistance)?;
            serializer
                .serialize_field("maxAngularDistance", &self.m_maxAngularDistance)?;
            serializer.serialize_field("attachmentType", &self.m_attachmentType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
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
pub enum AttachmentType {
    #[default]
    ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY = 0isize,
    ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT = 1isize,
    ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT = 2isize,
    ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL = 3isize,
    ATTACHMENT_TYPE_NONE = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for AttachmentType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY => {
                    __serializer
                        .serialize_field("ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY", &0u64)
                }
                Self::ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT => {
                    __serializer
                        .serialize_field("ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT", &1u64)
                }
                Self::ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT => {
                    __serializer
                        .serialize_field("ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT", &2u64)
                }
                Self::ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL => {
                    __serializer
                        .serialize_field("ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL", &3u64)
                }
                Self::ATTACHMENT_TYPE_NONE => {
                    __serializer.serialize_field("ATTACHMENT_TYPE_NONE", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum AttachmentType to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for AttachmentType {
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
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        4i8 => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3, 4",
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
                                    .eq_ignore_ascii_case(
                                        "ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY",
                                    ) => _serde::__private::Ok(__Field::__field0),
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT",
                                    ) => _serde::__private::Ok(__Field::__field1),
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case(
                                        "ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT",
                                    ) => _serde::__private::Ok(__Field::__field2),
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case(
                                        "ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL",
                                    ) => _serde::__private::Ok(__Field::__field3),
                            v if v == "4"
                                || v.eq_ignore_ascii_case("ATTACHMENT_TYPE_NONE") => {
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
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<AttachmentType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AttachmentType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum AttachmentType",
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
                                AttachmentType::ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AttachmentType::ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AttachmentType::ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AttachmentType::ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL,
                            )
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AttachmentType::ATTACHMENT_TYPE_NONE)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY",
                "ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT",
                "ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT",
                "ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL",
                "ATTACHMENT_TYPE_NONE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "AttachmentType",
                VARIANTS,
                _serde::de::ReadEnumSize::Int8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AttachmentType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
