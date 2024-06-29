use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaDeltaCompressedAnimation`
/// -         version: `0`
/// -       signature: `0x90a68d40`
/// -          size: 120(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaDeltaCompressedAnimation<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkaAnimation<'a>,
    /// # C++ Info
    /// -          name: `numberOfPoses`(ctype: `hkInt32`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numberOfPoses: i32,
    /// # C++ Info
    /// -          name: `blockSize`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blockSize: i32,
    /// # C++ Info
    /// -          name: `qFormat`(ctype: `struct hkaDeltaCompressedAnimationQuantizationFormat`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_qFormat: hkaDeltaCompressedAnimationQuantizationFormat,
    /// # C++ Info
    /// -          name: `quantizedDataIdx`(ctype: `hkUint32`)
    /// -        offset:  68(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_quantizedDataIdx: u32,
    /// # C++ Info
    /// -          name: `quantizedDataSize`(ctype: `hkUint32`)
    /// -        offset:  72(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_quantizedDataSize: u32,
    /// # C++ Info
    /// -          name: `staticMaskIdx`(ctype: `hkUint32`)
    /// -        offset:  76(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticMaskIdx: u32,
    /// # C++ Info
    /// -          name: `staticMaskSize`(ctype: `hkUint32`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticMaskSize: u32,
    /// # C++ Info
    /// -          name: `staticDOFsIdx`(ctype: `hkUint32`)
    /// -        offset:  84(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticDOFsIdx: u32,
    /// # C++ Info
    /// -          name: `staticDOFsSize`(ctype: `hkUint32`)
    /// -        offset:  88(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticDOFsSize: u32,
    /// # C++ Info
    /// -          name: `numStaticTransformDOFs`(ctype: `hkUint32`)
    /// -        offset:  92(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numStaticTransformDOFs: u32,
    /// # C++ Info
    /// -          name: `numDynamicTransformDOFs`(ctype: `hkUint32`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numDynamicTransformDOFs: u32,
    /// # C++ Info
    /// -          name: `totalBlockSize`(ctype: `hkUint32`)
    /// -        offset: 100(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_totalBlockSize: u32,
    /// # C++ Info
    /// -          name: `lastBlockSize`(ctype: `hkUint32`)
    /// -        offset: 104(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lastBlockSize: u32,
    /// # C++ Info
    /// -          name: `dataBuffer`(ctype: `hkArray<hkUint8>`)
    /// -        offset: 108(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_dataBuffer: Vec<u8>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkaDeltaCompressedAnimation<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaDeltaCompressedAnimation"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2426834240u32)
        }
    }
    impl<'a> __serde::Serialize for hkaDeltaCompressedAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkaDeltaCompressedAnimation", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("duration", &self.parent.m_duration)?;
            serializer
                .serialize_field(
                    "numberOfTransformTracks",
                    &self.parent.m_numberOfTransformTracks,
                )?;
            serializer
                .serialize_field(
                    "numberOfFloatTracks",
                    &self.parent.m_numberOfFloatTracks,
                )?;
            serializer
                .serialize_field("extractedMotion", &self.parent.m_extractedMotion)?;
            serializer
                .serialize_array_meta_field(
                    "annotationTracks",
                    &self.parent.m_annotationTracks,
                )?;
            serializer.serialize_field("numberOfPoses", &self.m_numberOfPoses)?;
            serializer.serialize_field("blockSize", &self.m_blockSize)?;
            serializer.serialize_field("qFormat", &self.m_qFormat)?;
            serializer.serialize_field("quantizedDataIdx", &self.m_quantizedDataIdx)?;
            serializer.serialize_field("quantizedDataSize", &self.m_quantizedDataSize)?;
            serializer.serialize_field("staticMaskIdx", &self.m_staticMaskIdx)?;
            serializer.serialize_field("staticMaskSize", &self.m_staticMaskSize)?;
            serializer.serialize_field("staticDOFsIdx", &self.m_staticDOFsIdx)?;
            serializer.serialize_field("staticDOFsSize", &self.m_staticDOFsSize)?;
            serializer
                .serialize_field(
                    "numStaticTransformDOFs",
                    &self.m_numStaticTransformDOFs,
                )?;
            serializer
                .serialize_field(
                    "numDynamicTransformDOFs",
                    &self.m_numDynamicTransformDOFs,
                )?;
            serializer.serialize_field("totalBlockSize", &self.m_totalBlockSize)?;
            serializer.serialize_field("lastBlockSize", &self.m_lastBlockSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("dataBuffer", &self.m_dataBuffer)?;
            serializer
                .serialize_array_field(
                    "annotationTracks",
                    &self.parent.m_annotationTracks,
                )?;
            serializer.serialize_array_field("dataBuffer", &self.m_dataBuffer)?;
            serializer.end()
        }
    }
};
