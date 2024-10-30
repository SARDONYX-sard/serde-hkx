use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSLookAtModifier`
/// - version: `4`
/// - signature: `0xd756fc25`
/// - size: `160`(x86)/`224`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSLookAtModifier<'a> {
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
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// - name: `lookAtTarget`(ctype: `hkBool`)
    /// - offset: ` 44`(x86)/` 80`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtTarget"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtTarget"))]
    pub m_lookAtTarget: bool,
    /// # C++ Info
    /// - name: `bones`(ctype: `hkArray<struct BSLookAtModifierBoneData>`)
    /// - offset: ` 48`(x86)/` 88`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "bones"))]
    #[cfg_attr(feature = "serde", serde(rename = "bones"))]
    pub m_bones: Vec<BSLookAtModifierBoneData>,
    /// # C++ Info
    /// - name: `eyeBones`(ctype: `hkArray<struct BSLookAtModifierBoneData>`)
    /// - offset: ` 60`(x86)/`104`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "eyeBones"))]
    #[cfg_attr(feature = "serde", serde(rename = "eyeBones"))]
    pub m_eyeBones: Vec<BSLookAtModifierBoneData>,
    /// # C++ Info
    /// - name: `limitAngleDegrees`(ctype: `hkReal`)
    /// - offset: ` 72`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleDegrees"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleDegrees"))]
    pub m_limitAngleDegrees: f32,
    /// # C++ Info
    /// - name: `limitAngleThresholdDegrees`(ctype: `hkReal`)
    /// - offset: ` 76`(x86)/`124`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleThresholdDegrees"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleThresholdDegrees"))]
    pub m_limitAngleThresholdDegrees: f32,
    /// # C++ Info
    /// - name: `continueLookOutsideOfLimit`(ctype: `hkBool`)
    /// - offset: ` 80`(x86)/`128`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "continueLookOutsideOfLimit"))]
    #[cfg_attr(feature = "serde", serde(rename = "continueLookOutsideOfLimit"))]
    pub m_continueLookOutsideOfLimit: bool,
    /// # C++ Info
    /// - name: `onGain`(ctype: `hkReal`)
    /// - offset: ` 84`(x86)/`132`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "onGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "onGain"))]
    pub m_onGain: f32,
    /// # C++ Info
    /// - name: `offGain`(ctype: `hkReal`)
    /// - offset: ` 88`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "offGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "offGain"))]
    pub m_offGain: f32,
    /// # C++ Info
    /// - name: `useBoneGains`(ctype: `hkBool`)
    /// - offset: ` 92`(x86)/`140`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "useBoneGains"))]
    #[cfg_attr(feature = "serde", serde(rename = "useBoneGains"))]
    pub m_useBoneGains: bool,
    /// # C++ Info
    /// - name: `targetLocation`(ctype: `hkVector4`)
    /// - offset: ` 96`(x86)/`144`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetLocation"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetLocation"))]
    pub m_targetLocation: Vector4,
    /// # C++ Info
    /// - name: `targetOutsideLimits`(ctype: `hkBool`)
    /// - offset: `112`(x86)/`160`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetOutsideLimits"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetOutsideLimits"))]
    pub m_targetOutsideLimits: bool,
    /// # C++ Info
    /// - name: `targetOutOfLimitEvent`(ctype: `struct hkbEventProperty`)
    /// - offset: `116`(x86)/`168`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetOutOfLimitEvent"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetOutOfLimitEvent"))]
    pub m_targetOutOfLimitEvent: hkbEventProperty,
    /// # C++ Info
    /// - name: `lookAtCamera`(ctype: `hkBool`)
    /// - offset: `124`(x86)/`184`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtCamera"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtCamera"))]
    pub m_lookAtCamera: bool,
    /// # C++ Info
    /// - name: `lookAtCameraX`(ctype: `hkReal`)
    /// - offset: `128`(x86)/`188`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtCameraX"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtCameraX"))]
    pub m_lookAtCameraX: f32,
    /// # C++ Info
    /// - name: `lookAtCameraY`(ctype: `hkReal`)
    /// - offset: `132`(x86)/`192`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtCameraY"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtCameraY"))]
    pub m_lookAtCameraY: f32,
    /// # C++ Info
    /// - name: `lookAtCameraZ`(ctype: `hkReal`)
    /// - offset: `136`(x86)/`196`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtCameraZ"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtCameraZ"))]
    pub m_lookAtCameraZ: f32,
    /// # C++ Info
    /// - name: `timeStep`(ctype: `hkReal`)
    /// - offset: `140`(x86)/`200`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeStep"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeStep"))]
    pub m_timeStep: f32,
    /// # C++ Info
    /// - name: `ballBonesValid`(ctype: `hkBool`)
    /// - offset: `144`(x86)/`204`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "ballBonesValid"))]
    #[cfg_attr(feature = "serde", serde(rename = "ballBonesValid"))]
    pub m_ballBonesValid: bool,
    /// # C++ Info
    /// - name: `pSkeletonMemory`(ctype: `void*`)
    /// - offset: `148`(x86)/`208`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "pSkeletonMemory"))]
    #[cfg_attr(feature = "serde", serde(rename = "pSkeletonMemory"))]
    pub m_pSkeletonMemory: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSLookAtModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSLookAtModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd756fc25)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v.extend(
                self
                    .m_bones
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_eyeBones
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(self.m_targetOutOfLimitEvent.deps_indexes());
            v.push(self.m_pSkeletonMemory.get());
            v
        }
    }
    impl<'a> _serde::Serialize for BSLookAtModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd756fc25)));
            let mut serializer = __serializer
                .serialize_struct("BSLookAtModifier", class_meta, (160u64, 224u64))?;
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
            serializer.serialize_field("lookAtTarget", &self.m_lookAtTarget)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "bones",
                    &self.m_bones,
                    TypeSize::Struct {
                        size_x86: 64u64,
                        size_x86_64: 64u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "eyeBones",
                    &self.m_eyeBones,
                    TypeSize::Struct {
                        size_x86: 64u64,
                        size_x86_64: 64u64,
                    },
                )?;
            serializer.serialize_field("limitAngleDegrees", &self.m_limitAngleDegrees)?;
            serializer
                .serialize_field(
                    "limitAngleThresholdDegrees",
                    &self.m_limitAngleThresholdDegrees,
                )?;
            serializer
                .serialize_field(
                    "continueLookOutsideOfLimit",
                    &self.m_continueLookOutsideOfLimit,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("onGain", &self.m_onGain)?;
            serializer.serialize_field("offGain", &self.m_offGain)?;
            serializer.serialize_field("useBoneGains", &self.m_useBoneGains)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("targetLocation", &self.m_targetLocation)?;
            serializer
                .serialize_field("targetOutsideLimits", &self.m_targetOutsideLimits)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field(
                    "targetOutOfLimitEvent",
                    &self.m_targetOutOfLimitEvent,
                )?;
            serializer.serialize_field("lookAtCamera", &self.m_lookAtCamera)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("lookAtCameraX", &self.m_lookAtCameraX)?;
            serializer.serialize_field("lookAtCameraY", &self.m_lookAtCameraY)?;
            serializer.serialize_field("lookAtCameraZ", &self.m_lookAtCameraZ)?;
            serializer.skip_field("timeStep", &self.m_timeStep)?;
            serializer.skip_field("ballBonesValid", &self.m_ballBonesValid)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("pSkeletonMemory", &self.m_pSkeletonMemory)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSLookAtModifier<'de> {
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
                m_lookAtTarget,
                m_bones,
                m_eyeBones,
                m_limitAngleDegrees,
                m_limitAngleThresholdDegrees,
                m_continueLookOutsideOfLimit,
                m_onGain,
                m_offGain,
                m_useBoneGains,
                m_targetLocation,
                m_targetOutsideLimits,
                m_targetOutOfLimitEvent,
                m_lookAtCamera,
                m_lookAtCameraX,
                m_lookAtCameraY,
                m_lookAtCameraZ,
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
                        "lookAtTarget" => Ok(__Field::m_lookAtTarget),
                        "bones" => Ok(__Field::m_bones),
                        "eyeBones" => Ok(__Field::m_eyeBones),
                        "limitAngleDegrees" => Ok(__Field::m_limitAngleDegrees),
                        "limitAngleThresholdDegrees" => {
                            Ok(__Field::m_limitAngleThresholdDegrees)
                        }
                        "continueLookOutsideOfLimit" => {
                            Ok(__Field::m_continueLookOutsideOfLimit)
                        }
                        "onGain" => Ok(__Field::m_onGain),
                        "offGain" => Ok(__Field::m_offGain),
                        "useBoneGains" => Ok(__Field::m_useBoneGains),
                        "targetLocation" => Ok(__Field::m_targetLocation),
                        "targetOutsideLimits" => Ok(__Field::m_targetOutsideLimits),
                        "targetOutOfLimitEvent" => Ok(__Field::m_targetOutOfLimitEvent),
                        "lookAtCamera" => Ok(__Field::m_lookAtCamera),
                        "lookAtCameraX" => Ok(__Field::m_lookAtCameraX),
                        "lookAtCameraY" => Ok(__Field::m_lookAtCameraY),
                        "lookAtCameraZ" => Ok(__Field::m_lookAtCameraZ),
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
            struct __BSLookAtModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<BSLookAtModifier<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __BSLookAtModifierVisitor<'de> {
                type Value = BSLookAtModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct BSLookAtModifier",
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
                    let mut m_lookAtTarget: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_bones: _serde::__private::Option<
                        Vec<BSLookAtModifierBoneData>,
                    > = _serde::__private::None;
                    let mut m_eyeBones: _serde::__private::Option<
                        Vec<BSLookAtModifierBoneData>,
                    > = _serde::__private::None;
                    let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleThresholdDegrees: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_continueLookOutsideOfLimit: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useBoneGains: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_targetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_targetOutsideLimits: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_targetOutOfLimitEvent: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_lookAtCamera: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_lookAtCameraX: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lookAtCameraY: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lookAtCameraZ: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_timeStep: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_ballBonesValid: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_pSkeletonMemory: _serde::__private::Option<Pointer> = _serde::__private::None;
                    for i in 0..19usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_lookAtTarget) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtTarget",
                                        ),
                                    );
                                }
                                m_lookAtTarget = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_bones) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("bones"),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 7usize)?;
                                m_bones = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<BSLookAtModifierBoneData>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_eyeBones) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eyeBones",
                                        ),
                                    );
                                }
                                m_eyeBones = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<BSLookAtModifierBoneData>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
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
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleThresholdDegrees,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleThresholdDegrees",
                                        ),
                                    );
                                }
                                m_limitAngleThresholdDegrees = _serde::__private::Some(
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
                                    &m_continueLookOutsideOfLimit,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "continueLookOutsideOfLimit",
                                        ),
                                    );
                                }
                                m_continueLookOutsideOfLimit = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                                __A::pad(&mut __map, 3usize, 3usize)?;
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
                                if _serde::__private::Option::is_some(&m_useBoneGains) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useBoneGains",
                                        ),
                                    );
                                }
                                m_useBoneGains = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_targetLocation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetLocation",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_targetLocation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_targetOutsideLimits,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetOutsideLimits",
                                        ),
                                    );
                                }
                                m_targetOutsideLimits = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_targetOutOfLimitEvent,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetOutOfLimitEvent",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 7usize)?;
                                m_targetOutOfLimitEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_lookAtCamera) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCamera",
                                        ),
                                    );
                                }
                                m_lookAtCamera = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_lookAtCameraX) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraX",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_lookAtCameraX = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_lookAtCameraY) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraY",
                                        ),
                                    );
                                }
                                m_lookAtCameraY = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_lookAtCameraZ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraZ",
                                        ),
                                    );
                                }
                                m_lookAtCameraZ = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_timeStep) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeStep",
                                        ),
                                    );
                                }
                                m_timeStep = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_ballBonesValid) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ballBonesValid",
                                        ),
                                    );
                                }
                                m_ballBonesValid = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_pSkeletonMemory) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pSkeletonMemory",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_pSkeletonMemory = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    __A::pad(&mut __map, 8usize, 8usize)?;
                    let m_lookAtTarget = match m_lookAtTarget {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtTarget",
                                ),
                            );
                        }
                    };
                    let m_bones = match m_bones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bones"),
                            );
                        }
                    };
                    let m_eyeBones = match m_eyeBones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("eyeBones"),
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
                    let m_limitAngleThresholdDegrees = match m_limitAngleThresholdDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleThresholdDegrees",
                                ),
                            );
                        }
                    };
                    let m_continueLookOutsideOfLimit = match m_continueLookOutsideOfLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "continueLookOutsideOfLimit",
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
                    let m_useBoneGains = match m_useBoneGains {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useBoneGains",
                                ),
                            );
                        }
                    };
                    let m_targetLocation = match m_targetLocation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetLocation",
                                ),
                            );
                        }
                    };
                    let m_targetOutsideLimits = match m_targetOutsideLimits {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetOutsideLimits",
                                ),
                            );
                        }
                    };
                    let m_targetOutOfLimitEvent = match m_targetOutOfLimitEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetOutOfLimitEvent",
                                ),
                            );
                        }
                    };
                    let m_lookAtCamera = match m_lookAtCamera {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCamera",
                                ),
                            );
                        }
                    };
                    let m_lookAtCameraX = match m_lookAtCameraX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraX",
                                ),
                            );
                        }
                    };
                    let m_lookAtCameraY = match m_lookAtCameraY {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraY",
                                ),
                            );
                        }
                    };
                    let m_lookAtCameraZ = match m_lookAtCameraZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraZ",
                                ),
                            );
                        }
                    };
                    let m_timeStep = match m_timeStep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("timeStep"),
                            );
                        }
                    };
                    let m_ballBonesValid = match m_ballBonesValid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ballBonesValid",
                                ),
                            );
                        }
                    };
                    let m_pSkeletonMemory = match m_pSkeletonMemory {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pSkeletonMemory",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BSLookAtModifier {
                        __ptr,
                        parent,
                        m_lookAtTarget,
                        m_bones,
                        m_eyeBones,
                        m_limitAngleDegrees,
                        m_limitAngleThresholdDegrees,
                        m_continueLookOutsideOfLimit,
                        m_onGain,
                        m_offGain,
                        m_useBoneGains,
                        m_targetLocation,
                        m_targetOutsideLimits,
                        m_targetOutOfLimitEvent,
                        m_lookAtCamera,
                        m_lookAtCameraX,
                        m_lookAtCameraY,
                        m_lookAtCameraZ,
                        m_timeStep,
                        m_ballBonesValid,
                        m_pSkeletonMemory,
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
                    let mut m_lookAtTarget: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_bones: _serde::__private::Option<
                        Vec<BSLookAtModifierBoneData>,
                    > = _serde::__private::None;
                    let mut m_eyeBones: _serde::__private::Option<
                        Vec<BSLookAtModifierBoneData>,
                    > = _serde::__private::None;
                    let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleThresholdDegrees: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_continueLookOutsideOfLimit: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useBoneGains: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_targetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_targetOutsideLimits: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_targetOutOfLimitEvent: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_lookAtCamera: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_lookAtCameraX: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lookAtCameraY: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lookAtCameraZ: _serde::__private::Option<f32> = _serde::__private::None;
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
                            __Field::m_lookAtTarget => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lookAtTarget) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtTarget",
                                        ),
                                    );
                                }
                                m_lookAtTarget = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bones => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bones) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("bones"),
                                    );
                                }
                                m_bones = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<BSLookAtModifierBoneData>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_eyeBones => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eyeBones) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eyeBones",
                                        ),
                                    );
                                }
                                m_eyeBones = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<BSLookAtModifierBoneData>,
                                    >(&mut __map) {
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
                            __Field::m_limitAngleThresholdDegrees => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleThresholdDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleThresholdDegrees",
                                        ),
                                    );
                                }
                                m_limitAngleThresholdDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_continueLookOutsideOfLimit => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_continueLookOutsideOfLimit,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "continueLookOutsideOfLimit",
                                        ),
                                    );
                                }
                                m_continueLookOutsideOfLimit = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                            __Field::m_useBoneGains => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_useBoneGains) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useBoneGains",
                                        ),
                                    );
                                }
                                m_useBoneGains = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_targetLocation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_targetLocation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetLocation",
                                        ),
                                    );
                                }
                                m_targetLocation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_targetOutsideLimits => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_targetOutsideLimits,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetOutsideLimits",
                                        ),
                                    );
                                }
                                m_targetOutsideLimits = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_targetOutOfLimitEvent => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_targetOutOfLimitEvent,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetOutOfLimitEvent",
                                        ),
                                    );
                                }
                                m_targetOutOfLimitEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lookAtCamera => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lookAtCamera) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCamera",
                                        ),
                                    );
                                }
                                m_lookAtCamera = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lookAtCameraX => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lookAtCameraX) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraX",
                                        ),
                                    );
                                }
                                m_lookAtCameraX = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lookAtCameraY => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lookAtCameraY) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraY",
                                        ),
                                    );
                                }
                                m_lookAtCameraY = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lookAtCameraZ => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lookAtCameraZ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraZ",
                                        ),
                                    );
                                }
                                m_lookAtCameraZ = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    let m_lookAtTarget = match m_lookAtTarget {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtTarget",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bones = match m_bones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bones"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_eyeBones = match m_eyeBones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("eyeBones"),
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
                    let m_limitAngleThresholdDegrees = match m_limitAngleThresholdDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleThresholdDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_continueLookOutsideOfLimit = match m_continueLookOutsideOfLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "continueLookOutsideOfLimit",
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
                    let m_useBoneGains = match m_useBoneGains {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useBoneGains",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_targetLocation = match m_targetLocation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetLocation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_targetOutsideLimits = match m_targetOutsideLimits {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetOutsideLimits",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_targetOutOfLimitEvent = match m_targetOutOfLimitEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetOutOfLimitEvent",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lookAtCamera = match m_lookAtCamera {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCamera",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lookAtCameraX = match m_lookAtCameraX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraX",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lookAtCameraY = match m_lookAtCameraY {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraY",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lookAtCameraZ = match m_lookAtCameraZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraZ",
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
                    _serde::__private::Ok(BSLookAtModifier {
                        __ptr,
                        parent,
                        m_lookAtTarget,
                        m_bones,
                        m_eyeBones,
                        m_limitAngleDegrees,
                        m_limitAngleThresholdDegrees,
                        m_continueLookOutsideOfLimit,
                        m_onGain,
                        m_offGain,
                        m_useBoneGains,
                        m_targetLocation,
                        m_targetOutsideLimits,
                        m_targetOutOfLimitEvent,
                        m_lookAtCamera,
                        m_lookAtCameraX,
                        m_lookAtCameraY,
                        m_lookAtCameraZ,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "lookAtTarget",
                "bones",
                "eyeBones",
                "limitAngleDegrees",
                "limitAngleThresholdDegrees",
                "continueLookOutsideOfLimit",
                "onGain",
                "offGain",
                "useBoneGains",
                "targetLocation",
                "targetOutsideLimits",
                "targetOutOfLimitEvent",
                "lookAtCamera",
                "lookAtCameraX",
                "lookAtCameraY",
                "lookAtCameraZ",
                "timeStep",
                "ballBonesValid",
                "pSkeletonMemory",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSLookAtModifier",
                FIELDS,
                __BSLookAtModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSLookAtModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
