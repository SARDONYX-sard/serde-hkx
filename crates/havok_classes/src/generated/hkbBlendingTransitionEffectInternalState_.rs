use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbBlendingTransitionEffectInternalState`
/// - version: `0`
/// - signature: `0xb18c70c2`
/// - size: ` 32`(x86)/` 48`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlendingTransitionEffectInternalState<'a> {
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
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hkReferencedObject<'a>,
    /// # C++ Info
    /// - name: `characterPoseAtBeginningOfTransition`(ctype: `hkArray<hkQsTransform>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "characterPoseAtBeginningOfTransition")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "characterPoseAtBeginningOfTransition")
    )]
    pub m_characterPoseAtBeginningOfTransition: Vec<QsTransform>,
    /// # C++ Info
    /// - name: `timeRemaining`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeRemaining"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeRemaining"))]
    pub m_timeRemaining: f32,
    /// # C++ Info
    /// - name: `timeInTransition`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeInTransition"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeInTransition"))]
    pub m_timeInTransition: f32,
    /// # C++ Info
    /// - name: `applySelfTransition`(ctype: `hkBool`)
    /// - offset: ` 28`(x86)/` 40`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "applySelfTransition"))]
    #[cfg_attr(feature = "serde", serde(rename = "applySelfTransition"))]
    pub m_applySelfTransition: bool,
    /// # C++ Info
    /// - name: `initializeCharacterPose`(ctype: `hkBool`)
    /// - offset: ` 29`(x86)/` 41`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "initializeCharacterPose"))]
    #[cfg_attr(feature = "serde", serde(rename = "initializeCharacterPose"))]
    pub m_initializeCharacterPose: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbBlendingTransitionEffectInternalState<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBlendingTransitionEffectInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb18c70c2)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkbBlendingTransitionEffectInternalState<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xb18c70c2)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbBlendingTransitionEffectInternalState",
                    class_meta,
                    (32u64, 48u64),
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "characterPoseAtBeginningOfTransition",
                    &self.m_characterPoseAtBeginningOfTransition,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("timeRemaining", &self.m_timeRemaining)?;
            serializer.serialize_field("timeInTransition", &self.m_timeInTransition)?;
            serializer
                .serialize_field("applySelfTransition", &self.m_applySelfTransition)?;
            serializer
                .serialize_field(
                    "initializeCharacterPose",
                    &self.m_initializeCharacterPose,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkbBlendingTransitionEffectInternalState<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_characterPoseAtBeginningOfTransition,
                m_timeRemaining,
                m_timeInTransition,
                m_applySelfTransition,
                m_initializeCharacterPose,
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
                        "characterPoseAtBeginningOfTransition" => {
                            Ok(__Field::m_characterPoseAtBeginningOfTransition)
                        }
                        "timeRemaining" => Ok(__Field::m_timeRemaining),
                        "timeInTransition" => Ok(__Field::m_timeInTransition),
                        "applySelfTransition" => Ok(__Field::m_applySelfTransition),
                        "initializeCharacterPose" => {
                            Ok(__Field::m_initializeCharacterPose)
                        }
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
            struct __hkbBlendingTransitionEffectInternalStateVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbBlendingTransitionEffectInternalState<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbBlendingTransitionEffectInternalStateVisitor<'de> {
                type Value = hkbBlendingTransitionEffectInternalState<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbBlendingTransitionEffectInternalState",
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
                    let mut m_characterPoseAtBeginningOfTransition: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_timeRemaining: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_timeInTransition: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_applySelfTransition: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_initializeCharacterPose: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterPoseAtBeginningOfTransition,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPoseAtBeginningOfTransition",
                                        ),
                                    );
                                }
                                m_characterPoseAtBeginningOfTransition = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_timeRemaining) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeRemaining",
                                        ),
                                    );
                                }
                                m_timeRemaining = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_timeInTransition) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeInTransition",
                                        ),
                                    );
                                }
                                m_timeInTransition = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_applySelfTransition,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "applySelfTransition",
                                        ),
                                    );
                                }
                                m_applySelfTransition = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_initializeCharacterPose,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initializeCharacterPose",
                                        ),
                                    );
                                }
                                m_initializeCharacterPose = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    __A::pad(&mut __map, 2usize, 6usize)?;
                    let m_characterPoseAtBeginningOfTransition = match m_characterPoseAtBeginningOfTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPoseAtBeginningOfTransition",
                                ),
                            );
                        }
                    };
                    let m_timeRemaining = match m_timeRemaining {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeRemaining",
                                ),
                            );
                        }
                    };
                    let m_timeInTransition = match m_timeInTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeInTransition",
                                ),
                            );
                        }
                    };
                    let m_applySelfTransition = match m_applySelfTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "applySelfTransition",
                                ),
                            );
                        }
                    };
                    let m_initializeCharacterPose = match m_initializeCharacterPose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initializeCharacterPose",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbBlendingTransitionEffectInternalState {
                        __ptr,
                        parent,
                        m_characterPoseAtBeginningOfTransition,
                        m_timeRemaining,
                        m_timeInTransition,
                        m_applySelfTransition,
                        m_initializeCharacterPose,
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
                    let mut m_characterPoseAtBeginningOfTransition: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_timeRemaining: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_timeInTransition: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_applySelfTransition: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_initializeCharacterPose: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_characterPoseAtBeginningOfTransition => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_characterPoseAtBeginningOfTransition,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPoseAtBeginningOfTransition",
                                        ),
                                    );
                                }
                                m_characterPoseAtBeginningOfTransition = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_timeRemaining => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_timeRemaining) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeRemaining",
                                        ),
                                    );
                                }
                                m_timeRemaining = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_timeInTransition => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_timeInTransition) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeInTransition",
                                        ),
                                    );
                                }
                                m_timeInTransition = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_applySelfTransition => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_applySelfTransition,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "applySelfTransition",
                                        ),
                                    );
                                }
                                m_applySelfTransition = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_initializeCharacterPose => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_initializeCharacterPose,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initializeCharacterPose",
                                        ),
                                    );
                                }
                                m_initializeCharacterPose = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    let m_characterPoseAtBeginningOfTransition = match m_characterPoseAtBeginningOfTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPoseAtBeginningOfTransition",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_timeRemaining = match m_timeRemaining {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeRemaining",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_timeInTransition = match m_timeInTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeInTransition",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_applySelfTransition = match m_applySelfTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "applySelfTransition",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_initializeCharacterPose = match m_initializeCharacterPose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initializeCharacterPose",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbBlendingTransitionEffectInternalState {
                        __ptr: __ptr.clone(),
                        parent,
                        m_characterPoseAtBeginningOfTransition,
                        m_timeRemaining,
                        m_timeInTransition,
                        m_applySelfTransition,
                        m_initializeCharacterPose,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "characterPoseAtBeginningOfTransition",
                "timeRemaining",
                "timeInTransition",
                "applySelfTransition",
                "initializeCharacterPose",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBlendingTransitionEffectInternalState",
                FIELDS,
                __hkbBlendingTransitionEffectInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbBlendingTransitionEffectInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
