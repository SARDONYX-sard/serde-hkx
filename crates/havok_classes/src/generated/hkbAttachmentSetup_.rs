use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbAttachmentSetup`
/// - version: `1`
/// - signature: `0x774632b`
/// - size: ` 40`(x86)/` 48`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// - name: `blendInTime`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "blendInTime"))]
    #[cfg_attr(feature = "serde", serde(rename = "blendInTime"))]
    pub m_blendInTime: f32,
    /// # C++ Info
    /// - name: `moveAttacherFraction`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "moveAttacherFraction"))]
    #[cfg_attr(feature = "serde", serde(rename = "moveAttacherFraction"))]
    pub m_moveAttacherFraction: f32,
    /// # C++ Info
    /// - name: `gain`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "gain"))]
    #[cfg_attr(feature = "serde", serde(rename = "gain"))]
    pub m_gain: f32,
    /// # C++ Info
    /// - name: `extrapolationTimeStep`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "extrapolationTimeStep"))]
    #[cfg_attr(feature = "serde", serde(rename = "extrapolationTimeStep"))]
    pub m_extrapolationTimeStep: f32,
    /// # C++ Info
    /// - name: `fixUpGain`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "fixUpGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "fixUpGain"))]
    pub m_fixUpGain: f32,
    /// # C++ Info
    /// - name: `maxLinearDistance`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxLinearDistance"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxLinearDistance"))]
    pub m_maxLinearDistance: f32,
    /// # C++ Info
    /// - name: `maxAngularDistance`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxAngularDistance"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxAngularDistance"))]
    pub m_maxAngularDistance: f32,
    /// # C++ Info
    /// - name: `attachmentType`(ctype: `enum AttachmentType`)
    /// - offset: ` 36`(x86)/` 44`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "attachmentType"))]
    #[cfg_attr(feature = "serde", serde(rename = "attachmentType"))]
    pub m_attachmentType: AttachmentType,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbAttachmentSetup {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbAttachmentSetup"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x774632b)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkbAttachmentSetup {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x774632b)));
            let mut serializer = __serializer
                .serialize_struct("hkbAttachmentSetup", class_meta, (40u64, 48u64))?;
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbAttachmentSetup {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_blendInTime,
                m_moveAttacherFraction,
                m_gain,
                m_extrapolationTimeStep,
                m_fixUpGain,
                m_maxLinearDistance,
                m_maxAngularDistance,
                m_attachmentType,
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
                        "blendInTime" => Ok(__Field::m_blendInTime),
                        "moveAttacherFraction" => Ok(__Field::m_moveAttacherFraction),
                        "gain" => Ok(__Field::m_gain),
                        "extrapolationTimeStep" => Ok(__Field::m_extrapolationTimeStep),
                        "fixUpGain" => Ok(__Field::m_fixUpGain),
                        "maxLinearDistance" => Ok(__Field::m_maxLinearDistance),
                        "maxAngularDistance" => Ok(__Field::m_maxAngularDistance),
                        "attachmentType" => Ok(__Field::m_attachmentType),
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
            struct __hkbAttachmentSetupVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbAttachmentSetup>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbAttachmentSetupVisitor<'de> {
                type Value = hkbAttachmentSetup;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbAttachmentSetup",
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
                    let mut m_blendInTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_moveAttacherFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_gain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_extrapolationTimeStep: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fixUpGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxLinearDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAngularDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_attachmentType: _serde::__private::Option<
                        AttachmentType,
                    > = _serde::__private::None;
                    for i in 0..8usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_blendInTime) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendInTime",
                                        ),
                                    );
                                }
                                m_blendInTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_moveAttacherFraction,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "moveAttacherFraction",
                                        ),
                                    );
                                }
                                m_moveAttacherFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_gain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("gain"),
                                    );
                                }
                                m_gain = _serde::__private::Some(
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
                                    &m_extrapolationTimeStep,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extrapolationTimeStep",
                                        ),
                                    );
                                }
                                m_extrapolationTimeStep = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_fixUpGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixUpGain",
                                        ),
                                    );
                                }
                                m_fixUpGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxLinearDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxLinearDistance",
                                        ),
                                    );
                                }
                                m_maxLinearDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxAngularDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAngularDistance",
                                        ),
                                    );
                                }
                                m_maxAngularDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_attachmentType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attachmentType",
                                        ),
                                    );
                                }
                                m_attachmentType = _serde::__private::Some(
                                    match __A::next_value::<AttachmentType>(&mut __map) {
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
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    let m_blendInTime = match m_blendInTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendInTime",
                                ),
                            );
                        }
                    };
                    let m_moveAttacherFraction = match m_moveAttacherFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "moveAttacherFraction",
                                ),
                            );
                        }
                    };
                    let m_gain = match m_gain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gain"),
                            );
                        }
                    };
                    let m_extrapolationTimeStep = match m_extrapolationTimeStep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extrapolationTimeStep",
                                ),
                            );
                        }
                    };
                    let m_fixUpGain = match m_fixUpGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fixUpGain",
                                ),
                            );
                        }
                    };
                    let m_maxLinearDistance = match m_maxLinearDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxLinearDistance",
                                ),
                            );
                        }
                    };
                    let m_maxAngularDistance = match m_maxAngularDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAngularDistance",
                                ),
                            );
                        }
                    };
                    let m_attachmentType = match m_attachmentType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attachmentType",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbAttachmentSetup {
                        __ptr,
                        parent,
                        m_blendInTime,
                        m_moveAttacherFraction,
                        m_gain,
                        m_extrapolationTimeStep,
                        m_fixUpGain,
                        m_maxLinearDistance,
                        m_maxAngularDistance,
                        m_attachmentType,
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
                    let mut m_blendInTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_moveAttacherFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_gain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_extrapolationTimeStep: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fixUpGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxLinearDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAngularDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_attachmentType: _serde::__private::Option<
                        AttachmentType,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_blendInTime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blendInTime) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendInTime",
                                        ),
                                    );
                                }
                                m_blendInTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_moveAttacherFraction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_moveAttacherFraction,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "moveAttacherFraction",
                                        ),
                                    );
                                }
                                m_moveAttacherFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_gain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_gain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("gain"),
                                    );
                                }
                                m_gain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_extrapolationTimeStep => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_extrapolationTimeStep,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extrapolationTimeStep",
                                        ),
                                    );
                                }
                                m_extrapolationTimeStep = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fixUpGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fixUpGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixUpGain",
                                        ),
                                    );
                                }
                                m_fixUpGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxLinearDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxLinearDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxLinearDistance",
                                        ),
                                    );
                                }
                                m_maxLinearDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxAngularDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxAngularDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAngularDistance",
                                        ),
                                    );
                                }
                                m_maxAngularDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_attachmentType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_attachmentType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attachmentType",
                                        ),
                                    );
                                }
                                m_attachmentType = _serde::__private::Some(
                                    match __A::next_value::<AttachmentType>(&mut __map) {
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
                    let m_blendInTime = match m_blendInTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendInTime",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_moveAttacherFraction = match m_moveAttacherFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "moveAttacherFraction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_gain = match m_gain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gain"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_extrapolationTimeStep = match m_extrapolationTimeStep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extrapolationTimeStep",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fixUpGain = match m_fixUpGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fixUpGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxLinearDistance = match m_maxLinearDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxLinearDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxAngularDistance = match m_maxAngularDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAngularDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_attachmentType = match m_attachmentType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attachmentType",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbAttachmentSetup {
                        __ptr,
                        parent,
                        m_blendInTime,
                        m_moveAttacherFraction,
                        m_gain,
                        m_extrapolationTimeStep,
                        m_fixUpGain,
                        m_maxLinearDistance,
                        m_maxAngularDistance,
                        m_attachmentType,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "blendInTime",
                "moveAttacherFraction",
                "gain",
                "extrapolationTimeStep",
                "fixUpGain",
                "maxLinearDistance",
                "maxAngularDistance",
                "attachmentType",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbAttachmentSetup",
                FIELDS,
                __hkbAttachmentSetupVisitor {
                    marker: _serde::__private::PhantomData::<hkbAttachmentSetup>,
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
                        _serde::de::ReadEnumSize::Int8,
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
                __Visitor {
                    marker: _serde::__private::PhantomData::<AttachmentType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
