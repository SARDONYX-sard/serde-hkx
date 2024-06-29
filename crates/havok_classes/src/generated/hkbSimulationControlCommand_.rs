use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSimulationControlCommand`
/// -         version: `0`
/// -       signature: `0x2a241367`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSimulationControlCommand {
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
    /// -          name: `command`(ctype: `enum SimulationControlCommand`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_command: SimulationControlCommand,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbSimulationControlCommand {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbSimulationControlCommand"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(707007335u32)
        }
    }
    impl __serde::Serialize for hkbSimulationControlCommand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbSimulationControlCommand", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("command", &self.m_command)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
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
pub enum SimulationControlCommand {
    #[default]
    COMMAND_PLAY = 0isize,
    COMMAND_PAUSE = 1isize,
    COMMAND_STEP = 2isize,
    COMMAND_STOP = 3isize,
    COMMAND_ACCUMULATE_MOTION = 4isize,
    COMMAND_DO_NOT_ACCUMULATE_MOTION = 5isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SimulationControlCommand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::COMMAND_PLAY => __serializer.serialize_field("COMMAND_PLAY", &0u64),
                Self::COMMAND_PAUSE => {
                    __serializer.serialize_field("COMMAND_PAUSE", &1u64)
                }
                Self::COMMAND_STEP => __serializer.serialize_field("COMMAND_STEP", &2u64),
                Self::COMMAND_STOP => __serializer.serialize_field("COMMAND_STOP", &3u64),
                Self::COMMAND_ACCUMULATE_MOTION => {
                    __serializer.serialize_field("COMMAND_ACCUMULATE_MOTION", &4u64)
                }
                Self::COMMAND_DO_NOT_ACCUMULATE_MOTION => {
                    __serializer
                        .serialize_field("COMMAND_DO_NOT_ACCUMULATE_MOTION", &5u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum SimulationControlCommand to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
