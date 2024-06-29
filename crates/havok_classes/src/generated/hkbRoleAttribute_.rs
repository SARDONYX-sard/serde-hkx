use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbRoleAttribute`
/// -         version: `0`
/// -       signature: `0x3eb2e082`
/// -          size:   4(x86)/  4(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbRoleAttribute {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `role`(ctype: `enum Role`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_role: Role,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags RoleFlags`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_flags: RoleFlags,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbRoleAttribute {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbRoleAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1051910274u32)
        }
    }
    impl __serde::Serialize for hkbRoleAttribute {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbRoleAttribute", class_meta)?;
            serializer.serialize_field("role", &self.m_role)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT16`
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
pub enum Role {
    #[default]
    ROLE_DEFAULT = 0isize,
    ROLE_FILE_NAME = 1isize,
    ROLE_BONE_INDEX = 2isize,
    ROLE_BONE_INDEX_MAP = 3isize,
    ROLE_EVENT_ID = 4isize,
    ROLE_VARIABLE_INDEX = 5isize,
    ROLE_ATTRIBUTE_INDEX = 6isize,
    ROLE_TIME = 7isize,
}
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_INT16`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)] pub
    struct RoleFlags : i16 { #[doc = "0"] const FLAG_NONE = 0i16; #[doc = "1"] const
    FLAG_RAGDOLL = 1i16; #[doc = "2"] const FLAG_NORMALIZED = 2i16; #[doc = "4"] const
    FLAG_NOT_VARIABLE = 4i16; #[doc = "8"] const FLAG_HIDDEN = 8i16; #[doc = "16"] const
    FLAG_OUTPUT = 16i16; #[doc = "32"] const FLAG_NOT_CHARACTER_PROPERTY = 32i16; }
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Role {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ROLE_DEFAULT => __serializer.serialize_field("ROLE_DEFAULT", &0u64),
                Self::ROLE_FILE_NAME => {
                    __serializer.serialize_field("ROLE_FILE_NAME", &1u64)
                }
                Self::ROLE_BONE_INDEX => {
                    __serializer.serialize_field("ROLE_BONE_INDEX", &2u64)
                }
                Self::ROLE_BONE_INDEX_MAP => {
                    __serializer.serialize_field("ROLE_BONE_INDEX_MAP", &3u64)
                }
                Self::ROLE_EVENT_ID => {
                    __serializer.serialize_field("ROLE_EVENT_ID", &4u64)
                }
                Self::ROLE_VARIABLE_INDEX => {
                    __serializer.serialize_field("ROLE_VARIABLE_INDEX", &5u64)
                }
                Self::ROLE_ATTRIBUTE_INDEX => {
                    __serializer.serialize_field("ROLE_ATTRIBUTE_INDEX", &6u64)
                }
                Self::ROLE_TIME => __serializer.serialize_field("ROLE_TIME", &7u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i16().ok_or(S::Error::custom("Failed enum Role to_i16"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for RoleFlags {
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
                    Self::FLAG_NONE => {
                        __serializer.serialize_field("FLAG_NONE", &Self::FLAG_NONE)
                    }
                    Self::FLAG_RAGDOLL => {
                        __serializer.serialize_field("FLAG_RAGDOLL", &Self::FLAG_RAGDOLL)
                    }
                    Self::FLAG_NORMALIZED => {
                        __serializer
                            .serialize_field("FLAG_NORMALIZED", &Self::FLAG_NORMALIZED)
                    }
                    Self::FLAG_NOT_VARIABLE => {
                        __serializer
                            .serialize_field(
                                "FLAG_NOT_VARIABLE",
                                &Self::FLAG_NOT_VARIABLE,
                            )
                    }
                    Self::FLAG_HIDDEN => {
                        __serializer.serialize_field("FLAG_HIDDEN", &Self::FLAG_HIDDEN)
                    }
                    Self::FLAG_OUTPUT => {
                        __serializer.serialize_field("FLAG_OUTPUT", &Self::FLAG_OUTPUT)
                    }
                    Self::FLAG_NOT_CHARACTER_PROPERTY => {
                        __serializer
                            .serialize_field(
                                "FLAG_NOT_CHARACTER_PROPERTY",
                                &Self::FLAG_NOT_CHARACTER_PROPERTY,
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
