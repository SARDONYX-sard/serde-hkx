use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterSetup`
/// -         version: `2`
/// -       signature: `0xe5a2a413`
/// -          size:  48(x86)/ 88(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterSetup {
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
    /// -          name: `retargetingSkeletonMappers`(ctype: `hkArray<hkaSkeletonMapper*>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_retargetingSkeletonMappers: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `animationSkeleton`(ctype: `struct hkaSkeleton*`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_animationSkeleton: Pointer,
    /// # C++ Info
    /// -          name: `ragdollToAnimationSkeletonMapper`(ctype: `struct hkaSkeletonMapper*`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_ragdollToAnimationSkeletonMapper: Pointer,
    /// # C++ Info
    /// -          name: `animationToRagdollSkeletonMapper`(ctype: `struct hkaSkeletonMapper*`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_animationToRagdollSkeletonMapper: Pointer,
    /// # C++ Info
    /// -          name: `animationBindingSet`(ctype: `void*`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_animationBindingSet: Pointer,
    /// # C++ Info
    /// -          name: `data`(ctype: `struct hkbCharacterData*`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_data: Pointer,
    /// # C++ Info
    /// -          name: `mirroredSkeleton`(ctype: `void*`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_mirroredSkeleton: Pointer,
    /// # C++ Info
    /// -          name: `characterPropertyIdMap`(ctype: `void*`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_characterPropertyIdMap: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbCharacterSetup {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterSetup"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3852641299u32)
        }
    }
    impl __serde::Serialize for hkbCharacterSetup {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3852641299u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterSetup", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "retargetingSkeletonMappers",
                    &self.m_retargetingSkeletonMappers,
                )?;
            serializer.serialize_field("animationSkeleton", &self.m_animationSkeleton)?;
            serializer
                .serialize_field(
                    "ragdollToAnimationSkeletonMapper",
                    &self.m_ragdollToAnimationSkeletonMapper,
                )?;
            serializer
                .serialize_field(
                    "animationToRagdollSkeletonMapper",
                    &self.m_animationToRagdollSkeletonMapper,
                )?;
            serializer.skip_field("animationBindingSet", &self.m_animationBindingSet)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer.skip_field("mirroredSkeleton", &self.m_mirroredSkeleton)?;
            serializer
                .skip_field("characterPropertyIdMap", &self.m_characterPropertyIdMap)?;
            serializer
                .serialize_array_field(
                    "retargetingSkeletonMappers",
                    &self.m_retargetingSkeletonMappers,
                )?;
            serializer.end()
        }
    }
};
