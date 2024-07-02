use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStorageExtendedMeshShape`
/// -         version: `0`
/// -       signature: `0xb469efbc`
/// -          size: 272(x86)/368(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStorageExtendedMeshShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpExtendedMeshShape,
    /// # C++ Info
    /// -          name: `meshstorage`(ctype: `hkArray<hkpStorageExtendedMeshShapeMeshSubpartStorage*>`)
    /// -        offset: 240(x86)/336(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_meshstorage: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `shapestorage`(ctype: `hkArray<hkpStorageExtendedMeshShapeShapeSubpartStorage*>`)
    /// -        offset: 252(x86)/352(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_shapestorage: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpStorageExtendedMeshShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStorageExtendedMeshShape"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3026841532u32)
        }
    }
    impl __serde::Serialize for hkpStorageExtendedMeshShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3026841532u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpStorageExtendedMeshShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field(
                    "disableWelding",
                    &self.parent.parent.m_disableWelding,
                )?;
            serializer
                .serialize_field(
                    "collectionType",
                    &self.parent.parent.m_collectionType,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field(
                    "embeddedTrianglesSubpart",
                    &self.parent.m_embeddedTrianglesSubpart,
                )?;
            serializer
                .serialize_field("aabbHalfExtents", &self.parent.m_aabbHalfExtents)?;
            serializer.serialize_field("aabbCenter", &self.parent.m_aabbCenter)?;
            serializer.skip_field("materialClass", &self.parent.m_materialClass)?;
            serializer
                .serialize_field(
                    "numBitsForSubpartIndex",
                    &self.parent.m_numBitsForSubpartIndex,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "trianglesSubparts",
                    &self.parent.m_trianglesSubparts,
                )?;
            serializer
                .serialize_array_meta_field(
                    "shapesSubparts",
                    &self.parent.m_shapesSubparts,
                )?;
            serializer
                .serialize_array_meta_field("weldingInfo", &self.parent.m_weldingInfo)?;
            serializer.serialize_field("weldingType", &self.parent.m_weldingType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "defaultCollisionFilterInfo",
                    &self.parent.m_defaultCollisionFilterInfo,
                )?;
            serializer
                .serialize_field(
                    "cachedNumChildShapes",
                    &self.parent.m_cachedNumChildShapes,
                )?;
            serializer.serialize_field("triangleRadius", &self.parent.m_triangleRadius)?;
            serializer.skip_field("padding", &self.parent.m_padding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_array_meta_field("meshstorage", &self.m_meshstorage)?;
            serializer.serialize_array_meta_field("shapestorage", &self.m_shapestorage)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "trianglesSubparts",
                    &self.parent.m_trianglesSubparts,
                )?;
            serializer
                .serialize_array_field("shapesSubparts", &self.parent.m_shapesSubparts)?;
            serializer.serialize_array_field("weldingInfo", &self.parent.m_weldingInfo)?;
            serializer.serialize_array_field("meshstorage", &self.m_meshstorage)?;
            serializer.serialize_array_field("shapestorage", &self.m_shapestorage)?;
            serializer.end()
        }
    }
};
