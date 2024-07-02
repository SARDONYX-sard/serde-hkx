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
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for BSDirectAtModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSDirectAtModifier"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(429917632u32)
        }
    }
    impl<'a> __serde::Serialize for BSDirectAtModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(429917632u32)));
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
