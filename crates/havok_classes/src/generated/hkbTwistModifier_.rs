use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbTwistModifier`
/// -         version: `1`
/// -       signature: `0xb6b76b32`
/// -          size: 112(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbTwistModifier<'a> {
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
    /// -          name: `axisOfRotation`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_axisOfRotation: Vector4,
    /// # C++ Info
    /// -          name: `twistAngle`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_twistAngle: f32,
    /// # C++ Info
    /// -          name: `startBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  68(x86)/100(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_startBoneIndex: i16,
    /// # C++ Info
    /// -          name: `endBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  70(x86)/102(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_endBoneIndex: i16,
    /// # C++ Info
    /// -          name: `setAngleMethod`(ctype: `enum SetAngleMethod`)
    /// -        offset:  72(x86)/104(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_setAngleMethod: SetAngleMethod,
    /// # C++ Info
    /// -          name: `rotationAxisCoordinates`(ctype: `enum RotationAxisCoordinates`)
    /// -        offset:  73(x86)/105(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_rotationAxisCoordinates: RotationAxisCoordinates,
    /// # C++ Info
    /// -          name: `isAdditive`(ctype: `hkBool`)
    /// -        offset:  74(x86)/106(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isAdditive: bool,
    /// # C++ Info
    /// -          name: `boneChainIndices`(ctype: `hkArray<void>`)
    /// -        offset:  76(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_boneChainIndices: Vec<()>,
    /// # C++ Info
    /// -          name: `parentBoneIndices`(ctype: `hkArray<void>`)
    /// -        offset:  88(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_parentBoneIndices: Vec<()>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbTwistModifier<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbTwistModifier"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3065473842u32)
        }
    }
    impl<'a> __serde::Serialize for hkbTwistModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbTwistModifier", class_meta)?;
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
            serializer.serialize_field("axisOfRotation", &self.m_axisOfRotation)?;
            serializer.serialize_field("twistAngle", &self.m_twistAngle)?;
            serializer.serialize_field("startBoneIndex", &self.m_startBoneIndex)?;
            serializer.serialize_field("endBoneIndex", &self.m_endBoneIndex)?;
            serializer.serialize_field("setAngleMethod", &self.m_setAngleMethod)?;
            serializer
                .serialize_field(
                    "rotationAxisCoordinates",
                    &self.m_rotationAxisCoordinates,
                )?;
            serializer.serialize_field("isAdditive", &self.m_isAdditive)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .skip_array_meta_field("boneChainIndices", &self.m_boneChainIndices)?;
            serializer
                .skip_array_meta_field("parentBoneIndices", &self.m_parentBoneIndices)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field("boneChainIndices", &self.m_boneChainIndices)?;
            serializer
                .serialize_array_field("parentBoneIndices", &self.m_parentBoneIndices)?;
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
pub enum SetAngleMethod {
    #[default]
    LINEAR = 0isize,
    RAMPED = 1isize,
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
pub enum RotationAxisCoordinates {
    #[default]
    ROTATION_AXIS_IN_MODEL_COORDINATES = 0isize,
    ROTATION_AXIS_IN_LOCAL_COORDINATES = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SetAngleMethod {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::LINEAR => __serializer.serialize_field("LINEAR", &0u64),
                Self::RAMPED => __serializer.serialize_field("RAMPED", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum SetAngleMethod to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for RotationAxisCoordinates {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ROTATION_AXIS_IN_MODEL_COORDINATES => {
                    __serializer
                        .serialize_field("ROTATION_AXIS_IN_MODEL_COORDINATES", &0u64)
                }
                Self::ROTATION_AXIS_IN_LOCAL_COORDINATES => {
                    __serializer
                        .serialize_field("ROTATION_AXIS_IN_LOCAL_COORDINATES", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum RotationAxisCoordinates to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};