use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbRoleAttribute`
/// -         version: `0`
/// -       signature: `0x3eb2e082`
/// -          size:   4(x86)/  4(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbRoleAttribute {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `role`(ctype: `enum Role`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_role: Role,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags RoleFlags`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_flags: RoleFlags,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbRoleAttribute {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbRoleAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1051910274u32)
        }
    }
    impl __serde::Serialize for hkbRoleAttribute {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbRoleAttribute", class_meta)?;
            serializer.serialize_field("role", &self.m_role)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT16`
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
pub enum Role {
    #[default]
    ROLE_DEFAULT = 0isize,
    ROLE_FILE_NAME = 1isize,
    ROLE_BONE_INDEX = 2isize,
    ROLE_BONE_INDEX_MAP = 3isize,
    ROLE_EVENT_ID = 4isize,
    ROLE_VARIABLE_INDEX = 5isize,
    ROLE_ATTRIBUTE_INDEX = 6isize,
    ROLE_TIME = 7isize,
}
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_INT16`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    RoleFlags : i16 { #[doc = "0"] const FLAG_NONE = 0i16; #[doc = "1"] const
    FLAG_RAGDOLL = 1i16; #[doc = "2"] const FLAG_NORMALIZED = 2i16; #[doc = "4"] const
    FLAG_NOT_VARIABLE = 4i16; #[doc = "8"] const FLAG_HIDDEN = 8i16; #[doc = "16"] const
    FLAG_OUTPUT = 16i16; #[doc = "32"] const FLAG_NOT_CHARACTER_PROPERTY = 32i16; }
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Role {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ROLE_DEFAULT => __serializer.serialize_field("ROLE_DEFAULT", &0u64),
                Self::ROLE_FILE_NAME => {
                    __serializer.serialize_field("ROLE_FILE_NAME", &1u64)
                }
                Self::ROLE_BONE_INDEX => {
                    __serializer.serialize_field("ROLE_BONE_INDEX", &2u64)
                }
                Self::ROLE_BONE_INDEX_MAP => {
                    __serializer.serialize_field("ROLE_BONE_INDEX_MAP", &3u64)
                }
                Self::ROLE_EVENT_ID => {
                    __serializer.serialize_field("ROLE_EVENT_ID", &4u64)
                }
                Self::ROLE_VARIABLE_INDEX => {
                    __serializer.serialize_field("ROLE_VARIABLE_INDEX", &5u64)
                }
                Self::ROLE_ATTRIBUTE_INDEX => {
                    __serializer.serialize_field("ROLE_ATTRIBUTE_INDEX", &6u64)
                }
                Self::ROLE_TIME => __serializer.serialize_field("ROLE_TIME", &7u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i16().ok_or(S::Error::custom("Failed enum Role to_i16"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for RoleFlags {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::FLAG_NONE => {
                        __serializer.serialize_field("FLAG_NONE", &Self::FLAG_NONE)
                    }
                    Self::FLAG_RAGDOLL => {
                        __serializer.serialize_field("FLAG_RAGDOLL", &Self::FLAG_RAGDOLL)
                    }
                    Self::FLAG_NORMALIZED => {
                        __serializer
                            .serialize_field("FLAG_NORMALIZED", &Self::FLAG_NORMALIZED)
                    }
                    Self::FLAG_NOT_VARIABLE => {
                        __serializer
                            .serialize_field(
                                "FLAG_NOT_VARIABLE",
                                &Self::FLAG_NOT_VARIABLE,
                            )
                    }
                    Self::FLAG_HIDDEN => {
                        __serializer.serialize_field("FLAG_HIDDEN", &Self::FLAG_HIDDEN)
                    }
                    Self::FLAG_OUTPUT => {
                        __serializer.serialize_field("FLAG_OUTPUT", &Self::FLAG_OUTPUT)
                    }
                    Self::FLAG_NOT_CHARACTER_PROPERTY => {
                        __serializer
                            .serialize_field(
                                "FLAG_NOT_CHARACTER_PROPERTY",
                                &Self::FLAG_NOT_CHARACTER_PROPERTY,
                            )
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
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
    impl<'de> _serde::Deserialize<'de> for Role {
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
                fn visit_int16<__E>(
                    self,
                    __value: i16,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i16 => _serde::__private::Ok(__Field::__field0),
                        1i16 => _serde::__private::Ok(__Field::__field1),
                        2i16 => _serde::__private::Ok(__Field::__field2),
                        3i16 => _serde::__private::Ok(__Field::__field3),
                        4i16 => _serde::__private::Ok(__Field::__field4),
                        5i16 => _serde::__private::Ok(__Field::__field5),
                        6i16 => _serde::__private::Ok(__Field::__field6),
                        7i16 => _serde::__private::Ok(__Field::__field7),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int16(__value),
                                    &"value(i16) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7",
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
                            v if v == "0" || v.eq_ignore_ascii_case("ROLE_DEFAULT") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("ROLE_FILE_NAME") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("ROLE_BONE_INDEX") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("ROLE_BONE_INDEX_MAP") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("ROLE_EVENT_ID") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("ROLE_VARIABLE_INDEX") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("ROLE_ATTRIBUTE_INDEX") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7" || v.eq_ignore_ascii_case("ROLE_TIME") => {
                                _serde::__private::Ok(__Field::__field7)
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
                marker: _serde::__private::PhantomData<Role>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Role;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Role")
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
                            _serde::__private::Ok(Role::ROLE_DEFAULT)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Role::ROLE_FILE_NAME)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Role::ROLE_BONE_INDEX)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Role::ROLE_BONE_INDEX_MAP)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Role::ROLE_EVENT_ID)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Role::ROLE_VARIABLE_INDEX)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Role::ROLE_ATTRIBUTE_INDEX)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Role::ROLE_TIME)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "ROLE_DEFAULT",
                "ROLE_FILE_NAME",
                "ROLE_BONE_INDEX",
                "ROLE_BONE_INDEX_MAP",
                "ROLE_EVENT_ID",
                "ROLE_VARIABLE_INDEX",
                "ROLE_ATTRIBUTE_INDEX",
                "ROLE_TIME",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Role",
                VARIANTS,
                _serde::de::ReadEnumSize::Int16,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Role>,
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
    impl<'de> _serde::Deserialize<'de> for RoleFlags {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<RoleFlags>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = RoleFlags;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct RoleFlags(flags)",
                    )
                }
                #[inline]
                fn visit_int16<__E>(
                    self,
                    __value: i16,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    Ok(RoleFlags::from_bits_retain(__value as _))
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match <RoleFlags as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Int16,
                __Visitor {
                    marker: _serde::__private::PhantomData::<RoleFlags>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
