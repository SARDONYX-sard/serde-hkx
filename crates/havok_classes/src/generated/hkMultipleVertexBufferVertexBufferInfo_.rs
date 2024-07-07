use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMultipleVertexBufferVertexBufferInfo`
/// -         version: `0`
/// -       signature: `0xdafbe0e6`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMultipleVertexBufferVertexBufferInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `vertexBuffer`(ctype: `struct hkMeshVertexBuffer*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_vertexBuffer: Pointer,
    /// # C++ Info
    /// -          name: `lockedVertices`(ctype: `void*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_lockedVertices: Pointer,
    /// # C++ Info
    /// -          name: `isLocked`(ctype: `hkBool`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isLocked: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkMultipleVertexBufferVertexBufferInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMultipleVertexBufferVertexBufferInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xdafbe0e6)
        }
    }
    impl _serde::Serialize for hkMultipleVertexBufferVertexBufferInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xdafbe0e6)));
            let mut serializer = __serializer
                .serialize_struct("hkMultipleVertexBufferVertexBufferInfo", class_meta)?;
            serializer.serialize_field("vertexBuffer", &self.m_vertexBuffer)?;
            serializer.skip_field("lockedVertices", &self.m_lockedVertices)?;
            serializer.serialize_field("isLocked", &self.m_isLocked)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
