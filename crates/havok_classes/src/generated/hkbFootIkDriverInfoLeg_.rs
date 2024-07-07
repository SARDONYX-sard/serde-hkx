use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkDriverInfoLeg`
/// -         version: `0`
/// -       signature: `0x224b18d1`
/// -          size:  96(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkDriverInfoLeg {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `prevAnkleRotLS`(ctype: `hkQuaternion`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_prevAnkleRotLS: Quaternion,
    /// # C++ Info
    /// -          name: `kneeAxisLS`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_kneeAxisLS: Vector4,
    /// # C++ Info
    /// -          name: `footEndLS`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_footEndLS: Vector4,
    /// # C++ Info
    /// -          name: `footPlantedAnkleHeightMS`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_footPlantedAnkleHeightMS: f32,
    /// # C++ Info
    /// -          name: `footRaisedAnkleHeightMS`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_footRaisedAnkleHeightMS: f32,
    /// # C++ Info
    /// -          name: `maxAnkleHeightMS`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAnkleHeightMS: f32,
    /// # C++ Info
    /// -          name: `minAnkleHeightMS`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minAnkleHeightMS: f32,
    /// # C++ Info
    /// -          name: `maxKneeAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxKneeAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `minKneeAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minKneeAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `maxAnkleAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  72(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAnkleAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `hipIndex`(ctype: `hkInt16`)
    /// -        offset:  76(x86)/ 76(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_hipIndex: i16,
    /// # C++ Info
    /// -          name: `kneeIndex`(ctype: `hkInt16`)
    /// -        offset:  78(x86)/ 78(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_kneeIndex: i16,
    /// # C++ Info
    /// -          name: `ankleIndex`(ctype: `hkInt16`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_ankleIndex: i16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbFootIkDriverInfoLeg {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkDriverInfoLeg"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x224b18d1)
        }
    }
    impl _serde::Serialize for hkbFootIkDriverInfoLeg {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x224b18d1)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkDriverInfoLeg", class_meta)?;
            serializer.skip_field("prevAnkleRotLS", &self.m_prevAnkleRotLS)?;
            serializer.serialize_field("kneeAxisLS", &self.m_kneeAxisLS)?;
            serializer.serialize_field("footEndLS", &self.m_footEndLS)?;
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
            serializer
                .serialize_field("maxAnkleAngleDegrees", &self.m_maxAnkleAngleDegrees)?;
            serializer.serialize_field("hipIndex", &self.m_hipIndex)?;
            serializer.serialize_field("kneeIndex", &self.m_kneeIndex)?;
            serializer.serialize_field("ankleIndex", &self.m_ankleIndex)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.end()
        }
    }
};
