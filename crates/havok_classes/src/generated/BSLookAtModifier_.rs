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
            _serde::__private::Signature::new(3612802085u32)
        }
    }
    impl<'a> _serde::Serialize for BSLookAtModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3612802085u32)));
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
