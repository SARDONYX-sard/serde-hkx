use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSkeletonMapperDataSimpleMapping`
/// -         version: `0`
/// -       signature: `0x3405deca`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSkeletonMapperDataSimpleMapping {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `boneA`(ctype: `hkInt16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_boneA: i16,
    /// # C++ Info
    /// -          name: `boneB`(ctype: `hkInt16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_boneB: i16,
    /// # C++ Info
    /// -          name: `aFromBTransform`(ctype: `hkQsTransform`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_aFromBTransform: QsTransform,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaSkeletonMapperDataSimpleMapping {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSkeletonMapperDataSimpleMapping"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3405deca)
        }
    }
    impl _serde::Serialize for hkaSkeletonMapperDataSimpleMapping {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3405deca)));
            let mut serializer = __serializer
                .serialize_struct("hkaSkeletonMapperDataSimpleMapping", class_meta)?;
            serializer.serialize_field("boneA", &self.m_boneA)?;
            serializer.serialize_field("boneB", &self.m_boneB)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_field("aFromBTransform", &self.m_aFromBTransform)?;
            serializer.end()
        }
    }
};
