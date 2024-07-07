use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSerializedDisplayRbTransforms`
/// -         version: `0`
/// -       signature: `0xc18650ac`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSerializedDisplayRbTransforms {
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
    /// -          name: `transforms`(ctype: `hkArray<struct hkpSerializedDisplayRbTransformsDisplayTransformPair>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_transforms: Vec<hkpSerializedDisplayRbTransformsDisplayTransformPair>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSerializedDisplayRbTransforms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSerializedDisplayRbTransforms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc18650ac)
        }
    }
    impl _serde::Serialize for hkpSerializedDisplayRbTransforms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc18650ac)));
            let mut serializer = __serializer
                .serialize_struct("hkpSerializedDisplayRbTransforms", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("transforms", &self.m_transforms)?;
            serializer.serialize_array_field("transforms", &self.m_transforms)?;
            serializer.end()
        }
    }
};
