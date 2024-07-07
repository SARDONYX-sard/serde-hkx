use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxSkinBinding`
/// -         version: `2`
/// -       signature: `0x5a93f338`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxSkinBinding<'a> {
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
    /// -          name: `nodeNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nodeNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `bindPose`(ctype: `hkArray<hkMatrix4>`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bindPose: Vec<Matrix4>,
    /// # C++ Info
    /// -          name: `initSkinTransform`(ctype: `hkMatrix4`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_initSkinTransform: Matrix4,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxSkinBinding<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxSkinBinding"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5a93f338)
        }
    }
    impl<'a> _serde::Serialize for hkxSkinBinding<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5a93f338)));
            let mut serializer = __serializer
                .serialize_struct("hkxSkinBinding", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("mesh", &self.m_mesh)?;
            serializer.serialize_array_meta_field("nodeNames", &self.m_nodeNames)?;
            serializer.serialize_array_meta_field("bindPose", &self.m_bindPose)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("initSkinTransform", &self.m_initSkinTransform)?;
            serializer.serialize_array_field("nodeNames", &self.m_nodeNames)?;
            serializer.serialize_array_field("bindPose", &self.m_bindPose)?;
            serializer.end()
        }
    }
};
