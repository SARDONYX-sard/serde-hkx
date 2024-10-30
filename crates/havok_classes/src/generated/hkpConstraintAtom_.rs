use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpConstraintAtom`
/// - version: `0`
/// - signature: `0x59d67ef6`
/// - size: `  2`(x86)/`  2`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `type`(ctype: `enum AtomType`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "type"))]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub m_type: AtomType,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x59d67ef6)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkpConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x59d67ef6)));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintAtom", class_meta, (2u64, 2u64))?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_type,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "type" => Ok(__Field::m_type),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkpConstraintAtomVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpConstraintAtom>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpConstraintAtomVisitor<'de> {
                type Value = hkpConstraintAtom;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpConstraintAtom",
                    )
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_type: _serde::__private::Option<AtomType> = _serde::__private::None;
                    for i in 0..1usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_type) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<AtomType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpConstraintAtom { __ptr, m_type })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_type: _serde::__private::Option<AtomType> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<AtomType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpConstraintAtom { __ptr, m_type })
                }
            }
            const FIELDS: &[&str] = &["type"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConstraintAtom",
                FIELDS,
                __hkpConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<hkpConstraintAtom>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_UINT16`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
pub enum AtomType {
    #[default]
    TYPE_INVALID = 0isize,
    TYPE_BRIDGE = 1isize,
    TYPE_SET_LOCAL_TRANSFORMS = 2isize,
    TYPE_SET_LOCAL_TRANSLATIONS = 3isize,
    TYPE_SET_LOCAL_ROTATIONS = 4isize,
    TYPE_BALL_SOCKET = 5isize,
    TYPE_STIFF_SPRING = 6isize,
    TYPE_LIN = 7isize,
    TYPE_LIN_SOFT = 8isize,
    TYPE_LIN_LIMIT = 9isize,
    TYPE_LIN_FRICTION = 10isize,
    TYPE_LIN_MOTOR = 11isize,
    TYPE_2D_ANG = 12isize,
    TYPE_ANG = 13isize,
    TYPE_ANG_LIMIT = 14isize,
    TYPE_TWIST_LIMIT = 15isize,
    TYPE_CONE_LIMIT = 16isize,
    TYPE_ANG_FRICTION = 17isize,
    TYPE_ANG_MOTOR = 18isize,
    TYPE_RAGDOLL_MOTOR = 19isize,
    TYPE_PULLEY = 20isize,
    TYPE_RACK_AND_PINION = 21isize,
    TYPE_COG_WHEEL = 22isize,
    TYPE_SETUP_STABILIZATION = 23isize,
    TYPE_OVERWRITE_PIVOT = 24isize,
    TYPE_CONTACT = 25isize,
    TYPE_MODIFIER_SOFT_CONTACT = 26isize,
    TYPE_MODIFIER_MASS_CHANGER = 27isize,
    TYPE_MODIFIER_VISCOUS_SURFACE = 28isize,
    TYPE_MODIFIER_MOVING_SURFACE = 29isize,
    TYPE_MODIFIER_IGNORE_CONSTRAINT = 30isize,
    TYPE_MODIFIER_CENTER_OF_MASS_CHANGER = 31isize,
    TYPE_MAX = 32isize,
}
///- size(C++): `TYPE_UINT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
pub enum SolvingMethod {
    #[default]
    METHOD_STABILIZED = 0isize,
    METHOD_OLD = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for AtomType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TYPE_INVALID => __serializer.serialize_field("TYPE_INVALID", &0u64),
                Self::TYPE_BRIDGE => __serializer.serialize_field("TYPE_BRIDGE", &1u64),
                Self::TYPE_SET_LOCAL_TRANSFORMS => {
                    __serializer.serialize_field("TYPE_SET_LOCAL_TRANSFORMS", &2u64)
                }
                Self::TYPE_SET_LOCAL_TRANSLATIONS => {
                    __serializer.serialize_field("TYPE_SET_LOCAL_TRANSLATIONS", &3u64)
                }
                Self::TYPE_SET_LOCAL_ROTATIONS => {
                    __serializer.serialize_field("TYPE_SET_LOCAL_ROTATIONS", &4u64)
                }
                Self::TYPE_BALL_SOCKET => {
                    __serializer.serialize_field("TYPE_BALL_SOCKET", &5u64)
                }
                Self::TYPE_STIFF_SPRING => {
                    __serializer.serialize_field("TYPE_STIFF_SPRING", &6u64)
                }
                Self::TYPE_LIN => __serializer.serialize_field("TYPE_LIN", &7u64),
                Self::TYPE_LIN_SOFT => {
                    __serializer.serialize_field("TYPE_LIN_SOFT", &8u64)
                }
                Self::TYPE_LIN_LIMIT => {
                    __serializer.serialize_field("TYPE_LIN_LIMIT", &9u64)
                }
                Self::TYPE_LIN_FRICTION => {
                    __serializer.serialize_field("TYPE_LIN_FRICTION", &10u64)
                }
                Self::TYPE_LIN_MOTOR => {
                    __serializer.serialize_field("TYPE_LIN_MOTOR", &11u64)
                }
                Self::TYPE_2D_ANG => __serializer.serialize_field("TYPE_2D_ANG", &12u64),
                Self::TYPE_ANG => __serializer.serialize_field("TYPE_ANG", &13u64),
                Self::TYPE_ANG_LIMIT => {
                    __serializer.serialize_field("TYPE_ANG_LIMIT", &14u64)
                }
                Self::TYPE_TWIST_LIMIT => {
                    __serializer.serialize_field("TYPE_TWIST_LIMIT", &15u64)
                }
                Self::TYPE_CONE_LIMIT => {
                    __serializer.serialize_field("TYPE_CONE_LIMIT", &16u64)
                }
                Self::TYPE_ANG_FRICTION => {
                    __serializer.serialize_field("TYPE_ANG_FRICTION", &17u64)
                }
                Self::TYPE_ANG_MOTOR => {
                    __serializer.serialize_field("TYPE_ANG_MOTOR", &18u64)
                }
                Self::TYPE_RAGDOLL_MOTOR => {
                    __serializer.serialize_field("TYPE_RAGDOLL_MOTOR", &19u64)
                }
                Self::TYPE_PULLEY => __serializer.serialize_field("TYPE_PULLEY", &20u64),
                Self::TYPE_RACK_AND_PINION => {
                    __serializer.serialize_field("TYPE_RACK_AND_PINION", &21u64)
                }
                Self::TYPE_COG_WHEEL => {
                    __serializer.serialize_field("TYPE_COG_WHEEL", &22u64)
                }
                Self::TYPE_SETUP_STABILIZATION => {
                    __serializer.serialize_field("TYPE_SETUP_STABILIZATION", &23u64)
                }
                Self::TYPE_OVERWRITE_PIVOT => {
                    __serializer.serialize_field("TYPE_OVERWRITE_PIVOT", &24u64)
                }
                Self::TYPE_CONTACT => {
                    __serializer.serialize_field("TYPE_CONTACT", &25u64)
                }
                Self::TYPE_MODIFIER_SOFT_CONTACT => {
                    __serializer.serialize_field("TYPE_MODIFIER_SOFT_CONTACT", &26u64)
                }
                Self::TYPE_MODIFIER_MASS_CHANGER => {
                    __serializer.serialize_field("TYPE_MODIFIER_MASS_CHANGER", &27u64)
                }
                Self::TYPE_MODIFIER_VISCOUS_SURFACE => {
                    __serializer.serialize_field("TYPE_MODIFIER_VISCOUS_SURFACE", &28u64)
                }
                Self::TYPE_MODIFIER_MOVING_SURFACE => {
                    __serializer.serialize_field("TYPE_MODIFIER_MOVING_SURFACE", &29u64)
                }
                Self::TYPE_MODIFIER_IGNORE_CONSTRAINT => {
                    __serializer
                        .serialize_field("TYPE_MODIFIER_IGNORE_CONSTRAINT", &30u64)
                }
                Self::TYPE_MODIFIER_CENTER_OF_MASS_CHANGER => {
                    __serializer
                        .serialize_field("TYPE_MODIFIER_CENTER_OF_MASS_CHANGER", &31u64)
                }
                Self::TYPE_MAX => __serializer.serialize_field("TYPE_MAX", &32u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u16()
                .ok_or(S::Error::custom("Failed enum AtomType to_u16"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SolvingMethod {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::METHOD_STABILIZED => {
                    __serializer.serialize_field("METHOD_STABILIZED", &0u64)
                }
                Self::METHOD_OLD => __serializer.serialize_field("METHOD_OLD", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum SolvingMethod to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for AtomType {
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
                __field13,
                __field14,
                __field15,
                __field16,
                __field17,
                __field18,
                __field19,
                __field20,
                __field21,
                __field22,
                __field23,
                __field24,
                __field25,
                __field26,
                __field27,
                __field28,
                __field29,
                __field30,
                __field31,
                __field32,
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
                fn visit_uint16<__E>(
                    self,
                    __value: u16,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u16 => _serde::__private::Ok(__Field::__field0),
                        1u16 => _serde::__private::Ok(__Field::__field1),
                        2u16 => _serde::__private::Ok(__Field::__field2),
                        3u16 => _serde::__private::Ok(__Field::__field3),
                        4u16 => _serde::__private::Ok(__Field::__field4),
                        5u16 => _serde::__private::Ok(__Field::__field5),
                        6u16 => _serde::__private::Ok(__Field::__field6),
                        7u16 => _serde::__private::Ok(__Field::__field7),
                        8u16 => _serde::__private::Ok(__Field::__field8),
                        9u16 => _serde::__private::Ok(__Field::__field9),
                        10u16 => _serde::__private::Ok(__Field::__field10),
                        11u16 => _serde::__private::Ok(__Field::__field11),
                        12u16 => _serde::__private::Ok(__Field::__field12),
                        13u16 => _serde::__private::Ok(__Field::__field13),
                        14u16 => _serde::__private::Ok(__Field::__field14),
                        15u16 => _serde::__private::Ok(__Field::__field15),
                        16u16 => _serde::__private::Ok(__Field::__field16),
                        17u16 => _serde::__private::Ok(__Field::__field17),
                        18u16 => _serde::__private::Ok(__Field::__field18),
                        19u16 => _serde::__private::Ok(__Field::__field19),
                        20u16 => _serde::__private::Ok(__Field::__field20),
                        21u16 => _serde::__private::Ok(__Field::__field21),
                        22u16 => _serde::__private::Ok(__Field::__field22),
                        23u16 => _serde::__private::Ok(__Field::__field23),
                        24u16 => _serde::__private::Ok(__Field::__field24),
                        25u16 => _serde::__private::Ok(__Field::__field25),
                        26u16 => _serde::__private::Ok(__Field::__field26),
                        27u16 => _serde::__private::Ok(__Field::__field27),
                        28u16 => _serde::__private::Ok(__Field::__field28),
                        29u16 => _serde::__private::Ok(__Field::__field29),
                        30u16 => _serde::__private::Ok(__Field::__field30),
                        31u16 => _serde::__private::Ok(__Field::__field31),
                        32u16 => _serde::__private::Ok(__Field::__field32),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint16(__value),
                                    &"value(u16) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32",
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
                            v if v == "0" || v.eq_ignore_ascii_case("TYPE_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("TYPE_BRIDGE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("TYPE_SET_LOCAL_TRANSFORMS") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("TYPE_SET_LOCAL_TRANSLATIONS") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("TYPE_SET_LOCAL_ROTATIONS") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("TYPE_BALL_SOCKET") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("TYPE_STIFF_SPRING") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7" || v.eq_ignore_ascii_case("TYPE_LIN") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("TYPE_LIN_SOFT") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "9" || v.eq_ignore_ascii_case("TYPE_LIN_LIMIT") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "10"
                                || v.eq_ignore_ascii_case("TYPE_LIN_FRICTION") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "11"
                                || v.eq_ignore_ascii_case("TYPE_LIN_MOTOR") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "12" || v.eq_ignore_ascii_case("TYPE_2D_ANG") => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            v if v == "13" || v.eq_ignore_ascii_case("TYPE_ANG") => {
                                _serde::__private::Ok(__Field::__field13)
                            }
                            v if v == "14"
                                || v.eq_ignore_ascii_case("TYPE_ANG_LIMIT") => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            v if v == "15"
                                || v.eq_ignore_ascii_case("TYPE_TWIST_LIMIT") => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            v if v == "16"
                                || v.eq_ignore_ascii_case("TYPE_CONE_LIMIT") => {
                                _serde::__private::Ok(__Field::__field16)
                            }
                            v if v == "17"
                                || v.eq_ignore_ascii_case("TYPE_ANG_FRICTION") => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            v if v == "18"
                                || v.eq_ignore_ascii_case("TYPE_ANG_MOTOR") => {
                                _serde::__private::Ok(__Field::__field18)
                            }
                            v if v == "19"
                                || v.eq_ignore_ascii_case("TYPE_RAGDOLL_MOTOR") => {
                                _serde::__private::Ok(__Field::__field19)
                            }
                            v if v == "20" || v.eq_ignore_ascii_case("TYPE_PULLEY") => {
                                _serde::__private::Ok(__Field::__field20)
                            }
                            v if v == "21"
                                || v.eq_ignore_ascii_case("TYPE_RACK_AND_PINION") => {
                                _serde::__private::Ok(__Field::__field21)
                            }
                            v if v == "22"
                                || v.eq_ignore_ascii_case("TYPE_COG_WHEEL") => {
                                _serde::__private::Ok(__Field::__field22)
                            }
                            v if v == "23"
                                || v.eq_ignore_ascii_case("TYPE_SETUP_STABILIZATION") => {
                                _serde::__private::Ok(__Field::__field23)
                            }
                            v if v == "24"
                                || v.eq_ignore_ascii_case("TYPE_OVERWRITE_PIVOT") => {
                                _serde::__private::Ok(__Field::__field24)
                            }
                            v if v == "25" || v.eq_ignore_ascii_case("TYPE_CONTACT") => {
                                _serde::__private::Ok(__Field::__field25)
                            }
                            v if v == "26"
                                || v.eq_ignore_ascii_case("TYPE_MODIFIER_SOFT_CONTACT") => {
                                _serde::__private::Ok(__Field::__field26)
                            }
                            v if v == "27"
                                || v.eq_ignore_ascii_case("TYPE_MODIFIER_MASS_CHANGER") => {
                                _serde::__private::Ok(__Field::__field27)
                            }
                            v if v == "28"
                                || v
                                    .eq_ignore_ascii_case("TYPE_MODIFIER_VISCOUS_SURFACE") => {
                                _serde::__private::Ok(__Field::__field28)
                            }
                            v if v == "29"
                                || v
                                    .eq_ignore_ascii_case("TYPE_MODIFIER_MOVING_SURFACE") => {
                                _serde::__private::Ok(__Field::__field29)
                            }
                            v if v == "30"
                                || v
                                    .eq_ignore_ascii_case("TYPE_MODIFIER_IGNORE_CONSTRAINT") => {
                                _serde::__private::Ok(__Field::__field30)
                            }
                            v if v == "31"
                                || v
                                    .eq_ignore_ascii_case(
                                        "TYPE_MODIFIER_CENTER_OF_MASS_CHANGER",
                                    ) => _serde::__private::Ok(__Field::__field31),
                            v if v == "32" || v.eq_ignore_ascii_case("TYPE_MAX") => {
                                _serde::__private::Ok(__Field::__field32)
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
                        _serde::de::ReadEnumSize::Uint16,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<AtomType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AtomType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum AtomType")
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
                            _serde::__private::Ok(AtomType::TYPE_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_BRIDGE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_SET_LOCAL_TRANSFORMS)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_SET_LOCAL_TRANSLATIONS)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_SET_LOCAL_ROTATIONS)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_BALL_SOCKET)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_STIFF_SPRING)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_LIN)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_LIN_SOFT)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_LIN_LIMIT)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_LIN_FRICTION)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_LIN_MOTOR)
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_2D_ANG)
                        }
                        (__Field::__field13, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_ANG)
                        }
                        (__Field::__field14, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_ANG_LIMIT)
                        }
                        (__Field::__field15, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_TWIST_LIMIT)
                        }
                        (__Field::__field16, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_CONE_LIMIT)
                        }
                        (__Field::__field17, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_ANG_FRICTION)
                        }
                        (__Field::__field18, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_ANG_MOTOR)
                        }
                        (__Field::__field19, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_RAGDOLL_MOTOR)
                        }
                        (__Field::__field20, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_PULLEY)
                        }
                        (__Field::__field21, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_RACK_AND_PINION)
                        }
                        (__Field::__field22, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_COG_WHEEL)
                        }
                        (__Field::__field23, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_SETUP_STABILIZATION)
                        }
                        (__Field::__field24, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_OVERWRITE_PIVOT)
                        }
                        (__Field::__field25, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_CONTACT)
                        }
                        (__Field::__field26, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_MODIFIER_SOFT_CONTACT)
                        }
                        (__Field::__field27, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_MODIFIER_MASS_CHANGER)
                        }
                        (__Field::__field28, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AtomType::TYPE_MODIFIER_VISCOUS_SURFACE,
                            )
                        }
                        (__Field::__field29, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_MODIFIER_MOVING_SURFACE)
                        }
                        (__Field::__field30, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AtomType::TYPE_MODIFIER_IGNORE_CONSTRAINT,
                            )
                        }
                        (__Field::__field31, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                AtomType::TYPE_MODIFIER_CENTER_OF_MASS_CHANGER,
                            )
                        }
                        (__Field::__field32, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AtomType::TYPE_MAX)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "TYPE_INVALID",
                "TYPE_BRIDGE",
                "TYPE_SET_LOCAL_TRANSFORMS",
                "TYPE_SET_LOCAL_TRANSLATIONS",
                "TYPE_SET_LOCAL_ROTATIONS",
                "TYPE_BALL_SOCKET",
                "TYPE_STIFF_SPRING",
                "TYPE_LIN",
                "TYPE_LIN_SOFT",
                "TYPE_LIN_LIMIT",
                "TYPE_LIN_FRICTION",
                "TYPE_LIN_MOTOR",
                "TYPE_2D_ANG",
                "TYPE_ANG",
                "TYPE_ANG_LIMIT",
                "TYPE_TWIST_LIMIT",
                "TYPE_CONE_LIMIT",
                "TYPE_ANG_FRICTION",
                "TYPE_ANG_MOTOR",
                "TYPE_RAGDOLL_MOTOR",
                "TYPE_PULLEY",
                "TYPE_RACK_AND_PINION",
                "TYPE_COG_WHEEL",
                "TYPE_SETUP_STABILIZATION",
                "TYPE_OVERWRITE_PIVOT",
                "TYPE_CONTACT",
                "TYPE_MODIFIER_SOFT_CONTACT",
                "TYPE_MODIFIER_MASS_CHANGER",
                "TYPE_MODIFIER_VISCOUS_SURFACE",
                "TYPE_MODIFIER_MOVING_SURFACE",
                "TYPE_MODIFIER_IGNORE_CONSTRAINT",
                "TYPE_MODIFIER_CENTER_OF_MASS_CHANGER",
                "TYPE_MAX",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "AtomType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AtomType>,
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
    impl<'de> _serde::Deserialize<'de> for SolvingMethod {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1",
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
                                || v.eq_ignore_ascii_case("METHOD_STABILIZED") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("METHOD_OLD") => {
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
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SolvingMethod>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SolvingMethod;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum SolvingMethod",
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
                            _serde::__private::Ok(SolvingMethod::METHOD_STABILIZED)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(SolvingMethod::METHOD_OLD)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "METHOD_STABILIZED",
                "METHOD_OLD",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "SolvingMethod",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SolvingMethod>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
