use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterData`
/// -         version: `7`
/// -       signature: `0x300d6808`
/// -          size: 144(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterData {
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
    /// -          name: `characterControllerInfo`(ctype: `struct hkbCharacterDataCharacterControllerInfo`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 24(x86_64)
    ///
    pub m_characterControllerInfo: hkbCharacterDataCharacterControllerInfo,
    /// # C++ Info
    /// -          name: `modelUpMS`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_modelUpMS: Vector4,
    /// # C++ Info
    /// -          name: `modelForwardMS`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_modelForwardMS: Vector4,
    /// # C++ Info
    /// -          name: `modelRightMS`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_modelRightMS: Vector4,
    /// # C++ Info
    /// -          name: `characterPropertyInfos`(ctype: `hkArray<struct hkbVariableInfo>`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_characterPropertyInfos: Vec<hkbVariableInfo>,
    /// # C++ Info
    /// -          name: `numBonesPerLod`(ctype: `hkArray<hkInt32>`)
    /// -        offset:  92(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_numBonesPerLod: Vec<i32>,
    /// # C++ Info
    /// -          name: `characterPropertyValues`(ctype: `struct hkbVariableValueSet*`)
    /// -        offset: 104(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_characterPropertyValues: Pointer,
    /// # C++ Info
    /// -          name: `footIkDriverInfo`(ctype: `struct hkbFootIkDriverInfo*`)
    /// -        offset: 108(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_footIkDriverInfo: Pointer,
    /// # C++ Info
    /// -          name: `handIkDriverInfo`(ctype: `struct hkbHandIkDriverInfo*`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_handIkDriverInfo: Pointer,
    /// # C++ Info
    /// -          name: `stringData`(ctype: `struct hkbCharacterStringData*`)
    /// -        offset: 116(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_stringData: Pointer,
    /// # C++ Info
    /// -          name: `mirroredSkeletonInfo`(ctype: `struct hkbMirroredSkeletonInfo*`)
    /// -        offset: 120(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_mirroredSkeletonInfo: Pointer,
    /// # C++ Info
    /// -          name: `scale`(ctype: `hkReal`)
    /// -        offset: 124(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_scale: f32,
    /// # C++ Info
    /// -          name: `numHands`(ctype: `hkInt16`)
    /// -        offset: 128(x86)/172(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numHands: i16,
    /// # C++ Info
    /// -          name: `numFloatSlots`(ctype: `hkInt16`)
    /// -        offset: 130(x86)/174(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numFloatSlots: i16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCharacterData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x300d6808)
        }
    }
    impl _serde::Serialize for hkbCharacterData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x300d6808)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "characterControllerInfo",
                    &self.m_characterControllerInfo,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("modelUpMS", &self.m_modelUpMS)?;
            serializer.serialize_field("modelForwardMS", &self.m_modelForwardMS)?;
            serializer.serialize_field("modelRightMS", &self.m_modelRightMS)?;
            serializer
                .serialize_array_meta_field(
                    "characterPropertyInfos",
                    &self.m_characterPropertyInfos,
                )?;
            serializer
                .serialize_array_meta_field("numBonesPerLod", &self.m_numBonesPerLod)?;
            serializer
                .serialize_field(
                    "characterPropertyValues",
                    &self.m_characterPropertyValues,
                )?;
            serializer.serialize_field("footIkDriverInfo", &self.m_footIkDriverInfo)?;
            serializer.serialize_field("handIkDriverInfo", &self.m_handIkDriverInfo)?;
            serializer.serialize_field("stringData", &self.m_stringData)?;
            serializer
                .serialize_field("mirroredSkeletonInfo", &self.m_mirroredSkeletonInfo)?;
            serializer.serialize_field("scale", &self.m_scale)?;
            serializer.skip_field("numHands", &self.m_numHands)?;
            serializer.skip_field("numFloatSlots", &self.m_numFloatSlots)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "characterPropertyInfos",
                    &self.m_characterPropertyInfos,
                )?;
            serializer.serialize_array_field("numBonesPerLod", &self.m_numBonesPerLod)?;
            serializer.end()
        }
    }
};
