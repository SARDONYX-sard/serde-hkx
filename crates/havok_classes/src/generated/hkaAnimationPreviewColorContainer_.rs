use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaAnimationPreviewColorContainer`
/// -         version: `1`
/// -       signature: `0x4bc4c3e0`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnimationPreviewColorContainer {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `previewColor`(ctype: `hkArray<hkUint32>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_previewColor: Vec<u32>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaAnimationPreviewColorContainer {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaAnimationPreviewColorContainer"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1271186400u32)
        }
    }
    impl _serde::Serialize for hkaAnimationPreviewColorContainer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1271186400u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaAnimationPreviewColorContainer", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("previewColor", &self.m_previewColor)?;
            serializer.serialize_array_field("previewColor", &self.m_previewColor)?;
            serializer.end()
        }
    }
};
