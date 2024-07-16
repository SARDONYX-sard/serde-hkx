use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbPoseMatchingGenerator`
/// - version: `2`
/// - signature: `0x29e271b4`
/// - size: `208`(x86)/`240`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbPoseMatchingGenerator<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbBlenderGenerator<'a>,
    /// # C++ Info
    /// - name: `worldFromModelRotation`(ctype: `hkQuaternion`)
    /// - offset: `128`(x86)/`160`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_worldFromModelRotation: Quaternion,
    /// # C++ Info
    /// - name: `blendSpeed`(ctype: `hkReal`)
    /// - offset: `144`(x86)/`176`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_blendSpeed: f32,
    /// # C++ Info
    /// - name: `minSpeedToSwitch`(ctype: `hkReal`)
    /// - offset: `148`(x86)/`180`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minSpeedToSwitch: f32,
    /// # C++ Info
    /// - name: `minSwitchTimeNoError`(ctype: `hkReal`)
    /// - offset: `152`(x86)/`184`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minSwitchTimeNoError: f32,
    /// # C++ Info
    /// - name: `minSwitchTimeFullError`(ctype: `hkReal`)
    /// - offset: `156`(x86)/`188`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minSwitchTimeFullError: f32,
    /// # C++ Info
    /// - name: `startPlayingEventId`(ctype: `hkInt32`)
    /// - offset: `160`(x86)/`192`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_startPlayingEventId: i32,
    /// # C++ Info
    /// - name: `startMatchingEventId`(ctype: `hkInt32`)
    /// - offset: `164`(x86)/`196`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_startMatchingEventId: i32,
    /// # C++ Info
    /// - name: `rootBoneIndex`(ctype: `hkInt16`)
    /// - offset: `168`(x86)/`200`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_rootBoneIndex: i16,
    /// # C++ Info
    /// - name: `otherBoneIndex`(ctype: `hkInt16`)
    /// - offset: `170`(x86)/`202`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_otherBoneIndex: i16,
    /// # C++ Info
    /// - name: `anotherBoneIndex`(ctype: `hkInt16`)
    /// - offset: `172`(x86)/`204`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_anotherBoneIndex: i16,
    /// # C++ Info
    /// - name: `pelvisIndex`(ctype: `hkInt16`)
    /// - offset: `174`(x86)/`206`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_pelvisIndex: i16,
    /// # C++ Info
    /// - name: `mode`(ctype: `enum Mode`)
    /// - offset: `176`(x86)/`208`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_mode: Mode,
    /// # C++ Info
    /// - name: `currentMatch`(ctype: `hkInt32`)
    /// - offset: `180`(x86)/`212`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_currentMatch: i32,
    /// # C++ Info
    /// - name: `bestMatch`(ctype: `hkInt32`)
    /// - offset: `184`(x86)/`216`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_bestMatch: i32,
    /// # C++ Info
    /// - name: `timeSinceBetterMatch`(ctype: `hkReal`)
    /// - offset: `188`(x86)/`220`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_timeSinceBetterMatch: f32,
    /// # C++ Info
    /// - name: `error`(ctype: `hkReal`)
    /// - offset: `192`(x86)/`224`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_error: f32,
    /// # C++ Info
    /// - name: `resetCurrentMatchLocalTime`(ctype: `hkBool`)
    /// - offset: `196`(x86)/`228`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_resetCurrentMatchLocalTime: bool,
    /// # C++ Info
    /// - name: `poseMatchingUtility`(ctype: `void*`)
    /// - offset: `200`(x86)/`232`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_poseMatchingUtility: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbPoseMatchingGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbPoseMatchingGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x29e271b4)
        }
    }
    impl<'a> _serde::Serialize for hkbPoseMatchingGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x29e271b4)));
            let mut serializer = __serializer
                .serialize_struct("hkbPoseMatchingGenerator", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field(
                    "name",
                    &self.parent.parent.parent.m_name,
                )?;
            serializer.skip_field("id", &self.parent.parent.parent.m_id)?;
            serializer
                .skip_field("cloneState", &self.parent.parent.parent.m_cloneState)?;
            serializer
                .skip_field("padNode", &self.parent.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "referencePoseWeightThreshold",
                    &self.parent.m_referencePoseWeightThreshold,
                )?;
            serializer.serialize_field("blendParameter", &self.parent.m_blendParameter)?;
            serializer
                .serialize_field(
                    "minCyclicBlendParameter",
                    &self.parent.m_minCyclicBlendParameter,
                )?;
            serializer
                .serialize_field(
                    "maxCyclicBlendParameter",
                    &self.parent.m_maxCyclicBlendParameter,
                )?;
            serializer
                .serialize_field(
                    "indexOfSyncMasterChild",
                    &self.parent.m_indexOfSyncMasterChild,
                )?;
            serializer.serialize_field("flags", &self.parent.m_flags)?;
            serializer
                .serialize_field("subtractLastChild", &self.parent.m_subtractLastChild)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_array_meta_field("children", &self.parent.m_children)?;
            serializer
                .skip_array_meta_field(
                    "childrenInternalStates",
                    &self.parent.m_childrenInternalStates,
                )?;
            serializer
                .skip_array_meta_field("sortedChildren", &self.parent.m_sortedChildren)?;
            serializer
                .skip_field("endIntervalWeight", &self.parent.m_endIntervalWeight)?;
            serializer
                .skip_field("numActiveChildren", &self.parent.m_numActiveChildren)?;
            serializer
                .skip_field("beginIntervalIndex", &self.parent.m_beginIntervalIndex)?;
            serializer.skip_field("endIntervalIndex", &self.parent.m_endIntervalIndex)?;
            serializer.skip_field("initSync", &self.parent.m_initSync)?;
            serializer
                .skip_field("doSubtractiveBlend", &self.parent.m_doSubtractiveBlend)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field(
                    "worldFromModelRotation",
                    &self.m_worldFromModelRotation,
                )?;
            serializer.serialize_field("blendSpeed", &self.m_blendSpeed)?;
            serializer.serialize_field("minSpeedToSwitch", &self.m_minSpeedToSwitch)?;
            serializer
                .serialize_field("minSwitchTimeNoError", &self.m_minSwitchTimeNoError)?;
            serializer
                .serialize_field(
                    "minSwitchTimeFullError",
                    &self.m_minSwitchTimeFullError,
                )?;
            serializer
                .serialize_field("startPlayingEventId", &self.m_startPlayingEventId)?;
            serializer
                .serialize_field("startMatchingEventId", &self.m_startMatchingEventId)?;
            serializer.serialize_field("rootBoneIndex", &self.m_rootBoneIndex)?;
            serializer.serialize_field("otherBoneIndex", &self.m_otherBoneIndex)?;
            serializer.serialize_field("anotherBoneIndex", &self.m_anotherBoneIndex)?;
            serializer.serialize_field("pelvisIndex", &self.m_pelvisIndex)?;
            serializer.serialize_field("mode", &self.m_mode)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("currentMatch", &self.m_currentMatch)?;
            serializer.skip_field("bestMatch", &self.m_bestMatch)?;
            serializer.skip_field("timeSinceBetterMatch", &self.m_timeSinceBetterMatch)?;
            serializer.skip_field("error", &self.m_error)?;
            serializer
                .skip_field(
                    "resetCurrentMatchLocalTime",
                    &self.m_resetCurrentMatchLocalTime,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("poseMatchingUtility", &self.m_poseMatchingUtility)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .serialize_stringptr_field("name", &self.parent.parent.parent.m_name)?;
            serializer.serialize_array_field("children", &self.parent.m_children)?;
            serializer
                .serialize_array_field(
                    "childrenInternalStates",
                    &self.parent.m_childrenInternalStates,
                )?;
            serializer
                .serialize_array_field("sortedChildren", &self.parent.m_sortedChildren)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_worldFromModelRotation,
    m_blendSpeed,
    m_minSpeedToSwitch,
    m_minSwitchTimeNoError,
    m_minSwitchTimeFullError,
    m_startPlayingEventId,
    m_startMatchingEventId,
    m_rootBoneIndex,
    m_otherBoneIndex,
    m_anotherBoneIndex,
    m_pelvisIndex,
    m_mode,
    m_currentMatch,
    m_bestMatch,
    m_timeSinceBetterMatch,
    m_error,
    m_resetCurrentMatchLocalTime,
    m_poseMatchingUtility,
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
            "worldFromModelRotation" => Ok(__Field::m_worldFromModelRotation),
            "blendSpeed" => Ok(__Field::m_blendSpeed),
            "minSpeedToSwitch" => Ok(__Field::m_minSpeedToSwitch),
            "minSwitchTimeNoError" => Ok(__Field::m_minSwitchTimeNoError),
            "minSwitchTimeFullError" => Ok(__Field::m_minSwitchTimeFullError),
            "startPlayingEventId" => Ok(__Field::m_startPlayingEventId),
            "startMatchingEventId" => Ok(__Field::m_startMatchingEventId),
            "rootBoneIndex" => Ok(__Field::m_rootBoneIndex),
            "otherBoneIndex" => Ok(__Field::m_otherBoneIndex),
            "anotherBoneIndex" => Ok(__Field::m_anotherBoneIndex),
            "pelvisIndex" => Ok(__Field::m_pelvisIndex),
            "mode" => Ok(__Field::m_mode),
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
pub(super) struct __hkbPoseMatchingGeneratorVisitor<'de> {
    marker: core::marker::PhantomData<hkbPoseMatchingGenerator<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbPoseMatchingGeneratorVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbPoseMatchingGenerator<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbPoseMatchingGenerator<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbPoseMatchingGeneratorVisitor<'de> {
    type Value = hkbPoseMatchingGenerator<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbPoseMatchingGenerator")
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
        let mut m_worldFromModelRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_blendSpeed: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minSpeedToSwitch: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minSwitchTimeNoError: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minSwitchTimeFullError: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_startPlayingEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_startMatchingEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_rootBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_otherBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_anotherBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_pelvisIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_mode: _serde::__private::Option<Mode> = _serde::__private::None;
        let mut m_currentMatch: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_bestMatch: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_timeSinceBetterMatch: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_error: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_resetCurrentMatchLocalTime: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_poseMatchingUtility: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..18usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_worldFromModelRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "worldFromModelRotation",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 0usize)?;
                    m_worldFromModelRotation = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_blendSpeed) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "blendSpeed",
                            ),
                        );
                    }
                    m_blendSpeed = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_minSpeedToSwitch) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minSpeedToSwitch",
                            ),
                        );
                    }
                    m_minSpeedToSwitch = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_minSwitchTimeNoError) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minSwitchTimeNoError",
                            ),
                        );
                    }
                    m_minSwitchTimeNoError = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_minSwitchTimeFullError) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minSwitchTimeFullError",
                            ),
                        );
                    }
                    m_minSwitchTimeFullError = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_startPlayingEventId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startPlayingEventId",
                            ),
                        );
                    }
                    m_startPlayingEventId = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_startMatchingEventId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startMatchingEventId",
                            ),
                        );
                    }
                    m_startMatchingEventId = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_rootBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rootBoneIndex",
                            ),
                        );
                    }
                    m_rootBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_otherBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "otherBoneIndex",
                            ),
                        );
                    }
                    m_otherBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_anotherBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "anotherBoneIndex",
                            ),
                        );
                    }
                    m_anotherBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_pelvisIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pelvisIndex",
                            ),
                        );
                    }
                    m_pelvisIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_mode) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("mode"),
                        );
                    }
                    m_mode = _serde::__private::Some(
                        match __A::next_value::<Mode>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_currentMatch) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentMatch",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_currentMatch = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_bestMatch) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bestMatch",
                            ),
                        );
                    }
                    m_bestMatch = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                14usize => {
                    if _serde::__private::Option::is_some(&m_timeSinceBetterMatch) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "timeSinceBetterMatch",
                            ),
                        );
                    }
                    m_timeSinceBetterMatch = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_error) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("error"),
                        );
                    }
                    m_error = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                16usize => {
                    if _serde::__private::Option::is_some(
                        &m_resetCurrentMatchLocalTime,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "resetCurrentMatchLocalTime",
                            ),
                        );
                    }
                    m_resetCurrentMatchLocalTime = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                17usize => {
                    if _serde::__private::Option::is_some(&m_poseMatchingUtility) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "poseMatchingUtility",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_poseMatchingUtility = _serde::__private::Some(
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
        __A::pad(&mut __map, 4usize, 0usize)?;
        let m_worldFromModelRotation = match m_worldFromModelRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "worldFromModelRotation",
                    ),
                );
            }
        };
        let m_blendSpeed = match m_blendSpeed {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blendSpeed"),
                );
            }
        };
        let m_minSpeedToSwitch = match m_minSpeedToSwitch {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("minSpeedToSwitch"),
                );
            }
        };
        let m_minSwitchTimeNoError = match m_minSwitchTimeNoError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minSwitchTimeNoError",
                    ),
                );
            }
        };
        let m_minSwitchTimeFullError = match m_minSwitchTimeFullError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minSwitchTimeFullError",
                    ),
                );
            }
        };
        let m_startPlayingEventId = match m_startPlayingEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "startPlayingEventId",
                    ),
                );
            }
        };
        let m_startMatchingEventId = match m_startMatchingEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "startMatchingEventId",
                    ),
                );
            }
        };
        let m_rootBoneIndex = match m_rootBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rootBoneIndex"),
                );
            }
        };
        let m_otherBoneIndex = match m_otherBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("otherBoneIndex"),
                );
            }
        };
        let m_anotherBoneIndex = match m_anotherBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("anotherBoneIndex"),
                );
            }
        };
        let m_pelvisIndex = match m_pelvisIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pelvisIndex"),
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
        let m_currentMatch = match m_currentMatch {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("currentMatch"),
                );
            }
        };
        let m_bestMatch = match m_bestMatch {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bestMatch"),
                );
            }
        };
        let m_timeSinceBetterMatch = match m_timeSinceBetterMatch {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "timeSinceBetterMatch",
                    ),
                );
            }
        };
        let m_error = match m_error {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("error"),
                );
            }
        };
        let m_resetCurrentMatchLocalTime = match m_resetCurrentMatchLocalTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "resetCurrentMatchLocalTime",
                    ),
                );
            }
        };
        let m_poseMatchingUtility = match m_poseMatchingUtility {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "poseMatchingUtility",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbPoseMatchingGenerator {
            __ptr,
            parent,
            m_worldFromModelRotation,
            m_blendSpeed,
            m_minSpeedToSwitch,
            m_minSwitchTimeNoError,
            m_minSwitchTimeFullError,
            m_startPlayingEventId,
            m_startMatchingEventId,
            m_rootBoneIndex,
            m_otherBoneIndex,
            m_anotherBoneIndex,
            m_pelvisIndex,
            m_mode,
            m_currentMatch,
            m_bestMatch,
            m_timeSinceBetterMatch,
            m_error,
            m_resetCurrentMatchLocalTime,
            m_poseMatchingUtility,
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
        let parent = __hkbBlenderGeneratorVisitor::visit_as_parent(&mut __map)?;
        let mut m_worldFromModelRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_blendSpeed: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minSpeedToSwitch: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minSwitchTimeNoError: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minSwitchTimeFullError: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_startPlayingEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_startMatchingEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_rootBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_otherBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_anotherBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_pelvisIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_mode: _serde::__private::Option<Mode> = _serde::__private::None;
        for _ in 0..12usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_worldFromModelRotation => {
                        if _serde::__private::Option::is_some(
                            &m_worldFromModelRotation,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "worldFromModelRotation",
                                ),
                            );
                        }
                        m_worldFromModelRotation = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_blendSpeed => {
                        if _serde::__private::Option::is_some(&m_blendSpeed) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "blendSpeed",
                                ),
                            );
                        }
                        m_blendSpeed = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_minSpeedToSwitch => {
                        if _serde::__private::Option::is_some(&m_minSpeedToSwitch) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "minSpeedToSwitch",
                                ),
                            );
                        }
                        m_minSpeedToSwitch = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_minSwitchTimeNoError => {
                        if _serde::__private::Option::is_some(&m_minSwitchTimeNoError) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "minSwitchTimeNoError",
                                ),
                            );
                        }
                        m_minSwitchTimeNoError = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_minSwitchTimeFullError => {
                        if _serde::__private::Option::is_some(
                            &m_minSwitchTimeFullError,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "minSwitchTimeFullError",
                                ),
                            );
                        }
                        m_minSwitchTimeFullError = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_startPlayingEventId => {
                        if _serde::__private::Option::is_some(&m_startPlayingEventId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "startPlayingEventId",
                                ),
                            );
                        }
                        m_startPlayingEventId = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_startMatchingEventId => {
                        if _serde::__private::Option::is_some(&m_startMatchingEventId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "startMatchingEventId",
                                ),
                            );
                        }
                        m_startMatchingEventId = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rootBoneIndex => {
                        if _serde::__private::Option::is_some(&m_rootBoneIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rootBoneIndex",
                                ),
                            );
                        }
                        m_rootBoneIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_otherBoneIndex => {
                        if _serde::__private::Option::is_some(&m_otherBoneIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "otherBoneIndex",
                                ),
                            );
                        }
                        m_otherBoneIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_anotherBoneIndex => {
                        if _serde::__private::Option::is_some(&m_anotherBoneIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "anotherBoneIndex",
                                ),
                            );
                        }
                        m_anotherBoneIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_pelvisIndex => {
                        if _serde::__private::Option::is_some(&m_pelvisIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pelvisIndex",
                                ),
                            );
                        }
                        m_pelvisIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_mode => {
                        if _serde::__private::Option::is_some(&m_mode) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("mode"),
                            );
                        }
                        m_mode = _serde::__private::Some(
                            match __A::next_value::<Mode>(&mut __map) {
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
        let m_worldFromModelRotation = match m_worldFromModelRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "worldFromModelRotation",
                    ),
                );
            }
        };
        let m_blendSpeed = match m_blendSpeed {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("blendSpeed"),
                );
            }
        };
        let m_minSpeedToSwitch = match m_minSpeedToSwitch {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("minSpeedToSwitch"),
                );
            }
        };
        let m_minSwitchTimeNoError = match m_minSwitchTimeNoError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minSwitchTimeNoError",
                    ),
                );
            }
        };
        let m_minSwitchTimeFullError = match m_minSwitchTimeFullError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minSwitchTimeFullError",
                    ),
                );
            }
        };
        let m_startPlayingEventId = match m_startPlayingEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "startPlayingEventId",
                    ),
                );
            }
        };
        let m_startMatchingEventId = match m_startMatchingEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "startMatchingEventId",
                    ),
                );
            }
        };
        let m_rootBoneIndex = match m_rootBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rootBoneIndex"),
                );
            }
        };
        let m_otherBoneIndex = match m_otherBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("otherBoneIndex"),
                );
            }
        };
        let m_anotherBoneIndex = match m_anotherBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("anotherBoneIndex"),
                );
            }
        };
        let m_pelvisIndex = match m_pelvisIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pelvisIndex"),
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
        _serde::__private::Ok(hkbPoseMatchingGenerator {
            __ptr,
            parent,
            m_worldFromModelRotation,
            m_blendSpeed,
            m_minSpeedToSwitch,
            m_minSwitchTimeNoError,
            m_minSwitchTimeFullError,
            m_startPlayingEventId,
            m_startMatchingEventId,
            m_rootBoneIndex,
            m_otherBoneIndex,
            m_anotherBoneIndex,
            m_pelvisIndex,
            m_mode,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbPoseMatchingGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "worldFromModelRotation",
                "blendSpeed",
                "minSpeedToSwitch",
                "minSwitchTimeNoError",
                "minSwitchTimeFullError",
                "startPlayingEventId",
                "startMatchingEventId",
                "rootBoneIndex",
                "otherBoneIndex",
                "anotherBoneIndex",
                "pelvisIndex",
                "mode",
                "currentMatch",
                "bestMatch",
                "timeSinceBetterMatch",
                "error",
                "resetCurrentMatchLocalTime",
                "poseMatchingUtility",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbPoseMatchingGenerator",
                FIELDS,
                __hkbPoseMatchingGeneratorVisitor {
                    marker: _serde::__private::PhantomData::<hkbPoseMatchingGenerator>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
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
pub enum Mode {
    #[default]
    MODE_MATCH = 0isize,
    MODE_PLAY = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Mode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MODE_MATCH => __serializer.serialize_field("MODE_MATCH", &0u64),
                Self::MODE_PLAY => __serializer.serialize_field("MODE_PLAY", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i8().ok_or(S::Error::custom("Failed enum Mode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for Mode {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
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
                            v if v == "0" || v.eq_ignore_ascii_case("MODE_MATCH") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("MODE_PLAY") => {
                                _serde::__private::Ok(__Field::__field1)
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
                marker: _serde::__private::PhantomData<Mode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Mode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Mode")
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
                            _serde::__private::Ok(Mode::MODE_MATCH)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Mode::MODE_PLAY)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["MODE_MATCH", "MODE_PLAY"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Mode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Mode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
