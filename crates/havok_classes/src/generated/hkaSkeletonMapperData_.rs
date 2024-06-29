use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSkeletonMapperData`
/// -         version: `1`
/// -       signature: `0x95687ea0`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSkeletonMapperData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `skeletonA`(ctype: `struct hkaSkeleton*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_skeletonA: Pointer,
    /// # C++ Info
    /// -          name: `skeletonB`(ctype: `struct hkaSkeleton*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_skeletonB: Pointer,
    /// # C++ Info
    /// -          name: `simpleMappings`(ctype: `hkArray<struct hkaSkeletonMapperDataSimpleMapping>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_simpleMappings: Vec<hkaSkeletonMapperDataSimpleMapping>,
    /// # C++ Info
    /// -          name: `chainMappings`(ctype: `hkArray<struct hkaSkeletonMapperDataChainMapping>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_chainMappings: Vec<hkaSkeletonMapperDataChainMapping>,
    /// # C++ Info
    /// -          name: `unmappedBones`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_unmappedBones: Vec<i16>,
    /// # C++ Info
    /// -          name: `extractedMotionMapping`(ctype: `hkQsTransform`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_extractedMotionMapping: QsTransform,
    /// # C++ Info
    /// -          name: `keepUnmappedLocal`(ctype: `hkBool`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_keepUnmappedLocal: bool,
    /// # C++ Info
    /// -          name: `mappingType`(ctype: `enum MappingType`)
    /// -        offset: 100(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_mappingType: MappingType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkaSkeletonMapperData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaSkeletonMapperData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2506653344u32)
        }
    }
    impl __serde::Serialize for hkaSkeletonMapperData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkaSkeletonMapperData", class_meta)?;
            serializer.serialize_field("skeletonA", &self.m_skeletonA)?;
            serializer.serialize_field("skeletonB", &self.m_skeletonB)?;
            serializer
                .serialize_array_meta_field("simpleMappings", &self.m_simpleMappings)?;
            serializer
                .serialize_array_meta_field("chainMappings", &self.m_chainMappings)?;
            serializer
                .serialize_array_meta_field("unmappedBones", &self.m_unmappedBones)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field(
                    "extractedMotionMapping",
                    &self.m_extractedMotionMapping,
                )?;
            serializer.serialize_field("keepUnmappedLocal", &self.m_keepUnmappedLocal)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("mappingType", &self.m_mappingType)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_array_field("simpleMappings", &self.m_simpleMappings)?;
            serializer.serialize_array_field("chainMappings", &self.m_chainMappings)?;
            serializer.serialize_array_field("unmappedBones", &self.m_unmappedBones)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT32`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum MappingType {
    #[default]
    HK_RAGDOLL_MAPPING = 0isize,
    HK_RETARGETING_MAPPING = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MappingType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HK_RAGDOLL_MAPPING => {
                    __serializer.serialize_field("HK_RAGDOLL_MAPPING", &0u64)
                }
                Self::HK_RETARGETING_MAPPING => {
                    __serializer.serialize_field("HK_RETARGETING_MAPPING", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i32()
                .ok_or(S::Error::custom("Failed enum MappingType to_i32"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
