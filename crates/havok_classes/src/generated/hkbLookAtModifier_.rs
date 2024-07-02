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
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbLookAtModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbLookAtModifier"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1026089062u32)
        }
    }
    impl<'a> __serde::Serialize for hkbLookAtModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1026089062u32)));
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
