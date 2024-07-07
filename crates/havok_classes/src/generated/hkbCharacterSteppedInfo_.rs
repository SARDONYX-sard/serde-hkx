use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterSteppedInfo`
/// -         version: `2`
/// -       signature: `0x2eda84f8`
/// -          size: 112(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterSteppedInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `deltaTime`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_deltaTime: f32,
    /// # C++ Info
    /// -          name: `worldFromModel`(ctype: `hkQsTransform`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_worldFromModel: QsTransform,
    /// # C++ Info
    /// -          name: `poseModelSpace`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_poseModelSpace: Vec<QsTransform>,
    /// # C++ Info
    /// -          name: `rigidAttachmentTransforms`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset:  92(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rigidAttachmentTransforms: Vec<QsTransform>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCharacterSteppedInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterSteppedInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2eda84f8)
        }
    }
    impl _serde::Serialize for hkbCharacterSteppedInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x2eda84f8)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterSteppedInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("deltaTime", &self.m_deltaTime)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("worldFromModel", &self.m_worldFromModel)?;
            serializer
                .serialize_array_meta_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer
                .serialize_array_meta_field(
                    "rigidAttachmentTransforms",
                    &self.m_rigidAttachmentTransforms,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_array_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer
                .serialize_array_field(
                    "rigidAttachmentTransforms",
                    &self.m_rigidAttachmentTransforms,
                )?;
            serializer.end()
        }
    }
};
