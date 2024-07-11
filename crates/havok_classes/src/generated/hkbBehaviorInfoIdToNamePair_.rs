use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorInfoIdToNamePair`
/// -         version: `0`
/// -       signature: `0x35a0439a`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorInfoIdToNamePair<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `behaviorName`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behaviorName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `nodeName`(ctype: `hkStringPtr`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_nodeName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `toolType`(ctype: `enum ToolNodeType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_toolType: ToolNodeType,
    /// # C++ Info
    /// -          name: `id`(ctype: `hkInt16`)
    /// -        offset:  10(x86)/ 18(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_id: i16,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbBehaviorInfoIdToNamePair<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBehaviorInfoIdToNamePair"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x35a0439a)
        }
    }
    impl<'a> _serde::Serialize for hkbBehaviorInfoIdToNamePair<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x35a0439a)));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorInfoIdToNamePair", class_meta)?;
            serializer
                .serialize_stringptr_meta_field("behaviorName", &self.m_behaviorName)?;
            serializer.serialize_stringptr_meta_field("nodeName", &self.m_nodeName)?;
            serializer.serialize_field("toolType", &self.m_toolType)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("id", &self.m_id)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_field("behaviorName", &self.m_behaviorName)?;
            serializer.serialize_stringptr_field("nodeName", &self.m_nodeName)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_behaviorName,
    m_nodeName,
    m_toolType,
    m_id,
    __ignore,
}
struct __FieldVisitor;
impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
    type Value = __Field;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "field identifier")
    }
    /// Intended for use in XML.
    #[allow(clippy::match_single_binding)]
    #[allow(clippy::reversed_empty_ranges)]
    #[allow(clippy::single_match)]
    fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
    where
        __E: _serde::de::Error,
    {
        match __value {
            "behaviorName" => Ok(__Field::m_behaviorName),
            "nodeName" => Ok(__Field::m_nodeName),
            "toolType" => Ok(__Field::m_toolType),
            "id" => Ok(__Field::m_id),
            _ => Ok(__Field::__ignore),
        }
    }
}
impl<'de> _serde::Deserialize<'de> for __Field {
    #[inline]
    fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
    }
}
pub(super) struct __hkbBehaviorInfoIdToNamePairVisitor<'de> {
    marker: core::marker::PhantomData<hkbBehaviorInfoIdToNamePair<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbBehaviorInfoIdToNamePairVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbBehaviorInfoIdToNamePair<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbBehaviorInfoIdToNamePair<'de>,
                >,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbBehaviorInfoIdToNamePairVisitor<'de> {
    type Value = hkbBehaviorInfoIdToNamePair<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbBehaviorInfoIdToNamePair",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_behaviorName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_nodeName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_toolType: _serde::__private::Option<ToolNodeType> = _serde::__private::None;
        let mut m_id: _serde::__private::Option<i16> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_behaviorName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "behaviorName",
                            ),
                        );
                    }
                    m_behaviorName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_nodeName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nodeName",
                            ),
                        );
                    }
                    m_nodeName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_toolType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "toolType",
                            ),
                        );
                    }
                    m_toolType = _serde::__private::Some(
                        match __A::next_value::<ToolNodeType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_id) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_id = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
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
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_behaviorName = match m_behaviorName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("behaviorName"),
                );
            }
        };
        let m_nodeName = match m_nodeName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nodeName"),
                );
            }
        };
        let m_toolType = match m_toolType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("toolType"),
                );
            }
        };
        let m_id = match m_id {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("id"),
                );
            }
        };
        _serde::__private::Ok(hkbBehaviorInfoIdToNamePair {
            __ptr: __A::class_ptr(&mut __map),
            m_behaviorName,
            m_nodeName,
            m_toolType,
            m_id,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_behaviorName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_nodeName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_toolType: _serde::__private::Option<ToolNodeType> = _serde::__private::None;
        let mut m_id: _serde::__private::Option<i16> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_behaviorName => {
                        if _serde::__private::Option::is_some(&m_behaviorName) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "behaviorName",
                                ),
                            );
                        }
                        m_behaviorName = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_nodeName => {
                        if _serde::__private::Option::is_some(&m_nodeName) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "nodeName",
                                ),
                            );
                        }
                        m_nodeName = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_toolType => {
                        if _serde::__private::Option::is_some(&m_toolType) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "toolType",
                                ),
                            );
                        }
                        m_toolType = _serde::__private::Some(
                            match __A::next_value::<ToolNodeType>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_id => {
                        if _serde::__private::Option::is_some(&m_id) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                            );
                        }
                        m_id = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
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
        }
        let m_behaviorName = match m_behaviorName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("behaviorName"),
                );
            }
        };
        let m_nodeName = match m_nodeName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nodeName"),
                );
            }
        };
        let m_toolType = match m_toolType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("toolType"),
                );
            }
        };
        let m_id = match m_id {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("id"),
                );
            }
        };
        _serde::__private::Ok(hkbBehaviorInfoIdToNamePair {
            __ptr: __A::class_ptr(&mut __map),
            m_behaviorName,
            m_nodeName,
            m_toolType,
            m_id,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbBehaviorInfoIdToNamePair<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["behaviorName", "nodeName", "toolType", "id"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBehaviorInfoIdToNamePair",
                FIELDS,
                __hkbBehaviorInfoIdToNamePairVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbBehaviorInfoIdToNamePair,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
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
pub enum ToolNodeType {
    #[default]
    NODE_TYPE_UNKNOWN = 0isize,
    NODE_TYPE_STATE_MACHINE = 1isize,
    NODE_TYPE_CLIP = 2isize,
    NODE_TYPE_BLEND = 3isize,
    NODE_TYPE_MODIFIER = 4isize,
    NODE_TYPE_GENERATOR = 5isize,
    NODE_TYPE_MODIFIER_GENERATOR = 6isize,
    NODE_TYPE_TRANSITION_EFFECT = 7isize,
    NODE_TYPE_BEHAVIOR_FILE_REFERENCE = 8isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ToolNodeType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NODE_TYPE_UNKNOWN => {
                    __serializer.serialize_field("NODE_TYPE_UNKNOWN", &0u64)
                }
                Self::NODE_TYPE_STATE_MACHINE => {
                    __serializer.serialize_field("NODE_TYPE_STATE_MACHINE", &1u64)
                }
                Self::NODE_TYPE_CLIP => {
                    __serializer.serialize_field("NODE_TYPE_CLIP", &2u64)
                }
                Self::NODE_TYPE_BLEND => {
                    __serializer.serialize_field("NODE_TYPE_BLEND", &3u64)
                }
                Self::NODE_TYPE_MODIFIER => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER", &4u64)
                }
                Self::NODE_TYPE_GENERATOR => {
                    __serializer.serialize_field("NODE_TYPE_GENERATOR", &5u64)
                }
                Self::NODE_TYPE_MODIFIER_GENERATOR => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER_GENERATOR", &6u64)
                }
                Self::NODE_TYPE_TRANSITION_EFFECT => {
                    __serializer.serialize_field("NODE_TYPE_TRANSITION_EFFECT", &7u64)
                }
                Self::NODE_TYPE_BEHAVIOR_FILE_REFERENCE => {
                    __serializer
                        .serialize_field("NODE_TYPE_BEHAVIOR_FILE_REFERENCE", &8u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ToolNodeType to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for ToolNodeType {
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
                            v if v == "0"
                                || v.eq_ignore_ascii_case("NODE_TYPE_UNKNOWN") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("NODE_TYPE_STATE_MACHINE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("NODE_TYPE_CLIP") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("NODE_TYPE_BLEND") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("NODE_TYPE_MODIFIER") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("NODE_TYPE_GENERATOR") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v
                                    .eq_ignore_ascii_case("NODE_TYPE_MODIFIER_GENERATOR") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("NODE_TYPE_TRANSITION_EFFECT") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8"
                                || v
                                    .eq_ignore_ascii_case(
                                        "NODE_TYPE_BEHAVIOR_FILE_REFERENCE",
                                    ) => _serde::__private::Ok(__Field::__field8),
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
                marker: _serde::__private::PhantomData<ToolNodeType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ToolNodeType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ToolNodeType",
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
                            _serde::__private::Ok(ToolNodeType::NODE_TYPE_UNKNOWN)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ToolNodeType::NODE_TYPE_STATE_MACHINE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ToolNodeType::NODE_TYPE_CLIP)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ToolNodeType::NODE_TYPE_BLEND)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ToolNodeType::NODE_TYPE_MODIFIER)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ToolNodeType::NODE_TYPE_GENERATOR)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ToolNodeType::NODE_TYPE_MODIFIER_GENERATOR,
                            )
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ToolNodeType::NODE_TYPE_TRANSITION_EFFECT,
                            )
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ToolNodeType::NODE_TYPE_BEHAVIOR_FILE_REFERENCE,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "NODE_TYPE_UNKNOWN",
                "NODE_TYPE_STATE_MACHINE",
                "NODE_TYPE_CLIP",
                "NODE_TYPE_BLEND",
                "NODE_TYPE_MODIFIER",
                "NODE_TYPE_GENERATOR",
                "NODE_TYPE_MODIFIER_GENERATOR",
                "NODE_TYPE_TRANSITION_EFFECT",
                "NODE_TYPE_BEHAVIOR_FILE_REFERENCE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ToolNodeType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ToolNodeType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
