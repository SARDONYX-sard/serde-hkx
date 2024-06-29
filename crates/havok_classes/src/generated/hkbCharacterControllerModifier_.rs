use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterControllerModifier`
/// -         version: `0`
/// -       signature: `0xf675d6fb`
/// -          size: 144(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterControllerModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `controlData`(ctype: `struct hkbCharacterControllerControlData`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_controlData: hkbCharacterControllerControlData,
    /// # C++ Info
    /// -          name: `initialVelocity`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_initialVelocity: Vector4,
    /// # C++ Info
    /// -          name: `initialVelocityCoordinates`(ctype: `enum InitialVelocityCoordinates`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_initialVelocityCoordinates: InitialVelocityCoordinates,
    /// # C++ Info
    /// -          name: `motionMode`(ctype: `enum MotionMode`)
    /// -        offset:  97(x86)/129(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_motionMode: MotionMode,
    /// # C++ Info
    /// -          name: `forceDownwardMomentum`(ctype: `hkBool`)
    /// -        offset:  98(x86)/130(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_forceDownwardMomentum: bool,
    /// # C++ Info
    /// -          name: `applyGravity`(ctype: `hkBool`)
    /// -        offset:  99(x86)/131(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_applyGravity: bool,
    /// # C++ Info
    /// -          name: `setInitialVelocity`(ctype: `hkBool`)
    /// -        offset: 100(x86)/132(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_setInitialVelocity: bool,
    /// # C++ Info
    /// -          name: `isTouchingGround`(ctype: `hkBool`)
    /// -        offset: 101(x86)/133(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isTouchingGround: bool,
    /// # C++ Info
    /// -          name: `gravity`(ctype: `hkVector4`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_gravity: Vector4,
    /// # C++ Info
    /// -          name: `timestep`(ctype: `hkReal`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timestep: f32,
    /// # C++ Info
    /// -          name: `isInitialVelocityAdded`(ctype: `hkBool`)
    /// -        offset: 132(x86)/164(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isInitialVelocityAdded: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbCharacterControllerModifier<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbCharacterControllerModifier"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4134917883u32)
        }
    }
    impl<'a> __serde::Serialize for hkbCharacterControllerModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterControllerModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("controlData", &self.m_controlData)?;
            serializer.serialize_field("initialVelocity", &self.m_initialVelocity)?;
            serializer
                .serialize_field(
                    "initialVelocityCoordinates",
                    &self.m_initialVelocityCoordinates,
                )?;
            serializer.serialize_field("motionMode", &self.m_motionMode)?;
            serializer
                .serialize_field(
                    "forceDownwardMomentum",
                    &self.m_forceDownwardMomentum,
                )?;
            serializer.serialize_field("applyGravity", &self.m_applyGravity)?;
            serializer
                .serialize_field("setInitialVelocity", &self.m_setInitialVelocity)?;
            serializer.serialize_field("isTouchingGround", &self.m_isTouchingGround)?;
            serializer.pad_field([0u8; 10usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.skip_field("gravity", &self.m_gravity)?;
            serializer.skip_field("timestep", &self.m_timestep)?;
            serializer
                .skip_field("isInitialVelocityAdded", &self.m_isInitialVelocityAdded)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
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
pub enum InitialVelocityCoordinates {
    #[default]
    INITIAL_VELOCITY_IN_WORLD_COORDINATES = 0isize,
    INITIAL_VELOCITY_IN_MODEL_COORDINATES = 1isize,
}
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
pub enum MotionMode {
    #[default]
    MOTION_MODE_FOLLOW_ANIMATION = 0isize,
    MOTION_MODE_DYNAMIC = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for InitialVelocityCoordinates {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::INITIAL_VELOCITY_IN_WORLD_COORDINATES => {
                    __serializer
                        .serialize_field("INITIAL_VELOCITY_IN_WORLD_COORDINATES", &0u64)
                }
                Self::INITIAL_VELOCITY_IN_MODEL_COORDINATES => {
                    __serializer
                        .serialize_field("INITIAL_VELOCITY_IN_MODEL_COORDINATES", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(
                    S::Error::custom("Failed enum InitialVelocityCoordinates to_i8"),
                )?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MotionMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MOTION_MODE_FOLLOW_ANIMATION => {
                    __serializer.serialize_field("MOTION_MODE_FOLLOW_ANIMATION", &0u64)
                }
                Self::MOTION_MODE_DYNAMIC => {
                    __serializer.serialize_field("MOTION_MODE_DYNAMIC", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum MotionMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};