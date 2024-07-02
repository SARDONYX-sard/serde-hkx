use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryMeshMaterial`
/// -         version: `0`
/// -       signature: `0x12156ee3`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryMeshMaterial<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkMeshMaterial,
    /// # C++ Info
    /// -          name: `materialName`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_materialName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `textures`(ctype: `hkArray<hkMeshTexture*>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_textures: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkMemoryMeshMaterial<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMemoryMeshMaterial"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(303394531u32)
        }
    }
    impl<'a> __serde::Serialize for hkMemoryMeshMaterial<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(303394531u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryMeshMaterial", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field("materialName", &self.m_materialName)?;
            serializer.serialize_array_meta_field("textures", &self.m_textures)?;
            serializer.serialize_stringptr_field("materialName", &self.m_materialName)?;
            serializer.serialize_array_field("textures", &self.m_textures)?;
            serializer.end()
        }
    }
};
