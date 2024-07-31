use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbFootIkModifierLeg`
/// - version: `2`
/// - signature: `0x9f3e3a04`
/// - size: `160`(x86)/`160`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkModifierLeg {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `originalAnkleTransformMS`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_originalAnkleTransformMS: QsTransform,
    /// # C++ Info
    /// - name: `prevAnkleRotLS`(ctype: `hkQuaternion`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_prevAnkleRotLS: Quaternion,
    /// # C++ Info
    /// - name: `kneeAxisLS`(ctype: `hkVector4`)
    /// - offset: ` 64`(x86)/` 64`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_kneeAxisLS: Vector4,
    /// # C++ Info
    /// - name: `footEndLS`(ctype: `hkVector4`)
    /// - offset: ` 80`(x86)/` 80`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_footEndLS: Vector4,
    /// # C++ Info
    /// - name: `ungroundedEvent`(ctype: `struct hkbEventProperty`)
    /// - offset: ` 96`(x86)/` 96`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    pub m_ungroundedEvent: hkbEventProperty,
    /// # C++ Info
    /// - name: `footPlantedAnkleHeightMS`(ctype: `hkReal`)
    /// - offset: `104`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_footPlantedAnkleHeightMS: f32,
    /// # C++ Info
    /// - name: `footRaisedAnkleHeightMS`(ctype: `hkReal`)
    /// - offset: `108`(x86)/`116`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_footRaisedAnkleHeightMS: f32,
    /// # C++ Info
    /// - name: `maxAnkleHeightMS`(ctype: `hkReal`)
    /// - offset: `112`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxAnkleHeightMS: f32,
    /// # C++ Info
    /// - name: `minAnkleHeightMS`(ctype: `hkReal`)
    /// - offset: `116`(x86)/`124`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minAnkleHeightMS: f32,
    /// # C++ Info
    /// - name: `maxKneeAngleDegrees`(ctype: `hkReal`)
    /// - offset: `120`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxKneeAngleDegrees: f32,
    /// # C++ Info
    /// - name: `minKneeAngleDegrees`(ctype: `hkReal`)
    /// - offset: `124`(x86)/`132`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minKneeAngleDegrees: f32,
    /// # C++ Info
    /// - name: `verticalError`(ctype: `hkReal`)
    /// - offset: `128`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_verticalError: f32,
    /// # C++ Info
    /// - name: `maxAnkleAngleDegrees`(ctype: `hkReal`)
    /// - offset: `132`(x86)/`140`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxAnkleAngleDegrees: f32,
    /// # C++ Info
    /// - name: `hipIndex`(ctype: `hkInt16`)
    /// - offset: `136`(x86)/`144`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_hipIndex: i16,
    /// # C++ Info
    /// - name: `kneeIndex`(ctype: `hkInt16`)
    /// - offset: `138`(x86)/`146`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_kneeIndex: i16,
    /// # C++ Info
    /// - name: `ankleIndex`(ctype: `hkInt16`)
    /// - offset: `140`(x86)/`148`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_ankleIndex: i16,
    /// # C++ Info
    /// - name: `hitSomething`(ctype: `hkBool`)
    /// - offset: `142`(x86)/`150`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_hitSomething: bool,
    /// # C++ Info
    /// - name: `isPlantedMS`(ctype: `hkBool`)
    /// - offset: `143`(x86)/`151`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isPlantedMS: bool,
    /// # C++ Info
    /// - name: `isOriginalAnkleTransformMSSet`(ctype: `hkBool`)
    /// - offset: `144`(x86)/`152`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isOriginalAnkleTransformMSSet: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbFootIkModifierLeg {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkModifierLeg"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9f3e3a04)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_ungroundedEvent.deps_indexes());
            v
        }
    }
    impl _serde::Serialize for hkbFootIkModifierLeg {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9f3e3a04)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkModifierLeg", class_meta)?;
            serializer
                .serialize_field(
                    "originalAnkleTransformMS",
                    &self.m_originalAnkleTransformMS,
                )?;
            serializer.skip_field("prevAnkleRotLS", &self.m_prevAnkleRotLS)?;
            serializer.serialize_field("kneeAxisLS", &self.m_kneeAxisLS)?;
            serializer.serialize_field("footEndLS", &self.m_footEndLS)?;
            serializer.serialize_field("ungroundedEvent", &self.m_ungroundedEvent)?;
            serializer
                .serialize_field(
                    "footPlantedAnkleHeightMS",
                    &self.m_footPlantedAnkleHeightMS,
                )?;
            serializer
                .serialize_field(
                    "footRaisedAnkleHeightMS",
                    &self.m_footRaisedAnkleHeightMS,
                )?;
            serializer.serialize_field("maxAnkleHeightMS", &self.m_maxAnkleHeightMS)?;
            serializer.serialize_field("minAnkleHeightMS", &self.m_minAnkleHeightMS)?;
            serializer
                .serialize_field("maxKneeAngleDegrees", &self.m_maxKneeAngleDegrees)?;
            serializer
                .serialize_field("minKneeAngleDegrees", &self.m_minKneeAngleDegrees)?;
            serializer.serialize_field("verticalError", &self.m_verticalError)?;
            serializer
                .serialize_field("maxAnkleAngleDegrees", &self.m_maxAnkleAngleDegrees)?;
            serializer.serialize_field("hipIndex", &self.m_hipIndex)?;
            serializer.serialize_field("kneeIndex", &self.m_kneeIndex)?;
            serializer.serialize_field("ankleIndex", &self.m_ankleIndex)?;
            serializer.serialize_field("hitSomething", &self.m_hitSomething)?;
            serializer.serialize_field("isPlantedMS", &self.m_isPlantedMS)?;
            serializer
                .serialize_field(
                    "isOriginalAnkleTransformMSSet",
                    &self.m_isOriginalAnkleTransformMSSet,
                )?;
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbFootIkModifierLeg {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_originalAnkleTransformMS,
                m_kneeAxisLS,
                m_footEndLS,
                m_ungroundedEvent,
                m_footPlantedAnkleHeightMS,
                m_footRaisedAnkleHeightMS,
                m_maxAnkleHeightMS,
                m_minAnkleHeightMS,
                m_maxKneeAngleDegrees,
                m_minKneeAngleDegrees,
                m_verticalError,
                m_maxAnkleAngleDegrees,
                m_hipIndex,
                m_kneeIndex,
                m_ankleIndex,
                m_hitSomething,
                m_isPlantedMS,
                m_isOriginalAnkleTransformMSSet,
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
                        "originalAnkleTransformMS" => {
                            Ok(__Field::m_originalAnkleTransformMS)
                        }
                        "kneeAxisLS" => Ok(__Field::m_kneeAxisLS),
                        "footEndLS" => Ok(__Field::m_footEndLS),
                        "ungroundedEvent" => Ok(__Field::m_ungroundedEvent),
                        "footPlantedAnkleHeightMS" => {
                            Ok(__Field::m_footPlantedAnkleHeightMS)
                        }
                        "footRaisedAnkleHeightMS" => {
                            Ok(__Field::m_footRaisedAnkleHeightMS)
                        }
                        "maxAnkleHeightMS" => Ok(__Field::m_maxAnkleHeightMS),
                        "minAnkleHeightMS" => Ok(__Field::m_minAnkleHeightMS),
                        "maxKneeAngleDegrees" => Ok(__Field::m_maxKneeAngleDegrees),
                        "minKneeAngleDegrees" => Ok(__Field::m_minKneeAngleDegrees),
                        "verticalError" => Ok(__Field::m_verticalError),
                        "maxAnkleAngleDegrees" => Ok(__Field::m_maxAnkleAngleDegrees),
                        "hipIndex" => Ok(__Field::m_hipIndex),
                        "kneeIndex" => Ok(__Field::m_kneeIndex),
                        "ankleIndex" => Ok(__Field::m_ankleIndex),
                        "hitSomething" => Ok(__Field::m_hitSomething),
                        "isPlantedMS" => Ok(__Field::m_isPlantedMS),
                        "isOriginalAnkleTransformMSSet" => {
                            Ok(__Field::m_isOriginalAnkleTransformMSSet)
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
            struct __hkbFootIkModifierLegVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbFootIkModifierLeg>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbFootIkModifierLegVisitor<'de> {
                type Value = hkbFootIkModifierLeg;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbFootIkModifierLeg",
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
                    let mut m_originalAnkleTransformMS: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_prevAnkleRotLS: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_kneeAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_footEndLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ungroundedEvent: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_footPlantedAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_footRaisedAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxKneeAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minKneeAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_verticalError: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAnkleAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_hipIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_kneeIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_ankleIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_hitSomething: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isPlantedMS: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isOriginalAnkleTransformMSSet: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    for i in 0..19usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_originalAnkleTransformMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalAnkleTransformMS",
                                        ),
                                    );
                                }
                                m_originalAnkleTransformMS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_prevAnkleRotLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "prevAnkleRotLS",
                                        ),
                                    );
                                }
                                m_prevAnkleRotLS = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_kneeAxisLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "kneeAxisLS",
                                        ),
                                    );
                                }
                                m_kneeAxisLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_footEndLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footEndLS",
                                        ),
                                    );
                                }
                                m_footEndLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_ungroundedEvent) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ungroundedEvent",
                                        ),
                                    );
                                }
                                m_ungroundedEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_footPlantedAnkleHeightMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footPlantedAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_footPlantedAnkleHeightMS = _serde::__private::Some(
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
                                    &m_footRaisedAnkleHeightMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footRaisedAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_footRaisedAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_maxAnkleHeightMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_maxAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_minAnkleHeightMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_minAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxKneeAngleDegrees,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxKneeAngleDegrees",
                                        ),
                                    );
                                }
                                m_maxKneeAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minKneeAngleDegrees,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minKneeAngleDegrees",
                                        ),
                                    );
                                }
                                m_minKneeAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_verticalError) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalError",
                                        ),
                                    );
                                }
                                m_verticalError = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxAnkleAngleDegrees,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAnkleAngleDegrees",
                                        ),
                                    );
                                }
                                m_maxAnkleAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_hipIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hipIndex",
                                        ),
                                    );
                                }
                                m_hipIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_kneeIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "kneeIndex",
                                        ),
                                    );
                                }
                                m_kneeIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_ankleIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ankleIndex",
                                        ),
                                    );
                                }
                                m_ankleIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_hitSomething) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hitSomething",
                                        ),
                                    );
                                }
                                m_hitSomething = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_isPlantedMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isPlantedMS",
                                        ),
                                    );
                                }
                                m_isPlantedMS = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_isOriginalAnkleTransformMSSet,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isOriginalAnkleTransformMSSet",
                                        ),
                                    );
                                }
                                m_isOriginalAnkleTransformMSSet = _serde::__private::Some(
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
                    __A::pad(&mut __map, 15usize, 7usize)?;
                    let m_originalAnkleTransformMS = match m_originalAnkleTransformMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalAnkleTransformMS",
                                ),
                            );
                        }
                    };
                    let m_prevAnkleRotLS = match m_prevAnkleRotLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "prevAnkleRotLS",
                                ),
                            );
                        }
                    };
                    let m_kneeAxisLS = match m_kneeAxisLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "kneeAxisLS",
                                ),
                            );
                        }
                    };
                    let m_footEndLS = match m_footEndLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footEndLS",
                                ),
                            );
                        }
                    };
                    let m_ungroundedEvent = match m_ungroundedEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ungroundedEvent",
                                ),
                            );
                        }
                    };
                    let m_footPlantedAnkleHeightMS = match m_footPlantedAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footPlantedAnkleHeightMS",
                                ),
                            );
                        }
                    };
                    let m_footRaisedAnkleHeightMS = match m_footRaisedAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footRaisedAnkleHeightMS",
                                ),
                            );
                        }
                    };
                    let m_maxAnkleHeightMS = match m_maxAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAnkleHeightMS",
                                ),
                            );
                        }
                    };
                    let m_minAnkleHeightMS = match m_minAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minAnkleHeightMS",
                                ),
                            );
                        }
                    };
                    let m_maxKneeAngleDegrees = match m_maxKneeAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxKneeAngleDegrees",
                                ),
                            );
                        }
                    };
                    let m_minKneeAngleDegrees = match m_minKneeAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minKneeAngleDegrees",
                                ),
                            );
                        }
                    };
                    let m_verticalError = match m_verticalError {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalError",
                                ),
                            );
                        }
                    };
                    let m_maxAnkleAngleDegrees = match m_maxAnkleAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAnkleAngleDegrees",
                                ),
                            );
                        }
                    };
                    let m_hipIndex = match m_hipIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("hipIndex"),
                            );
                        }
                    };
                    let m_kneeIndex = match m_kneeIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "kneeIndex",
                                ),
                            );
                        }
                    };
                    let m_ankleIndex = match m_ankleIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ankleIndex",
                                ),
                            );
                        }
                    };
                    let m_hitSomething = match m_hitSomething {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "hitSomething",
                                ),
                            );
                        }
                    };
                    let m_isPlantedMS = match m_isPlantedMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isPlantedMS",
                                ),
                            );
                        }
                    };
                    let m_isOriginalAnkleTransformMSSet = match m_isOriginalAnkleTransformMSSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isOriginalAnkleTransformMSSet",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbFootIkModifierLeg {
                        __ptr,
                        m_originalAnkleTransformMS,
                        m_prevAnkleRotLS,
                        m_kneeAxisLS,
                        m_footEndLS,
                        m_ungroundedEvent,
                        m_footPlantedAnkleHeightMS,
                        m_footRaisedAnkleHeightMS,
                        m_maxAnkleHeightMS,
                        m_minAnkleHeightMS,
                        m_maxKneeAngleDegrees,
                        m_minKneeAngleDegrees,
                        m_verticalError,
                        m_maxAnkleAngleDegrees,
                        m_hipIndex,
                        m_kneeIndex,
                        m_ankleIndex,
                        m_hitSomething,
                        m_isPlantedMS,
                        m_isOriginalAnkleTransformMSSet,
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
                    let mut m_originalAnkleTransformMS: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_kneeAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_footEndLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ungroundedEvent: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_footPlantedAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_footRaisedAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minAnkleHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxKneeAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minKneeAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_verticalError: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAnkleAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_hipIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_kneeIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_ankleIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_hitSomething: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isPlantedMS: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isOriginalAnkleTransformMSSet: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_originalAnkleTransformMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_originalAnkleTransformMS,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalAnkleTransformMS",
                                        ),
                                    );
                                }
                                m_originalAnkleTransformMS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_kneeAxisLS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_kneeAxisLS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "kneeAxisLS",
                                        ),
                                    );
                                }
                                m_kneeAxisLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_footEndLS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_footEndLS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footEndLS",
                                        ),
                                    );
                                }
                                m_footEndLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ungroundedEvent => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ungroundedEvent) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ungroundedEvent",
                                        ),
                                    );
                                }
                                m_ungroundedEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_footPlantedAnkleHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_footPlantedAnkleHeightMS,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footPlantedAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_footPlantedAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_footRaisedAnkleHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_footRaisedAnkleHeightMS,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footRaisedAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_footRaisedAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxAnkleHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxAnkleHeightMS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_maxAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minAnkleHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_minAnkleHeightMS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minAnkleHeightMS",
                                        ),
                                    );
                                }
                                m_minAnkleHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxKneeAngleDegrees => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxKneeAngleDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxKneeAngleDegrees",
                                        ),
                                    );
                                }
                                m_maxKneeAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minKneeAngleDegrees => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minKneeAngleDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minKneeAngleDegrees",
                                        ),
                                    );
                                }
                                m_minKneeAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_verticalError => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_verticalError) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalError",
                                        ),
                                    );
                                }
                                m_verticalError = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxAnkleAngleDegrees => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxAnkleAngleDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAnkleAngleDegrees",
                                        ),
                                    );
                                }
                                m_maxAnkleAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_hipIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_hipIndex) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hipIndex",
                                        ),
                                    );
                                }
                                m_hipIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_kneeIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_kneeIndex) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "kneeIndex",
                                        ),
                                    );
                                }
                                m_kneeIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ankleIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ankleIndex) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ankleIndex",
                                        ),
                                    );
                                }
                                m_ankleIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_hitSomething => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_hitSomething) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hitSomething",
                                        ),
                                    );
                                }
                                m_hitSomething = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isPlantedMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isPlantedMS) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isPlantedMS",
                                        ),
                                    );
                                }
                                m_isPlantedMS = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isOriginalAnkleTransformMSSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_isOriginalAnkleTransformMSSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isOriginalAnkleTransformMSSet",
                                        ),
                                    );
                                }
                                m_isOriginalAnkleTransformMSSet = _serde::__private::Some(
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
                    let m_originalAnkleTransformMS = match m_originalAnkleTransformMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalAnkleTransformMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_kneeAxisLS = match m_kneeAxisLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "kneeAxisLS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_footEndLS = match m_footEndLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footEndLS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ungroundedEvent = match m_ungroundedEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ungroundedEvent",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_footPlantedAnkleHeightMS = match m_footPlantedAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footPlantedAnkleHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_footRaisedAnkleHeightMS = match m_footRaisedAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footRaisedAnkleHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxAnkleHeightMS = match m_maxAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAnkleHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minAnkleHeightMS = match m_minAnkleHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minAnkleHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxKneeAngleDegrees = match m_maxKneeAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxKneeAngleDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minKneeAngleDegrees = match m_minKneeAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minKneeAngleDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_verticalError = match m_verticalError {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalError",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxAnkleAngleDegrees = match m_maxAnkleAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAnkleAngleDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_hipIndex = match m_hipIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("hipIndex"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_kneeIndex = match m_kneeIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "kneeIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ankleIndex = match m_ankleIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ankleIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_hitSomething = match m_hitSomething {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "hitSomething",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isPlantedMS = match m_isPlantedMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isPlantedMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isOriginalAnkleTransformMSSet = match m_isOriginalAnkleTransformMSSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isOriginalAnkleTransformMSSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbFootIkModifierLeg {
                        __ptr,
                        m_originalAnkleTransformMS,
                        m_kneeAxisLS,
                        m_footEndLS,
                        m_ungroundedEvent,
                        m_footPlantedAnkleHeightMS,
                        m_footRaisedAnkleHeightMS,
                        m_maxAnkleHeightMS,
                        m_minAnkleHeightMS,
                        m_maxKneeAngleDegrees,
                        m_minKneeAngleDegrees,
                        m_verticalError,
                        m_maxAnkleAngleDegrees,
                        m_hipIndex,
                        m_kneeIndex,
                        m_ankleIndex,
                        m_hitSomething,
                        m_isPlantedMS,
                        m_isOriginalAnkleTransformMSSet,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "originalAnkleTransformMS",
                "prevAnkleRotLS",
                "kneeAxisLS",
                "footEndLS",
                "ungroundedEvent",
                "footPlantedAnkleHeightMS",
                "footRaisedAnkleHeightMS",
                "maxAnkleHeightMS",
                "minAnkleHeightMS",
                "maxKneeAngleDegrees",
                "minKneeAngleDegrees",
                "verticalError",
                "maxAnkleAngleDegrees",
                "hipIndex",
                "kneeIndex",
                "ankleIndex",
                "hitSomething",
                "isPlantedMS",
                "isOriginalAnkleTransformMSSet",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbFootIkModifierLeg",
                FIELDS,
                __hkbFootIkModifierLegVisitor {
                    marker: _serde::__private::PhantomData::<hkbFootIkModifierLeg>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
