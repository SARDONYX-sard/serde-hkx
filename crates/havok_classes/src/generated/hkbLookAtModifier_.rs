use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbLookAtModifier`
/// -         version: `3`
/// -       signature: `0x3d28e066`
/// -          size: 208(x86)/240(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbLookAtModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `targetWS`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_targetWS: Vector4,
    /// # C++ Info
    /// -          name: `headForwardLS`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_headForwardLS: Vector4,
    /// # C++ Info
    /// -          name: `neckForwardLS`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_neckForwardLS: Vector4,
    /// # C++ Info
    /// -          name: `neckRightLS`(ctype: `hkVector4`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_neckRightLS: Vector4,
    /// # C++ Info
    /// -          name: `eyePositionHS`(ctype: `hkVector4`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_eyePositionHS: Vector4,
    /// # C++ Info
    /// -          name: `newTargetGain`(ctype: `hkReal`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_newTargetGain: f32,
    /// # C++ Info
    /// -          name: `onGain`(ctype: `hkReal`)
    /// -        offset: 132(x86)/164(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_onGain: f32,
    /// # C++ Info
    /// -          name: `offGain`(ctype: `hkReal`)
    /// -        offset: 136(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offGain: f32,
    /// # C++ Info
    /// -          name: `limitAngleDegrees`(ctype: `hkReal`)
    /// -        offset: 140(x86)/172(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `limitAngleLeft`(ctype: `hkReal`)
    /// -        offset: 144(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleLeft: f32,
    /// # C++ Info
    /// -          name: `limitAngleRight`(ctype: `hkReal`)
    /// -        offset: 148(x86)/180(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleRight: f32,
    /// # C++ Info
    /// -          name: `limitAngleUp`(ctype: `hkReal`)
    /// -        offset: 152(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleUp: f32,
    /// # C++ Info
    /// -          name: `limitAngleDown`(ctype: `hkReal`)
    /// -        offset: 156(x86)/188(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleDown: f32,
    /// # C++ Info
    /// -          name: `headIndex`(ctype: `hkInt16`)
    /// -        offset: 160(x86)/192(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_headIndex: i16,
    /// # C++ Info
    /// -          name: `neckIndex`(ctype: `hkInt16`)
    /// -        offset: 162(x86)/194(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_neckIndex: i16,
    /// # C++ Info
    /// -          name: `isOn`(ctype: `hkBool`)
    /// -        offset: 164(x86)/196(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isOn: bool,
    /// # C++ Info
    /// -          name: `individualLimitsOn`(ctype: `hkBool`)
    /// -        offset: 165(x86)/197(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_individualLimitsOn: bool,
    /// # C++ Info
    /// -          name: `isTargetInsideLimitCone`(ctype: `hkBool`)
    /// -        offset: 166(x86)/198(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isTargetInsideLimitCone: bool,
    /// # C++ Info
    /// -          name: `lookAtLastTargetWS`(ctype: `hkVector4`)
    /// -        offset: 176(x86)/208(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_lookAtLastTargetWS: Vector4,
    /// # C++ Info
    /// -          name: `lookAtWeight`(ctype: `hkReal`)
    /// -        offset: 192(x86)/224(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_lookAtWeight: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbLookAtModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbLookAtModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3d28e066)
        }
    }
    impl<'a> _serde::Serialize for hkbLookAtModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3d28e066)));
            let mut serializer = __serializer
                .serialize_struct("hkbLookAtModifier", class_meta)?;
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
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("targetWS", &self.m_targetWS)?;
            serializer.serialize_field("headForwardLS", &self.m_headForwardLS)?;
            serializer.serialize_field("neckForwardLS", &self.m_neckForwardLS)?;
            serializer.serialize_field("neckRightLS", &self.m_neckRightLS)?;
            serializer.serialize_field("eyePositionHS", &self.m_eyePositionHS)?;
            serializer.serialize_field("newTargetGain", &self.m_newTargetGain)?;
            serializer.serialize_field("onGain", &self.m_onGain)?;
            serializer.serialize_field("offGain", &self.m_offGain)?;
            serializer.serialize_field("limitAngleDegrees", &self.m_limitAngleDegrees)?;
            serializer.serialize_field("limitAngleLeft", &self.m_limitAngleLeft)?;
            serializer.serialize_field("limitAngleRight", &self.m_limitAngleRight)?;
            serializer.serialize_field("limitAngleUp", &self.m_limitAngleUp)?;
            serializer.serialize_field("limitAngleDown", &self.m_limitAngleDown)?;
            serializer.serialize_field("headIndex", &self.m_headIndex)?;
            serializer.serialize_field("neckIndex", &self.m_neckIndex)?;
            serializer.serialize_field("isOn", &self.m_isOn)?;
            serializer
                .serialize_field("individualLimitsOn", &self.m_individualLimitsOn)?;
            serializer
                .serialize_field(
                    "isTargetInsideLimitCone",
                    &self.m_isTargetInsideLimitCone,
                )?;
            serializer.pad_field([0u8; 9usize].as_slice(), [0u8; 9usize].as_slice())?;
            serializer.skip_field("lookAtLastTargetWS", &self.m_lookAtLastTargetWS)?;
            serializer.skip_field("lookAtWeight", &self.m_lookAtWeight)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_targetWS,
    m_headForwardLS,
    m_neckForwardLS,
    m_neckRightLS,
    m_eyePositionHS,
    m_newTargetGain,
    m_onGain,
    m_offGain,
    m_limitAngleDegrees,
    m_limitAngleLeft,
    m_limitAngleRight,
    m_limitAngleUp,
    m_limitAngleDown,
    m_headIndex,
    m_neckIndex,
    m_isOn,
    m_individualLimitsOn,
    m_isTargetInsideLimitCone,
    m_lookAtLastTargetWS,
    m_lookAtWeight,
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
            "targetWS" => Ok(__Field::m_targetWS),
            "headForwardLS" => Ok(__Field::m_headForwardLS),
            "neckForwardLS" => Ok(__Field::m_neckForwardLS),
            "neckRightLS" => Ok(__Field::m_neckRightLS),
            "eyePositionHS" => Ok(__Field::m_eyePositionHS),
            "newTargetGain" => Ok(__Field::m_newTargetGain),
            "onGain" => Ok(__Field::m_onGain),
            "offGain" => Ok(__Field::m_offGain),
            "limitAngleDegrees" => Ok(__Field::m_limitAngleDegrees),
            "limitAngleLeft" => Ok(__Field::m_limitAngleLeft),
            "limitAngleRight" => Ok(__Field::m_limitAngleRight),
            "limitAngleUp" => Ok(__Field::m_limitAngleUp),
            "limitAngleDown" => Ok(__Field::m_limitAngleDown),
            "headIndex" => Ok(__Field::m_headIndex),
            "neckIndex" => Ok(__Field::m_neckIndex),
            "isOn" => Ok(__Field::m_isOn),
            "individualLimitsOn" => Ok(__Field::m_individualLimitsOn),
            "isTargetInsideLimitCone" => Ok(__Field::m_isTargetInsideLimitCone),
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
pub(super) struct __hkbLookAtModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbLookAtModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbLookAtModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbLookAtModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbLookAtModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbLookAtModifierVisitor<'de> {
    type Value = hkbLookAtModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbLookAtModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_targetWS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_headForwardLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_neckForwardLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_neckRightLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_eyePositionHS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_newTargetGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleLeft: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleRight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleUp: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleDown: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_headIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_neckIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_isOn: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_individualLimitsOn: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isTargetInsideLimitCone: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_lookAtLastTargetWS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_lookAtWeight: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..20usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_targetWS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "targetWS",
                            ),
                        );
                    }
                    m_targetWS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_headForwardLS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "headForwardLS",
                            ),
                        );
                    }
                    m_headForwardLS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_neckForwardLS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "neckForwardLS",
                            ),
                        );
                    }
                    m_neckForwardLS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_neckRightLS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "neckRightLS",
                            ),
                        );
                    }
                    m_neckRightLS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_eyePositionHS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "eyePositionHS",
                            ),
                        );
                    }
                    m_eyePositionHS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_newTargetGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "newTargetGain",
                            ),
                        );
                    }
                    m_newTargetGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_onGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("onGain"),
                        );
                    }
                    m_onGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_offGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("offGain"),
                        );
                    }
                    m_offGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_limitAngleDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitAngleDegrees",
                            ),
                        );
                    }
                    m_limitAngleDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_limitAngleLeft) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitAngleLeft",
                            ),
                        );
                    }
                    m_limitAngleLeft = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_limitAngleRight) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitAngleRight",
                            ),
                        );
                    }
                    m_limitAngleRight = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_limitAngleUp) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitAngleUp",
                            ),
                        );
                    }
                    m_limitAngleUp = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_limitAngleDown) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitAngleDown",
                            ),
                        );
                    }
                    m_limitAngleDown = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_headIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "headIndex",
                            ),
                        );
                    }
                    m_headIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                14usize => {
                    if _serde::__private::Option::is_some(&m_neckIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "neckIndex",
                            ),
                        );
                    }
                    m_neckIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_isOn) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("isOn"),
                        );
                    }
                    m_isOn = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                16usize => {
                    if _serde::__private::Option::is_some(&m_individualLimitsOn) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "individualLimitsOn",
                            ),
                        );
                    }
                    m_individualLimitsOn = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                17usize => {
                    if _serde::__private::Option::is_some(&m_isTargetInsideLimitCone) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isTargetInsideLimitCone",
                            ),
                        );
                    }
                    m_isTargetInsideLimitCone = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                18usize => {
                    if _serde::__private::Option::is_some(&m_lookAtLastTargetWS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "lookAtLastTargetWS",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 9usize, 9usize)?;
                    m_lookAtLastTargetWS = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                19usize => {
                    if _serde::__private::Option::is_some(&m_lookAtWeight) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "lookAtWeight",
                            ),
                        );
                    }
                    m_lookAtWeight = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
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
        __A::pad(&mut __map, 12usize, 12usize)?;
        let m_targetWS = match m_targetWS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetWS"),
                );
            }
        };
        let m_headForwardLS = match m_headForwardLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("headForwardLS"),
                );
            }
        };
        let m_neckForwardLS = match m_neckForwardLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("neckForwardLS"),
                );
            }
        };
        let m_neckRightLS = match m_neckRightLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("neckRightLS"),
                );
            }
        };
        let m_eyePositionHS = match m_eyePositionHS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("eyePositionHS"),
                );
            }
        };
        let m_newTargetGain = match m_newTargetGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("newTargetGain"),
                );
            }
        };
        let m_onGain = match m_onGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onGain"),
                );
            }
        };
        let m_offGain = match m_offGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offGain"),
                );
            }
        };
        let m_limitAngleDegrees = match m_limitAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDegrees"),
                );
            }
        };
        let m_limitAngleLeft = match m_limitAngleLeft {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleLeft"),
                );
            }
        };
        let m_limitAngleRight = match m_limitAngleRight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleRight"),
                );
            }
        };
        let m_limitAngleUp = match m_limitAngleUp {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleUp"),
                );
            }
        };
        let m_limitAngleDown = match m_limitAngleDown {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDown"),
                );
            }
        };
        let m_headIndex = match m_headIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("headIndex"),
                );
            }
        };
        let m_neckIndex = match m_neckIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("neckIndex"),
                );
            }
        };
        let m_isOn = match m_isOn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isOn"),
                );
            }
        };
        let m_individualLimitsOn = match m_individualLimitsOn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "individualLimitsOn",
                    ),
                );
            }
        };
        let m_isTargetInsideLimitCone = match m_isTargetInsideLimitCone {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "isTargetInsideLimitCone",
                    ),
                );
            }
        };
        let m_lookAtLastTargetWS = match m_lookAtLastTargetWS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "lookAtLastTargetWS",
                    ),
                );
            }
        };
        let m_lookAtWeight = match m_lookAtWeight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lookAtWeight"),
                );
            }
        };
        _serde::__private::Ok(hkbLookAtModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_targetWS,
            m_headForwardLS,
            m_neckForwardLS,
            m_neckRightLS,
            m_eyePositionHS,
            m_newTargetGain,
            m_onGain,
            m_offGain,
            m_limitAngleDegrees,
            m_limitAngleLeft,
            m_limitAngleRight,
            m_limitAngleUp,
            m_limitAngleDown,
            m_headIndex,
            m_neckIndex,
            m_isOn,
            m_individualLimitsOn,
            m_isTargetInsideLimitCone,
            m_lookAtLastTargetWS,
            m_lookAtWeight,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_targetWS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_headForwardLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_neckForwardLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_neckRightLS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_eyePositionHS: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_newTargetGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleLeft: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleRight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleUp: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleDown: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_headIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_neckIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_isOn: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_individualLimitsOn: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isTargetInsideLimitCone: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..18usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_targetWS => {
                        if _serde::__private::Option::is_some(&m_targetWS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "targetWS",
                                ),
                            );
                        }
                        m_targetWS = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_headForwardLS => {
                        if _serde::__private::Option::is_some(&m_headForwardLS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "headForwardLS",
                                ),
                            );
                        }
                        m_headForwardLS = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_neckForwardLS => {
                        if _serde::__private::Option::is_some(&m_neckForwardLS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "neckForwardLS",
                                ),
                            );
                        }
                        m_neckForwardLS = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_neckRightLS => {
                        if _serde::__private::Option::is_some(&m_neckRightLS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "neckRightLS",
                                ),
                            );
                        }
                        m_neckRightLS = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_eyePositionHS => {
                        if _serde::__private::Option::is_some(&m_eyePositionHS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "eyePositionHS",
                                ),
                            );
                        }
                        m_eyePositionHS = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_newTargetGain => {
                        if _serde::__private::Option::is_some(&m_newTargetGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "newTargetGain",
                                ),
                            );
                        }
                        m_newTargetGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_onGain => {
                        if _serde::__private::Option::is_some(&m_onGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("onGain"),
                            );
                        }
                        m_onGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_offGain => {
                        if _serde::__private::Option::is_some(&m_offGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "offGain",
                                ),
                            );
                        }
                        m_offGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_limitAngleDegrees => {
                        if _serde::__private::Option::is_some(&m_limitAngleDegrees) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "limitAngleDegrees",
                                ),
                            );
                        }
                        m_limitAngleDegrees = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_limitAngleLeft => {
                        if _serde::__private::Option::is_some(&m_limitAngleLeft) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "limitAngleLeft",
                                ),
                            );
                        }
                        m_limitAngleLeft = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_limitAngleRight => {
                        if _serde::__private::Option::is_some(&m_limitAngleRight) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "limitAngleRight",
                                ),
                            );
                        }
                        m_limitAngleRight = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_limitAngleUp => {
                        if _serde::__private::Option::is_some(&m_limitAngleUp) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "limitAngleUp",
                                ),
                            );
                        }
                        m_limitAngleUp = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_limitAngleDown => {
                        if _serde::__private::Option::is_some(&m_limitAngleDown) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "limitAngleDown",
                                ),
                            );
                        }
                        m_limitAngleDown = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_headIndex => {
                        if _serde::__private::Option::is_some(&m_headIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "headIndex",
                                ),
                            );
                        }
                        m_headIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_neckIndex => {
                        if _serde::__private::Option::is_some(&m_neckIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "neckIndex",
                                ),
                            );
                        }
                        m_neckIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_isOn => {
                        if _serde::__private::Option::is_some(&m_isOn) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("isOn"),
                            );
                        }
                        m_isOn = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_individualLimitsOn => {
                        if _serde::__private::Option::is_some(&m_individualLimitsOn) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "individualLimitsOn",
                                ),
                            );
                        }
                        m_individualLimitsOn = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_isTargetInsideLimitCone => {
                        if _serde::__private::Option::is_some(
                            &m_isTargetInsideLimitCone,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isTargetInsideLimitCone",
                                ),
                            );
                        }
                        m_isTargetInsideLimitCone = _serde::__private::Some(
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
        }
        let m_targetWS = match m_targetWS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetWS"),
                );
            }
        };
        let m_headForwardLS = match m_headForwardLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("headForwardLS"),
                );
            }
        };
        let m_neckForwardLS = match m_neckForwardLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("neckForwardLS"),
                );
            }
        };
        let m_neckRightLS = match m_neckRightLS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("neckRightLS"),
                );
            }
        };
        let m_eyePositionHS = match m_eyePositionHS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("eyePositionHS"),
                );
            }
        };
        let m_newTargetGain = match m_newTargetGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("newTargetGain"),
                );
            }
        };
        let m_onGain = match m_onGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onGain"),
                );
            }
        };
        let m_offGain = match m_offGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offGain"),
                );
            }
        };
        let m_limitAngleDegrees = match m_limitAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDegrees"),
                );
            }
        };
        let m_limitAngleLeft = match m_limitAngleLeft {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleLeft"),
                );
            }
        };
        let m_limitAngleRight = match m_limitAngleRight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleRight"),
                );
            }
        };
        let m_limitAngleUp = match m_limitAngleUp {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleUp"),
                );
            }
        };
        let m_limitAngleDown = match m_limitAngleDown {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDown"),
                );
            }
        };
        let m_headIndex = match m_headIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("headIndex"),
                );
            }
        };
        let m_neckIndex = match m_neckIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("neckIndex"),
                );
            }
        };
        let m_isOn = match m_isOn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isOn"),
                );
            }
        };
        let m_individualLimitsOn = match m_individualLimitsOn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "individualLimitsOn",
                    ),
                );
            }
        };
        let m_isTargetInsideLimitCone = match m_isTargetInsideLimitCone {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "isTargetInsideLimitCone",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbLookAtModifier {
            __ptr,
            parent,
            m_targetWS,
            m_headForwardLS,
            m_neckForwardLS,
            m_neckRightLS,
            m_eyePositionHS,
            m_newTargetGain,
            m_onGain,
            m_offGain,
            m_limitAngleDegrees,
            m_limitAngleLeft,
            m_limitAngleRight,
            m_limitAngleUp,
            m_limitAngleDown,
            m_headIndex,
            m_neckIndex,
            m_isOn,
            m_individualLimitsOn,
            m_isTargetInsideLimitCone,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbLookAtModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "targetWS",
                "headForwardLS",
                "neckForwardLS",
                "neckRightLS",
                "eyePositionHS",
                "newTargetGain",
                "onGain",
                "offGain",
                "limitAngleDegrees",
                "limitAngleLeft",
                "limitAngleRight",
                "limitAngleUp",
                "limitAngleDown",
                "headIndex",
                "neckIndex",
                "isOn",
                "individualLimitsOn",
                "isTargetInsideLimitCone",
                "lookAtLastTargetWS",
                "lookAtWeight",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbLookAtModifier",
                FIELDS,
                __hkbLookAtModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbLookAtModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
