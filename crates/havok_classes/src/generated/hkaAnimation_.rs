use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaAnimation`
/// -         version: `1`
/// -       signature: `0xa6fa7e88`
/// -          size:  40(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnimation<'a> {
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
    /// -          name: `type`(ctype: `enum AnimationType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_type: AnimationType,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `numberOfTransformTracks`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numberOfTransformTracks: i32,
    /// # C++ Info
    /// -          name: `numberOfFloatTracks`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numberOfFloatTracks: i32,
    /// # C++ Info
    /// -          name: `extractedMotion`(ctype: `struct hkaAnimatedReferenceFrame*`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_extractedMotion: Pointer,
    /// # C++ Info
    /// -          name: `annotationTracks`(ctype: `hkArray<struct hkaAnnotationTrack>`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_annotationTracks: Vec<hkaAnnotationTrack<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkaAnimation<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaAnimation"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2801434248u32)
        }
    }
    impl<'a> __serde::Serialize for hkaAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkaAnimation", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer
                .serialize_field(
                    "numberOfTransformTracks",
                    &self.m_numberOfTransformTracks,
                )?;
            serializer
                .serialize_field("numberOfFloatTracks", &self.m_numberOfFloatTracks)?;
            serializer.serialize_field("extractedMotion", &self.m_extractedMotion)?;
            serializer
                .serialize_array_meta_field(
                    "annotationTracks",
                    &self.m_annotationTracks,
                )?;
            serializer
                .serialize_array_field("annotationTracks", &self.m_annotationTracks)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT32`
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
pub enum AnimationType {
    #[default]
    HK_UNKNOWN_ANIMATION = 0isize,
    HK_INTERLEAVED_ANIMATION = 1isize,
    HK_DELTA_COMPRESSED_ANIMATION = 2isize,
    HK_WAVELET_COMPRESSED_ANIMATION = 3isize,
    HK_MIRRORED_ANIMATION = 4isize,
    HK_SPLINE_COMPRESSED_ANIMATION = 5isize,
    HK_QUANTIZED_COMPRESSED_ANIMATION = 6isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for AnimationType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HK_UNKNOWN_ANIMATION => {
                    __serializer.serialize_field("HK_UNKNOWN_ANIMATION", &0u64)
                }
                Self::HK_INTERLEAVED_ANIMATION => {
                    __serializer.serialize_field("HK_INTERLEAVED_ANIMATION", &1u64)
                }
                Self::HK_DELTA_COMPRESSED_ANIMATION => {
                    __serializer.serialize_field("HK_DELTA_COMPRESSED_ANIMATION", &2u64)
                }
                Self::HK_WAVELET_COMPRESSED_ANIMATION => {
                    __serializer
                        .serialize_field("HK_WAVELET_COMPRESSED_ANIMATION", &3u64)
                }
                Self::HK_MIRRORED_ANIMATION => {
                    __serializer.serialize_field("HK_MIRRORED_ANIMATION", &4u64)
                }
                Self::HK_SPLINE_COMPRESSED_ANIMATION => {
                    __serializer.serialize_field("HK_SPLINE_COMPRESSED_ANIMATION", &5u64)
                }
                Self::HK_QUANTIZED_COMPRESSED_ANIMATION => {
                    __serializer
                        .serialize_field("HK_QUANTIZED_COMPRESSED_ANIMATION", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i32()
                .ok_or(S::Error::custom("Failed enum AnimationType to_i32"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
