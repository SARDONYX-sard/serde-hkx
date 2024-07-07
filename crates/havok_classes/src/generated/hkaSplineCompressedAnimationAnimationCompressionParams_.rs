use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSplineCompressedAnimationAnimationCompressionParams`
/// -         version: `0`
/// -       signature: `0xde830789`
/// -          size:   4(x86)/  4(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSplineCompressedAnimationAnimationCompressionParams {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `maxFramesPerBlock`(ctype: `hkUint16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_maxFramesPerBlock: u16,
    /// # C++ Info
    /// -          name: `enableSampleSingleTracks`(ctype: `hkBool`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableSampleSingleTracks: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaSplineCompressedAnimationAnimationCompressionParams {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSplineCompressedAnimationAnimationCompressionParams"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xde830789)
        }
    }
    impl _serde::Serialize for hkaSplineCompressedAnimationAnimationCompressionParams {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xde830789)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaSplineCompressedAnimationAnimationCompressionParams",
                    class_meta,
                )?;
            serializer.serialize_field("maxFramesPerBlock", &self.m_maxFramesPerBlock)?;
            serializer
                .serialize_field(
                    "enableSampleSingleTracks",
                    &self.m_enableSampleSingleTracks,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
