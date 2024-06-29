use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSplineCompressedAnimationTrackCompressionParams`
/// -         version: `0`
/// -       signature: `0x42e878d3`
/// -          size:  28(x86)/ 28(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSplineCompressedAnimationTrackCompressionParams {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `rotationTolerance`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_rotationTolerance: f32,
    /// # C++ Info
    /// -          name: `translationTolerance`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_translationTolerance: f32,
    /// # C++ Info
    /// -          name: `scaleTolerance`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_scaleTolerance: f32,
    /// # C++ Info
    /// -          name: `floatingTolerance`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_floatingTolerance: f32,
    /// # C++ Info
    /// -          name: `rotationDegree`(ctype: `hkUint16`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_rotationDegree: u16,
    /// # C++ Info
    /// -          name: `translationDegree`(ctype: `hkUint16`)
    /// -        offset:  18(x86)/ 18(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_translationDegree: u16,
    /// # C++ Info
    /// -          name: `scaleDegree`(ctype: `hkUint16`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_scaleDegree: u16,
    /// # C++ Info
    /// -          name: `floatingDegree`(ctype: `hkUint16`)
    /// -        offset:  22(x86)/ 22(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_floatingDegree: u16,
    /// # C++ Info
    /// -          name: `rotationQuantizationType`(ctype: `enum RotationQuantization`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_rotationQuantizationType: RotationQuantization,
    /// # C++ Info
    /// -          name: `translationQuantizationType`(ctype: `enum ScalarQuantization`)
    /// -        offset:  25(x86)/ 25(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_translationQuantizationType: ScalarQuantization,
    /// # C++ Info
    /// -          name: `scaleQuantizationType`(ctype: `enum ScalarQuantization`)
    /// -        offset:  26(x86)/ 26(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_scaleQuantizationType: ScalarQuantization,
    /// # C++ Info
    /// -          name: `floatQuantizationType`(ctype: `enum ScalarQuantization`)
    /// -        offset:  27(x86)/ 27(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_floatQuantizationType: ScalarQuantization,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkaSplineCompressedAnimationTrackCompressionParams {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaSplineCompressedAnimationTrackCompressionParams"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1122531539u32)
        }
    }
    impl __serde::Serialize for hkaSplineCompressedAnimationTrackCompressionParams {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaSplineCompressedAnimationTrackCompressionParams",
                    class_meta,
                )?;
            serializer.serialize_field("rotationTolerance", &self.m_rotationTolerance)?;
            serializer
                .serialize_field("translationTolerance", &self.m_translationTolerance)?;
            serializer.serialize_field("scaleTolerance", &self.m_scaleTolerance)?;
            serializer.serialize_field("floatingTolerance", &self.m_floatingTolerance)?;
            serializer.serialize_field("rotationDegree", &self.m_rotationDegree)?;
            serializer.serialize_field("translationDegree", &self.m_translationDegree)?;
            serializer.serialize_field("scaleDegree", &self.m_scaleDegree)?;
            serializer.serialize_field("floatingDegree", &self.m_floatingDegree)?;
            serializer
                .serialize_field(
                    "rotationQuantizationType",
                    &self.m_rotationQuantizationType,
                )?;
            serializer
                .serialize_field(
                    "translationQuantizationType",
                    &self.m_translationQuantizationType,
                )?;
            serializer
                .serialize_field(
                    "scaleQuantizationType",
                    &self.m_scaleQuantizationType,
                )?;
            serializer
                .serialize_field(
                    "floatQuantizationType",
                    &self.m_floatQuantizationType,
                )?;
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
pub enum RotationQuantization {
    #[default]
    POLAR32 = 0isize,
    THREECOMP40 = 1isize,
    THREECOMP48 = 2isize,
    THREECOMP24 = 3isize,
    STRAIGHT16 = 4isize,
    UNCOMPRESSED = 5isize,
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
pub enum ScalarQuantization {
    #[default]
    BITS8 = 0isize,
    BITS16 = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for RotationQuantization {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::POLAR32 => __serializer.serialize_field("POLAR32", &0u64),
                Self::THREECOMP40 => __serializer.serialize_field("THREECOMP40", &1u64),
                Self::THREECOMP48 => __serializer.serialize_field("THREECOMP48", &2u64),
                Self::THREECOMP24 => __serializer.serialize_field("THREECOMP24", &3u64),
                Self::STRAIGHT16 => __serializer.serialize_field("STRAIGHT16", &4u64),
                Self::UNCOMPRESSED => __serializer.serialize_field("UNCOMPRESSED", &5u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum RotationQuantization to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ScalarQuantization {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BITS8 => __serializer.serialize_field("BITS8", &0u64),
                Self::BITS16 => __serializer.serialize_field("BITS16", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ScalarQuantization to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
