use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterDataCharacterControllerInfo`
/// -         version: `0`
/// -       signature: `0xa0f415bf`
/// -          size:  16(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterDataCharacterControllerInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `capsuleHeight`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_capsuleHeight: f32,
    /// # C++ Info
    /// -          name: `capsuleRadius`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_capsuleRadius: f32,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `characterControllerCinfo`(ctype: `struct hkpCharacterControllerCinfo*`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_characterControllerCinfo: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbCharacterDataCharacterControllerInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterDataCharacterControllerInfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2700350911u32)
        }
    }
    impl __serde::Serialize for hkbCharacterDataCharacterControllerInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2700350911u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbCharacterDataCharacterControllerInfo",
                    class_meta,
                )?;
            serializer.serialize_field("capsuleHeight", &self.m_capsuleHeight)?;
            serializer.serialize_field("capsuleRadius", &self.m_capsuleRadius)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "characterControllerCinfo",
                    &self.m_characterControllerCinfo,
                )?;
            serializer.end()
        }
    }
};
