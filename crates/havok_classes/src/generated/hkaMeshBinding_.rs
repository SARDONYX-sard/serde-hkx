use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaMeshBinding`
/// -         version: `1`
/// -       signature: `0x81d9950b`
/// -          size:  44(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaMeshBinding<'a> {
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
    /// -          name: `mesh`(ctype: `struct hkxMesh*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_mesh: Pointer,
    /// # C++ Info
    /// -          name: `originalSkeletonName`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_originalSkeletonName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `skeleton`(ctype: `struct hkaSkeleton*`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_skeleton: Pointer,
    /// # C++ Info
    /// -          name: `mappings`(ctype: `hkArray<struct hkaMeshBindingMapping>`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_mappings: Vec<hkaMeshBindingMapping>,
    /// # C++ Info
    /// -          name: `boneFromSkinMeshTransforms`(ctype: `hkArray<hkTransform>`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_boneFromSkinMeshTransforms: Vec<Transform>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaMeshBinding<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaMeshBinding"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x81d9950b)
        }
    }
    impl<'a> _serde::Serialize for hkaMeshBinding<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x81d9950b)));
            let mut serializer = __serializer
                .serialize_struct("hkaMeshBinding", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("mesh", &self.m_mesh)?;
            serializer
                .serialize_stringptr_meta_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer.serialize_field("skeleton", &self.m_skeleton)?;
            serializer.serialize_array_meta_field("mappings", &self.m_mappings)?;
            serializer
                .serialize_array_meta_field(
                    "boneFromSkinMeshTransforms",
                    &self.m_boneFromSkinMeshTransforms,
                )?;
            serializer
                .serialize_stringptr_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer.serialize_array_field("mappings", &self.m_mappings)?;
            serializer
                .serialize_array_field(
                    "boneFromSkinMeshTransforms",
                    &self.m_boneFromSkinMeshTransforms,
                )?;
            serializer.end()
        }
    }
};
