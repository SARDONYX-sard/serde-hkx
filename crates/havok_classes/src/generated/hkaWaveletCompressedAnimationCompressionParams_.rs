use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaWaveletCompressedAnimationCompressionParams`
/// -         version: `0`
/// -       signature: `0x27c6cafa`
/// -          size:  36(x86)/ 36(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaWaveletCompressedAnimationCompressionParams {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `quantizationBits`(ctype: `hkUint16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_quantizationBits: u16,
    /// # C++ Info
    /// -          name: `blockSize`(ctype: `hkUint16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_blockSize: u16,
    /// # C++ Info
    /// -          name: `preserve`(ctype: `hkUint16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_preserve: u16,
    /// # C++ Info
    /// -          name: `truncProp`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_truncProp: f32,
    /// # C++ Info
    /// -          name: `useOldStyleTruncation`(ctype: `hkBool`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useOldStyleTruncation: bool,
    /// # C++ Info
    /// -          name: `absolutePositionTolerance`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_absolutePositionTolerance: f32,
    /// # C++ Info
    /// -          name: `relativePositionTolerance`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_relativePositionTolerance: f32,
    /// # C++ Info
    /// -          name: `rotationTolerance`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_rotationTolerance: f32,
    /// # C++ Info
    /// -          name: `scaleTolerance`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_scaleTolerance: f32,
    /// # C++ Info
    /// -          name: `absoluteFloatTolerance`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_absoluteFloatTolerance: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkaWaveletCompressedAnimationCompressionParams {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaWaveletCompressedAnimationCompressionParams"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(667339514u32)
        }
    }
    impl __serde::Serialize for hkaWaveletCompressedAnimationCompressionParams {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaWaveletCompressedAnimationCompressionParams",
                    class_meta,
                )?;
            serializer.serialize_field("quantizationBits", &self.m_quantizationBits)?;
            serializer.serialize_field("blockSize", &self.m_blockSize)?;
            serializer.serialize_field("preserve", &self.m_preserve)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("truncProp", &self.m_truncProp)?;
            serializer
                .serialize_field(
                    "useOldStyleTruncation",
                    &self.m_useOldStyleTruncation,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "absolutePositionTolerance",
                    &self.m_absolutePositionTolerance,
                )?;
            serializer
                .serialize_field(
                    "relativePositionTolerance",
                    &self.m_relativePositionTolerance,
                )?;
            serializer.serialize_field("rotationTolerance", &self.m_rotationTolerance)?;
            serializer.serialize_field("scaleTolerance", &self.m_scaleTolerance)?;
            serializer
                .serialize_field(
                    "absoluteFloatTolerance",
                    &self.m_absoluteFloatTolerance,
                )?;
            serializer.end()
        }
    }
};
