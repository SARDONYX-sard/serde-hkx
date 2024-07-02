use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCombineTransformsModifier`
/// -         version: `0`
/// -       signature: `0xfd1f0b79`
/// -          size: 160(x86)/192(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCombineTransformsModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `translationOut`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_translationOut: Vector4,
    /// # C++ Info
    /// -          name: `rotationOut`(ctype: `hkQuaternion`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotationOut: Quaternion,
    /// # C++ Info
    /// -          name: `leftTranslation`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_leftTranslation: Vector4,
    /// # C++ Info
    /// -          name: `leftRotation`(ctype: `hkQuaternion`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_leftRotation: Quaternion,
    /// # C++ Info
    /// -          name: `rightTranslation`(ctype: `hkVector4`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rightTranslation: Vector4,
    /// # C++ Info
    /// -          name: `rightRotation`(ctype: `hkQuaternion`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rightRotation: Quaternion,
    /// # C++ Info
    /// -          name: `invertLeftTransform`(ctype: `hkBool`)
    /// -        offset: 144(x86)/176(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_invertLeftTransform: bool,
    /// # C++ Info
    /// -          name: `invertRightTransform`(ctype: `hkBool`)
    /// -        offset: 145(x86)/177(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_invertRightTransform: bool,
    /// # C++ Info
    /// -          name: `invertResult`(ctype: `hkBool`)
    /// -        offset: 146(x86)/178(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_invertResult: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCombineTransformsModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCombineTransformsModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(4246670201u32)
        }
    }
    impl<'a> _serde::Serialize for hkbCombineTransformsModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(4246670201u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbCombineTransformsModifier", class_meta)?;
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
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("translationOut", &self.m_translationOut)?;
            serializer.serialize_field("rotationOut", &self.m_rotationOut)?;
            serializer.serialize_field("leftTranslation", &self.m_leftTranslation)?;
            serializer.serialize_field("leftRotation", &self.m_leftRotation)?;
            serializer.serialize_field("rightTranslation", &self.m_rightTranslation)?;
            serializer.serialize_field("rightRotation", &self.m_rightRotation)?;
            serializer
                .serialize_field("invertLeftTransform", &self.m_invertLeftTransform)?;
            serializer
                .serialize_field("invertRightTransform", &self.m_invertRightTransform)?;
            serializer.serialize_field("invertResult", &self.m_invertResult)?;
            serializer.pad_field([0u8; 13usize].as_slice(), [0u8; 13usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
