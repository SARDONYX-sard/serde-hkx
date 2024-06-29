use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbWorldFromModelModeData`
/// -         version: `0`
/// -       signature: `0xa3af8783`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbWorldFromModelModeData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `poseMatchingBone0`(ctype: `hkInt16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_poseMatchingBone0: i16,
    /// # C++ Info
    /// -          name: `poseMatchingBone1`(ctype: `hkInt16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_poseMatchingBone1: i16,
    /// # C++ Info
    /// -          name: `poseMatchingBone2`(ctype: `hkInt16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_poseMatchingBone2: i16,
    /// # C++ Info
    /// -          name: `mode`(ctype: `enum WorldFromModelMode`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_mode: WorldFromModelMode,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbWorldFromModelModeData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbWorldFromModelModeData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2746189699u32)
        }
    }
    impl __serde::Serialize for hkbWorldFromModelModeData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbWorldFromModelModeData", class_meta)?;
            serializer.serialize_field("poseMatchingBone0", &self.m_poseMatchingBone0)?;
            serializer.serialize_field("poseMatchingBone1", &self.m_poseMatchingBone1)?;
            serializer.serialize_field("poseMatchingBone2", &self.m_poseMatchingBone2)?;
            serializer.serialize_field("mode", &self.m_mode)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
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
pub enum WorldFromModelMode {
    #[default]
    WORLD_FROM_MODEL_MODE_USE_OLD = 0isize,
    WORLD_FROM_MODEL_MODE_USE_INPUT = 1isize,
    WORLD_FROM_MODEL_MODE_COMPUTE = 2isize,
    WORLD_FROM_MODEL_MODE_NONE = 3isize,
    WORLD_FROM_MODEL_MODE_RAGDOLL = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for WorldFromModelMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::WORLD_FROM_MODEL_MODE_USE_OLD => {
                    __serializer.serialize_field("WORLD_FROM_MODEL_MODE_USE_OLD", &0u64)
                }
                Self::WORLD_FROM_MODEL_MODE_USE_INPUT => {
                    __serializer
                        .serialize_field("WORLD_FROM_MODEL_MODE_USE_INPUT", &1u64)
                }
                Self::WORLD_FROM_MODEL_MODE_COMPUTE => {
                    __serializer.serialize_field("WORLD_FROM_MODEL_MODE_COMPUTE", &2u64)
                }
                Self::WORLD_FROM_MODEL_MODE_NONE => {
                    __serializer.serialize_field("WORLD_FROM_MODEL_MODE_NONE", &3u64)
                }
                Self::WORLD_FROM_MODEL_MODE_RAGDOLL => {
                    __serializer.serialize_field("WORLD_FROM_MODEL_MODE_RAGDOLL", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum WorldFromModelMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
