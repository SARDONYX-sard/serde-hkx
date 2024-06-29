use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterControlCommand`
/// -         version: `1`
/// -       signature: `0x7a195d1d`
/// -          size:  24(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterControlCommand {
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
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `command`(ctype: `enum CharacterControlCommand`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_command: CharacterControlCommand,
    /// # C++ Info
    /// -          name: `padding`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_padding: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbCharacterControlCommand {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbCharacterControlCommand"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2048482589u32)
        }
    }
    impl __serde::Serialize for hkbCharacterControlCommand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterControlCommand", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("command", &self.m_command)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("padding", &self.m_padding)?;
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
pub enum CharacterControlCommand {
    #[default]
    COMMAND_HIDE = 0isize,
    COMMAND_SHOW = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for CharacterControlCommand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::COMMAND_HIDE => __serializer.serialize_field("COMMAND_HIDE", &0u64),
                Self::COMMAND_SHOW => __serializer.serialize_field("COMMAND_SHOW", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum CharacterControlCommand to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
