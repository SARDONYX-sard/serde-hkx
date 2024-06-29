use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStorageMeshShapeSubpartStorage`
/// -         version: `0`
/// -       signature: `0xbf27438`
/// -          size:  80(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStorageMeshShapeSubpartStorage {
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
    /// -          name: `vertices`(ctype: `hkArray<hkReal>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vertices: Vec<f32>,
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
    /// -          name: `materialIndices`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materialIndices: Vec<u8>,
    /// # C++ Info
    /// -          name: `materials`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materials: Vec<u32>,
    /// # C++ Info
    /// -          name: `materialIndices16`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  68(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materialIndices16: Vec<u16>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpStorageMeshShapeSubpartStorage {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpStorageMeshShapeSubpartStorage"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(200438840u32)
        }
    }
    impl __serde::Serialize for hkpStorageMeshShapeSubpartStorage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpStorageMeshShapeSubpartStorage", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_meta_field("indices16", &self.m_indices16)?;
            serializer.serialize_array_meta_field("indices32", &self.m_indices32)?;
            serializer
                .serialize_array_meta_field("materialIndices", &self.m_materialIndices)?;
            serializer.serialize_array_meta_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_meta_field(
                    "materialIndices16",
                    &self.m_materialIndices16,
                )?;
            serializer.serialize_array_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_field("indices16", &self.m_indices16)?;
            serializer.serialize_array_field("indices32", &self.m_indices32)?;
            serializer
                .serialize_array_field("materialIndices", &self.m_materialIndices)?;
            serializer.serialize_array_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_field("materialIndices16", &self.m_materialIndices16)?;
            serializer.end()
        }
    }
};
