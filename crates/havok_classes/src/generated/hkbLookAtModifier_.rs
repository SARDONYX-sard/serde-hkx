use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbLookAtModifier`
/// - version: `3`
/// - signature: `0x3d28e066`
/// - size: `208`(x86)/`240`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// - name: `targetWS`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetWS"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetWS"))]
    pub m_targetWS: Vector4,
    /// # C++ Info
    /// - name: `headForwardLS`(ctype: `hkVector4`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "headForwardLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "headForwardLS"))]
    pub m_headForwardLS: Vector4,
    /// # C++ Info
    /// - name: `neckForwardLS`(ctype: `hkVector4`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "neckForwardLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "neckForwardLS"))]
    pub m_neckForwardLS: Vector4,
    /// # C++ Info
    /// - name: `neckRightLS`(ctype: `hkVector4`)
    /// - offset: ` 96`(x86)/`128`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "neckRightLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "neckRightLS"))]
    pub m_neckRightLS: Vector4,
    /// # C++ Info
    /// - name: `eyePositionHS`(ctype: `hkVector4`)
    /// - offset: `112`(x86)/`144`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "eyePositionHS"))]
    #[cfg_attr(feature = "serde", serde(rename = "eyePositionHS"))]
    pub m_eyePositionHS: Vector4,
    /// # C++ Info
    /// - name: `newTargetGain`(ctype: `hkReal`)
    /// - offset: `128`(x86)/`160`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "newTargetGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "newTargetGain"))]
    pub m_newTargetGain: f32,
    /// # C++ Info
    /// - name: `onGain`(ctype: `hkReal`)
    /// - offset: `132`(x86)/`164`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "onGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "onGain"))]
    pub m_onGain: f32,
    /// # C++ Info
    /// - name: `offGain`(ctype: `hkReal`)
    /// - offset: `136`(x86)/`168`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "offGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "offGain"))]
    pub m_offGain: f32,
    /// # C++ Info
    /// - name: `limitAngleDegrees`(ctype: `hkReal`)
    /// - offset: `140`(x86)/`172`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleDegrees"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleDegrees"))]
    pub m_limitAngleDegrees: f32,
    /// # C++ Info
    /// - name: `limitAngleLeft`(ctype: `hkReal`)
    /// - offset: `144`(x86)/`176`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleLeft"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleLeft"))]
    pub m_limitAngleLeft: f32,
    /// # C++ Info
    /// - name: `limitAngleRight`(ctype: `hkReal`)
    /// - offset: `148`(x86)/`180`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleRight"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleRight"))]
    pub m_limitAngleRight: f32,
    /// # C++ Info
    /// - name: `limitAngleUp`(ctype: `hkReal`)
    /// - offset: `152`(x86)/`184`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleUp"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleUp"))]
    pub m_limitAngleUp: f32,
    /// # C++ Info
    /// - name: `limitAngleDown`(ctype: `hkReal`)
    /// - offset: `156`(x86)/`188`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleDown"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleDown"))]
    pub m_limitAngleDown: f32,
    /// # C++ Info
    /// - name: `headIndex`(ctype: `hkInt16`)
    /// - offset: `160`(x86)/`192`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "headIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "headIndex"))]
    pub m_headIndex: i16,
    /// # C++ Info
    /// - name: `neckIndex`(ctype: `hkInt16`)
    /// - offset: `162`(x86)/`194`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "neckIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "neckIndex"))]
    pub m_neckIndex: i16,
    /// # C++ Info
    /// - name: `isOn`(ctype: `hkBool`)
    /// - offset: `164`(x86)/`196`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "isOn"))]
    #[cfg_attr(feature = "serde", serde(rename = "isOn"))]
    pub m_isOn: bool,
    /// # C++ Info
    /// - name: `individualLimitsOn`(ctype: `hkBool`)
    /// - offset: `165`(x86)/`197`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "individualLimitsOn"))]
    #[cfg_attr(feature = "serde", serde(rename = "individualLimitsOn"))]
    pub m_individualLimitsOn: bool,
    /// # C++ Info
    /// - name: `isTargetInsideLimitCone`(ctype: `hkBool`)
    /// - offset: `166`(x86)/`198`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "isTargetInsideLimitCone"))]
    #[cfg_attr(feature = "serde", serde(rename = "isTargetInsideLimitCone"))]
    pub m_isTargetInsideLimitCone: bool,
    /// # C++ Info
    /// - name: `lookAtLastTargetWS`(ctype: `hkVector4`)
    /// - offset: `176`(x86)/`208`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtLastTargetWS"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtLastTargetWS"))]
    pub m_lookAtLastTargetWS: Vector4,
    /// # C++ Info
    /// - name: `lookAtWeight`(ctype: `hkReal`)
    /// - offset: `192`(x86)/`224`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtWeight"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtWeight"))]
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
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v
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
                .serialize_struct("hkbLookAtModifier", class_meta, (208u64, 240u64))?;
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
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer
                .skip_fixed_array_field(
                    "padModifier",
                    self.parent.m_padModifier.as_slice(),
                    TypeSize::NonPtr,
                )?;
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
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbLookAtModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_enable,
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
                        "enable" => Ok(__Field::m_enable),
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
                        "isTargetInsideLimitCone" => {
                            Ok(__Field::m_isTargetInsideLimitCone)
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
            struct __hkbLookAtModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbLookAtModifier<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbLookAtModifierVisitor<'de> {
                type Value = hkbLookAtModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbLookAtModifier",
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
                                __A::pad(&mut __map, 4usize, 0usize)?;
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
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegrees,
                                ) {
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
                                if _serde::__private::Option::is_some(
                                    &m_individualLimitsOn,
                                ) {
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
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_lookAtLastTargetWS,
                                ) {
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "headForwardLS",
                                ),
                            );
                        }
                    };
                    let m_neckForwardLS = match m_neckForwardLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "neckForwardLS",
                                ),
                            );
                        }
                    };
                    let m_neckRightLS = match m_neckRightLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "neckRightLS",
                                ),
                            );
                        }
                    };
                    let m_eyePositionHS = match m_eyePositionHS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eyePositionHS",
                                ),
                            );
                        }
                    };
                    let m_newTargetGain = match m_newTargetGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "newTargetGain",
                                ),
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegrees",
                                ),
                            );
                        }
                    };
                    let m_limitAngleLeft = match m_limitAngleLeft {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleLeft",
                                ),
                            );
                        }
                    };
                    let m_limitAngleRight = match m_limitAngleRight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleRight",
                                ),
                            );
                        }
                    };
                    let m_limitAngleUp = match m_limitAngleUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleUp",
                                ),
                            );
                        }
                    };
                    let m_limitAngleDown = match m_limitAngleDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDown",
                                ),
                            );
                        }
                    };
                    let m_headIndex = match m_headIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "headIndex",
                                ),
                            );
                        }
                    };
                    let m_neckIndex = match m_neckIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "neckIndex",
                                ),
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtWeight",
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
                        m_lookAtLastTargetWS,
                        m_lookAtWeight,
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
                    let mut m_enable: _serde::__private::Option<bool> = _serde::__private::None;
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
                            __Field::m_enable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_enable) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("enable"),
                                    );
                                }
                                m_enable = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_targetWS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_targetWS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_headForwardLS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_neckForwardLS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_neckRightLS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eyePositionHS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_newTargetGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_onGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_offGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_limitAngleLeft) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_limitAngleRight) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_limitAngleUp) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_limitAngleDown) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_headIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_neckIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isOn) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_individualLimitsOn,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_isTargetInsideLimitCone,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                    let m_enable = match m_enable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enable"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_targetWS = match m_targetWS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("targetWS"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_headForwardLS = match m_headForwardLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "headForwardLS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_neckForwardLS = match m_neckForwardLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "neckForwardLS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_neckRightLS = match m_neckRightLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "neckRightLS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_eyePositionHS = match m_eyePositionHS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eyePositionHS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_newTargetGain = match m_newTargetGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "newTargetGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_onGain = match m_onGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("onGain"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_offGain = match m_offGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("offGain"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleDegrees = match m_limitAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleLeft = match m_limitAngleLeft {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleLeft",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleRight = match m_limitAngleRight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleRight",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleUp = match m_limitAngleUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleUp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleDown = match m_limitAngleDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDown",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_headIndex = match m_headIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "headIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_neckIndex = match m_neckIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "neckIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isOn = match m_isOn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isOn"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_individualLimitsOn = match m_individualLimitsOn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "individualLimitsOn",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isTargetInsideLimitCone = match m_isTargetInsideLimitCone {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isTargetInsideLimitCone",
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
                    let parent = hkbModifier {
                        __ptr,
                        parent,
                        m_enable,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
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
