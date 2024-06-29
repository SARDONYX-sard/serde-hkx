use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStorageMeshShape`
/// -         version: `0`
/// -       signature: `0xbefd8b39`
/// -          size: 112(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStorageMeshShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpMeshShape,
    /// # C++ Info
    /// -          name: `storage`(ctype: `hkArray<hkpStorageMeshShapeSubpartStorage*>`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_storage: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpStorageMeshShape {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpStorageMeshShape"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3204287289u32)
        }
    }
    impl __serde::Serialize for hkpStorageMeshShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpStorageMeshShape", class_meta)?;
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
            serializer.serialize_field("scaling", &self.parent.m_scaling)?;
            serializer
                .serialize_field(
                    "numBitsForSubpartIndex",
                    &self.parent.m_numBitsForSubpartIndex,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("subparts", &self.parent.m_subparts)?;
            serializer
                .serialize_array_meta_field("weldingInfo", &self.parent.m_weldingInfo)?;
            serializer.serialize_field("weldingType", &self.parent.m_weldingType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("radius", &self.parent.m_radius)?;
            serializer.serialize_field("pad", &self.parent.m_pad.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("storage", &self.m_storage)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_array_field("subparts", &self.parent.m_subparts)?;
            serializer.serialize_array_field("weldingInfo", &self.parent.m_weldingInfo)?;
            serializer.serialize_array_field("storage", &self.m_storage)?;
            serializer.end()
        }
    }
};
