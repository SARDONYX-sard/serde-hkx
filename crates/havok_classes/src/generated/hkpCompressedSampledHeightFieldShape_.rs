use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCompressedSampledHeightFieldShape`
/// -         version: `0`
/// -       signature: `0x97b6e143`
/// -          size: 128(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCompressedSampledHeightFieldShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpSampledHeightFieldShape,
    /// # C++ Info
    /// -          name: `storage`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_storage: Vec<u16>,
    /// # C++ Info
    /// -          name: `triangleFlip`(ctype: `hkBool`)
    /// -        offset: 108(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_triangleFlip: bool,
    /// # C++ Info
    /// -          name: `offset`(ctype: `hkReal`)
    /// -        offset: 112(x86)/132(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offset: f32,
    /// # C++ Info
    /// -          name: `scale`(ctype: `hkReal`)
    /// -        offset: 116(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_scale: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCompressedSampledHeightFieldShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCompressedSampledHeightFieldShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2545344835u32)
        }
    }
    impl _serde::Serialize for hkpCompressedSampledHeightFieldShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2545344835u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpCompressedSampledHeightFieldShape", class_meta)?;
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
            serializer.serialize_field("xRes", &self.parent.m_xRes)?;
            serializer.serialize_field("zRes", &self.parent.m_zRes)?;
            serializer.serialize_field("heightCenter", &self.parent.m_heightCenter)?;
            serializer
                .serialize_field(
                    "useProjectionBasedHeight",
                    &self.parent.m_useProjectionBasedHeight,
                )?;
            serializer
                .serialize_field("heightfieldType", &self.parent.m_heightfieldType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field("intToFloatScale", &self.parent.m_intToFloatScale)?;
            serializer
                .serialize_field("floatToIntScale", &self.parent.m_floatToIntScale)?;
            serializer
                .serialize_field(
                    "floatToIntOffsetFloorCorrected",
                    &self.parent.m_floatToIntOffsetFloorCorrected,
                )?;
            serializer.serialize_field("extents", &self.parent.m_extents)?;
            serializer.serialize_array_meta_field("storage", &self.m_storage)?;
            serializer.serialize_field("triangleFlip", &self.m_triangleFlip)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.serialize_field("scale", &self.m_scale)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("storage", &self.m_storage)?;
            serializer.end()
        }
    }
};
