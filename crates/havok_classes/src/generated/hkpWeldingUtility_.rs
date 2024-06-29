use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpWeldingUtility`
/// -         version: `0`
/// -       signature: `0xb2b41feb`
/// -          size:   1(x86)/  1(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpWeldingUtility {
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
    impl __serde::HavokClass for hkpWeldingUtility {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpWeldingUtility"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2998149099u32)
        }
    }
    impl __serde::Serialize for hkpWeldingUtility {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpWeldingUtility", class_meta)?;
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
pub enum WeldingType {
    #[default]
    WELDING_TYPE_ANTICLOCKWISE = 0isize,
    WELDING_TYPE_CLOCKWISE = 4isize,
    WELDING_TYPE_TWO_SIDED = 5isize,
    WELDING_TYPE_NONE = 6isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for WeldingType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::WELDING_TYPE_ANTICLOCKWISE => {
                    __serializer.serialize_field("WELDING_TYPE_ANTICLOCKWISE", &0u64)
                }
                Self::WELDING_TYPE_CLOCKWISE => {
                    __serializer.serialize_field("WELDING_TYPE_CLOCKWISE", &4u64)
                }
                Self::WELDING_TYPE_TWO_SIDED => {
                    __serializer.serialize_field("WELDING_TYPE_TWO_SIDED", &5u64)
                }
                Self::WELDING_TYPE_NONE => {
                    __serializer.serialize_field("WELDING_TYPE_NONE", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum WeldingType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
