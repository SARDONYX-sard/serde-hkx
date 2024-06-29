use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSimpleMeshShape`
/// -         version: `0`
/// -       signature: `0x16b3c811`
/// -          size:  68(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSimpleMeshShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShapeCollection,
    /// # C++ Info
    /// -          name: `vertices`(ctype: `hkArray<hkVector4>`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vertices: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `triangles`(ctype: `hkArray<struct hkpSimpleMeshShapeTriangle>`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_triangles: Vec<hkpSimpleMeshShapeTriangle>,
    /// # C++ Info
    /// -          name: `materialIndices`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materialIndices: Vec<u8>,
    /// # C++ Info
    /// -          name: `radius`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_radius: f32,
    /// # C++ Info
    /// -          name: `weldingType`(ctype: `enum WeldingType`)
    /// -        offset:  64(x86)/100(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_weldingType: WeldingType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSimpleMeshShape {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSimpleMeshShape"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(380880913u32)
        }
    }
    impl __serde::Serialize for hkpSimpleMeshShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSimpleMeshShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("disableWelding", &self.parent.m_disableWelding)?;
            serializer.serialize_field("collectionType", &self.parent.m_collectionType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_array_meta_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_meta_field("triangles", &self.m_triangles)?;
            serializer
                .serialize_array_meta_field("materialIndices", &self.m_materialIndices)?;
            serializer.serialize_field("radius", &self.m_radius)?;
            serializer.serialize_field("weldingType", &self.m_weldingType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_array_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_field("triangles", &self.m_triangles)?;
            serializer
                .serialize_array_field("materialIndices", &self.m_materialIndices)?;
            serializer.end()
        }
    }
};
