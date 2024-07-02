use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaAnnotationTrackAnnotation`
/// -         version: `0`
/// -       signature: `0x623bf34f`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnnotationTrackAnnotation<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_time: f32,
    /// # C++ Info
    /// -          name: `text`(ctype: `hkStringPtr`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_text: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaAnnotationTrackAnnotation<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaAnnotationTrackAnnotation"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1648096079u32)
        }
    }
    impl<'a> _serde::Serialize for hkaAnnotationTrackAnnotation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1648096079u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaAnnotationTrackAnnotation", class_meta)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("text", &self.m_text)?;
            serializer.serialize_stringptr_field("text", &self.m_text)?;
            serializer.end()
        }
    }
};
