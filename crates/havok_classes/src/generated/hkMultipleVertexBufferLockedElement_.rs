use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMultipleVertexBufferLockedElement`
/// -         version: `0`
/// -       signature: `0xa0e22afc`
/// -          size:   7(x86)/  7(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMultipleVertexBufferLockedElement {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `vertexBufferIndex`(ctype: `hkUint8`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_vertexBufferIndex: u8,
    /// # C++ Info
    /// -          name: `elementIndex`(ctype: `hkUint8`)
    /// -        offset:   1(x86)/  1(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_elementIndex: u8,
    /// # C++ Info
    /// -          name: `lockedBufferIndex`(ctype: `hkUint8`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_lockedBufferIndex: u8,
    /// # C++ Info
    /// -          name: `vertexFormatIndex`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_vertexFormatIndex: u8,
    /// # C++ Info
    /// -          name: `lockFlags`(ctype: `hkUint8`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_lockFlags: u8,
    /// # C++ Info
    /// -          name: `outputBufferIndex`(ctype: `hkUint8`)
    /// -        offset:   5(x86)/  5(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_outputBufferIndex: u8,
    /// # C++ Info
    /// -          name: `emulatedIndex`(ctype: `hkInt8`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_emulatedIndex: i8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkMultipleVertexBufferLockedElement {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMultipleVertexBufferLockedElement"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2699176700u32)
        }
    }
    impl __serde::Serialize for hkMultipleVertexBufferLockedElement {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMultipleVertexBufferLockedElement", class_meta)?;
            serializer.serialize_field("vertexBufferIndex", &self.m_vertexBufferIndex)?;
            serializer.serialize_field("elementIndex", &self.m_elementIndex)?;
            serializer.serialize_field("lockedBufferIndex", &self.m_lockedBufferIndex)?;
            serializer.serialize_field("vertexFormatIndex", &self.m_vertexFormatIndex)?;
            serializer.serialize_field("lockFlags", &self.m_lockFlags)?;
            serializer.serialize_field("outputBufferIndex", &self.m_outputBufferIndex)?;
            serializer.serialize_field("emulatedIndex", &self.m_emulatedIndex)?;
            serializer.end()
        }
    }
};
