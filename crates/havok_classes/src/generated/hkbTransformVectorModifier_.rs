use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbTransformVectorModifier`
/// -         version: `0`
/// -       signature: `0xf93e0e24`
/// -          size: 128(x86)/160(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbTransformVectorModifier<'a> {
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
    /// -          name: `rotation`(ctype: `hkQuaternion`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotation: Quaternion,
    /// # C++ Info
    /// -          name: `translation`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_translation: Vector4,
    /// # C++ Info
    /// -          name: `vectorIn`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vectorIn: Vector4,
    /// # C++ Info
    /// -          name: `vectorOut`(ctype: `hkVector4`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vectorOut: Vector4,
    /// # C++ Info
    /// -          name: `rotateOnly`(ctype: `hkBool`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_rotateOnly: bool,
    /// # C++ Info
    /// -          name: `inverse`(ctype: `hkBool`)
    /// -        offset: 113(x86)/145(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_inverse: bool,
    /// # C++ Info
    /// -          name: `computeOnActivate`(ctype: `hkBool`)
    /// -        offset: 114(x86)/146(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_computeOnActivate: bool,
    /// # C++ Info
    /// -          name: `computeOnModify`(ctype: `hkBool`)
    /// -        offset: 115(x86)/147(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_computeOnModify: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbTransformVectorModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbTransformVectorModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(4181593636u32)
        }
    }
    impl<'a> _serde::Serialize for hkbTransformVectorModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(4181593636u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbTransformVectorModifier", class_meta)?;
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
            serializer.serialize_field("rotation", &self.m_rotation)?;
            serializer.serialize_field("translation", &self.m_translation)?;
            serializer.serialize_field("vectorIn", &self.m_vectorIn)?;
            serializer.serialize_field("vectorOut", &self.m_vectorOut)?;
            serializer.serialize_field("rotateOnly", &self.m_rotateOnly)?;
            serializer.serialize_field("inverse", &self.m_inverse)?;
            serializer.serialize_field("computeOnActivate", &self.m_computeOnActivate)?;
            serializer.serialize_field("computeOnModify", &self.m_computeOnModify)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
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
