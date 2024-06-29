use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConstraintInstance`
/// -         version: `1`
/// -       signature: `0x34eba5f`
/// -          size:  56(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintInstance<'a> {
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
    /// -          name: `owner`(ctype: `void*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_owner: Pointer,
    /// # C++ Info
    /// -          name: `data`(ctype: `struct hkpConstraintData*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_data: Pointer,
    /// # C++ Info
    /// -          name: `constraintModifiers`(ctype: `struct hkpModifierConstraintAtom*`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_constraintModifiers: Pointer,
    /// # C++ Info
    /// -          name: `entities`(ctype: `struct hkpEntity*`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_entities: [Pointer; 2usize],
    /// # C++ Info
    /// -          name: `priority`(ctype: `enum ConstraintPriority`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_priority: ConstraintPriority,
    /// # C++ Info
    /// -          name: `wantRuntime`(ctype: `hkBool`)
    /// -        offset:  29(x86)/ 57(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_wantRuntime: bool,
    /// # C++ Info
    /// -          name: `destructionRemapInfo`(ctype: `enum OnDestructionRemapInfo`)
    /// -        offset:  30(x86)/ 58(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_destructionRemapInfo: OnDestructionRemapInfo,
    /// # C++ Info
    /// -          name: `listeners`(ctype: `struct hkpConstraintInstanceSmallArraySerializeOverrideType`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_listeners: hkpConstraintInstanceSmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  40(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:  44(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
    /// # C++ Info
    /// -          name: `internal`(ctype: `void*`)
    /// -        offset:  48(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_internal: Pointer,
    /// # C++ Info
    /// -          name: `uid`(ctype: `hkUint32`)
    /// -        offset:  52(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_uid: u32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpConstraintInstance<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpConstraintInstance"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(55491167u32)
        }
    }
    impl<'a> __serde::Serialize for hkpConstraintInstance<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintInstance", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("owner", &self.m_owner)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer
                .serialize_field("constraintModifiers", &self.m_constraintModifiers)?;
            serializer.serialize_field("entities", &self.m_entities.as_slice())?;
            serializer.serialize_field("priority", &self.m_priority)?;
            serializer.serialize_field("wantRuntime", &self.m_wantRuntime)?;
            serializer
                .serialize_field("destructionRemapInfo", &self.m_destructionRemapInfo)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.skip_field("listeners", &self.m_listeners)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.skip_field("internal", &self.m_internal)?;
            serializer.skip_field("uid", &self.m_uid)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
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
pub enum ConstraintPriority {
    #[default]
    PRIORITY_INVALID = 0isize,
    PRIORITY_PSI = 1isize,
    PRIORITY_SIMPLIFIED_TOI_UNUSED = 2isize,
    PRIORITY_TOI = 3isize,
    PRIORITY_TOI_HIGHER = 4isize,
    PRIORITY_TOI_FORCED = 5isize,
    NUM_PRIORITIES = 6isize,
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
pub enum OnDestructionRemapInfo {
    #[default]
    ON_DESTRUCTION_REMAP = 0isize,
    ON_DESTRUCTION_REMOVE = 1isize,
    ON_DESTRUCTION_RESET_REMOVE = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ConstraintPriority {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::PRIORITY_INVALID => {
                    __serializer.serialize_field("PRIORITY_INVALID", &0u64)
                }
                Self::PRIORITY_PSI => __serializer.serialize_field("PRIORITY_PSI", &1u64),
                Self::PRIORITY_SIMPLIFIED_TOI_UNUSED => {
                    __serializer.serialize_field("PRIORITY_SIMPLIFIED_TOI_UNUSED", &2u64)
                }
                Self::PRIORITY_TOI => __serializer.serialize_field("PRIORITY_TOI", &3u64),
                Self::PRIORITY_TOI_HIGHER => {
                    __serializer.serialize_field("PRIORITY_TOI_HIGHER", &4u64)
                }
                Self::PRIORITY_TOI_FORCED => {
                    __serializer.serialize_field("PRIORITY_TOI_FORCED", &5u64)
                }
                Self::NUM_PRIORITIES => {
                    __serializer.serialize_field("NUM_PRIORITIES", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ConstraintPriority to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for OnDestructionRemapInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ON_DESTRUCTION_REMAP => {
                    __serializer.serialize_field("ON_DESTRUCTION_REMAP", &0u64)
                }
                Self::ON_DESTRUCTION_REMOVE => {
                    __serializer.serialize_field("ON_DESTRUCTION_REMOVE", &1u64)
                }
                Self::ON_DESTRUCTION_RESET_REMOVE => {
                    __serializer.serialize_field("ON_DESTRUCTION_RESET_REMOVE", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum OnDestructionRemapInfo to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
