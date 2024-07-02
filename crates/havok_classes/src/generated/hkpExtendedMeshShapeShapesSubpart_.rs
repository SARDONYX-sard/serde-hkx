use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpExtendedMeshShapeShapesSubpart`
/// -         version: `1`
/// -       signature: `0xf204b155`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpExtendedMeshShapeShapesSubpart {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpExtendedMeshShapeSubpart,
    /// # C++ Info
    /// -          name: `childShapes`(ctype: `hkArray<hkpConvexShape*>`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childShapes: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `rotation`(ctype: `hkQuaternion`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotation: Quaternion,
    /// # C++ Info
    /// -          name: `translation`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_translation: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpExtendedMeshShapeShapesSubpart {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpExtendedMeshShapeShapesSubpart"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(4060393813u32)
        }
    }
    impl _serde::Serialize for hkpExtendedMeshShapeShapesSubpart {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(4060393813u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpExtendedMeshShapeShapesSubpart", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer
                .serialize_field(
                    "materialIndexStridingType",
                    &self.parent.m_materialIndexStridingType,
                )?;
            serializer.skip_field("materialStriding", &self.parent.m_materialStriding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field("materialIndexBase", &self.parent.m_materialIndexBase)?;
            serializer
                .serialize_field(
                    "materialIndexStriding",
                    &self.parent.m_materialIndexStriding,
                )?;
            serializer.serialize_field("numMaterials", &self.parent.m_numMaterials)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialBase", &self.parent.m_materialBase)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_array_meta_field("childShapes", &self.m_childShapes)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("rotation", &self.m_rotation)?;
            serializer.serialize_field("translation", &self.m_translation)?;
            serializer.serialize_array_field("childShapes", &self.m_childShapes)?;
            serializer.end()
        }
    }
};
