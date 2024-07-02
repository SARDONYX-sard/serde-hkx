use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxScene`
/// -         version: `1`
/// -       signature: `0x5f673ddd`
/// -          size: 176(x86)/224(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxScene<'a> {
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
    /// -          name: `modeller`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_modeller: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `asset`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_asset: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `sceneLength`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sceneLength: f32,
    /// # C++ Info
    /// -          name: `rootNode`(ctype: `struct hkxNode*`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rootNode: Pointer,
    /// # C++ Info
    /// -          name: `selectionSets`(ctype: `hkArray<hkxNodeSelectionSet*>`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_selectionSets: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `cameras`(ctype: `hkArray<hkxCamera*>`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_cameras: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `lights`(ctype: `hkArray<hkxLight*>`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_lights: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `meshes`(ctype: `hkArray<hkxMesh*>`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_meshes: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `materials`(ctype: `hkArray<hkxMaterial*>`)
    /// -        offset:  72(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materials: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `inplaceTextures`(ctype: `hkArray<hkxTextureInplace*>`)
    /// -        offset:  84(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_inplaceTextures: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `externalTextures`(ctype: `hkArray<hkxTextureFile*>`)
    /// -        offset:  96(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_externalTextures: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `skinBindings`(ctype: `hkArray<hkxSkinBinding*>`)
    /// -        offset: 108(x86)/160(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_skinBindings: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `appliedTransform`(ctype: `hkMatrix3`)
    /// -        offset: 128(x86)/176(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_appliedTransform: Matrix3,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkxScene<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxScene"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1600601565u32)
        }
    }
    impl<'a> __serde::Serialize for hkxScene<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1600601565u32)));
            let mut serializer = __serializer.serialize_struct("hkxScene", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("modeller", &self.m_modeller)?;
            serializer.serialize_stringptr_meta_field("asset", &self.m_asset)?;
            serializer.serialize_field("sceneLength", &self.m_sceneLength)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("rootNode", &self.m_rootNode)?;
            serializer
                .serialize_array_meta_field("selectionSets", &self.m_selectionSets)?;
            serializer.serialize_array_meta_field("cameras", &self.m_cameras)?;
            serializer.serialize_array_meta_field("lights", &self.m_lights)?;
            serializer.serialize_array_meta_field("meshes", &self.m_meshes)?;
            serializer.serialize_array_meta_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_meta_field("inplaceTextures", &self.m_inplaceTextures)?;
            serializer
                .serialize_array_meta_field(
                    "externalTextures",
                    &self.m_externalTextures,
                )?;
            serializer.serialize_array_meta_field("skinBindings", &self.m_skinBindings)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("appliedTransform", &self.m_appliedTransform)?;
            serializer.serialize_stringptr_field("modeller", &self.m_modeller)?;
            serializer.serialize_stringptr_field("asset", &self.m_asset)?;
            serializer.serialize_array_field("selectionSets", &self.m_selectionSets)?;
            serializer.serialize_array_field("cameras", &self.m_cameras)?;
            serializer.serialize_array_field("lights", &self.m_lights)?;
            serializer.serialize_array_field("meshes", &self.m_meshes)?;
            serializer.serialize_array_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_field("inplaceTextures", &self.m_inplaceTextures)?;
            serializer
                .serialize_array_field("externalTextures", &self.m_externalTextures)?;
            serializer.serialize_array_field("skinBindings", &self.m_skinBindings)?;
            serializer.end()
        }
    }
};
