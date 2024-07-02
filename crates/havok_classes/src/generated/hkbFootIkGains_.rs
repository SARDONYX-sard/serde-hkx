use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkGains`
/// -         version: `0`
/// -       signature: `0xa681b7f0`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkGains {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `onOffGain`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_onOffGain: f32,
    /// # C++ Info
    /// -          name: `groundAscendingGain`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_groundAscendingGain: f32,
    /// # C++ Info
    /// -          name: `groundDescendingGain`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_groundDescendingGain: f32,
    /// # C++ Info
    /// -          name: `footPlantedGain`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_footPlantedGain: f32,
    /// # C++ Info
    /// -          name: `footRaisedGain`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_footRaisedGain: f32,
    /// # C++ Info
    /// -          name: `footUnlockGain`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_footUnlockGain: f32,
    /// # C++ Info
    /// -          name: `worldFromModelFeedbackGain`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_worldFromModelFeedbackGain: f32,
    /// # C++ Info
    /// -          name: `errorUpDownBias`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_errorUpDownBias: f32,
    /// # C++ Info
    /// -          name: `alignWorldFromModelGain`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_alignWorldFromModelGain: f32,
    /// # C++ Info
    /// -          name: `hipOrientationGain`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_hipOrientationGain: f32,
    /// # C++ Info
    /// -          name: `maxKneeAngleDifference`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxKneeAngleDifference: f32,
    /// # C++ Info
    /// -          name: `ankleOrientationGain`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_ankleOrientationGain: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbFootIkGains {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkGains"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2793519088u32)
        }
    }
    impl __serde::Serialize for hkbFootIkGains {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2793519088u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkGains", class_meta)?;
            serializer.serialize_field("onOffGain", &self.m_onOffGain)?;
            serializer
                .serialize_field("groundAscendingGain", &self.m_groundAscendingGain)?;
            serializer
                .serialize_field("groundDescendingGain", &self.m_groundDescendingGain)?;
            serializer.serialize_field("footPlantedGain", &self.m_footPlantedGain)?;
            serializer.serialize_field("footRaisedGain", &self.m_footRaisedGain)?;
            serializer.serialize_field("footUnlockGain", &self.m_footUnlockGain)?;
            serializer
                .serialize_field(
                    "worldFromModelFeedbackGain",
                    &self.m_worldFromModelFeedbackGain,
                )?;
            serializer.serialize_field("errorUpDownBias", &self.m_errorUpDownBias)?;
            serializer
                .serialize_field(
                    "alignWorldFromModelGain",
                    &self.m_alignWorldFromModelGain,
                )?;
            serializer
                .serialize_field("hipOrientationGain", &self.m_hipOrientationGain)?;
            serializer
                .serialize_field(
                    "maxKneeAngleDifference",
                    &self.m_maxKneeAngleDifference,
                )?;
            serializer
                .serialize_field("ankleOrientationGain", &self.m_ankleOrientationGain)?;
            serializer.end()
        }
    }
};
