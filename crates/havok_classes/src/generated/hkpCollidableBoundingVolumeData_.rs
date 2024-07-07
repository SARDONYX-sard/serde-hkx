use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCollidableBoundingVolumeData`
/// -         version: `0`
/// -       signature: `0xb5f0e6b1`
/// -          size:  44(x86)/ 56(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCollidableBoundingVolumeData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `min`(ctype: `hkUint32[3]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_min: [u32; 3usize],
    /// # C++ Info
    /// -          name: `expansionMin`(ctype: `hkUint8[3]`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   3(x86)/  3(x86_64)
    ///
    pub m_expansionMin: [u8; 3usize],
    /// # C++ Info
    /// -          name: `expansionShift`(ctype: `hkUint8`)
    /// -        offset:  15(x86)/ 15(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_expansionShift: u8,
    /// # C++ Info
    /// -          name: `max`(ctype: `hkUint32[3]`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_max: [u32; 3usize],
    /// # C++ Info
    /// -          name: `expansionMax`(ctype: `hkUint8[3]`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   3(x86)/  3(x86_64)
    ///
    pub m_expansionMax: [u8; 3usize],
    /// # C++ Info
    /// -          name: `padding`(ctype: `hkUint8`)
    /// -        offset:  31(x86)/ 31(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_padding: u8,
    /// # C++ Info
    /// -          name: `numChildShapeAabbs`(ctype: `hkUint16`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numChildShapeAabbs: u16,
    /// # C++ Info
    /// -          name: `capacityChildShapeAabbs`(ctype: `hkUint16`)
    /// -        offset:  34(x86)/ 34(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_capacityChildShapeAabbs: u16,
    /// # C++ Info
    /// -          name: `childShapeAabbs`(ctype: `void*`)
    /// -        offset:  36(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_childShapeAabbs: Pointer,
    /// # C++ Info
    /// -          name: `childShapeKeys`(ctype: `void*`)
    /// -        offset:  40(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_childShapeKeys: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCollidableBoundingVolumeData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCollidableBoundingVolumeData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb5f0e6b1)
        }
    }
    impl _serde::Serialize for hkpCollidableBoundingVolumeData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xb5f0e6b1)));
            let mut serializer = __serializer
                .serialize_struct("hkpCollidableBoundingVolumeData", class_meta)?;
            serializer.serialize_field("min", &self.m_min.as_slice())?;
            serializer.serialize_field("expansionMin", &self.m_expansionMin.as_slice())?;
            serializer.serialize_field("expansionShift", &self.m_expansionShift)?;
            serializer.serialize_field("max", &self.m_max.as_slice())?;
            serializer.serialize_field("expansionMax", &self.m_expansionMax.as_slice())?;
            serializer.serialize_field("padding", &self.m_padding)?;
            serializer.skip_field("numChildShapeAabbs", &self.m_numChildShapeAabbs)?;
            serializer
                .skip_field("capacityChildShapeAabbs", &self.m_capacityChildShapeAabbs)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("childShapeAabbs", &self.m_childShapeAabbs)?;
            serializer.skip_field("childShapeKeys", &self.m_childShapeKeys)?;
            serializer.end()
        }
    }
};
