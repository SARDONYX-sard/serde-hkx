use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMultipleVertexBufferElementInfo`
/// -         version: `0`
/// -       signature: `0x4731fb1b`
/// -          size:   2(x86)/  2(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMultipleVertexBufferElementInfo {
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
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkMultipleVertexBufferElementInfo {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMultipleVertexBufferElementInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1194457883u32)
        }
    }
    impl __serde::Serialize for hkMultipleVertexBufferElementInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMultipleVertexBufferElementInfo", class_meta)?;
            serializer.serialize_field("vertexBufferIndex", &self.m_vertexBufferIndex)?;
            serializer.serialize_field("elementIndex", &self.m_elementIndex)?;
            serializer.end()
        }
    }
};
