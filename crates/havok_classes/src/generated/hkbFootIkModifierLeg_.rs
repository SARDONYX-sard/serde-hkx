use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkModifierLeg`
/// -         version: `2`
/// -       signature: `0x9f3e3a04`
/// -          size: 160(x86)/160(x86_64)
/// -          vtable: false
///
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
    /// -          name: `originalAnkleTransformMS`(ctype: `hkQsTransform`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_originalAnkleTransformMS: QsTransform,
    /// # C++ Info
    /// -          name: `prevAnkleRotLS`(ctype: `hkQuaternion`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_prevAnkleRotLS: Quaternion,
    /// # C++ Info
    /// -          name: `kneeAxisLS`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_kneeAxisLS: Vector4,
    /// # C++ Info
    /// -          name: `footEndLS`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_footEndLS: Vector4,
    /// # C++ Info
    /// -          name: `ungroundedEvent`(ctype: `struct hkbEventProperty`)
    /// -        offset:  96(x86)/ 96(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_ungroundedEvent: hkbEventProperty,
    /// # C++ Info
    /// -          name: `footPlantedAnkleHeightMS`(ctype: `hkReal`)
    /// -        offset: 104(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_footPlantedAnkleHeightMS: f32,
    /// # C++ Info
    /// -          name: `footRaisedAnkleHeightMS`(ctype: `hkReal`)
    /// -        offset: 108(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_footRaisedAnkleHeightMS: f32,
    /// # C++ Info
    /// -          name: `maxAnkleHeightMS`(ctype: `hkReal`)
    /// -        offset: 112(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAnkleHeightMS: f32,
    /// # C++ Info
    /// -          name: `minAnkleHeightMS`(ctype: `hkReal`)
    /// -        offset: 116(x86)/124(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minAnkleHeightMS: f32,
    /// # C++ Info
    /// -          name: `maxKneeAngleDegrees`(ctype: `hkReal`)
    /// -        offset: 120(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxKneeAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `minKneeAngleDegrees`(ctype: `hkReal`)
    /// -        offset: 124(x86)/132(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minKneeAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `verticalError`(ctype: `hkReal`)
    /// -        offset: 128(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalError: f32,
    /// # C++ Info
    /// -          name: `maxAnkleAngleDegrees`(ctype: `hkReal`)
    /// -        offset: 132(x86)/140(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAnkleAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `hipIndex`(ctype: `hkInt16`)
    /// -        offset: 136(x86)/144(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_hipIndex: i16,
    /// # C++ Info
    /// -          name: `kneeIndex`(ctype: `hkInt16`)
    /// -        offset: 138(x86)/146(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_kneeIndex: i16,
    /// # C++ Info
    /// -          name: `ankleIndex`(ctype: `hkInt16`)
    /// -        offset: 140(x86)/148(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_ankleIndex: i16,
    /// # C++ Info
    /// -          name: `hitSomething`(ctype: `hkBool`)
    /// -        offset: 142(x86)/150(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hitSomething: bool,
    /// # C++ Info
    /// -          name: `isPlantedMS`(ctype: `hkBool`)
    /// -        offset: 143(x86)/151(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isPlantedMS: bool,
    /// # C++ Info
    /// -          name: `isOriginalAnkleTransformMSSet`(ctype: `hkBool`)
    /// -        offset: 144(x86)/152(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isOriginalAnkleTransformMSSet: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbFootIkModifierLeg {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkModifierLeg"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2671655428u32)
        }
    }
    impl __serde::Serialize for hkbFootIkModifierLeg {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2671655428u32)));
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
