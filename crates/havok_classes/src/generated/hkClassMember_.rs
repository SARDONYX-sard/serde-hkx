use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkClassMember`
/// -         version: `0`
/// -       signature: `0x5c7ea4c2`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkClassMember<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `name`(ctype: `char*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: CString<'a>,
    /// # C++ Info
    /// -          name: `class`(ctype: `struct hkClass*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_class: Pointer,
    /// # C++ Info
    /// -          name: `enum`(ctype: `struct hkClassEnum*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_enum: Pointer,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum Type`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: Type,
    /// # C++ Info
    /// -          name: `subtype`(ctype: `enum Type`)
    /// -        offset:  13(x86)/ 25(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_subtype: Type,
    /// # C++ Info
    /// -          name: `cArraySize`(ctype: `hkInt16`)
    /// -        offset:  14(x86)/ 26(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_cArraySize: i16,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags FlagValues`)
    /// -        offset:  16(x86)/ 28(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_flags: FlagValues,
    /// # C++ Info
    /// -          name: `offset`(ctype: `hkUint16`)
    /// -        offset:  18(x86)/ 30(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_offset: u16,
    /// # C++ Info
    /// -          name: `attributes`(ctype: `struct hkCustomAttributes*`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_attributes: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkClassMember<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkClassMember"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1551803586u32)
        }
    }
    impl<'a> __serde::Serialize for hkClassMember<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkClassMember", class_meta)?;
            serializer.serialize_cstring_meta_field("name", &self.m_name)?;
            serializer.serialize_field("class", &self.m_class)?;
            serializer.serialize_field("enum", &self.m_enum)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("subtype", &self.m_subtype)?;
            serializer.serialize_field("cArraySize", &self.m_cArraySize)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.skip_field("attributes", &self.m_attributes)?;
            serializer.serialize_cstring_field("name", &self.m_name)?;
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
pub enum Type {
    #[default]
    TYPE_VOID = 0isize,
    TYPE_BOOL = 1isize,
    TYPE_CHAR = 2isize,
    TYPE_INT8 = 3isize,
    TYPE_UINT8 = 4isize,
    TYPE_INT16 = 5isize,
    TYPE_UINT16 = 6isize,
    TYPE_INT32 = 7isize,
    TYPE_UINT32 = 8isize,
    TYPE_INT64 = 9isize,
    TYPE_UINT64 = 10isize,
    TYPE_REAL = 11isize,
    TYPE_VECTOR4 = 12isize,
    TYPE_QUATERNION = 13isize,
    TYPE_MATRIX3 = 14isize,
    TYPE_ROTATION = 15isize,
    TYPE_QSTRANSFORM = 16isize,
    TYPE_MATRIX4 = 17isize,
    TYPE_TRANSFORM = 18isize,
    TYPE_ZERO = 19isize,
    TYPE_POINTER = 20isize,
    TYPE_FUNCTIONPOINTER = 21isize,
    TYPE_ARRAY = 22isize,
    TYPE_INPLACEARRAY = 23isize,
    TYPE_ENUM = 24isize,
    TYPE_STRUCT = 25isize,
    TYPE_SIMPLEARRAY = 26isize,
    TYPE_HOMOGENEOUSARRAY = 27isize,
    TYPE_VARIANT = 28isize,
    TYPE_CSTRING = 29isize,
    TYPE_ULONG = 30isize,
    TYPE_FLAGS = 31isize,
    TYPE_HALF = 32isize,
    TYPE_STRINGPTR = 33isize,
    TYPE_RELARRAY = 34isize,
    TYPE_MAX = 35isize,
}
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)] pub
    struct FlagValues : u32 { #[doc = "0"] const FLAGS_NONE = 0u32; #[doc = "128"] const
    ALIGN_8 = 128u32; #[doc = "256"] const ALIGN_16 = 256u32; #[doc = "512"] const
    NOT_OWNED = 512u32; #[doc = "1024"] const SERIALIZE_IGNORED = 1024u32; }
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Type {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TYPE_VOID => __serializer.serialize_field("TYPE_VOID", &0u64),
                Self::TYPE_BOOL => __serializer.serialize_field("TYPE_BOOL", &1u64),
                Self::TYPE_CHAR => __serializer.serialize_field("TYPE_CHAR", &2u64),
                Self::TYPE_INT8 => __serializer.serialize_field("TYPE_INT8", &3u64),
                Self::TYPE_UINT8 => __serializer.serialize_field("TYPE_UINT8", &4u64),
                Self::TYPE_INT16 => __serializer.serialize_field("TYPE_INT16", &5u64),
                Self::TYPE_UINT16 => __serializer.serialize_field("TYPE_UINT16", &6u64),
                Self::TYPE_INT32 => __serializer.serialize_field("TYPE_INT32", &7u64),
                Self::TYPE_UINT32 => __serializer.serialize_field("TYPE_UINT32", &8u64),
                Self::TYPE_INT64 => __serializer.serialize_field("TYPE_INT64", &9u64),
                Self::TYPE_UINT64 => __serializer.serialize_field("TYPE_UINT64", &10u64),
                Self::TYPE_REAL => __serializer.serialize_field("TYPE_REAL", &11u64),
                Self::TYPE_VECTOR4 => {
                    __serializer.serialize_field("TYPE_VECTOR4", &12u64)
                }
                Self::TYPE_QUATERNION => {
                    __serializer.serialize_field("TYPE_QUATERNION", &13u64)
                }
                Self::TYPE_MATRIX3 => {
                    __serializer.serialize_field("TYPE_MATRIX3", &14u64)
                }
                Self::TYPE_ROTATION => {
                    __serializer.serialize_field("TYPE_ROTATION", &15u64)
                }
                Self::TYPE_QSTRANSFORM => {
                    __serializer.serialize_field("TYPE_QSTRANSFORM", &16u64)
                }
                Self::TYPE_MATRIX4 => {
                    __serializer.serialize_field("TYPE_MATRIX4", &17u64)
                }
                Self::TYPE_TRANSFORM => {
                    __serializer.serialize_field("TYPE_TRANSFORM", &18u64)
                }
                Self::TYPE_ZERO => __serializer.serialize_field("TYPE_ZERO", &19u64),
                Self::TYPE_POINTER => {
                    __serializer.serialize_field("TYPE_POINTER", &20u64)
                }
                Self::TYPE_FUNCTIONPOINTER => {
                    __serializer.serialize_field("TYPE_FUNCTIONPOINTER", &21u64)
                }
                Self::TYPE_ARRAY => __serializer.serialize_field("TYPE_ARRAY", &22u64),
                Self::TYPE_INPLACEARRAY => {
                    __serializer.serialize_field("TYPE_INPLACEARRAY", &23u64)
                }
                Self::TYPE_ENUM => __serializer.serialize_field("TYPE_ENUM", &24u64),
                Self::TYPE_STRUCT => __serializer.serialize_field("TYPE_STRUCT", &25u64),
                Self::TYPE_SIMPLEARRAY => {
                    __serializer.serialize_field("TYPE_SIMPLEARRAY", &26u64)
                }
                Self::TYPE_HOMOGENEOUSARRAY => {
                    __serializer.serialize_field("TYPE_HOMOGENEOUSARRAY", &27u64)
                }
                Self::TYPE_VARIANT => {
                    __serializer.serialize_field("TYPE_VARIANT", &28u64)
                }
                Self::TYPE_CSTRING => {
                    __serializer.serialize_field("TYPE_CSTRING", &29u64)
                }
                Self::TYPE_ULONG => __serializer.serialize_field("TYPE_ULONG", &30u64),
                Self::TYPE_FLAGS => __serializer.serialize_field("TYPE_FLAGS", &31u64),
                Self::TYPE_HALF => __serializer.serialize_field("TYPE_HALF", &32u64),
                Self::TYPE_STRINGPTR => {
                    __serializer.serialize_field("TYPE_STRINGPTR", &33u64)
                }
                Self::TYPE_RELARRAY => {
                    __serializer.serialize_field("TYPE_RELARRAY", &34u64)
                }
                Self::TYPE_MAX => __serializer.serialize_field("TYPE_MAX", &35u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_u8().ok_or(S::Error::custom("Failed enum Type to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for FlagValues {
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
                    Self::FLAGS_NONE => {
                        __serializer.serialize_field("FLAGS_NONE", &Self::FLAGS_NONE)
                    }
                    Self::ALIGN_8 => {
                        __serializer.serialize_field("ALIGN_8", &Self::ALIGN_8)
                    }
                    Self::ALIGN_16 => {
                        __serializer.serialize_field("ALIGN_16", &Self::ALIGN_16)
                    }
                    Self::NOT_OWNED => {
                        __serializer.serialize_field("NOT_OWNED", &Self::NOT_OWNED)
                    }
                    Self::SERIALIZE_IGNORED => {
                        __serializer
                            .serialize_field(
                                "SERIALIZE_IGNORED",
                                &Self::SERIALIZE_IGNORED,
                            )
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
