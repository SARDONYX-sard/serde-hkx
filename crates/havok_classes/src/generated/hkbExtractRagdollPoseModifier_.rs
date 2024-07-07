use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbExtractRagdollPoseModifier`
/// -         version: `1`
/// -       signature: `0x804dcbab`
/// -          size:  52(x86)/ 88(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbExtractRagdollPoseModifier<'a> {
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
    /// -          name: `poseMatchingBone0`(ctype: `hkInt16`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_poseMatchingBone0: i16,
    /// # C++ Info
    /// -          name: `poseMatchingBone1`(ctype: `hkInt16`)
    /// -        offset:  46(x86)/ 82(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_poseMatchingBone1: i16,
    /// # C++ Info
    /// -          name: `poseMatchingBone2`(ctype: `hkInt16`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_poseMatchingBone2: i16,
    /// # C++ Info
    /// -          name: `enableComputeWorldFromModel`(ctype: `hkBool`)
    /// -        offset:  50(x86)/ 86(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableComputeWorldFromModel: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbExtractRagdollPoseModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbExtractRagdollPoseModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x804dcbab)
        }
    }
    impl<'a> _serde::Serialize for hkbExtractRagdollPoseModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x804dcbab)));
            let mut serializer = __serializer
                .serialize_struct("hkbExtractRagdollPoseModifier", class_meta)?;
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
            serializer.serialize_field("poseMatchingBone0", &self.m_poseMatchingBone0)?;
            serializer.serialize_field("poseMatchingBone1", &self.m_poseMatchingBone1)?;
            serializer.serialize_field("poseMatchingBone2", &self.m_poseMatchingBone2)?;
            serializer
                .serialize_field(
                    "enableComputeWorldFromModel",
                    &self.m_enableComputeWorldFromModel,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
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
