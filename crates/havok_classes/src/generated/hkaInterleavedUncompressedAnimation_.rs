use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaInterleavedUncompressedAnimation`
/// -         version: `0`
/// -       signature: `0x930af031`
/// -          size:  64(x86)/ 88(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaInterleavedUncompressedAnimation<'a> {
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
    /// -          name: `transforms`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_transforms: Vec<QsTransform>,
    /// # C++ Info
    /// -          name: `floats`(ctype: `hkArray<hkReal>`)
    /// -        offset:  52(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_floats: Vec<f32>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkaInterleavedUncompressedAnimation<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaInterleavedUncompressedAnimation"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2466967601u32)
        }
    }
    impl<'a> __serde::Serialize for hkaInterleavedUncompressedAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkaInterleavedUncompressedAnimation", class_meta)?;
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
            serializer.serialize_array_meta_field("transforms", &self.m_transforms)?;
            serializer.serialize_array_meta_field("floats", &self.m_floats)?;
            serializer
                .serialize_array_field(
                    "annotationTracks",
                    &self.parent.m_annotationTracks,
                )?;
            serializer.serialize_array_field("transforms", &self.m_transforms)?;
            serializer.serialize_array_field("floats", &self.m_floats)?;
            serializer.end()
        }
    }
};
