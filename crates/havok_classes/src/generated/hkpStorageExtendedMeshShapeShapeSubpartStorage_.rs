use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStorageExtendedMeshShapeShapeSubpartStorage`
/// -         version: `2`
/// -       signature: `0x3f7d804c`
/// -          size:  44(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStorageExtendedMeshShapeShapeSubpartStorage {
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
    /// -          name: `materialIndices`(ctype: `hkArray<hkUint8>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materialIndices: Vec<u8>,
    /// # C++ Info
    /// -          name: `materials`(ctype: `hkArray<struct hkpStorageExtendedMeshShapeMaterial>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materials: Vec<hkpStorageExtendedMeshShapeMaterial>,
    /// # C++ Info
    /// -          name: `materialIndices16`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materialIndices16: Vec<u16>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpStorageExtendedMeshShapeShapeSubpartStorage {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStorageExtendedMeshShapeShapeSubpartStorage"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3f7d804c)
        }
    }
    impl _serde::Serialize for hkpStorageExtendedMeshShapeShapeSubpartStorage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3f7d804c)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpStorageExtendedMeshShapeShapeSubpartStorage",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("materialIndices", &self.m_materialIndices)?;
            serializer.serialize_array_meta_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_meta_field(
                    "materialIndices16",
                    &self.m_materialIndices16,
                )?;
            serializer
                .serialize_array_field("materialIndices", &self.m_materialIndices)?;
            serializer.serialize_array_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_field("materialIndices16", &self.m_materialIndices16)?;
            serializer.end()
        }
    }
};
