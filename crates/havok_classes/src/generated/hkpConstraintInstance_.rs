use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConstraintInstance`
/// -         version: `1`
/// -       signature: `0x34eba5f`
/// -          size:  56(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintInstance<'a> {
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
    /// -          name: `owner`(ctype: `void*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_owner: Pointer,
    /// # C++ Info
    /// -          name: `data`(ctype: `struct hkpConstraintData*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_data: Pointer,
    /// # C++ Info
    /// -          name: `constraintModifiers`(ctype: `struct hkpModifierConstraintAtom*`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_constraintModifiers: Pointer,
    /// # C++ Info
    /// -          name: `entities`(ctype: `struct hkpEntity*`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_entities: [Pointer; 2usize],
    /// # C++ Info
    /// -          name: `priority`(ctype: `enum ConstraintPriority`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_priority: ConstraintPriority,
    /// # C++ Info
    /// -          name: `wantRuntime`(ctype: `hkBool`)
    /// -        offset:  29(x86)/ 57(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_wantRuntime: bool,
    /// # C++ Info
    /// -          name: `destructionRemapInfo`(ctype: `enum OnDestructionRemapInfo`)
    /// -        offset:  30(x86)/ 58(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_destructionRemapInfo: OnDestructionRemapInfo,
    /// # C++ Info
    /// -          name: `listeners`(ctype: `struct hkpConstraintInstanceSmallArraySerializeOverrideType`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_listeners: hkpConstraintInstanceSmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  40(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:  44(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
    /// # C++ Info
    /// -          name: `internal`(ctype: `void*`)
    /// -        offset:  48(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_internal: Pointer,
    /// # C++ Info
    /// -          name: `uid`(ctype: `hkUint32`)
    /// -        offset:  52(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_uid: u32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpConstraintInstance<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpConstraintInstance"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(55491167u32)
        }
    }
    impl<'a> __serde::Serialize for hkpConstraintInstance<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintInstance", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("owner", &self.m_owner)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer
                .serialize_field("constraintModifiers", &self.m_constraintModifiers)?;
            serializer.serialize_field("entities", &self.m_entities.as_slice())?;
            serializer.serialize_field("priority", &self.m_priority)?;
            serializer.serialize_field("wantRuntime", &self.m_wantRuntime)?;
            serializer
                .serialize_field("destructionRemapInfo", &self.m_destructionRemapInfo)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.skip_field("listeners", &self.m_listeners)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.skip_field("internal", &self.m_internal)?;
            serializer.skip_field("uid", &self.m_uid)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
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
pub enum ConstraintPriority {
    #[default]
    PRIORITY_INVALID = 0isize,
    PRIORITY_PSI = 1isize,
    PRIORITY_SIMPLIFIED_TOI_UNUSED = 2isize,
    PRIORITY_TOI = 3isize,
    PRIORITY_TOI_HIGHER = 4isize,
    PRIORITY_TOI_FORCED = 5isize,
    NUM_PRIORITIES = 6isize,
}
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
pub enum OnDestructionRemapInfo {
    #[default]
    ON_DESTRUCTION_REMAP = 0isize,
    ON_DESTRUCTION_REMOVE = 1isize,
    ON_DESTRUCTION_RESET_REMOVE = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ConstraintPriority {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::PRIORITY_INVALID => {
                    __serializer.serialize_field("PRIORITY_INVALID", &0u64)
                }
                Self::PRIORITY_PSI => __serializer.serialize_field("PRIORITY_PSI", &1u64),
                Self::PRIORITY_SIMPLIFIED_TOI_UNUSED => {
                    __serializer.serialize_field("PRIORITY_SIMPLIFIED_TOI_UNUSED", &2u64)
                }
                Self::PRIORITY_TOI => __serializer.serialize_field("PRIORITY_TOI", &3u64),
                Self::PRIORITY_TOI_HIGHER => {
                    __serializer.serialize_field("PRIORITY_TOI_HIGHER", &4u64)
                }
                Self::PRIORITY_TOI_FORCED => {
                    __serializer.serialize_field("PRIORITY_TOI_FORCED", &5u64)
                }
                Self::NUM_PRIORITIES => {
                    __serializer.serialize_field("NUM_PRIORITIES", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ConstraintPriority to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for OnDestructionRemapInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ON_DESTRUCTION_REMAP => {
                    __serializer.serialize_field("ON_DESTRUCTION_REMAP", &0u64)
                }
                Self::ON_DESTRUCTION_REMOVE => {
                    __serializer.serialize_field("ON_DESTRUCTION_REMOVE", &1u64)
                }
                Self::ON_DESTRUCTION_RESET_REMOVE => {
                    __serializer.serialize_field("ON_DESTRUCTION_RESET_REMOVE", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum OnDestructionRemapInfo to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for ConstraintPriority {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 5, 6",
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
                                || v.eq_ignore_ascii_case("PRIORITY_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("PRIORITY_PSI") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case("PRIORITY_SIMPLIFIED_TOI_UNUSED") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("PRIORITY_TOI") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("PRIORITY_TOI_HIGHER") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("PRIORITY_TOI_FORCED") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6" || v.eq_ignore_ascii_case("NUM_PRIORITIES") => {
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
                marker: _serde::__private::PhantomData<ConstraintPriority>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ConstraintPriority;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ConstraintPriority",
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
                            _serde::__private::Ok(ConstraintPriority::PRIORITY_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintPriority::PRIORITY_PSI)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintPriority::PRIORITY_SIMPLIFIED_TOI_UNUSED,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintPriority::PRIORITY_TOI)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintPriority::PRIORITY_TOI_HIGHER,
                            )
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintPriority::PRIORITY_TOI_FORCED,
                            )
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintPriority::NUM_PRIORITIES)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "PRIORITY_INVALID",
                "PRIORITY_PSI",
                "PRIORITY_SIMPLIFIED_TOI_UNUSED",
                "PRIORITY_TOI",
                "PRIORITY_TOI_HIGHER",
                "PRIORITY_TOI_FORCED",
                "NUM_PRIORITIES",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ConstraintPriority",
                VARIANTS,
                _serde::de::ReadEnumSize::Uint8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ConstraintPriority>,
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
    impl<'de> _serde::Deserialize<'de> for OnDestructionRemapInfo {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2",
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
                                || v.eq_ignore_ascii_case("ON_DESTRUCTION_REMAP") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("ON_DESTRUCTION_REMOVE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("ON_DESTRUCTION_RESET_REMOVE") => {
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
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<OnDestructionRemapInfo>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = OnDestructionRemapInfo;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum OnDestructionRemapInfo",
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
                                OnDestructionRemapInfo::ON_DESTRUCTION_REMAP,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                OnDestructionRemapInfo::ON_DESTRUCTION_REMOVE,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                OnDestructionRemapInfo::ON_DESTRUCTION_RESET_REMOVE,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "ON_DESTRUCTION_REMAP",
                "ON_DESTRUCTION_REMOVE",
                "ON_DESTRUCTION_RESET_REMOVE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "OnDestructionRemapInfo",
                VARIANTS,
                _serde::de::ReadEnumSize::Uint8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<OnDestructionRemapInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
