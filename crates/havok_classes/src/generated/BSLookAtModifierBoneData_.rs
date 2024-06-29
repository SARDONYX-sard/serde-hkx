use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSLookAtModifierBoneData`
/// -         version: `0`
/// -       signature: `0x29efee59`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSLookAtModifierBoneData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `index`(ctype: `hkInt16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_index: i16,
    /// # C++ Info
    /// -          name: `fwdAxisLS`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_fwdAxisLS: Vector4,
    /// # C++ Info
    /// -          name: `limitAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `onGain`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_onGain: f32,
    /// # C++ Info
    /// -          name: `offGain`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offGain: f32,
    /// # C++ Info
    /// -          name: `enabled`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 44(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enabled: bool,
    /// # C++ Info
    /// -          name: `currentFwdAxisLS`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_currentFwdAxisLS: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for BSLookAtModifierBoneData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"BSLookAtModifierBoneData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(703589977u32)
        }
    }
    impl __serde::Serialize for BSLookAtModifierBoneData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("BSLookAtModifierBoneData", class_meta)?;
            serializer.serialize_field("index", &self.m_index)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("fwdAxisLS", &self.m_fwdAxisLS)?;
            serializer.serialize_field("limitAngleDegrees", &self.m_limitAngleDegrees)?;
            serializer.serialize_field("onGain", &self.m_onGain)?;
            serializer.serialize_field("offGain", &self.m_offGain)?;
            serializer.serialize_field("enabled", &self.m_enabled)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("currentFwdAxisLS", &self.m_currentFwdAxisLS)?;
            serializer.end()
        }
    }
};
