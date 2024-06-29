use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMoppCode`
/// -         version: `0`
/// -       signature: `0x924c2661`
/// -          size:  48(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMoppCode {
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
    /// -          name: `info`(ctype: `struct hkpMoppCodeCodeInfo`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_info: hkpMoppCodeCodeInfo,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_data: Vec<u8>,
    /// # C++ Info
    /// -          name: `buildType`(ctype: `enum BuildType`)
    /// -        offset:  44(x86)/ 48(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_buildType: BuildType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpMoppCode {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpMoppCode"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2454464097u32)
        }
    }
    impl __serde::Serialize for hkpMoppCode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpMoppCode", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("info", &self.m_info)?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_field("buildType", &self.m_buildType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer.serialize_array_field("data", &self.m_data)?;
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
pub enum BuildType {
    #[default]
    BUILT_WITH_CHUNK_SUBDIVISION = 0isize,
    BUILT_WITHOUT_CHUNK_SUBDIVISION = 1isize,
    BUILD_NOT_SET = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BuildType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BUILT_WITH_CHUNK_SUBDIVISION => {
                    __serializer.serialize_field("BUILT_WITH_CHUNK_SUBDIVISION", &0u64)
                }
                Self::BUILT_WITHOUT_CHUNK_SUBDIVISION => {
                    __serializer
                        .serialize_field("BUILT_WITHOUT_CHUNK_SUBDIVISION", &1u64)
                }
                Self::BUILD_NOT_SET => {
                    __serializer.serialize_field("BUILD_NOT_SET", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BuildType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
