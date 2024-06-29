use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbPoweredRagdollControlsModifier`
/// -         version: `5`
/// -       signature: `0x7cb54065`
/// -          size:  96(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbPoweredRagdollControlsModifier<'a> {
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
    /// -          name: `controlData`(ctype: `struct hkbPoweredRagdollControlData`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_controlData: hkbPoweredRagdollControlData,
    /// # C++ Info
    /// -          name: `bones`(ctype: `struct hkbBoneIndexArray*`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bones: Pointer,
    /// # C++ Info
    /// -          name: `worldFromModelModeData`(ctype: `struct hkbWorldFromModelModeData`)
    /// -        offset:  84(x86)/120(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_worldFromModelModeData: hkbWorldFromModelModeData,
    /// # C++ Info
    /// -          name: `boneWeights`(ctype: `struct hkbBoneWeightArray*`)
    /// -        offset:  92(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_boneWeights: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbPoweredRagdollControlsModifier<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbPoweredRagdollControlsModifier"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2092253285u32)
        }
    }
    impl<'a> __serde::Serialize for hkbPoweredRagdollControlsModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbPoweredRagdollControlsModifier", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("controlData", &self.m_controlData)?;
            serializer.serialize_field("bones", &self.m_bones)?;
            serializer
                .serialize_field(
                    "worldFromModelModeData",
                    &self.m_worldFromModelModeData,
                )?;
            serializer.serialize_field("boneWeights", &self.m_boneWeights)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
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