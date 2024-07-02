use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbKeyframeBonesModifierKeyframeInfo`
/// -         version: `0`
/// -       signature: `0x72deb7a6`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbKeyframeBonesModifierKeyframeInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `keyframedPosition`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_keyframedPosition: Vector4,
    /// # C++ Info
    /// -          name: `keyframedRotation`(ctype: `hkQuaternion`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_keyframedRotation: Quaternion,
    /// # C++ Info
    /// -          name: `boneIndex`(ctype: `hkInt16`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_boneIndex: i16,
    /// # C++ Info
    /// -          name: `isValid`(ctype: `hkBool`)
    /// -        offset:  34(x86)/ 34(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isValid: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbKeyframeBonesModifierKeyframeInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbKeyframeBonesModifierKeyframeInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1927198630u32)
        }
    }
    impl _serde::Serialize for hkbKeyframeBonesModifierKeyframeInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1927198630u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbKeyframeBonesModifierKeyframeInfo", class_meta)?;
            serializer.serialize_field("keyframedPosition", &self.m_keyframedPosition)?;
            serializer.serialize_field("keyframedRotation", &self.m_keyframedRotation)?;
            serializer.serialize_field("boneIndex", &self.m_boneIndex)?;
            serializer.serialize_field("isValid", &self.m_isValid)?;
            serializer.pad_field([0u8; 13usize].as_slice(), [0u8; 13usize].as_slice())?;
            serializer.end()
        }
    }
};
