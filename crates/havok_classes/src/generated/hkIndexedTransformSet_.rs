use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkIndexedTransformSet`
/// -         version: `1`
/// -       signature: `0x87fe6b5c`
/// -          size:  72(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkIndexedTransformSet<'a> {
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
    /// -          name: `matrices`(ctype: `hkArray<hkMatrix4>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_matrices: Vec<Matrix4>,
    /// # C++ Info
    /// -          name: `inverseMatrices`(ctype: `hkArray<hkMatrix4>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_inverseMatrices: Vec<Matrix4>,
    /// # C++ Info
    /// -          name: `matricesOrder`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_matricesOrder: Vec<i16>,
    /// # C++ Info
    /// -          name: `matricesNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_matricesNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `indexMappings`(ctype: `hkArray<struct hkMeshBoneIndexMapping>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_indexMappings: Vec<hkMeshBoneIndexMapping>,
    /// # C++ Info
    /// -          name: `allMatricesAreAffine`(ctype: `hkBool`)
    /// -        offset:  68(x86)/ 96(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_allMatricesAreAffine: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkIndexedTransformSet<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkIndexedTransformSet"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2281597788u32)
        }
    }
    impl<'a> __serde::Serialize for hkIndexedTransformSet<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkIndexedTransformSet", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("matrices", &self.m_matrices)?;
            serializer
                .serialize_array_meta_field("inverseMatrices", &self.m_inverseMatrices)?;
            serializer
                .serialize_array_meta_field("matricesOrder", &self.m_matricesOrder)?;
            serializer
                .serialize_array_meta_field("matricesNames", &self.m_matricesNames)?;
            serializer
                .serialize_array_meta_field("indexMappings", &self.m_indexMappings)?;
            serializer
                .serialize_field("allMatricesAreAffine", &self.m_allMatricesAreAffine)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_field("matrices", &self.m_matrices)?;
            serializer
                .serialize_array_field("inverseMatrices", &self.m_inverseMatrices)?;
            serializer.serialize_array_field("matricesOrder", &self.m_matricesOrder)?;
            serializer.serialize_array_field("matricesNames", &self.m_matricesNames)?;
            serializer.serialize_array_field("indexMappings", &self.m_indexMappings)?;
            serializer.end()
        }
    }
};
