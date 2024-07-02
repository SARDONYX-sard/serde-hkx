use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpFirstPersonGun`
/// -         version: `0`
/// -       signature: `0x852ab70b`
/// -          size:  32(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpFirstPersonGun<'a> {
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
    /// -          name: `type`(ctype: `enum unknown`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_type: u8,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `keyboardKey`(ctype: `enum KeyboardKey`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_keyboardKey: KeyboardKey,
    /// # C++ Info
    /// -          name: `listeners`(ctype: `hkArray<void*>`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_listeners: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpFirstPersonGun<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpFirstPersonGun"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2234169099u32)
        }
    }
    impl<'a> _serde::Serialize for hkpFirstPersonGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2234169099u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpFirstPersonGun", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("keyboardKey", &self.m_keyboardKey)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_array_meta_field("listeners", &self.m_listeners)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("listeners", &self.m_listeners)?;
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
pub enum Type {
    #[default]
    WEAPON_TYPE_INVALID = 0isize,
    WEAPON_TYPE_BALLGUN = 1isize,
    WEAPON_TYPE_GRENADEGUN = 2isize,
    WEAPON_TYPE_GRAVITYGUN = 3isize,
    WEAPON_TYPE_MOUNTEDBALLGUN = 4isize,
    WEAPON_TYPE_TWEAKERGUN = 5isize,
    WEAPON_TYPE_MISSILEGUN = 6isize,
    WEAPON_TYPE_RAYCASTGUN = 7isize,
    WEAPON_TYPE_SPHEREGUN = 8isize,
    WEAPON_TYPE_STICKYGUN = 9isize,
    WEAPON_TYPE_NUM_TYPES = 10isize,
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
pub enum KeyboardKey {
    #[default]
    KEY_F1 = 112isize,
    KEY_F2 = 113isize,
    KEY_F3 = 114isize,
    KEY_F4 = 115isize,
    KEY_F5 = 116isize,
    KEY_F6 = 117isize,
    KEY_F7 = 118isize,
    KEY_F8 = 119isize,
    KEY_F9 = 120isize,
    KEY_F10 = 121isize,
    KEY_F11 = 122isize,
    KEY_F12 = 123isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Type {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::WEAPON_TYPE_INVALID => {
                    __serializer.serialize_field("WEAPON_TYPE_INVALID", &0u64)
                }
                Self::WEAPON_TYPE_BALLGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_BALLGUN", &1u64)
                }
                Self::WEAPON_TYPE_GRENADEGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_GRENADEGUN", &2u64)
                }
                Self::WEAPON_TYPE_GRAVITYGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_GRAVITYGUN", &3u64)
                }
                Self::WEAPON_TYPE_MOUNTEDBALLGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_MOUNTEDBALLGUN", &4u64)
                }
                Self::WEAPON_TYPE_TWEAKERGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_TWEAKERGUN", &5u64)
                }
                Self::WEAPON_TYPE_MISSILEGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_MISSILEGUN", &6u64)
                }
                Self::WEAPON_TYPE_RAYCASTGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_RAYCASTGUN", &7u64)
                }
                Self::WEAPON_TYPE_SPHEREGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_SPHEREGUN", &8u64)
                }
                Self::WEAPON_TYPE_STICKYGUN => {
                    __serializer.serialize_field("WEAPON_TYPE_STICKYGUN", &9u64)
                }
                Self::WEAPON_TYPE_NUM_TYPES => {
                    __serializer.serialize_field("WEAPON_TYPE_NUM_TYPES", &10u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_u8().ok_or(S::Error::custom("Failed enum Type to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for KeyboardKey {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::KEY_F1 => __serializer.serialize_field("KEY_F1", &112u64),
                Self::KEY_F2 => __serializer.serialize_field("KEY_F2", &113u64),
                Self::KEY_F3 => __serializer.serialize_field("KEY_F3", &114u64),
                Self::KEY_F4 => __serializer.serialize_field("KEY_F4", &115u64),
                Self::KEY_F5 => __serializer.serialize_field("KEY_F5", &116u64),
                Self::KEY_F6 => __serializer.serialize_field("KEY_F6", &117u64),
                Self::KEY_F7 => __serializer.serialize_field("KEY_F7", &118u64),
                Self::KEY_F8 => __serializer.serialize_field("KEY_F8", &119u64),
                Self::KEY_F9 => __serializer.serialize_field("KEY_F9", &120u64),
                Self::KEY_F10 => __serializer.serialize_field("KEY_F10", &121u64),
                Self::KEY_F11 => __serializer.serialize_field("KEY_F11", &122u64),
                Self::KEY_F12 => __serializer.serialize_field("KEY_F12", &123u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum KeyboardKey to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for Type {
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
                        9u8 => _serde::__private::Ok(__Field::__field9),
                        10u8 => _serde::__private::Ok(__Field::__field10),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10",
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
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_BALLGUN") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_GRENADEGUN") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_GRAVITYGUN") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_MOUNTEDBALLGUN") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_TWEAKERGUN") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_MISSILEGUN") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_RAYCASTGUN") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_SPHEREGUN") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "9"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_STICKYGUN") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "10"
                                || v.eq_ignore_ascii_case("WEAPON_TYPE_NUM_TYPES") => {
                                _serde::__private::Ok(__Field::__field10)
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
                marker: _serde::__private::PhantomData<Type>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Type;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Type")
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
                            _serde::__private::Ok(Type::WEAPON_TYPE_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_BALLGUN)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_GRENADEGUN)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_GRAVITYGUN)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_MOUNTEDBALLGUN)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_TWEAKERGUN)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_MISSILEGUN)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_RAYCASTGUN)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_SPHEREGUN)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_STICKYGUN)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::WEAPON_TYPE_NUM_TYPES)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "WEAPON_TYPE_INVALID",
                "WEAPON_TYPE_BALLGUN",
                "WEAPON_TYPE_GRENADEGUN",
                "WEAPON_TYPE_GRAVITYGUN",
                "WEAPON_TYPE_MOUNTEDBALLGUN",
                "WEAPON_TYPE_TWEAKERGUN",
                "WEAPON_TYPE_MISSILEGUN",
                "WEAPON_TYPE_RAYCASTGUN",
                "WEAPON_TYPE_SPHEREGUN",
                "WEAPON_TYPE_STICKYGUN",
                "WEAPON_TYPE_NUM_TYPES",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Type",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Type>,
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
    impl<'de> _serde::Deserialize<'de> for KeyboardKey {
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
                        112u8 => _serde::__private::Ok(__Field::__field0),
                        113u8 => _serde::__private::Ok(__Field::__field1),
                        114u8 => _serde::__private::Ok(__Field::__field2),
                        115u8 => _serde::__private::Ok(__Field::__field3),
                        116u8 => _serde::__private::Ok(__Field::__field4),
                        117u8 => _serde::__private::Ok(__Field::__field5),
                        118u8 => _serde::__private::Ok(__Field::__field6),
                        119u8 => _serde::__private::Ok(__Field::__field7),
                        120u8 => _serde::__private::Ok(__Field::__field8),
                        121u8 => _serde::__private::Ok(__Field::__field9),
                        122u8 => _serde::__private::Ok(__Field::__field10),
                        123u8 => _serde::__private::Ok(__Field::__field11),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123",
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
                            v if v == "112" || v.eq_ignore_ascii_case("KEY_F1") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "113" || v.eq_ignore_ascii_case("KEY_F2") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "114" || v.eq_ignore_ascii_case("KEY_F3") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "115" || v.eq_ignore_ascii_case("KEY_F4") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "116" || v.eq_ignore_ascii_case("KEY_F5") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "117" || v.eq_ignore_ascii_case("KEY_F6") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "118" || v.eq_ignore_ascii_case("KEY_F7") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "119" || v.eq_ignore_ascii_case("KEY_F8") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "120" || v.eq_ignore_ascii_case("KEY_F9") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "121" || v.eq_ignore_ascii_case("KEY_F10") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "122" || v.eq_ignore_ascii_case("KEY_F11") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "123" || v.eq_ignore_ascii_case("KEY_F12") => {
                                _serde::__private::Ok(__Field::__field11)
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
                marker: _serde::__private::PhantomData<KeyboardKey>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = KeyboardKey;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum KeyboardKey",
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
                            _serde::__private::Ok(KeyboardKey::KEY_F1)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F2)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F3)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F4)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F5)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F6)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F7)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F8)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F9)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F10)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F11)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(KeyboardKey::KEY_F12)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "KEY_F1",
                "KEY_F2",
                "KEY_F3",
                "KEY_F4",
                "KEY_F5",
                "KEY_F6",
                "KEY_F7",
                "KEY_F8",
                "KEY_F9",
                "KEY_F10",
                "KEY_F11",
                "KEY_F12",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "KeyboardKey",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<KeyboardKey>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
