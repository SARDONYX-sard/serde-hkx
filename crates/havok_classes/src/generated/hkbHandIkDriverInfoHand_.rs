use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbHandIkDriverInfoHand`
/// -         version: `1`
/// -       signature: `0x14dfe1dd`
/// -          size:  96(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbHandIkDriverInfoHand<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `elbowAxisLS`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_elbowAxisLS: Vector4,
    /// # C++ Info
    /// -          name: `backHandNormalLS`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_backHandNormalLS: Vector4,
    /// # C++ Info
    /// -          name: `handOffsetLS`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_handOffsetLS: Vector4,
    /// # C++ Info
    /// -          name: `handOrienationOffsetLS`(ctype: `hkQuaternion`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_handOrienationOffsetLS: Quaternion,
    /// # C++ Info
    /// -          name: `maxElbowAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxElbowAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `minElbowAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minElbowAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `shoulderIndex`(ctype: `hkInt16`)
    /// -        offset:  72(x86)/ 72(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_shoulderIndex: i16,
    /// # C++ Info
    /// -          name: `shoulderSiblingIndex`(ctype: `hkInt16`)
    /// -        offset:  74(x86)/ 74(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_shoulderSiblingIndex: i16,
    /// # C++ Info
    /// -          name: `elbowIndex`(ctype: `hkInt16`)
    /// -        offset:  76(x86)/ 76(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_elbowIndex: i16,
    /// # C++ Info
    /// -          name: `elbowSiblingIndex`(ctype: `hkInt16`)
    /// -        offset:  78(x86)/ 78(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_elbowSiblingIndex: i16,
    /// # C++ Info
    /// -          name: `wristIndex`(ctype: `hkInt16`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_wristIndex: i16,
    /// # C++ Info
    /// -          name: `enforceEndPosition`(ctype: `hkBool`)
    /// -        offset:  82(x86)/ 82(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enforceEndPosition: bool,
    /// # C++ Info
    /// -          name: `enforceEndRotation`(ctype: `hkBool`)
    /// -        offset:  83(x86)/ 83(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enforceEndRotation: bool,
    /// # C++ Info
    /// -          name: `localFrameName`(ctype: `hkStringPtr`)
    /// -        offset:  84(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_localFrameName: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbHandIkDriverInfoHand<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbHandIkDriverInfoHand"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(350216669u32)
        }
    }
    impl<'a> __serde::Serialize for hkbHandIkDriverInfoHand<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbHandIkDriverInfoHand", class_meta)?;
            serializer.serialize_field("elbowAxisLS", &self.m_elbowAxisLS)?;
            serializer.serialize_field("backHandNormalLS", &self.m_backHandNormalLS)?;
            serializer.serialize_field("handOffsetLS", &self.m_handOffsetLS)?;
            serializer
                .serialize_field(
                    "handOrienationOffsetLS",
                    &self.m_handOrienationOffsetLS,
                )?;
            serializer
                .serialize_field("maxElbowAngleDegrees", &self.m_maxElbowAngleDegrees)?;
            serializer
                .serialize_field("minElbowAngleDegrees", &self.m_minElbowAngleDegrees)?;
            serializer.serialize_field("shoulderIndex", &self.m_shoulderIndex)?;
            serializer
                .serialize_field("shoulderSiblingIndex", &self.m_shoulderSiblingIndex)?;
            serializer.serialize_field("elbowIndex", &self.m_elbowIndex)?;
            serializer.serialize_field("elbowSiblingIndex", &self.m_elbowSiblingIndex)?;
            serializer.serialize_field("wristIndex", &self.m_wristIndex)?;
            serializer
                .serialize_field("enforceEndPosition", &self.m_enforceEndPosition)?;
            serializer
                .serialize_field("enforceEndRotation", &self.m_enforceEndRotation)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "localFrameName",
                    &self.m_localFrameName,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_stringptr_field("localFrameName", &self.m_localFrameName)?;
            serializer.end()
        }
    }
};
