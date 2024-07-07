use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpShapeInfo`
/// -         version: `0`
/// -       signature: `0xea7f1d08`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpShapeInfo<'a> {
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
    /// -          name: `shape`(ctype: `struct hkpShape*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_shape: Pointer,
    /// # C++ Info
    /// -          name: `isHierarchicalCompound`(ctype: `hkBool`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isHierarchicalCompound: bool,
    /// # C++ Info
    /// -          name: `hkdShapesCollected`(ctype: `hkBool`)
    /// -        offset:  13(x86)/ 25(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hkdShapesCollected: bool,
    /// # C++ Info
    /// -          name: `childShapeNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childShapeNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `childTransforms`(ctype: `hkArray<hkTransform>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childTransforms: Vec<Transform>,
    /// # C++ Info
    /// -          name: `transform`(ctype: `hkTransform`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_transform: Transform,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpShapeInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpShapeInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xea7f1d08)
        }
    }
    impl<'a> _serde::Serialize for hkpShapeInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xea7f1d08)));
            let mut serializer = __serializer
                .serialize_struct("hkpShapeInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer
                .serialize_field(
                    "isHierarchicalCompound",
                    &self.m_isHierarchicalCompound,
                )?;
            serializer
                .serialize_field("hkdShapesCollected", &self.m_hkdShapesCollected)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_meta_field("childShapeNames", &self.m_childShapeNames)?;
            serializer
                .serialize_array_meta_field("childTransforms", &self.m_childTransforms)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer
                .serialize_array_field("childShapeNames", &self.m_childShapeNames)?;
            serializer
                .serialize_array_field("childTransforms", &self.m_childTransforms)?;
            serializer.end()
        }
    }
};
