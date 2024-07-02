use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMultipleVertexBuffer`
/// -         version: `0`
/// -       signature: `0xde3ab602`
/// -          size: 324(x86)/352(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMultipleVertexBuffer {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkMeshVertexBuffer,
    /// # C++ Info
    /// -          name: `vertexFormat`(ctype: `struct hkVertexFormat`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size: 260(x86)/260(x86_64)
    ///
    pub m_vertexFormat: hkVertexFormat,
    /// # C++ Info
    /// -          name: `lockedElements`(ctype: `hkArray<struct hkMultipleVertexBufferLockedElement>`)
    /// -        offset: 268(x86)/280(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_lockedElements: Vec<hkMultipleVertexBufferLockedElement>,
    /// # C++ Info
    /// -          name: `lockedBuffer`(ctype: `struct hkMemoryMeshVertexBuffer*`)
    /// -        offset: 280(x86)/296(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_lockedBuffer: Pointer,
    /// # C++ Info
    /// -          name: `elementInfos`(ctype: `hkArray<struct hkMultipleVertexBufferElementInfo>`)
    /// -        offset: 284(x86)/304(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_elementInfos: Vec<hkMultipleVertexBufferElementInfo>,
    /// # C++ Info
    /// -          name: `vertexBufferInfos`(ctype: `hkArray<struct hkMultipleVertexBufferVertexBufferInfo>`)
    /// -        offset: 296(x86)/320(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vertexBufferInfos: Vec<hkMultipleVertexBufferVertexBufferInfo>,
    /// # C++ Info
    /// -          name: `numVertices`(ctype: `hkInt32`)
    /// -        offset: 308(x86)/336(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numVertices: i32,
    /// # C++ Info
    /// -          name: `isLocked`(ctype: `hkBool`)
    /// -        offset: 312(x86)/340(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isLocked: bool,
    /// # C++ Info
    /// -          name: `updateCount`(ctype: `hkUint32`)
    /// -        offset: 316(x86)/344(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_updateCount: u32,
    /// # C++ Info
    /// -          name: `writeLock`(ctype: `hkBool`)
    /// -        offset: 320(x86)/348(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_writeLock: bool,
    /// # C++ Info
    /// -          name: `isSharable`(ctype: `hkBool`)
    /// -        offset: 321(x86)/349(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isSharable: bool,
    /// # C++ Info
    /// -          name: `constructionComplete`(ctype: `hkBool`)
    /// -        offset: 322(x86)/350(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_constructionComplete: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkMultipleVertexBuffer {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMultipleVertexBuffer"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3728389634u32)
        }
    }
    impl _serde::Serialize for hkMultipleVertexBuffer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3728389634u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMultipleVertexBuffer", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("vertexFormat", &self.m_vertexFormat)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("lockedElements", &self.m_lockedElements)?;
            serializer.serialize_field("lockedBuffer", &self.m_lockedBuffer)?;
            serializer.serialize_array_meta_field("elementInfos", &self.m_elementInfos)?;
            serializer
                .serialize_array_meta_field(
                    "vertexBufferInfos",
                    &self.m_vertexBufferInfos,
                )?;
            serializer.serialize_field("numVertices", &self.m_numVertices)?;
            serializer.serialize_field("isLocked", &self.m_isLocked)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("updateCount", &self.m_updateCount)?;
            serializer.serialize_field("writeLock", &self.m_writeLock)?;
            serializer.serialize_field("isSharable", &self.m_isSharable)?;
            serializer
                .serialize_field("constructionComplete", &self.m_constructionComplete)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_array_field("lockedElements", &self.m_lockedElements)?;
            serializer.serialize_array_field("elementInfos", &self.m_elementInfos)?;
            serializer
                .serialize_array_field("vertexBufferInfos", &self.m_vertexBufferInfos)?;
            serializer.end()
        }
    }
};
