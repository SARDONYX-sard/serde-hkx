use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxVertexDescription`
/// -         version: `1`
/// -       signature: `0x2df6313d`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxVertexDescription {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `decls`(ctype: `hkArray<struct hkxVertexDescriptionElementDecl>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_decls: Vec<hkxVertexDescriptionElementDecl>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxVertexDescription {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxVertexDescription"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(771109181u32)
        }
    }
    impl __serde::Serialize for hkxVertexDescription {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxVertexDescription", class_meta)?;
            serializer.serialize_array_meta_field("decls", &self.m_decls)?;
            serializer.serialize_array_field("decls", &self.m_decls)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT16`
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
pub enum DataType {
    #[default]
    HKX_DT_NONE = 0isize,
    HKX_DT_UINT8 = 1isize,
    HKX_DT_INT16 = 2isize,
    HKX_DT_UINT32 = 3isize,
    HKX_DT_FLOAT = 4isize,
}
///- size(C++): `TYPE_UINT16`
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
pub enum DataUsage {
    #[default]
    HKX_DU_NONE = 0isize,
    HKX_DU_POSITION = 1isize,
    HKX_DU_COLOR = 2isize,
    HKX_DU_NORMAL = 4isize,
    HKX_DU_TANGENT = 8isize,
    HKX_DU_BINORMAL = 16isize,
    HKX_DU_TEXCOORD = 32isize,
    HKX_DU_BLENDWEIGHTS = 64isize,
    HKX_DU_BLENDINDICES = 128isize,
    HKX_DU_USERDATA = 256isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for DataType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HKX_DT_NONE => __serializer.serialize_field("HKX_DT_NONE", &0u64),
                Self::HKX_DT_UINT8 => __serializer.serialize_field("HKX_DT_UINT8", &1u64),
                Self::HKX_DT_INT16 => __serializer.serialize_field("HKX_DT_INT16", &2u64),
                Self::HKX_DT_UINT32 => {
                    __serializer.serialize_field("HKX_DT_UINT32", &3u64)
                }
                Self::HKX_DT_FLOAT => __serializer.serialize_field("HKX_DT_FLOAT", &4u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u16()
                .ok_or(S::Error::custom("Failed enum DataType to_u16"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for DataUsage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HKX_DU_NONE => __serializer.serialize_field("HKX_DU_NONE", &0u64),
                Self::HKX_DU_POSITION => {
                    __serializer.serialize_field("HKX_DU_POSITION", &1u64)
                }
                Self::HKX_DU_COLOR => __serializer.serialize_field("HKX_DU_COLOR", &2u64),
                Self::HKX_DU_NORMAL => {
                    __serializer.serialize_field("HKX_DU_NORMAL", &4u64)
                }
                Self::HKX_DU_TANGENT => {
                    __serializer.serialize_field("HKX_DU_TANGENT", &8u64)
                }
                Self::HKX_DU_BINORMAL => {
                    __serializer.serialize_field("HKX_DU_BINORMAL", &16u64)
                }
                Self::HKX_DU_TEXCOORD => {
                    __serializer.serialize_field("HKX_DU_TEXCOORD", &32u64)
                }
                Self::HKX_DU_BLENDWEIGHTS => {
                    __serializer.serialize_field("HKX_DU_BLENDWEIGHTS", &64u64)
                }
                Self::HKX_DU_BLENDINDICES => {
                    __serializer.serialize_field("HKX_DU_BLENDINDICES", &128u64)
                }
                Self::HKX_DU_USERDATA => {
                    __serializer.serialize_field("HKX_DU_USERDATA", &256u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u16()
                .ok_or(S::Error::custom("Failed enum DataUsage to_u16"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
