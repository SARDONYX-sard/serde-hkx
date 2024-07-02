use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSkeletonMapperDataChainMapping`
/// -         version: `0`
/// -       signature: `0xa528f7cf`
/// -          size: 112(x86)/112(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSkeletonMapperDataChainMapping {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `startBoneA`(ctype: `hkInt16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_startBoneA: i16,
    /// # C++ Info
    /// -          name: `endBoneA`(ctype: `hkInt16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_endBoneA: i16,
    /// # C++ Info
    /// -          name: `startBoneB`(ctype: `hkInt16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_startBoneB: i16,
    /// # C++ Info
    /// -          name: `endBoneB`(ctype: `hkInt16`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_endBoneB: i16,
    /// # C++ Info
    /// -          name: `startAFromBTransform`(ctype: `hkQsTransform`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_startAFromBTransform: QsTransform,
    /// # C++ Info
    /// -          name: `endAFromBTransform`(ctype: `hkQsTransform`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_endAFromBTransform: QsTransform,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaSkeletonMapperDataChainMapping {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSkeletonMapperDataChainMapping"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2770925519u32)
        }
    }
    impl _serde::Serialize for hkaSkeletonMapperDataChainMapping {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2770925519u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaSkeletonMapperDataChainMapping", class_meta)?;
            serializer.serialize_field("startBoneA", &self.m_startBoneA)?;
            serializer.serialize_field("endBoneA", &self.m_endBoneA)?;
            serializer.serialize_field("startBoneB", &self.m_startBoneB)?;
            serializer.serialize_field("endBoneB", &self.m_endBoneB)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field("startAFromBTransform", &self.m_startAFromBTransform)?;
            serializer
                .serialize_field("endAFromBTransform", &self.m_endAFromBTransform)?;
            serializer.end()
        }
    }
};
