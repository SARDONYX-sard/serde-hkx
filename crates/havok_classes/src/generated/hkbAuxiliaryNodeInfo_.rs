use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbAuxiliaryNodeInfo`
/// -         version: `1`
/// -       signature: `0xca0888ca`
/// -          size:  28(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbAuxiliaryNodeInfo<'a> {
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
    /// -          name: `type`(ctype: `enum NodeType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: NodeType,
    /// # C++ Info
    /// -          name: `depth`(ctype: `hkUint8`)
    /// -        offset:   9(x86)/ 17(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_depth: u8,
    /// # C++ Info
    /// -          name: `referenceBehaviorName`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_referenceBehaviorName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `selfTransitionNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_selfTransitionNames: Vec<StringPtr<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbAuxiliaryNodeInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbAuxiliaryNodeInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3389556938u32)
        }
    }
    impl<'a> _serde::Serialize for hkbAuxiliaryNodeInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3389556938u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbAuxiliaryNodeInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("depth", &self.m_depth)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "referenceBehaviorName",
                    &self.m_referenceBehaviorName,
                )?;
            serializer
                .serialize_array_meta_field(
                    "selfTransitionNames",
                    &self.m_selfTransitionNames,
                )?;
            serializer
                .serialize_stringptr_field(
                    "referenceBehaviorName",
                    &self.m_referenceBehaviorName,
                )?;
            serializer
                .serialize_array_field(
                    "selfTransitionNames",
                    &self.m_selfTransitionNames,
                )?;
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
pub enum NodeType {
    #[default]
    NODE_TYPE_UNKNOWN = 0isize,
    NODE_TYPE_NODE = 1isize,
    NODE_TYPE_TRANSITION = 2isize,
    NODE_TYPE_WILDCARD_TRANSITION = 3isize,
    NODE_TYPE_STATE = 4isize,
    NODE_TYPE_STATE_MACHINE = 5isize,
    NODE_TYPE_MODIFIER_GENERATOR = 6isize,
    NODE_TYPE_MODIFIER = 7isize,
    NODE_TYPE_CLIP = 8isize,
    NODE_TYPE_BLEND = 9isize,
    NODE_TYPE_TRANSITION_EFFECT = 10isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for NodeType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NODE_TYPE_UNKNOWN => {
                    __serializer.serialize_field("NODE_TYPE_UNKNOWN", &0u64)
                }
                Self::NODE_TYPE_NODE => {
                    __serializer.serialize_field("NODE_TYPE_NODE", &1u64)
                }
                Self::NODE_TYPE_TRANSITION => {
                    __serializer.serialize_field("NODE_TYPE_TRANSITION", &2u64)
                }
                Self::NODE_TYPE_WILDCARD_TRANSITION => {
                    __serializer.serialize_field("NODE_TYPE_WILDCARD_TRANSITION", &3u64)
                }
                Self::NODE_TYPE_STATE => {
                    __serializer.serialize_field("NODE_TYPE_STATE", &4u64)
                }
                Self::NODE_TYPE_STATE_MACHINE => {
                    __serializer.serialize_field("NODE_TYPE_STATE_MACHINE", &5u64)
                }
                Self::NODE_TYPE_MODIFIER_GENERATOR => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER_GENERATOR", &6u64)
                }
                Self::NODE_TYPE_MODIFIER => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER", &7u64)
                }
                Self::NODE_TYPE_CLIP => {
                    __serializer.serialize_field("NODE_TYPE_CLIP", &8u64)
                }
                Self::NODE_TYPE_BLEND => {
                    __serializer.serialize_field("NODE_TYPE_BLEND", &9u64)
                }
                Self::NODE_TYPE_TRANSITION_EFFECT => {
                    __serializer.serialize_field("NODE_TYPE_TRANSITION_EFFECT", &10u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum NodeType to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for NodeType {
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
                                || v.eq_ignore_ascii_case("NODE_TYPE_UNKNOWN") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("NODE_TYPE_NODE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("NODE_TYPE_TRANSITION") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case("NODE_TYPE_WILDCARD_TRANSITION") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("NODE_TYPE_STATE") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("NODE_TYPE_STATE_MACHINE") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v
                                    .eq_ignore_ascii_case("NODE_TYPE_MODIFIER_GENERATOR") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("NODE_TYPE_MODIFIER") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("NODE_TYPE_CLIP") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "9"
                                || v.eq_ignore_ascii_case("NODE_TYPE_BLEND") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "10"
                                || v.eq_ignore_ascii_case("NODE_TYPE_TRANSITION_EFFECT") => {
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
                marker: _serde::__private::PhantomData<NodeType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = NodeType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum NodeType")
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
                            _serde::__private::Ok(NodeType::NODE_TYPE_UNKNOWN)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_NODE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_TRANSITION)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                NodeType::NODE_TYPE_WILDCARD_TRANSITION,
                            )
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_STATE)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_STATE_MACHINE)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_MODIFIER_GENERATOR)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_MODIFIER)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_CLIP)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_BLEND)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(NodeType::NODE_TYPE_TRANSITION_EFFECT)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "NODE_TYPE_UNKNOWN",
                "NODE_TYPE_NODE",
                "NODE_TYPE_TRANSITION",
                "NODE_TYPE_WILDCARD_TRANSITION",
                "NODE_TYPE_STATE",
                "NODE_TYPE_STATE_MACHINE",
                "NODE_TYPE_MODIFIER_GENERATOR",
                "NODE_TYPE_MODIFIER",
                "NODE_TYPE_CLIP",
                "NODE_TYPE_BLEND",
                "NODE_TYPE_TRANSITION_EFFECT",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "NodeType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<NodeType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
