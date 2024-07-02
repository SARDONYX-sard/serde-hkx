use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterStringData`
/// -         version: `5`
/// -       signature: `0x655b42bc`
/// -          size: 132(x86)/192(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterStringData<'a> {
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
    /// -          name: `deformableSkinNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_deformableSkinNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `rigidSkinNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rigidSkinNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `animationNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_animationNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `animationFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_animationFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `characterPropertyNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_characterPropertyNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `retargetingSkeletonMapperFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  68(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_retargetingSkeletonMapperFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `lodNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_lodNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `mirroredSyncPointSubstringsA`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  92(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_mirroredSyncPointSubstringsA: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `mirroredSyncPointSubstringsB`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset: 104(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_mirroredSyncPointSubstringsB: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset: 116(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `rigName`(ctype: `hkStringPtr`)
    /// -        offset: 120(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rigName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `ragdollName`(ctype: `hkStringPtr`)
    /// -        offset: 124(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_ragdollName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `behaviorFilename`(ctype: `hkStringPtr`)
    /// -        offset: 128(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behaviorFilename: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCharacterStringData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterStringData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1700479676u32)
        }
    }
    impl<'a> _serde::Serialize for hkbCharacterStringData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1700479676u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterStringData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "deformableSkinNames",
                    &self.m_deformableSkinNames,
                )?;
            serializer
                .serialize_array_meta_field("rigidSkinNames", &self.m_rigidSkinNames)?;
            serializer
                .serialize_array_meta_field("animationNames", &self.m_animationNames)?;
            serializer
                .serialize_array_meta_field(
                    "animationFilenames",
                    &self.m_animationFilenames,
                )?;
            serializer
                .serialize_array_meta_field(
                    "characterPropertyNames",
                    &self.m_characterPropertyNames,
                )?;
            serializer
                .serialize_array_meta_field(
                    "retargetingSkeletonMapperFilenames",
                    &self.m_retargetingSkeletonMapperFilenames,
                )?;
            serializer.serialize_array_meta_field("lodNames", &self.m_lodNames)?;
            serializer
                .serialize_array_meta_field(
                    "mirroredSyncPointSubstringsA",
                    &self.m_mirroredSyncPointSubstringsA,
                )?;
            serializer
                .serialize_array_meta_field(
                    "mirroredSyncPointSubstringsB",
                    &self.m_mirroredSyncPointSubstringsB,
                )?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_stringptr_meta_field("rigName", &self.m_rigName)?;
            serializer
                .serialize_stringptr_meta_field("ragdollName", &self.m_ragdollName)?;
            serializer
                .serialize_stringptr_meta_field(
                    "behaviorFilename",
                    &self.m_behaviorFilename,
                )?;
            serializer
                .serialize_array_field(
                    "deformableSkinNames",
                    &self.m_deformableSkinNames,
                )?;
            serializer.serialize_array_field("rigidSkinNames", &self.m_rigidSkinNames)?;
            serializer.serialize_array_field("animationNames", &self.m_animationNames)?;
            serializer
                .serialize_array_field(
                    "animationFilenames",
                    &self.m_animationFilenames,
                )?;
            serializer
                .serialize_array_field(
                    "characterPropertyNames",
                    &self.m_characterPropertyNames,
                )?;
            serializer
                .serialize_array_field(
                    "retargetingSkeletonMapperFilenames",
                    &self.m_retargetingSkeletonMapperFilenames,
                )?;
            serializer.serialize_array_field("lodNames", &self.m_lodNames)?;
            serializer
                .serialize_array_field(
                    "mirroredSyncPointSubstringsA",
                    &self.m_mirroredSyncPointSubstringsA,
                )?;
            serializer
                .serialize_array_field(
                    "mirroredSyncPointSubstringsB",
                    &self.m_mirroredSyncPointSubstringsB,
                )?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_stringptr_field("rigName", &self.m_rigName)?;
            serializer.serialize_stringptr_field("ragdollName", &self.m_ragdollName)?;
            serializer
                .serialize_stringptr_field(
                    "behaviorFilename",
                    &self.m_behaviorFilename,
                )?;
            serializer.end()
        }
    }
};
