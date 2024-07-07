use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxVertexBuffer`
/// -         version: `1`
/// -       signature: `0x4ab10615`
/// -          size: 104(x86)/136(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxVertexBuffer {
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
    /// -          name: `data`(ctype: `struct hkxVertexBufferVertexData`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  84(x86)/104(x86_64)
    ///
    pub m_data: hkxVertexBufferVertexData,
    /// # C++ Info
    /// -          name: `desc`(ctype: `struct hkxVertexDescription`)
    /// -        offset:  92(x86)/120(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_desc: hkxVertexDescription,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxVertexBuffer {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxVertexBuffer"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x4ab10615)
        }
    }
    impl _serde::Serialize for hkxVertexBuffer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x4ab10615)));
            let mut serializer = __serializer
                .serialize_struct("hkxVertexBuffer", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer.serialize_field("desc", &self.m_desc)?;
            serializer.end()
        }
    }
};
