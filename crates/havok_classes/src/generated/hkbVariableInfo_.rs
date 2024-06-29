use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbVariableInfo`
/// -         version: `1`
/// -       signature: `0x9e746ba2`
/// -          size:   6(x86)/  6(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbVariableInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `role`(ctype: `struct hkbRoleAttribute`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_role: hkbRoleAttribute,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum VariableType`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: VariableType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbVariableInfo {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbVariableInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2658429858u32)
        }
    }
    impl __serde::Serialize for hkbVariableInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbVariableInfo", class_meta)?;
            serializer.serialize_field("role", &self.m_role)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
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
pub enum VariableType {
    #[default]
    VARIABLE_TYPE_INVALID = -1isize,
    VARIABLE_TYPE_BOOL = 0isize,
    VARIABLE_TYPE_INT8 = 1isize,
    VARIABLE_TYPE_INT16 = 2isize,
    VARIABLE_TYPE_INT32 = 3isize,
    VARIABLE_TYPE_REAL = 4isize,
    VARIABLE_TYPE_POINTER = 5isize,
    VARIABLE_TYPE_VECTOR3 = 6isize,
    VARIABLE_TYPE_VECTOR4 = 7isize,
    VARIABLE_TYPE_QUATERNION = 8isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for VariableType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::VARIABLE_TYPE_INVALID => {
                    __serializer
                        .serialize_field(
                            "VARIABLE_TYPE_INVALID",
                            &18446744073709551615u64,
                        )
                }
                Self::VARIABLE_TYPE_BOOL => {
                    __serializer.serialize_field("VARIABLE_TYPE_BOOL", &0u64)
                }
                Self::VARIABLE_TYPE_INT8 => {
                    __serializer.serialize_field("VARIABLE_TYPE_INT8", &1u64)
                }
                Self::VARIABLE_TYPE_INT16 => {
                    __serializer.serialize_field("VARIABLE_TYPE_INT16", &2u64)
                }
                Self::VARIABLE_TYPE_INT32 => {
                    __serializer.serialize_field("VARIABLE_TYPE_INT32", &3u64)
                }
                Self::VARIABLE_TYPE_REAL => {
                    __serializer.serialize_field("VARIABLE_TYPE_REAL", &4u64)
                }
                Self::VARIABLE_TYPE_POINTER => {
                    __serializer.serialize_field("VARIABLE_TYPE_POINTER", &5u64)
                }
                Self::VARIABLE_TYPE_VECTOR3 => {
                    __serializer.serialize_field("VARIABLE_TYPE_VECTOR3", &6u64)
                }
                Self::VARIABLE_TYPE_VECTOR4 => {
                    __serializer.serialize_field("VARIABLE_TYPE_VECTOR4", &7u64)
                }
                Self::VARIABLE_TYPE_QUATERNION => {
                    __serializer.serialize_field("VARIABLE_TYPE_QUATERNION", &8u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum VariableType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
