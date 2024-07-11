use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSLookAtModifier`
/// -         version: `4`
/// -       signature: `0xd756fc25`
/// -          size: 160(x86)/224(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
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
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `lookAtTarget`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_lookAtTarget: bool,
    /// # C++ Info
    /// -          name: `bones`(ctype: `hkArray<struct BSLookAtModifierBoneData>`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bones: Vec<BSLookAtModifierBoneData>,
    /// # C++ Info
    /// -          name: `eyeBones`(ctype: `hkArray<struct BSLookAtModifierBoneData>`)
    /// -        offset:  60(x86)/104(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_eyeBones: Vec<BSLookAtModifierBoneData>,
    /// # C++ Info
    /// -          name: `limitAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  72(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `limitAngleThresholdDegrees`(ctype: `hkReal`)
    /// -        offset:  76(x86)/124(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleThresholdDegrees: f32,
    /// # C++ Info
    /// -          name: `continueLookOutsideOfLimit`(ctype: `hkBool`)
    /// -        offset:  80(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_continueLookOutsideOfLimit: bool,
    /// # C++ Info
    /// -          name: `onGain`(ctype: `hkReal`)
    /// -        offset:  84(x86)/132(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_onGain: f32,
    /// # C++ Info
    /// -          name: `offGain`(ctype: `hkReal`)
    /// -        offset:  88(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offGain: f32,
    /// # C++ Info
    /// -          name: `useBoneGains`(ctype: `hkBool`)
    /// -        offset:  92(x86)/140(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useBoneGains: bool,
    /// # C++ Info
    /// -          name: `targetLocation`(ctype: `hkVector4`)
    /// -        offset:  96(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_targetLocation: Vector4,
    /// # C++ Info
    /// -          name: `targetOutsideLimits`(ctype: `hkBool`)
    /// -        offset: 112(x86)/160(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_targetOutsideLimits: bool,
    /// # C++ Info
    /// -          name: `targetOutOfLimitEvent`(ctype: `struct hkbEventProperty`)
    /// -        offset: 116(x86)/168(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_targetOutOfLimitEvent: hkbEventProperty,
    /// # C++ Info
    /// -          name: `lookAtCamera`(ctype: `hkBool`)
    /// -        offset: 124(x86)/184(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_lookAtCamera: bool,
    /// # C++ Info
    /// -          name: `lookAtCameraX`(ctype: `hkReal`)
    /// -        offset: 128(x86)/188(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lookAtCameraX: f32,
    /// # C++ Info
    /// -          name: `lookAtCameraY`(ctype: `hkReal`)
    /// -        offset: 132(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lookAtCameraY: f32,
    /// # C++ Info
    /// -          name: `lookAtCameraZ`(ctype: `hkReal`)
    /// -        offset: 136(x86)/196(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lookAtCameraZ: f32,
    /// # C++ Info
    /// -          name: `timeStep`(ctype: `hkReal`)
    /// -        offset: 140(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeStep: f32,
    /// # C++ Info
    /// -          name: `ballBonesValid`(ctype: `hkBool`)
    /// -        offset: 144(x86)/204(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_ballBonesValid: bool,
    /// # C++ Info
    /// -          name: `pSkeletonMemory`(ctype: `void*`)
    /// -        offset: 148(x86)/208(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
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
                .serialize_struct("BSLookAtModifier", class_meta)?;
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
            serializer.serialize_field("lookAtTarget", &self.m_lookAtTarget)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_meta_field("bones", &self.m_bones)?;
            serializer.serialize_array_meta_field("eyeBones", &self.m_eyeBones)?;
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
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("bones", &self.m_bones)?;
            serializer.serialize_array_field("eyeBones", &self.m_eyeBones)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
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
            "lookAtTarget" => Ok(__Field::m_lookAtTarget),
            "bones" => Ok(__Field::m_bones),
            "eyeBones" => Ok(__Field::m_eyeBones),
            "limitAngleDegrees" => Ok(__Field::m_limitAngleDegrees),
            "limitAngleThresholdDegrees" => Ok(__Field::m_limitAngleThresholdDegrees),
            "continueLookOutsideOfLimit" => Ok(__Field::m_continueLookOutsideOfLimit),
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
    fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
    }
}
pub(super) struct __BSLookAtModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSLookAtModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSLookAtModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSLookAtModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSLookAtModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSLookAtModifierVisitor<'de> {
    type Value = BSLookAtModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSLookAtModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_lookAtTarget: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bones: _serde::__private::Option<Vec<BSLookAtModifierBoneData>> = _serde::__private::None;
        let mut m_eyeBones: _serde::__private::Option<Vec<BSLookAtModifierBoneData>> = _serde::__private::None;
        let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleThresholdDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_continueLookOutsideOfLimit: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_useBoneGains: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_targetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_targetOutsideLimits: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_targetOutOfLimitEvent: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
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
                    if _serde::__private::Option::is_some(&m_targetOutsideLimits) {
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
                    if _serde::__private::Option::is_some(&m_targetOutOfLimitEvent) {
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
                    <__A::Error as _serde::de::Error>::missing_field("lookAtTarget"),
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
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDegrees"),
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
                    <__A::Error as _serde::de::Error>::missing_field("useBoneGains"),
                );
            }
        };
        let m_targetLocation = match m_targetLocation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetLocation"),
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
                    <__A::Error as _serde::de::Error>::missing_field("lookAtCamera"),
                );
            }
        };
        let m_lookAtCameraX = match m_lookAtCameraX {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lookAtCameraX"),
                );
            }
        };
        let m_lookAtCameraY = match m_lookAtCameraY {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lookAtCameraY"),
                );
            }
        };
        let m_lookAtCameraZ = match m_lookAtCameraZ {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lookAtCameraZ"),
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
                    <__A::Error as _serde::de::Error>::missing_field("ballBonesValid"),
                );
            }
        };
        let m_pSkeletonMemory = match m_pSkeletonMemory {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pSkeletonMemory"),
                );
            }
        };
        _serde::__private::Ok(BSLookAtModifier {
            __ptr: __A::class_ptr(&mut __map),
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
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_lookAtTarget: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bones: _serde::__private::Option<Vec<BSLookAtModifierBoneData>> = _serde::__private::None;
        let mut m_eyeBones: _serde::__private::Option<Vec<BSLookAtModifierBoneData>> = _serde::__private::None;
        let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitAngleThresholdDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_continueLookOutsideOfLimit: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_useBoneGains: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_targetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_targetOutsideLimits: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_targetOutOfLimitEvent: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_lookAtCamera: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_lookAtCameraX: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_lookAtCameraY: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_lookAtCameraZ: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..16usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_lookAtTarget => {
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
                    __Field::m_bones => {
                        if _serde::__private::Option::is_some(&m_bones) {
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
                    __Field::m_limitAngleThresholdDegrees => {
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
                    __Field::m_continueLookOutsideOfLimit => {
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
                    __Field::m_useBoneGains => {
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
                    __Field::m_targetLocation => {
                        if _serde::__private::Option::is_some(&m_targetLocation) {
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
                        if _serde::__private::Option::is_some(&m_targetOutsideLimits) {
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
                        if _serde::__private::Option::is_some(&m_targetOutOfLimitEvent) {
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
                    __Field::m_lookAtCameraX => {
                        if _serde::__private::Option::is_some(&m_lookAtCameraX) {
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
                    __Field::m_lookAtCameraZ => {
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
                    _ => {}
                }
            }
        }
        let m_lookAtTarget = match m_lookAtTarget {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lookAtTarget"),
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
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDegrees"),
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
                    <__A::Error as _serde::de::Error>::missing_field("useBoneGains"),
                );
            }
        };
        let m_targetLocation = match m_targetLocation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetLocation"),
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
                    <__A::Error as _serde::de::Error>::missing_field("lookAtCamera"),
                );
            }
        };
        let m_lookAtCameraX = match m_lookAtCameraX {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lookAtCameraX"),
                );
            }
        };
        let m_lookAtCameraY = match m_lookAtCameraY {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lookAtCameraY"),
                );
            }
        };
        let m_lookAtCameraZ = match m_lookAtCameraZ {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lookAtCameraZ"),
                );
            }
        };
        _serde::__private::Ok(BSLookAtModifier {
            __ptr: __A::class_ptr(&mut __map),
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSLookAtModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
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
