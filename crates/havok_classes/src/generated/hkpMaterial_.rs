use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMaterial`
/// -         version: `2`
/// -       signature: `0x33be6570`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMaterial {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `responseType`(ctype: `enum ResponseType`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_responseType: ResponseType,
    /// # C++ Info
    /// -          name: `rollingFrictionMultiplier`(ctype: `hkHalf`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_rollingFrictionMultiplier: f16,
    /// # C++ Info
    /// -          name: `friction`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_friction: f32,
    /// # C++ Info
    /// -          name: `restitution`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_restitution: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpMaterial {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpMaterial"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(868115824u32)
        }
    }
    impl __serde::Serialize for hkpMaterial {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpMaterial", class_meta)?;
            serializer.serialize_field("responseType", &self.m_responseType)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "rollingFrictionMultiplier",
                    &self.m_rollingFrictionMultiplier,
                )?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("restitution", &self.m_restitution)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
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
pub enum ResponseType {
    #[default]
    RESPONSE_INVALID = 0isize,
    RESPONSE_SIMPLE_CONTACT = 1isize,
    RESPONSE_REPORTING = 2isize,
    RESPONSE_NONE = 3isize,
    RESPONSE_MAX_ID = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ResponseType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::RESPONSE_INVALID => {
                    __serializer.serialize_field("RESPONSE_INVALID", &0u64)
                }
                Self::RESPONSE_SIMPLE_CONTACT => {
                    __serializer.serialize_field("RESPONSE_SIMPLE_CONTACT", &1u64)
                }
                Self::RESPONSE_REPORTING => {
                    __serializer.serialize_field("RESPONSE_REPORTING", &2u64)
                }
                Self::RESPONSE_NONE => {
                    __serializer.serialize_field("RESPONSE_NONE", &3u64)
                }
                Self::RESPONSE_MAX_ID => {
                    __serializer.serialize_field("RESPONSE_MAX_ID", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum ResponseType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
