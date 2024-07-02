use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaQuantizedAnimationTrackCompressionParams`
/// -         version: `0`
/// -       signature: `0xf7d64649`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaQuantizedAnimationTrackCompressionParams {
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
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkaQuantizedAnimationTrackCompressionParams {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaQuantizedAnimationTrackCompressionParams"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4158015049u32)
        }
    }
    impl __serde::Serialize for hkaQuantizedAnimationTrackCompressionParams {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(4158015049u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaQuantizedAnimationTrackCompressionParams",
                    class_meta,
                )?;
            serializer.serialize_field("rotationTolerance", &self.m_rotationTolerance)?;
            serializer
                .serialize_field("translationTolerance", &self.m_translationTolerance)?;
            serializer.serialize_field("scaleTolerance", &self.m_scaleTolerance)?;
            serializer.serialize_field("floatingTolerance", &self.m_floatingTolerance)?;
            serializer.end()
        }
    }
};
