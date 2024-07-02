use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryMeshVertexBuffer`
/// -         version: `1`
/// -       signature: `0xa2e50753`
/// -          size: 424(x86)/440(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryMeshVertexBuffer {
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
    /// -          name: `format`(ctype: `struct hkVertexFormat`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size: 260(x86)/260(x86_64)
    ///
    pub m_format: hkVertexFormat,
    /// # C++ Info
    /// -          name: `elementOffsets`(ctype: `hkInt32[32]`)
    /// -        offset: 268(x86)/276(x86_64)
    /// -     type_size: 128(x86)/128(x86_64)
    ///
    pub m_elementOffsets: [i32; 32usize],
    /// # C++ Info
    /// -          name: `memory`(ctype: `hkArray<hkUint8>`)
    /// -        offset: 396(x86)/408(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_memory: Vec<u8>,
    /// # C++ Info
    /// -          name: `vertexStride`(ctype: `hkInt32`)
    /// -        offset: 408(x86)/424(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vertexStride: i32,
    /// # C++ Info
    /// -          name: `locked`(ctype: `hkBool`)
    /// -        offset: 412(x86)/428(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_locked: bool,
    /// # C++ Info
    /// -          name: `numVertices`(ctype: `hkInt32`)
    /// -        offset: 416(x86)/432(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numVertices: i32,
    /// # C++ Info
    /// -          name: `isBigEndian`(ctype: `hkBool`)
    /// -        offset: 420(x86)/436(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isBigEndian: bool,
    /// # C++ Info
    /// -          name: `isSharable`(ctype: `hkBool`)
    /// -        offset: 421(x86)/437(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isSharable: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkMemoryMeshVertexBuffer {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMemoryMeshVertexBuffer"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2732918611u32)
        }
    }
    impl __serde::Serialize for hkMemoryMeshVertexBuffer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2732918611u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryMeshVertexBuffer", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("format", &self.m_format)?;
            serializer
                .serialize_field("elementOffsets", &self.m_elementOffsets.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("memory", &self.m_memory)?;
            serializer.serialize_field("vertexStride", &self.m_vertexStride)?;
            serializer.serialize_field("locked", &self.m_locked)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("numVertices", &self.m_numVertices)?;
            serializer.serialize_field("isBigEndian", &self.m_isBigEndian)?;
            serializer.serialize_field("isSharable", &self.m_isSharable)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_array_field("memory", &self.m_memory)?;
            serializer.end()
        }
    }
};
