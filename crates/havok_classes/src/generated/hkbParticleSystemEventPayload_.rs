use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbParticleSystemEventPayload`
/// -         version: `0`
/// -       signature: `0x9df46cd6`
/// -          size:  64(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbParticleSystemEventPayload {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbEventPayload,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum SystemType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: SystemType,
    /// # C++ Info
    /// -          name: `emitBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  10(x86)/ 18(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_emitBoneIndex: i16,
    /// # C++ Info
    /// -          name: `offset`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_offset: Vector4,
    /// # C++ Info
    /// -          name: `direction`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_direction: Vector4,
    /// # C++ Info
    /// -          name: `numParticles`(ctype: `hkInt32`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numParticles: i32,
    /// # C++ Info
    /// -          name: `speed`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_speed: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbParticleSystemEventPayload {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbParticleSystemEventPayload"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2650041558u32)
        }
    }
    impl __serde::Serialize for hkbParticleSystemEventPayload {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbParticleSystemEventPayload", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("emitBoneIndex", &self.m_emitBoneIndex)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.serialize_field("direction", &self.m_direction)?;
            serializer.serialize_field("numParticles", &self.m_numParticles)?;
            serializer.serialize_field("speed", &self.m_speed)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
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
pub enum SystemType {
    #[default]
    DEBRIS = 0isize,
    DUST = 1isize,
    EXPLOSION = 2isize,
    SMOKE = 3isize,
    SPARKS = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SystemType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::DEBRIS => __serializer.serialize_field("DEBRIS", &0u64),
                Self::DUST => __serializer.serialize_field("DUST", &1u64),
                Self::EXPLOSION => __serializer.serialize_field("EXPLOSION", &2u64),
                Self::SMOKE => __serializer.serialize_field("SMOKE", &3u64),
                Self::SPARKS => __serializer.serialize_field("SPARKS", &4u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum SystemType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
