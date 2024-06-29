use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryMeshShape`
/// -         version: `0`
/// -       signature: `0xb743a578`
/// -          size:  48(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryMeshShape<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkMeshShape,
    /// # C++ Info
    /// -          name: `sections`(ctype: `hkArray<struct hkMeshSectionCinfo>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_sections: Vec<hkMeshSectionCinfo>,
    /// # C++ Info
    /// -          name: `indices16`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_indices16: Vec<u16>,
    /// # C++ Info
    /// -          name: `indices32`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_indices32: Vec<u32>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkMemoryMeshShape<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMemoryMeshShape"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3074663800u32)
        }
    }
    impl<'a> __serde::Serialize for hkMemoryMeshShape<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryMeshShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("sections", &self.m_sections)?;
            serializer.serialize_array_meta_field("indices16", &self.m_indices16)?;
            serializer.serialize_array_meta_field("indices32", &self.m_indices32)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_array_field("sections", &self.m_sections)?;
            serializer.serialize_array_field("indices16", &self.m_indices16)?;
            serializer.serialize_array_field("indices32", &self.m_indices32)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
