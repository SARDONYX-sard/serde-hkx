use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSplineCompressedAnimation`
/// -         version: `0`
/// -       signature: `0x792ee0bb`
/// -          size: 132(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSplineCompressedAnimation<'a> {
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
    /// -          name: `numFrames`(ctype: `hkInt32`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numFrames: i32,
    /// # C++ Info
    /// -          name: `numBlocks`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numBlocks: i32,
    /// # C++ Info
    /// -          name: `maxFramesPerBlock`(ctype: `hkInt32`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxFramesPerBlock: i32,
    /// # C++ Info
    /// -          name: `maskAndQuantizationSize`(ctype: `hkInt32`)
    /// -        offset:  52(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maskAndQuantizationSize: i32,
    /// # C++ Info
    /// -          name: `blockDuration`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blockDuration: f32,
    /// # C++ Info
    /// -          name: `blockInverseDuration`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blockInverseDuration: f32,
    /// # C++ Info
    /// -          name: `frameDuration`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_frameDuration: f32,
    /// # C++ Info
    /// -          name: `blockOffsets`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  68(x86)/ 88(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_blockOffsets: Vec<u32>,
    /// # C++ Info
    /// -          name: `floatBlockOffsets`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  80(x86)/104(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_floatBlockOffsets: Vec<u32>,
    /// # C++ Info
    /// -          name: `transformOffsets`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  92(x86)/120(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_transformOffsets: Vec<u32>,
    /// # C++ Info
    /// -          name: `floatOffsets`(ctype: `hkArray<hkUint32>`)
    /// -        offset: 104(x86)/136(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_floatOffsets: Vec<u32>,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkArray<hkUint8>`)
    /// -        offset: 116(x86)/152(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_data: Vec<u8>,
    /// # C++ Info
    /// -          name: `endian`(ctype: `hkInt32`)
    /// -        offset: 128(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_endian: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkaSplineCompressedAnimation<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaSplineCompressedAnimation"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2033115323u32)
        }
    }
    impl<'a> __serde::Serialize for hkaSplineCompressedAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkaSplineCompressedAnimation", class_meta)?;
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
            serializer.serialize_field("numFrames", &self.m_numFrames)?;
            serializer.serialize_field("numBlocks", &self.m_numBlocks)?;
            serializer.serialize_field("maxFramesPerBlock", &self.m_maxFramesPerBlock)?;
            serializer
                .serialize_field(
                    "maskAndQuantizationSize",
                    &self.m_maskAndQuantizationSize,
                )?;
            serializer.serialize_field("blockDuration", &self.m_blockDuration)?;
            serializer
                .serialize_field("blockInverseDuration", &self.m_blockInverseDuration)?;
            serializer.serialize_field("frameDuration", &self.m_frameDuration)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("blockOffsets", &self.m_blockOffsets)?;
            serializer
                .serialize_array_meta_field(
                    "floatBlockOffsets",
                    &self.m_floatBlockOffsets,
                )?;
            serializer
                .serialize_array_meta_field(
                    "transformOffsets",
                    &self.m_transformOffsets,
                )?;
            serializer.serialize_array_meta_field("floatOffsets", &self.m_floatOffsets)?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_field("endian", &self.m_endian)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "annotationTracks",
                    &self.parent.m_annotationTracks,
                )?;
            serializer.serialize_array_field("blockOffsets", &self.m_blockOffsets)?;
            serializer
                .serialize_array_field("floatBlockOffsets", &self.m_floatBlockOffsets)?;
            serializer
                .serialize_array_field("transformOffsets", &self.m_transformOffsets)?;
            serializer.serialize_array_field("floatOffsets", &self.m_floatOffsets)?;
            serializer.serialize_array_field("data", &self.m_data)?;
            serializer.end()
        }
    }
};
