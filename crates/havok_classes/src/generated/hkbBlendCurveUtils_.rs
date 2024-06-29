use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBlendCurveUtils`
/// -         version: `0`
/// -       signature: `0x23041af0`
/// -          size:   1(x86)/  1(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlendCurveUtils {
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
    impl __serde::HavokClass for hkbBlendCurveUtils {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbBlendCurveUtils"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(587471600u32)
        }
    }
    impl __serde::Serialize for hkbBlendCurveUtils {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbBlendCurveUtils", class_meta)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
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
pub enum BlendCurve {
    #[default]
    BLEND_CURVE_SMOOTH = 0isize,
    BLEND_CURVE_LINEAR = 1isize,
    BLEND_CURVE_LINEAR_TO_SMOOTH = 2isize,
    BLEND_CURVE_SMOOTH_TO_LINEAR = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BlendCurve {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BLEND_CURVE_SMOOTH => {
                    __serializer.serialize_field("BLEND_CURVE_SMOOTH", &0u64)
                }
                Self::BLEND_CURVE_LINEAR => {
                    __serializer.serialize_field("BLEND_CURVE_LINEAR", &1u64)
                }
                Self::BLEND_CURVE_LINEAR_TO_SMOOTH => {
                    __serializer.serialize_field("BLEND_CURVE_LINEAR_TO_SMOOTH", &2u64)
                }
                Self::BLEND_CURVE_SMOOTH_TO_LINEAR => {
                    __serializer.serialize_field("BLEND_CURVE_SMOOTH_TO_LINEAR", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BlendCurve to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
