use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterAddedInfo`
/// -         version: `0`
/// -       signature: `0x3544e182`
/// -          size:  96(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterAddedInfo<'a> {
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
    /// -          name: `instanceName`(ctype: `hkStringPtr`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_instanceName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `templateName`(ctype: `hkStringPtr`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_templateName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `fullPathToProject`(ctype: `hkStringPtr`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_fullPathToProject: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `skeleton`(ctype: `struct hkaSkeleton*`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_skeleton: Pointer,
    /// # C++ Info
    /// -          name: `worldFromModel`(ctype: `hkQsTransform`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_worldFromModel: QsTransform,
    /// # C++ Info
    /// -          name: `poseModelSpace`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_poseModelSpace: Vec<QsTransform>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCharacterAddedInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterAddedInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(893706626u32)
        }
    }
    impl<'a> _serde::Serialize for hkbCharacterAddedInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(893706626u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterAddedInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer
                .serialize_stringptr_meta_field("instanceName", &self.m_instanceName)?;
            serializer
                .serialize_stringptr_meta_field("templateName", &self.m_templateName)?;
            serializer
                .serialize_stringptr_meta_field(
                    "fullPathToProject",
                    &self.m_fullPathToProject,
                )?;
            serializer.serialize_field("skeleton", &self.m_skeleton)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("worldFromModel", &self.m_worldFromModel)?;
            serializer
                .serialize_array_meta_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_stringptr_field("instanceName", &self.m_instanceName)?;
            serializer.serialize_stringptr_field("templateName", &self.m_templateName)?;
            serializer
                .serialize_stringptr_field(
                    "fullPathToProject",
                    &self.m_fullPathToProject,
                )?;
            serializer.serialize_array_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer.end()
        }
    }
};
