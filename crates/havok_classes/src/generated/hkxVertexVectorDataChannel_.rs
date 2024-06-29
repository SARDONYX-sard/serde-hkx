use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxVertexVectorDataChannel`
/// -         version: `1`
/// -       signature: `0x2ea63179`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxVertexVectorDataChannel {
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
    /// -          name: `perVertexVectors`(ctype: `hkArray<hkVector4>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_perVertexVectors: Vec<Vector4>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxVertexVectorDataChannel {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxVertexVectorDataChannel"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(782643577u32)
        }
    }
    impl __serde::Serialize for hkxVertexVectorDataChannel {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxVertexVectorDataChannel", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "perVertexVectors",
                    &self.m_perVertexVectors,
                )?;
            serializer
                .serialize_array_field("perVertexVectors", &self.m_perVertexVectors)?;
            serializer.end()
        }
    }
};
