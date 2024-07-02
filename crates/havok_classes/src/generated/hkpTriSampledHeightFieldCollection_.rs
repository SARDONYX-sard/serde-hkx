use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTriSampledHeightFieldCollection`
/// -         version: `0`
/// -       signature: `0xc291ddde`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTriSampledHeightFieldCollection {
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
    /// -          name: `heightfield`(ctype: `struct hkpSampledHeightFieldShape*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_heightfield: Pointer,
    /// # C++ Info
    /// -          name: `childSize`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_childSize: i32,
    /// # C++ Info
    /// -          name: `radius`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_radius: f32,
    /// # C++ Info
    /// -          name: `weldingInfo`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_weldingInfo: Vec<u16>,
    /// # C++ Info
    /// -          name: `triangleExtrusion`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_triangleExtrusion: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpTriSampledHeightFieldCollection {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTriSampledHeightFieldCollection"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3264339422u32)
        }
    }
    impl _serde::Serialize for hkpTriSampledHeightFieldCollection {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3264339422u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpTriSampledHeightFieldCollection", class_meta)?;
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
            serializer.serialize_field("heightfield", &self.m_heightfield)?;
            serializer.skip_field("childSize", &self.m_childSize)?;
            serializer.serialize_field("radius", &self.m_radius)?;
            serializer.serialize_array_meta_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.serialize_field("triangleExtrusion", &self.m_triangleExtrusion)?;
            serializer.serialize_array_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.end()
        }
    }
};
