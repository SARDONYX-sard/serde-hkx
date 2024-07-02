use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxMeshSection`
/// -         version: `1`
/// -       signature: `0xe2286cf8`
/// -          size:  40(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMeshSection {
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
    /// -          name: `vertexBuffer`(ctype: `struct hkxVertexBuffer*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_vertexBuffer: Pointer,
    /// # C++ Info
    /// -          name: `indexBuffers`(ctype: `hkArray<hkxIndexBuffer*>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_indexBuffers: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `material`(ctype: `struct hkxMaterial*`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_material: Pointer,
    /// # C++ Info
    /// -          name: `userChannels`(ctype: `hkArray<hkReferencedObject*>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_userChannels: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxMeshSection {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxMeshSection"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3794300152u32)
        }
    }
    impl __serde::Serialize for hkxMeshSection {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3794300152u32)));
            let mut serializer = __serializer
                .serialize_struct("hkxMeshSection", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("vertexBuffer", &self.m_vertexBuffer)?;
            serializer.serialize_array_meta_field("indexBuffers", &self.m_indexBuffers)?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.serialize_array_meta_field("userChannels", &self.m_userChannels)?;
            serializer.serialize_array_field("indexBuffers", &self.m_indexBuffers)?;
            serializer.serialize_array_field("userChannels", &self.m_userChannels)?;
            serializer.end()
        }
    }
};
