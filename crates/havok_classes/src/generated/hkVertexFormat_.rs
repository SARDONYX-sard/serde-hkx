use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkVertexFormat`
/// -         version: `0`
/// -       signature: `0xf11e3ff7`
/// -          size: 260(x86)/260(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkVertexFormat {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `elements`(ctype: `struct hkVertexFormatElement[32]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/256(x86_64)
    ///
    pub m_elements: [hkVertexFormatElement; 32usize],
    /// # C++ Info
    /// -          name: `numElements`(ctype: `hkInt32`)
    /// -        offset: 256(x86)/256(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numElements: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkVertexFormat {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkVertexFormat"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4045291511u32)
        }
    }
    impl __serde::Serialize for hkVertexFormat {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkVertexFormat", class_meta)?;
            serializer.serialize_field("elements", &self.m_elements.as_slice())?;
            serializer.pad_field([0u8; 248usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("numElements", &self.m_numElements)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT8`
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
pub enum ComponentType {
    #[default]
    TYPE_NONE = 0isize,
    TYPE_INT8 = 1isize,
    TYPE_UINT8 = 2isize,
    TYPE_INT16 = 3isize,
    TYPE_UINT16 = 4isize,
    TYPE_INT32 = 5isize,
    TYPE_UINT32 = 6isize,
    TYPE_UINT8_DWORD = 7isize,
    TYPE_ARGB32 = 8isize,
    TYPE_FLOAT16 = 9isize,
    TYPE_FLOAT32 = 10isize,
    TYPE_VECTOR4 = 11isize,
    TYPE_LAST = 12isize,
}
///- size(C++): `TYPE_UINT8`
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
pub enum ComponentUsage {
    #[default]
    USAGE_NONE = 0isize,
    USAGE_POSITION = 1isize,
    USAGE_NORMAL = 2isize,
    USAGE_COLOR = 3isize,
    USAGE_TANGENT = 4isize,
    USAGE_BINORMAL = 5isize,
    USAGE_BLEND_MATRIX_INDEX = 6isize,
    USAGE_BLEND_WEIGHTS = 7isize,
    USAGE_BLEND_WEIGHTS_LAST_IMPLIED = 8isize,
    USAGE_TEX_COORD = 9isize,
    USAGE_POINT_SIZE = 10isize,
    USAGE_USER = 11isize,
    USAGE_LAST = 12isize,
}
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT8`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)] pub
    struct HintFlags : u8 { #[doc = "1"] const FLAG_READ = 1u8; #[doc = "2"] const
    FLAG_WRITE = 2u8; #[doc = "4"] const FLAG_DYNAMIC = 4u8; #[doc = "8"] const
    FLAG_NOT_SHARED = 8u8; }
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ComponentType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TYPE_NONE => __serializer.serialize_field("TYPE_NONE", &0u64),
                Self::TYPE_INT8 => __serializer.serialize_field("TYPE_INT8", &1u64),
                Self::TYPE_UINT8 => __serializer.serialize_field("TYPE_UINT8", &2u64),
                Self::TYPE_INT16 => __serializer.serialize_field("TYPE_INT16", &3u64),
                Self::TYPE_UINT16 => __serializer.serialize_field("TYPE_UINT16", &4u64),
                Self::TYPE_INT32 => __serializer.serialize_field("TYPE_INT32", &5u64),
                Self::TYPE_UINT32 => __serializer.serialize_field("TYPE_UINT32", &6u64),
                Self::TYPE_UINT8_DWORD => {
                    __serializer.serialize_field("TYPE_UINT8_DWORD", &7u64)
                }
                Self::TYPE_ARGB32 => __serializer.serialize_field("TYPE_ARGB32", &8u64),
                Self::TYPE_FLOAT16 => __serializer.serialize_field("TYPE_FLOAT16", &9u64),
                Self::TYPE_FLOAT32 => {
                    __serializer.serialize_field("TYPE_FLOAT32", &10u64)
                }
                Self::TYPE_VECTOR4 => {
                    __serializer.serialize_field("TYPE_VECTOR4", &11u64)
                }
                Self::TYPE_LAST => __serializer.serialize_field("TYPE_LAST", &12u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ComponentType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ComponentUsage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::USAGE_NONE => __serializer.serialize_field("USAGE_NONE", &0u64),
                Self::USAGE_POSITION => {
                    __serializer.serialize_field("USAGE_POSITION", &1u64)
                }
                Self::USAGE_NORMAL => __serializer.serialize_field("USAGE_NORMAL", &2u64),
                Self::USAGE_COLOR => __serializer.serialize_field("USAGE_COLOR", &3u64),
                Self::USAGE_TANGENT => {
                    __serializer.serialize_field("USAGE_TANGENT", &4u64)
                }
                Self::USAGE_BINORMAL => {
                    __serializer.serialize_field("USAGE_BINORMAL", &5u64)
                }
                Self::USAGE_BLEND_MATRIX_INDEX => {
                    __serializer.serialize_field("USAGE_BLEND_MATRIX_INDEX", &6u64)
                }
                Self::USAGE_BLEND_WEIGHTS => {
                    __serializer.serialize_field("USAGE_BLEND_WEIGHTS", &7u64)
                }
                Self::USAGE_BLEND_WEIGHTS_LAST_IMPLIED => {
                    __serializer
                        .serialize_field("USAGE_BLEND_WEIGHTS_LAST_IMPLIED", &8u64)
                }
                Self::USAGE_TEX_COORD => {
                    __serializer.serialize_field("USAGE_TEX_COORD", &9u64)
                }
                Self::USAGE_POINT_SIZE => {
                    __serializer.serialize_field("USAGE_POINT_SIZE", &10u64)
                }
                Self::USAGE_USER => __serializer.serialize_field("USAGE_USER", &11u64),
                Self::USAGE_LAST => __serializer.serialize_field("USAGE_LAST", &12u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ComponentUsage to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for HintFlags {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::FLAG_READ => {
                        __serializer.serialize_field("FLAG_READ", &Self::FLAG_READ)
                    }
                    Self::FLAG_WRITE => {
                        __serializer.serialize_field("FLAG_WRITE", &Self::FLAG_WRITE)
                    }
                    Self::FLAG_DYNAMIC => {
                        __serializer.serialize_field("FLAG_DYNAMIC", &Self::FLAG_DYNAMIC)
                    }
                    Self::FLAG_NOT_SHARED => {
                        __serializer
                            .serialize_field("FLAG_NOT_SHARED", &Self::FLAG_NOT_SHARED)
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
            __serializer.end()
        }
    }
};
