use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryMeshBody`
/// -         version: `0`
/// -       signature: `0x94a620a8`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryMeshBody<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkMeshBody,
    /// # C++ Info
    /// -          name: `transform`(ctype: `hkMatrix4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_transform: Matrix4,
    /// # C++ Info
    /// -          name: `transformSet`(ctype: `struct hkIndexedTransformSet*`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_transformSet: Pointer,
    /// # C++ Info
    /// -          name: `shape`(ctype: `struct hkMeshShape*`)
    /// -        offset:  84(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_shape: Pointer,
    /// # C++ Info
    /// -          name: `vertexBuffers`(ctype: `hkArray<hkMeshVertexBuffer*>`)
    /// -        offset:  88(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vertexBuffers: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset: 100(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkMemoryMeshBody<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMemoryMeshBody"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2493915304u32)
        }
    }
    impl<'a> _serde::Serialize for hkMemoryMeshBody<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2493915304u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryMeshBody", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer.serialize_field("transformSet", &self.m_transformSet)?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer
                .serialize_array_meta_field("vertexBuffers", &self.m_vertexBuffers)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_array_field("vertexBuffers", &self.m_vertexBuffers)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
