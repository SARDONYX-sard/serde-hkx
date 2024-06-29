use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMotion`
/// -         version: `3`
/// -       signature: `0x98aadb4f`
/// -          size: 288(x86)/320(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMotion {
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
    /// -          name: `type`(ctype: `enum MotionType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: MotionType,
    /// # C++ Info
    /// -          name: `deactivationIntegrateCounter`(ctype: `hkUint8`)
    /// -        offset:   9(x86)/ 17(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_deactivationIntegrateCounter: u8,
    /// # C++ Info
    /// -          name: `deactivationNumInactiveFrames`(ctype: `hkUint16[2]`)
    /// -        offset:  10(x86)/ 18(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_deactivationNumInactiveFrames: [u16; 2usize],
    /// # C++ Info
    /// -          name: `motionState`(ctype: `struct hkMotionState`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size: 176(x86)/176(x86_64)
    ///
    pub m_motionState: hkMotionState,
    /// # C++ Info
    /// -          name: `inertiaAndMassInv`(ctype: `hkVector4`)
    /// -        offset: 192(x86)/208(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_inertiaAndMassInv: Vector4,
    /// # C++ Info
    /// -          name: `linearVelocity`(ctype: `hkVector4`)
    /// -        offset: 208(x86)/224(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_linearVelocity: Vector4,
    /// # C++ Info
    /// -          name: `angularVelocity`(ctype: `hkVector4`)
    /// -        offset: 224(x86)/240(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_angularVelocity: Vector4,
    /// # C++ Info
    /// -          name: `deactivationRefPosition`(ctype: `hkVector4[2]`)
    /// -        offset: 240(x86)/256(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_deactivationRefPosition: [Vector4; 2usize],
    /// # C++ Info
    /// -          name: `deactivationRefOrientation`(ctype: `hkUint32[2]`)
    /// -        offset: 272(x86)/288(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_deactivationRefOrientation: [u32; 2usize],
    /// # C++ Info
    /// -          name: `savedMotion`(ctype: `struct hkpMaxSizeMotion*`)
    /// -        offset: 280(x86)/296(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_savedMotion: Pointer,
    /// # C++ Info
    /// -          name: `savedQualityTypeIndex`(ctype: `hkUint16`)
    /// -        offset: 284(x86)/304(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_savedQualityTypeIndex: u16,
    /// # C++ Info
    /// -          name: `gravityFactor`(ctype: `hkHalf`)
    /// -        offset: 286(x86)/306(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_gravityFactor: f16,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpMotion {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpMotion"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2561334095u32)
        }
    }
    impl __serde::Serialize for hkpMotion {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer.serialize_struct("hkpMotion", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer
                .serialize_field(
                    "deactivationIntegrateCounter",
                    &self.m_deactivationIntegrateCounter,
                )?;
            serializer
                .serialize_field(
                    "deactivationNumInactiveFrames",
                    &self.m_deactivationNumInactiveFrames.as_slice(),
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.serialize_field("motionState", &self.m_motionState)?;
            serializer.serialize_field("inertiaAndMassInv", &self.m_inertiaAndMassInv)?;
            serializer.serialize_field("linearVelocity", &self.m_linearVelocity)?;
            serializer.serialize_field("angularVelocity", &self.m_angularVelocity)?;
            serializer
                .serialize_field(
                    "deactivationRefPosition",
                    &self.m_deactivationRefPosition.as_slice(),
                )?;
            serializer
                .serialize_field(
                    "deactivationRefOrientation",
                    &self.m_deactivationRefOrientation.as_slice(),
                )?;
            serializer.serialize_field("savedMotion", &self.m_savedMotion)?;
            serializer
                .serialize_field(
                    "savedQualityTypeIndex",
                    &self.m_savedQualityTypeIndex,
                )?;
            serializer.serialize_field("gravityFactor", &self.m_gravityFactor)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
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
pub enum MotionType {
    #[default]
    MOTION_INVALID = 0isize,
    MOTION_DYNAMIC = 1isize,
    MOTION_SPHERE_INERTIA = 2isize,
    MOTION_BOX_INERTIA = 3isize,
    MOTION_KEYFRAMED = 4isize,
    MOTION_FIXED = 5isize,
    MOTION_THIN_BOX_INERTIA = 6isize,
    MOTION_CHARACTER = 7isize,
    MOTION_MAX_ID = 8isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MotionType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MOTION_INVALID => {
                    __serializer.serialize_field("MOTION_INVALID", &0u64)
                }
                Self::MOTION_DYNAMIC => {
                    __serializer.serialize_field("MOTION_DYNAMIC", &1u64)
                }
                Self::MOTION_SPHERE_INERTIA => {
                    __serializer.serialize_field("MOTION_SPHERE_INERTIA", &2u64)
                }
                Self::MOTION_BOX_INERTIA => {
                    __serializer.serialize_field("MOTION_BOX_INERTIA", &3u64)
                }
                Self::MOTION_KEYFRAMED => {
                    __serializer.serialize_field("MOTION_KEYFRAMED", &4u64)
                }
                Self::MOTION_FIXED => __serializer.serialize_field("MOTION_FIXED", &5u64),
                Self::MOTION_THIN_BOX_INERTIA => {
                    __serializer.serialize_field("MOTION_THIN_BOX_INERTIA", &6u64)
                }
                Self::MOTION_CHARACTER => {
                    __serializer.serialize_field("MOTION_CHARACTER", &7u64)
                }
                Self::MOTION_MAX_ID => {
                    __serializer.serialize_field("MOTION_MAX_ID", &8u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum MotionType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
