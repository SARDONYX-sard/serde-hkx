use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaAnnotationTrack`
/// -         version: `0`
/// -       signature: `0xd4114fdd`
/// -          size:  16(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnnotationTrack<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `trackName`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_trackName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `annotations`(ctype: `hkArray<struct hkaAnnotationTrackAnnotation>`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_annotations: Vec<hkaAnnotationTrackAnnotation<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaAnnotationTrack<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaAnnotationTrack"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3557904349u32)
        }
    }
    impl<'a> _serde::Serialize for hkaAnnotationTrack<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3557904349u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaAnnotationTrack", class_meta)?;
            serializer.serialize_stringptr_meta_field("trackName", &self.m_trackName)?;
            serializer.serialize_array_meta_field("annotations", &self.m_annotations)?;
            serializer.serialize_stringptr_field("trackName", &self.m_trackName)?;
            serializer.serialize_array_field("annotations", &self.m_annotations)?;
            serializer.end()
        }
    }
};
