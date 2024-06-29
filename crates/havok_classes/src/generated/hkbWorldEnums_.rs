use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbWorldEnums`
/// -         version: `0`
/// -       signature: `0x25640b46`
/// -          size:   1(x86)/  1(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbWorldEnums {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbWorldEnums {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbWorldEnums"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(627313478u32)
        }
    }
    impl __serde::Serialize for hkbWorldEnums {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbWorldEnums", class_meta)?;
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
pub enum SimulationState {
    #[default]
    SIMULATION_STATE_PLAY = 0isize,
    SIMULATION_STATE_PAUSE = 1isize,
    SIMULATION_STATE_STEP = 2isize,
    SIMULATION_STATE_STOP = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SimulationState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::SIMULATION_STATE_PLAY => {
                    __serializer.serialize_field("SIMULATION_STATE_PLAY", &0u64)
                }
                Self::SIMULATION_STATE_PAUSE => {
                    __serializer.serialize_field("SIMULATION_STATE_PAUSE", &1u64)
                }
                Self::SIMULATION_STATE_STEP => {
                    __serializer.serialize_field("SIMULATION_STATE_STEP", &2u64)
                }
                Self::SIMULATION_STATE_STOP => {
                    __serializer.serialize_field("SIMULATION_STATE_STOP", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum SimulationState to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
