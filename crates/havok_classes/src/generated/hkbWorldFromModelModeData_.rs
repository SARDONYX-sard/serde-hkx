use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbWorldFromModelModeData`
/// - version: `0`
/// - signature: `0xa3af8783`
/// - size: `  8`(x86)/`  8`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbWorldFromModelModeData {
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
    /// - name: `poseMatchingBone0`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "poseMatchingBone0"))]
    #[cfg_attr(feature = "serde", serde(rename = "poseMatchingBone0"))]
    pub m_poseMatchingBone0: i16,
    /// # C++ Info
    /// - name: `poseMatchingBone1`(ctype: `hkInt16`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "poseMatchingBone1"))]
    #[cfg_attr(feature = "serde", serde(rename = "poseMatchingBone1"))]
    pub m_poseMatchingBone1: i16,
    /// # C++ Info
    /// - name: `poseMatchingBone2`(ctype: `hkInt16`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "poseMatchingBone2"))]
    #[cfg_attr(feature = "serde", serde(rename = "poseMatchingBone2"))]
    pub m_poseMatchingBone2: i16,
    /// # C++ Info
    /// - name: `mode`(ctype: `enum WorldFromModelMode`)
    /// - offset: `  6`(x86)/`  6`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "mode"))]
    #[cfg_attr(feature = "serde", serde(rename = "mode"))]
    pub m_mode: WorldFromModelMode,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbWorldFromModelModeData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbWorldFromModelModeData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa3af8783)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkbWorldFromModelModeData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa3af8783)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbWorldFromModelModeData",
                    class_meta,
                    (8u64, 8u64),
                )?;
            serializer.serialize_field("poseMatchingBone0", &self.m_poseMatchingBone0)?;
            serializer.serialize_field("poseMatchingBone1", &self.m_poseMatchingBone1)?;
            serializer.serialize_field("poseMatchingBone2", &self.m_poseMatchingBone2)?;
            serializer.serialize_field("mode", &self.m_mode)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbWorldFromModelModeData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_poseMatchingBone0,
                m_poseMatchingBone1,
                m_poseMatchingBone2,
                m_mode,
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
                        "poseMatchingBone0" => Ok(__Field::m_poseMatchingBone0),
                        "poseMatchingBone1" => Ok(__Field::m_poseMatchingBone1),
                        "poseMatchingBone2" => Ok(__Field::m_poseMatchingBone2),
                        "mode" => Ok(__Field::m_mode),
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
            struct __hkbWorldFromModelModeDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbWorldFromModelModeData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbWorldFromModelModeDataVisitor<'de> {
                type Value = hkbWorldFromModelModeData;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbWorldFromModelModeData",
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
                    let mut m_poseMatchingBone0: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_poseMatchingBone1: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_poseMatchingBone2: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_mode: _serde::__private::Option<WorldFromModelMode> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_poseMatchingBone0,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseMatchingBone0",
                                        ),
                                    );
                                }
                                m_poseMatchingBone0 = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_poseMatchingBone1,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseMatchingBone1",
                                        ),
                                    );
                                }
                                m_poseMatchingBone1 = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_poseMatchingBone2,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseMatchingBone2",
                                        ),
                                    );
                                }
                                m_poseMatchingBone2 = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_mode) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("mode"),
                                    );
                                }
                                m_mode = _serde::__private::Some(
                                    match __A::next_value::<WorldFromModelMode>(&mut __map) {
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
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    let m_poseMatchingBone0 = match m_poseMatchingBone0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseMatchingBone0",
                                ),
                            );
                        }
                    };
                    let m_poseMatchingBone1 = match m_poseMatchingBone1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseMatchingBone1",
                                ),
                            );
                        }
                    };
                    let m_poseMatchingBone2 = match m_poseMatchingBone2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseMatchingBone2",
                                ),
                            );
                        }
                    };
                    let m_mode = match m_mode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("mode"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbWorldFromModelModeData {
                        __ptr,
                        m_poseMatchingBone0,
                        m_poseMatchingBone1,
                        m_poseMatchingBone2,
                        m_mode,
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
                    let mut m_poseMatchingBone0: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_poseMatchingBone1: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_poseMatchingBone2: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_mode: _serde::__private::Option<WorldFromModelMode> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_poseMatchingBone0 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_poseMatchingBone0,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseMatchingBone0",
                                        ),
                                    );
                                }
                                m_poseMatchingBone0 = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_poseMatchingBone1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_poseMatchingBone1,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseMatchingBone1",
                                        ),
                                    );
                                }
                                m_poseMatchingBone1 = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_poseMatchingBone2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_poseMatchingBone2,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseMatchingBone2",
                                        ),
                                    );
                                }
                                m_poseMatchingBone2 = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_mode) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("mode"),
                                    );
                                }
                                m_mode = _serde::__private::Some(
                                    match __A::next_value::<WorldFromModelMode>(&mut __map) {
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
                    let m_poseMatchingBone0 = match m_poseMatchingBone0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseMatchingBone0",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_poseMatchingBone1 = match m_poseMatchingBone1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseMatchingBone1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_poseMatchingBone2 = match m_poseMatchingBone2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseMatchingBone2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mode = match m_mode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("mode"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbWorldFromModelModeData {
                        __ptr,
                        m_poseMatchingBone0,
                        m_poseMatchingBone1,
                        m_poseMatchingBone2,
                        m_mode,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "poseMatchingBone0",
                "poseMatchingBone1",
                "poseMatchingBone2",
                "mode",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbWorldFromModelModeData",
                FIELDS,
                __hkbWorldFromModelModeDataVisitor {
                    marker: _serde::__private::PhantomData::<hkbWorldFromModelModeData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
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
pub enum WorldFromModelMode {
    #[default]
    WORLD_FROM_MODEL_MODE_USE_OLD = 0isize,
    WORLD_FROM_MODEL_MODE_USE_INPUT = 1isize,
    WORLD_FROM_MODEL_MODE_COMPUTE = 2isize,
    WORLD_FROM_MODEL_MODE_NONE = 3isize,
    WORLD_FROM_MODEL_MODE_RAGDOLL = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for WorldFromModelMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::WORLD_FROM_MODEL_MODE_USE_OLD => {
                    __serializer.serialize_field("WORLD_FROM_MODEL_MODE_USE_OLD", &0u64)
                }
                Self::WORLD_FROM_MODEL_MODE_USE_INPUT => {
                    __serializer
                        .serialize_field("WORLD_FROM_MODEL_MODE_USE_INPUT", &1u64)
                }
                Self::WORLD_FROM_MODEL_MODE_COMPUTE => {
                    __serializer.serialize_field("WORLD_FROM_MODEL_MODE_COMPUTE", &2u64)
                }
                Self::WORLD_FROM_MODEL_MODE_NONE => {
                    __serializer.serialize_field("WORLD_FROM_MODEL_MODE_NONE", &3u64)
                }
                Self::WORLD_FROM_MODEL_MODE_RAGDOLL => {
                    __serializer.serialize_field("WORLD_FROM_MODEL_MODE_RAGDOLL", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum WorldFromModelMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for WorldFromModelMode {
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
                                    .eq_ignore_ascii_case("WORLD_FROM_MODEL_MODE_USE_OLD") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case("WORLD_FROM_MODEL_MODE_USE_INPUT") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case("WORLD_FROM_MODEL_MODE_COMPUTE") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("WORLD_FROM_MODEL_MODE_NONE") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v
                                    .eq_ignore_ascii_case("WORLD_FROM_MODEL_MODE_RAGDOLL") => {
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<WorldFromModelMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = WorldFromModelMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum WorldFromModelMode",
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
                                WorldFromModelMode::WORLD_FROM_MODEL_MODE_USE_OLD,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                WorldFromModelMode::WORLD_FROM_MODEL_MODE_USE_INPUT,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                WorldFromModelMode::WORLD_FROM_MODEL_MODE_COMPUTE,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                WorldFromModelMode::WORLD_FROM_MODEL_MODE_NONE,
                            )
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                WorldFromModelMode::WORLD_FROM_MODEL_MODE_RAGDOLL,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "WORLD_FROM_MODEL_MODE_USE_OLD",
                "WORLD_FROM_MODEL_MODE_USE_INPUT",
                "WORLD_FROM_MODEL_MODE_COMPUTE",
                "WORLD_FROM_MODEL_MODE_NONE",
                "WORLD_FROM_MODEL_MODE_RAGDOLL",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "WorldFromModelMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<WorldFromModelMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
