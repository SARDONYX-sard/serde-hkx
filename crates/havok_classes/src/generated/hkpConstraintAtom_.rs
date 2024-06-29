use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConstraintAtom`
/// -         version: `0`
/// -       signature: `0x59d67ef6`
/// -          size:   2(x86)/  2(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum AtomType`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_type: AtomType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1507229430u32)
        }
    }
    impl __serde::Serialize for hkpConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT16`
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
pub enum AtomType {
    #[default]
    TYPE_INVALID = 0isize,
    TYPE_BRIDGE = 1isize,
    TYPE_SET_LOCAL_TRANSFORMS = 2isize,
    TYPE_SET_LOCAL_TRANSLATIONS = 3isize,
    TYPE_SET_LOCAL_ROTATIONS = 4isize,
    TYPE_BALL_SOCKET = 5isize,
    TYPE_STIFF_SPRING = 6isize,
    TYPE_LIN = 7isize,
    TYPE_LIN_SOFT = 8isize,
    TYPE_LIN_LIMIT = 9isize,
    TYPE_LIN_FRICTION = 10isize,
    TYPE_LIN_MOTOR = 11isize,
    TYPE_2D_ANG = 12isize,
    TYPE_ANG = 13isize,
    TYPE_ANG_LIMIT = 14isize,
    TYPE_TWIST_LIMIT = 15isize,
    TYPE_CONE_LIMIT = 16isize,
    TYPE_ANG_FRICTION = 17isize,
    TYPE_ANG_MOTOR = 18isize,
    TYPE_RAGDOLL_MOTOR = 19isize,
    TYPE_PULLEY = 20isize,
    TYPE_RACK_AND_PINION = 21isize,
    TYPE_COG_WHEEL = 22isize,
    TYPE_SETUP_STABILIZATION = 23isize,
    TYPE_OVERWRITE_PIVOT = 24isize,
    TYPE_CONTACT = 25isize,
    TYPE_MODIFIER_SOFT_CONTACT = 26isize,
    TYPE_MODIFIER_MASS_CHANGER = 27isize,
    TYPE_MODIFIER_VISCOUS_SURFACE = 28isize,
    TYPE_MODIFIER_MOVING_SURFACE = 29isize,
    TYPE_MODIFIER_IGNORE_CONSTRAINT = 30isize,
    TYPE_MODIFIER_CENTER_OF_MASS_CHANGER = 31isize,
    TYPE_MAX = 32isize,
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
pub enum SolvingMethod {
    #[default]
    METHOD_STABILIZED = 0isize,
    METHOD_OLD = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for AtomType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TYPE_INVALID => __serializer.serialize_field("TYPE_INVALID", &0u64),
                Self::TYPE_BRIDGE => __serializer.serialize_field("TYPE_BRIDGE", &1u64),
                Self::TYPE_SET_LOCAL_TRANSFORMS => {
                    __serializer.serialize_field("TYPE_SET_LOCAL_TRANSFORMS", &2u64)
                }
                Self::TYPE_SET_LOCAL_TRANSLATIONS => {
                    __serializer.serialize_field("TYPE_SET_LOCAL_TRANSLATIONS", &3u64)
                }
                Self::TYPE_SET_LOCAL_ROTATIONS => {
                    __serializer.serialize_field("TYPE_SET_LOCAL_ROTATIONS", &4u64)
                }
                Self::TYPE_BALL_SOCKET => {
                    __serializer.serialize_field("TYPE_BALL_SOCKET", &5u64)
                }
                Self::TYPE_STIFF_SPRING => {
                    __serializer.serialize_field("TYPE_STIFF_SPRING", &6u64)
                }
                Self::TYPE_LIN => __serializer.serialize_field("TYPE_LIN", &7u64),
                Self::TYPE_LIN_SOFT => {
                    __serializer.serialize_field("TYPE_LIN_SOFT", &8u64)
                }
                Self::TYPE_LIN_LIMIT => {
                    __serializer.serialize_field("TYPE_LIN_LIMIT", &9u64)
                }
                Self::TYPE_LIN_FRICTION => {
                    __serializer.serialize_field("TYPE_LIN_FRICTION", &10u64)
                }
                Self::TYPE_LIN_MOTOR => {
                    __serializer.serialize_field("TYPE_LIN_MOTOR", &11u64)
                }
                Self::TYPE_2D_ANG => __serializer.serialize_field("TYPE_2D_ANG", &12u64),
                Self::TYPE_ANG => __serializer.serialize_field("TYPE_ANG", &13u64),
                Self::TYPE_ANG_LIMIT => {
                    __serializer.serialize_field("TYPE_ANG_LIMIT", &14u64)
                }
                Self::TYPE_TWIST_LIMIT => {
                    __serializer.serialize_field("TYPE_TWIST_LIMIT", &15u64)
                }
                Self::TYPE_CONE_LIMIT => {
                    __serializer.serialize_field("TYPE_CONE_LIMIT", &16u64)
                }
                Self::TYPE_ANG_FRICTION => {
                    __serializer.serialize_field("TYPE_ANG_FRICTION", &17u64)
                }
                Self::TYPE_ANG_MOTOR => {
                    __serializer.serialize_field("TYPE_ANG_MOTOR", &18u64)
                }
                Self::TYPE_RAGDOLL_MOTOR => {
                    __serializer.serialize_field("TYPE_RAGDOLL_MOTOR", &19u64)
                }
                Self::TYPE_PULLEY => __serializer.serialize_field("TYPE_PULLEY", &20u64),
                Self::TYPE_RACK_AND_PINION => {
                    __serializer.serialize_field("TYPE_RACK_AND_PINION", &21u64)
                }
                Self::TYPE_COG_WHEEL => {
                    __serializer.serialize_field("TYPE_COG_WHEEL", &22u64)
                }
                Self::TYPE_SETUP_STABILIZATION => {
                    __serializer.serialize_field("TYPE_SETUP_STABILIZATION", &23u64)
                }
                Self::TYPE_OVERWRITE_PIVOT => {
                    __serializer.serialize_field("TYPE_OVERWRITE_PIVOT", &24u64)
                }
                Self::TYPE_CONTACT => {
                    __serializer.serialize_field("TYPE_CONTACT", &25u64)
                }
                Self::TYPE_MODIFIER_SOFT_CONTACT => {
                    __serializer.serialize_field("TYPE_MODIFIER_SOFT_CONTACT", &26u64)
                }
                Self::TYPE_MODIFIER_MASS_CHANGER => {
                    __serializer.serialize_field("TYPE_MODIFIER_MASS_CHANGER", &27u64)
                }
                Self::TYPE_MODIFIER_VISCOUS_SURFACE => {
                    __serializer.serialize_field("TYPE_MODIFIER_VISCOUS_SURFACE", &28u64)
                }
                Self::TYPE_MODIFIER_MOVING_SURFACE => {
                    __serializer.serialize_field("TYPE_MODIFIER_MOVING_SURFACE", &29u64)
                }
                Self::TYPE_MODIFIER_IGNORE_CONSTRAINT => {
                    __serializer
                        .serialize_field("TYPE_MODIFIER_IGNORE_CONSTRAINT", &30u64)
                }
                Self::TYPE_MODIFIER_CENTER_OF_MASS_CHANGER => {
                    __serializer
                        .serialize_field("TYPE_MODIFIER_CENTER_OF_MASS_CHANGER", &31u64)
                }
                Self::TYPE_MAX => __serializer.serialize_field("TYPE_MAX", &32u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u16()
                .ok_or(S::Error::custom("Failed enum AtomType to_u16"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SolvingMethod {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::METHOD_STABILIZED => {
                    __serializer.serialize_field("METHOD_STABILIZED", &0u64)
                }
                Self::METHOD_OLD => __serializer.serialize_field("METHOD_OLD", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum SolvingMethod to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
