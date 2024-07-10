use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSDirectAtModifier`
/// -         version: `0`
/// -       signature: `0x19a005c0`
/// -          size: 176(x86)/224(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSDirectAtModifier<'a> {
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
    /// -          name: `directAtTarget`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_directAtTarget: bool,
    /// # C++ Info
    /// -          name: `sourceBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  46(x86)/ 82(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_sourceBoneIndex: i16,
    /// # C++ Info
    /// -          name: `startBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_startBoneIndex: i16,
    /// # C++ Info
    /// -          name: `endBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  50(x86)/ 86(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_endBoneIndex: i16,
    /// # C++ Info
    /// -          name: `limitHeadingDegrees`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitHeadingDegrees: f32,
    /// # C++ Info
    /// -          name: `limitPitchDegrees`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitPitchDegrees: f32,
    /// # C++ Info
    /// -          name: `offsetHeadingDegrees`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offsetHeadingDegrees: f32,
    /// # C++ Info
    /// -          name: `offsetPitchDegrees`(ctype: `hkReal`)
    /// -        offset:  64(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offsetPitchDegrees: f32,
    /// # C++ Info
    /// -          name: `onGain`(ctype: `hkReal`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_onGain: f32,
    /// # C++ Info
    /// -          name: `offGain`(ctype: `hkReal`)
    /// -        offset:  72(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offGain: f32,
    /// # C++ Info
    /// -          name: `targetLocation`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_targetLocation: Vector4,
    /// # C++ Info
    /// -          name: `userInfo`(ctype: `hkUint32`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_userInfo: u32,
    /// # C++ Info
    /// -          name: `directAtCamera`(ctype: `hkBool`)
    /// -        offset: 100(x86)/132(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_directAtCamera: bool,
    /// # C++ Info
    /// -          name: `directAtCameraX`(ctype: `hkReal`)
    /// -        offset: 104(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_directAtCameraX: f32,
    /// # C++ Info
    /// -          name: `directAtCameraY`(ctype: `hkReal`)
    /// -        offset: 108(x86)/140(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_directAtCameraY: f32,
    /// # C++ Info
    /// -          name: `directAtCameraZ`(ctype: `hkReal`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_directAtCameraZ: f32,
    /// # C++ Info
    /// -          name: `active`(ctype: `hkBool`)
    /// -        offset: 116(x86)/148(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_active: bool,
    /// # C++ Info
    /// -          name: `currentHeadingOffset`(ctype: `hkReal`)
    /// -        offset: 120(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_currentHeadingOffset: f32,
    /// # C++ Info
    /// -          name: `currentPitchOffset`(ctype: `hkReal`)
    /// -        offset: 124(x86)/156(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_currentPitchOffset: f32,
    /// # C++ Info
    /// -          name: `timeStep`(ctype: `hkReal`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeStep: f32,
    /// # C++ Info
    /// -          name: `pSkeletonMemory`(ctype: `void*`)
    /// -        offset: 132(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pSkeletonMemory: Pointer,
    /// # C++ Info
    /// -          name: `hasTarget`(ctype: `hkBool`)
    /// -        offset: 136(x86)/176(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_hasTarget: bool,
    /// # C++ Info
    /// -          name: `directAtTargetLocation`(ctype: `hkVector4`)
    /// -        offset: 144(x86)/192(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_directAtTargetLocation: Vector4,
    /// # C++ Info
    /// -          name: `boneChainIndices`(ctype: `hkArray<void>`)
    /// -        offset: 160(x86)/208(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_boneChainIndices: Vec<()>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSDirectAtModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSDirectAtModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x19a005c0)
        }
    }
    impl<'a> _serde::Serialize for BSDirectAtModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x19a005c0)));
            let mut serializer = __serializer
                .serialize_struct("BSDirectAtModifier", class_meta)?;
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
            serializer.serialize_field("directAtTarget", &self.m_directAtTarget)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("sourceBoneIndex", &self.m_sourceBoneIndex)?;
            serializer.serialize_field("startBoneIndex", &self.m_startBoneIndex)?;
            serializer.serialize_field("endBoneIndex", &self.m_endBoneIndex)?;
            serializer
                .serialize_field("limitHeadingDegrees", &self.m_limitHeadingDegrees)?;
            serializer.serialize_field("limitPitchDegrees", &self.m_limitPitchDegrees)?;
            serializer
                .serialize_field("offsetHeadingDegrees", &self.m_offsetHeadingDegrees)?;
            serializer
                .serialize_field("offsetPitchDegrees", &self.m_offsetPitchDegrees)?;
            serializer.serialize_field("onGain", &self.m_onGain)?;
            serializer.serialize_field("offGain", &self.m_offGain)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("targetLocation", &self.m_targetLocation)?;
            serializer.serialize_field("userInfo", &self.m_userInfo)?;
            serializer.serialize_field("directAtCamera", &self.m_directAtCamera)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("directAtCameraX", &self.m_directAtCameraX)?;
            serializer.serialize_field("directAtCameraY", &self.m_directAtCameraY)?;
            serializer.serialize_field("directAtCameraZ", &self.m_directAtCameraZ)?;
            serializer.serialize_field("active", &self.m_active)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field("currentHeadingOffset", &self.m_currentHeadingOffset)?;
            serializer
                .serialize_field("currentPitchOffset", &self.m_currentPitchOffset)?;
            serializer.skip_field("timeStep", &self.m_timeStep)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("pSkeletonMemory", &self.m_pSkeletonMemory)?;
            serializer.skip_field("hasTarget", &self.m_hasTarget)?;
            serializer.pad_field([0u8; 7usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer
                .skip_field("directAtTargetLocation", &self.m_directAtTargetLocation)?;
            serializer
                .skip_array_meta_field("boneChainIndices", &self.m_boneChainIndices)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field("boneChainIndices", &self.m_boneChainIndices)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_directAtTarget,
    m_sourceBoneIndex,
    m_startBoneIndex,
    m_endBoneIndex,
    m_limitHeadingDegrees,
    m_limitPitchDegrees,
    m_offsetHeadingDegrees,
    m_offsetPitchDegrees,
    m_onGain,
    m_offGain,
    m_targetLocation,
    m_userInfo,
    m_directAtCamera,
    m_directAtCameraX,
    m_directAtCameraY,
    m_directAtCameraZ,
    m_active,
    m_currentHeadingOffset,
    m_currentPitchOffset,
    m_timeStep,
    m_pSkeletonMemory,
    m_hasTarget,
    m_directAtTargetLocation,
    m_boneChainIndices,
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
            "directAtTarget" => Ok(__Field::m_directAtTarget),
            "sourceBoneIndex" => Ok(__Field::m_sourceBoneIndex),
            "startBoneIndex" => Ok(__Field::m_startBoneIndex),
            "endBoneIndex" => Ok(__Field::m_endBoneIndex),
            "limitHeadingDegrees" => Ok(__Field::m_limitHeadingDegrees),
            "limitPitchDegrees" => Ok(__Field::m_limitPitchDegrees),
            "offsetHeadingDegrees" => Ok(__Field::m_offsetHeadingDegrees),
            "offsetPitchDegrees" => Ok(__Field::m_offsetPitchDegrees),
            "onGain" => Ok(__Field::m_onGain),
            "offGain" => Ok(__Field::m_offGain),
            "targetLocation" => Ok(__Field::m_targetLocation),
            "userInfo" => Ok(__Field::m_userInfo),
            "directAtCamera" => Ok(__Field::m_directAtCamera),
            "directAtCameraX" => Ok(__Field::m_directAtCameraX),
            "directAtCameraY" => Ok(__Field::m_directAtCameraY),
            "directAtCameraZ" => Ok(__Field::m_directAtCameraZ),
            "active" => Ok(__Field::m_active),
            "currentHeadingOffset" => Ok(__Field::m_currentHeadingOffset),
            "currentPitchOffset" => Ok(__Field::m_currentPitchOffset),
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
pub(super) struct __BSDirectAtModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSDirectAtModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSDirectAtModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSDirectAtModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSDirectAtModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSDirectAtModifierVisitor<'de> {
    type Value = BSDirectAtModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSDirectAtModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_directAtTarget: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_sourceBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_startBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_limitHeadingDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitPitchDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offsetHeadingDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offsetPitchDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_targetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_userInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_directAtCamera: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_directAtCameraX: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_directAtCameraY: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_directAtCameraZ: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_active: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_currentHeadingOffset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_currentPitchOffset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_timeStep: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_pSkeletonMemory: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_hasTarget: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_directAtTargetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_boneChainIndices: _serde::__private::Option<Vec<()>> = _serde::__private::None;
        for i in 0..24usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_directAtTarget) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtTarget",
                            ),
                        );
                    }
                    m_directAtTarget = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_sourceBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sourceBoneIndex",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_sourceBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_startBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startBoneIndex",
                            ),
                        );
                    }
                    m_startBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_endBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endBoneIndex",
                            ),
                        );
                    }
                    m_endBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_limitHeadingDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitHeadingDegrees",
                            ),
                        );
                    }
                    m_limitHeadingDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_limitPitchDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitPitchDegrees",
                            ),
                        );
                    }
                    m_limitPitchDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_offsetHeadingDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "offsetHeadingDegrees",
                            ),
                        );
                    }
                    m_offsetHeadingDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_offsetPitchDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "offsetPitchDegrees",
                            ),
                        );
                    }
                    m_offsetPitchDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
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
                9usize => {
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
                10usize => {
                    if _serde::__private::Option::is_some(&m_targetLocation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "targetLocation",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 4usize, 0usize)?;
                    m_targetLocation = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_userInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userInfo",
                            ),
                        );
                    }
                    m_userInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_directAtCamera) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtCamera",
                            ),
                        );
                    }
                    m_directAtCamera = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_directAtCameraX) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtCameraX",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_directAtCameraX = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                14usize => {
                    if _serde::__private::Option::is_some(&m_directAtCameraY) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtCameraY",
                            ),
                        );
                    }
                    m_directAtCameraY = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_directAtCameraZ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtCameraZ",
                            ),
                        );
                    }
                    m_directAtCameraZ = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                16usize => {
                    if _serde::__private::Option::is_some(&m_active) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("active"),
                        );
                    }
                    m_active = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                17usize => {
                    if _serde::__private::Option::is_some(&m_currentHeadingOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentHeadingOffset",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_currentHeadingOffset = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                18usize => {
                    if _serde::__private::Option::is_some(&m_currentPitchOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentPitchOffset",
                            ),
                        );
                    }
                    m_currentPitchOffset = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                19usize => {
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
                20usize => {
                    if _serde::__private::Option::is_some(&m_pSkeletonMemory) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pSkeletonMemory",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_pSkeletonMemory = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                21usize => {
                    if _serde::__private::Option::is_some(&m_hasTarget) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hasTarget",
                            ),
                        );
                    }
                    m_hasTarget = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                22usize => {
                    if _serde::__private::Option::is_some(&m_directAtTargetLocation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtTargetLocation",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 7usize, 15usize)?;
                    m_directAtTargetLocation = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                23usize => {
                    if _serde::__private::Option::is_some(&m_boneChainIndices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "boneChainIndices",
                            ),
                        );
                    }
                    m_boneChainIndices = _serde::__private::Some(
                        match __A::next_value::<Vec<()>>(&mut __map) {
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
        let m_directAtTarget = match m_directAtTarget {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtTarget"),
                );
            }
        };
        let m_sourceBoneIndex = match m_sourceBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sourceBoneIndex"),
                );
            }
        };
        let m_startBoneIndex = match m_startBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startBoneIndex"),
                );
            }
        };
        let m_endBoneIndex = match m_endBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endBoneIndex"),
                );
            }
        };
        let m_limitHeadingDegrees = match m_limitHeadingDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "limitHeadingDegrees",
                    ),
                );
            }
        };
        let m_limitPitchDegrees = match m_limitPitchDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitPitchDegrees"),
                );
            }
        };
        let m_offsetHeadingDegrees = match m_offsetHeadingDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "offsetHeadingDegrees",
                    ),
                );
            }
        };
        let m_offsetPitchDegrees = match m_offsetPitchDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "offsetPitchDegrees",
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
        let m_targetLocation = match m_targetLocation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetLocation"),
                );
            }
        };
        let m_userInfo = match m_userInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userInfo"),
                );
            }
        };
        let m_directAtCamera = match m_directAtCamera {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtCamera"),
                );
            }
        };
        let m_directAtCameraX = match m_directAtCameraX {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtCameraX"),
                );
            }
        };
        let m_directAtCameraY = match m_directAtCameraY {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtCameraY"),
                );
            }
        };
        let m_directAtCameraZ = match m_directAtCameraZ {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtCameraZ"),
                );
            }
        };
        let m_active = match m_active {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("active"),
                );
            }
        };
        let m_currentHeadingOffset = match m_currentHeadingOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "currentHeadingOffset",
                    ),
                );
            }
        };
        let m_currentPitchOffset = match m_currentPitchOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "currentPitchOffset",
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
        let m_pSkeletonMemory = match m_pSkeletonMemory {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pSkeletonMemory"),
                );
            }
        };
        let m_hasTarget = match m_hasTarget {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hasTarget"),
                );
            }
        };
        let m_directAtTargetLocation = match m_directAtTargetLocation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "directAtTargetLocation",
                    ),
                );
            }
        };
        let m_boneChainIndices = match m_boneChainIndices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("boneChainIndices"),
                );
            }
        };
        _serde::__private::Ok(BSDirectAtModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_directAtTarget,
            m_sourceBoneIndex,
            m_startBoneIndex,
            m_endBoneIndex,
            m_limitHeadingDegrees,
            m_limitPitchDegrees,
            m_offsetHeadingDegrees,
            m_offsetPitchDegrees,
            m_onGain,
            m_offGain,
            m_targetLocation,
            m_userInfo,
            m_directAtCamera,
            m_directAtCameraX,
            m_directAtCameraY,
            m_directAtCameraZ,
            m_active,
            m_currentHeadingOffset,
            m_currentPitchOffset,
            m_timeStep,
            m_pSkeletonMemory,
            m_hasTarget,
            m_directAtTargetLocation,
            m_boneChainIndices,
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
        let mut m_directAtTarget: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_sourceBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_startBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_limitHeadingDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_limitPitchDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offsetHeadingDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offsetPitchDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_targetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_userInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_directAtCamera: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_directAtCameraX: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_directAtCameraY: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_directAtCameraZ: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_active: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_currentHeadingOffset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_currentPitchOffset: _serde::__private::Option<f32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_directAtTarget => {
                    if _serde::__private::Option::is_some(&m_directAtTarget) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtTarget",
                            ),
                        );
                    }
                    m_directAtTarget = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_sourceBoneIndex => {
                    if _serde::__private::Option::is_some(&m_sourceBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sourceBoneIndex",
                            ),
                        );
                    }
                    m_sourceBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_startBoneIndex => {
                    if _serde::__private::Option::is_some(&m_startBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startBoneIndex",
                            ),
                        );
                    }
                    m_startBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_endBoneIndex => {
                    if _serde::__private::Option::is_some(&m_endBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endBoneIndex",
                            ),
                        );
                    }
                    m_endBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_limitHeadingDegrees => {
                    if _serde::__private::Option::is_some(&m_limitHeadingDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitHeadingDegrees",
                            ),
                        );
                    }
                    m_limitHeadingDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_limitPitchDegrees => {
                    if _serde::__private::Option::is_some(&m_limitPitchDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitPitchDegrees",
                            ),
                        );
                    }
                    m_limitPitchDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_offsetHeadingDegrees => {
                    if _serde::__private::Option::is_some(&m_offsetHeadingDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "offsetHeadingDegrees",
                            ),
                        );
                    }
                    m_offsetHeadingDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_offsetPitchDegrees => {
                    if _serde::__private::Option::is_some(&m_offsetPitchDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "offsetPitchDegrees",
                            ),
                        );
                    }
                    m_offsetPitchDegrees = _serde::__private::Some(
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
                __Field::m_userInfo => {
                    if _serde::__private::Option::is_some(&m_userInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userInfo",
                            ),
                        );
                    }
                    m_userInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_directAtCamera => {
                    if _serde::__private::Option::is_some(&m_directAtCamera) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtCamera",
                            ),
                        );
                    }
                    m_directAtCamera = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_directAtCameraX => {
                    if _serde::__private::Option::is_some(&m_directAtCameraX) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtCameraX",
                            ),
                        );
                    }
                    m_directAtCameraX = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_directAtCameraY => {
                    if _serde::__private::Option::is_some(&m_directAtCameraY) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtCameraY",
                            ),
                        );
                    }
                    m_directAtCameraY = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_directAtCameraZ => {
                    if _serde::__private::Option::is_some(&m_directAtCameraZ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directAtCameraZ",
                            ),
                        );
                    }
                    m_directAtCameraZ = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_active => {
                    if _serde::__private::Option::is_some(&m_active) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("active"),
                        );
                    }
                    m_active = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_currentHeadingOffset => {
                    if _serde::__private::Option::is_some(&m_currentHeadingOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentHeadingOffset",
                            ),
                        );
                    }
                    m_currentHeadingOffset = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_currentPitchOffset => {
                    if _serde::__private::Option::is_some(&m_currentPitchOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentPitchOffset",
                            ),
                        );
                    }
                    m_currentPitchOffset = _serde::__private::Some(
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
        let m_directAtTarget = match m_directAtTarget {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtTarget"),
                );
            }
        };
        let m_sourceBoneIndex = match m_sourceBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sourceBoneIndex"),
                );
            }
        };
        let m_startBoneIndex = match m_startBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startBoneIndex"),
                );
            }
        };
        let m_endBoneIndex = match m_endBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endBoneIndex"),
                );
            }
        };
        let m_limitHeadingDegrees = match m_limitHeadingDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "limitHeadingDegrees",
                    ),
                );
            }
        };
        let m_limitPitchDegrees = match m_limitPitchDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitPitchDegrees"),
                );
            }
        };
        let m_offsetHeadingDegrees = match m_offsetHeadingDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "offsetHeadingDegrees",
                    ),
                );
            }
        };
        let m_offsetPitchDegrees = match m_offsetPitchDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "offsetPitchDegrees",
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
        let m_targetLocation = match m_targetLocation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetLocation"),
                );
            }
        };
        let m_userInfo = match m_userInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userInfo"),
                );
            }
        };
        let m_directAtCamera = match m_directAtCamera {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtCamera"),
                );
            }
        };
        let m_directAtCameraX = match m_directAtCameraX {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtCameraX"),
                );
            }
        };
        let m_directAtCameraY = match m_directAtCameraY {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtCameraY"),
                );
            }
        };
        let m_directAtCameraZ = match m_directAtCameraZ {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("directAtCameraZ"),
                );
            }
        };
        let m_active = match m_active {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("active"),
                );
            }
        };
        let m_currentHeadingOffset = match m_currentHeadingOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "currentHeadingOffset",
                    ),
                );
            }
        };
        let m_currentPitchOffset = match m_currentPitchOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "currentPitchOffset",
                    ),
                );
            }
        };
        _serde::__private::Ok(BSDirectAtModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_directAtTarget,
            m_sourceBoneIndex,
            m_startBoneIndex,
            m_endBoneIndex,
            m_limitHeadingDegrees,
            m_limitPitchDegrees,
            m_offsetHeadingDegrees,
            m_offsetPitchDegrees,
            m_onGain,
            m_offGain,
            m_targetLocation,
            m_userInfo,
            m_directAtCamera,
            m_directAtCameraX,
            m_directAtCameraY,
            m_directAtCameraZ,
            m_active,
            m_currentHeadingOffset,
            m_currentPitchOffset,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSDirectAtModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "directAtTarget",
                "sourceBoneIndex",
                "startBoneIndex",
                "endBoneIndex",
                "limitHeadingDegrees",
                "limitPitchDegrees",
                "offsetHeadingDegrees",
                "offsetPitchDegrees",
                "onGain",
                "offGain",
                "targetLocation",
                "userInfo",
                "directAtCamera",
                "directAtCameraX",
                "directAtCameraY",
                "directAtCameraZ",
                "active",
                "currentHeadingOffset",
                "currentPitchOffset",
                "timeStep",
                "pSkeletonMemory",
                "hasTarget",
                "directAtTargetLocation",
                "boneChainIndices",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSDirectAtModifier",
                FIELDS,
                __BSDirectAtModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSDirectAtModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
