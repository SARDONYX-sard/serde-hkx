use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPointToPathConstraintData`
/// -         version: `0`
/// -       signature: `0x8e7cb5da`
/// -          size: 176(x86)/192(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPointToPathConstraintData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintData,
    /// # C++ Info
    /// -          name: `atoms`(ctype: `struct hkpBridgeAtoms`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_atoms: hkpBridgeAtoms,
    /// # C++ Info
    /// -          name: `path`(ctype: `struct hkpParametricCurve*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_path: Pointer,
    /// # C++ Info
    /// -          name: `maxFrictionForce`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxFrictionForce: f32,
    /// # C++ Info
    /// -          name: `angularConstrainedDOF`(ctype: `enum OrientationConstraintType`)
    /// -        offset:  32(x86)/ 60(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_angularConstrainedDOF: OrientationConstraintType,
    /// # C++ Info
    /// -          name: `transform_OS_KS`(ctype: `hkTransform[2]`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size: 128(x86)/128(x86_64)
    ///
    pub m_transform_OS_KS: [Transform; 2usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpPointToPathConstraintData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpPointToPathConstraintData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2390537690u32)
        }
    }
    impl __serde::Serialize for hkpPointToPathConstraintData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpPointToPathConstraintData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.serialize_field("path", &self.m_path)?;
            serializer.serialize_field("maxFrictionForce", &self.m_maxFrictionForce)?;
            serializer
                .serialize_field(
                    "angularConstrainedDOF",
                    &self.m_angularConstrainedDOF,
                )?;
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field("transform_OS_KS", &self.m_transform_OS_KS.as_slice())?;
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
pub enum OrientationConstraintType {
    #[default]
    CONSTRAIN_ORIENTATION_INVALID = 0isize,
    CONSTRAIN_ORIENTATION_NONE = 1isize,
    CONSTRAIN_ORIENTATION_ALLOW_SPIN = 2isize,
    CONSTRAIN_ORIENTATION_TO_PATH = 3isize,
    CONSTRAIN_ORIENTATION_MAX_ID = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for OrientationConstraintType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::CONSTRAIN_ORIENTATION_INVALID => {
                    __serializer.serialize_field("CONSTRAIN_ORIENTATION_INVALID", &0u64)
                }
                Self::CONSTRAIN_ORIENTATION_NONE => {
                    __serializer.serialize_field("CONSTRAIN_ORIENTATION_NONE", &1u64)
                }
                Self::CONSTRAIN_ORIENTATION_ALLOW_SPIN => {
                    __serializer
                        .serialize_field("CONSTRAIN_ORIENTATION_ALLOW_SPIN", &2u64)
                }
                Self::CONSTRAIN_ORIENTATION_TO_PATH => {
                    __serializer.serialize_field("CONSTRAIN_ORIENTATION_TO_PATH", &3u64)
                }
                Self::CONSTRAIN_ORIENTATION_MAX_ID => {
                    __serializer.serialize_field("CONSTRAIN_ORIENTATION_MAX_ID", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum OrientationConstraintType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
