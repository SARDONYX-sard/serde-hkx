use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbTransitionEffect`
/// - version: `0`
/// - signature: `0x945da157`
/// - size: ` 44`(x86)/` 80`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbTransitionEffect<'a> {
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
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// - name: `selfTransitionMode`(ctype: `enum SelfTransitionMode`)
    /// - offset: ` 40`(x86)/` 72`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "selfTransitionMode"))]
    #[cfg_attr(feature = "serde", serde(rename = "selfTransitionMode"))]
    pub m_selfTransitionMode: SelfTransitionMode,
    /// # C++ Info
    /// - name: `eventMode`(ctype: `enum EventMode`)
    /// - offset: ` 41`(x86)/` 73`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "eventMode"))]
    #[cfg_attr(feature = "serde", serde(rename = "eventMode"))]
    pub m_eventMode: EventMode,
    /// # C++ Info
    /// - name: `defaultEventMode`(ctype: `enum unknown`)
    /// - offset: ` 42`(x86)/` 74`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "defaultEventMode"))]
    #[cfg_attr(feature = "serde", serde(rename = "defaultEventMode"))]
    pub m_defaultEventMode: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbTransitionEffect<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbTransitionEffect"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x945da157)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbTransitionEffect<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x945da157)));
            let mut serializer = __serializer
                .serialize_struct("hkbTransitionEffect", class_meta, (44u64, 80u64))?;
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
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer
                .skip_fixed_array_field(
                    "padNode",
                    self.parent.parent.m_padNode.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("selfTransitionMode", &self.m_selfTransitionMode)?;
            serializer.serialize_field("eventMode", &self.m_eventMode)?;
            serializer.skip_field("defaultEventMode", &self.m_defaultEventMode)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbTransitionEffect<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_selfTransitionMode,
                m_eventMode,
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
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "selfTransitionMode" => Ok(__Field::m_selfTransitionMode),
                        "eventMode" => Ok(__Field::m_eventMode),
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
            struct __hkbTransitionEffectVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbTransitionEffect<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbTransitionEffectVisitor<'de> {
                type Value = hkbTransitionEffect<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbTransitionEffect",
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
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_selfTransitionMode: _serde::__private::Option<
                        SelfTransitionMode,
                    > = _serde::__private::None;
                    let mut m_eventMode: _serde::__private::Option<EventMode> = _serde::__private::None;
                    let mut m_defaultEventMode: _serde::__private::Option<i8> = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_selfTransitionMode,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "selfTransitionMode",
                                        ),
                                    );
                                }
                                m_selfTransitionMode = _serde::__private::Some(
                                    match __A::next_value::<SelfTransitionMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_eventMode) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventMode",
                                        ),
                                    );
                                }
                                m_eventMode = _serde::__private::Some(
                                    match __A::next_value::<EventMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_defaultEventMode) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaultEventMode",
                                        ),
                                    );
                                }
                                m_defaultEventMode = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
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
                    __A::pad(&mut __map, 1usize, 5usize)?;
                    let m_selfTransitionMode = match m_selfTransitionMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "selfTransitionMode",
                                ),
                            );
                        }
                    };
                    let m_eventMode = match m_eventMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventMode",
                                ),
                            );
                        }
                    };
                    let m_defaultEventMode = match m_defaultEventMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "defaultEventMode",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbTransitionEffect {
                        __ptr,
                        parent,
                        m_selfTransitionMode,
                        m_eventMode,
                        m_defaultEventMode,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_variableBindingSet: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_selfTransitionMode: _serde::__private::Option<
                        SelfTransitionMode,
                    > = _serde::__private::None;
                    let mut m_eventMode: _serde::__private::Option<EventMode> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_selfTransitionMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_selfTransitionMode,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "selfTransitionMode",
                                        ),
                                    );
                                }
                                m_selfTransitionMode = _serde::__private::Some(
                                    match __A::next_value::<SelfTransitionMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_eventMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eventMode) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventMode",
                                        ),
                                    );
                                }
                                m_eventMode = _serde::__private::Some(
                                    match __A::next_value::<EventMode>(&mut __map) {
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
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_selfTransitionMode = match m_selfTransitionMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "selfTransitionMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_eventMode = match m_eventMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkbBindable {
                        __ptr,
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr,
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbGenerator { __ptr, parent };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbTransitionEffect {
                        __ptr,
                        parent,
                        m_selfTransitionMode,
                        m_eventMode,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "selfTransitionMode",
                "eventMode",
                "defaultEventMode",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbTransitionEffect",
                FIELDS,
                __hkbTransitionEffectVisitor {
                    marker: _serde::__private::PhantomData::<hkbTransitionEffect>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `SelfTransitionMode`(ctype: `hkEnum<SelfTransitionMode, hkInt8>`)
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
pub enum SelfTransitionMode {
    #[default]
    SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC = 0isize,
    SELF_TRANSITION_MODE_CONTINUE = 1isize,
    SELF_TRANSITION_MODE_RESET = 2isize,
    SELF_TRANSITION_MODE_BLEND = 3isize,
}
/// # C++ Info
/// - name: `EventMode`(ctype: `hkEnum<EventMode, hkInt8>`)
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
pub enum EventMode {
    #[default]
    EVENT_MODE_DEFAULT = 0isize,
    EVENT_MODE_PROCESS_ALL = 1isize,
    EVENT_MODE_IGNORE_FROM_GENERATOR = 2isize,
    EVENT_MODE_IGNORE_TO_GENERATOR = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SelfTransitionMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC => {
                    __serializer
                        .serialize_field(
                            "SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC",
                            &0u64,
                        )
                }
                Self::SELF_TRANSITION_MODE_CONTINUE => {
                    __serializer.serialize_field("SELF_TRANSITION_MODE_CONTINUE", &1u64)
                }
                Self::SELF_TRANSITION_MODE_RESET => {
                    __serializer.serialize_field("SELF_TRANSITION_MODE_RESET", &2u64)
                }
                Self::SELF_TRANSITION_MODE_BLEND => {
                    __serializer.serialize_field("SELF_TRANSITION_MODE_BLEND", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum SelfTransitionMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for EventMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::EVENT_MODE_DEFAULT => {
                    __serializer.serialize_field("EVENT_MODE_DEFAULT", &0u64)
                }
                Self::EVENT_MODE_PROCESS_ALL => {
                    __serializer.serialize_field("EVENT_MODE_PROCESS_ALL", &1u64)
                }
                Self::EVENT_MODE_IGNORE_FROM_GENERATOR => {
                    __serializer
                        .serialize_field("EVENT_MODE_IGNORE_FROM_GENERATOR", &2u64)
                }
                Self::EVENT_MODE_IGNORE_TO_GENERATOR => {
                    __serializer.serialize_field("EVENT_MODE_IGNORE_TO_GENERATOR", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum EventMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for SelfTransitionMode {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3",
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
                                        "SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC",
                                    ) => _serde::__private::Ok(__Field::__field0),
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case("SELF_TRANSITION_MODE_CONTINUE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("SELF_TRANSITION_MODE_RESET") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("SELF_TRANSITION_MODE_BLEND") => {
                                _serde::__private::Ok(__Field::__field3)
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
                marker: _serde::__private::PhantomData<SelfTransitionMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SelfTransitionMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum SelfTransitionMode",
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
                                SelfTransitionMode::SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SelfTransitionMode::SELF_TRANSITION_MODE_CONTINUE,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SelfTransitionMode::SELF_TRANSITION_MODE_RESET,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SelfTransitionMode::SELF_TRANSITION_MODE_BLEND,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC",
                "SELF_TRANSITION_MODE_CONTINUE",
                "SELF_TRANSITION_MODE_RESET",
                "SELF_TRANSITION_MODE_BLEND",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "SelfTransitionMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SelfTransitionMode>,
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
    impl<'de> _serde::Deserialize<'de> for EventMode {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3",
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
                                || v.eq_ignore_ascii_case("EVENT_MODE_DEFAULT") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("EVENT_MODE_PROCESS_ALL") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case(
                                        "EVENT_MODE_IGNORE_FROM_GENERATOR",
                                    ) => _serde::__private::Ok(__Field::__field2),
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case("EVENT_MODE_IGNORE_TO_GENERATOR") => {
                                _serde::__private::Ok(__Field::__field3)
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
                marker: _serde::__private::PhantomData<EventMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = EventMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum EventMode",
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
                            _serde::__private::Ok(EventMode::EVENT_MODE_DEFAULT)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(EventMode::EVENT_MODE_PROCESS_ALL)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                EventMode::EVENT_MODE_IGNORE_FROM_GENERATOR,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                EventMode::EVENT_MODE_IGNORE_TO_GENERATOR,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "EVENT_MODE_DEFAULT",
                "EVENT_MODE_PROCESS_ALL",
                "EVENT_MODE_IGNORE_FROM_GENERATOR",
                "EVENT_MODE_IGNORE_TO_GENERATOR",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "EventMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<EventMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
