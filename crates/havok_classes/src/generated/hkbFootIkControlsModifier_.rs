use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkControlsModifier`
/// -         version: `1`
/// -       signature: `0xe5b6f544`
/// -          size: 144(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkControlsModifier<'a> {
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
    /// -          name: `controlData`(ctype: `struct hkbFootIkControlData`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_controlData: hkbFootIkControlData,
    /// # C++ Info
    /// -          name: `legs`(ctype: `hkArray<struct hkbFootIkControlsModifierLeg>`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_legs: Vec<hkbFootIkControlsModifierLeg>,
    /// # C++ Info
    /// -          name: `errorOutTranslation`(ctype: `hkVector4`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_errorOutTranslation: Vector4,
    /// # C++ Info
    /// -          name: `alignWithGroundRotation`(ctype: `hkQuaternion`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_alignWithGroundRotation: Quaternion,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbFootIkControlsModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkControlsModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3853972804u32)
        }
    }
    impl<'a> _serde::Serialize for hkbFootIkControlsModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3853972804u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkControlsModifier", class_meta)?;
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
            serializer.serialize_field("controlData", &self.m_controlData)?;
            serializer.serialize_array_meta_field("legs", &self.m_legs)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field("errorOutTranslation", &self.m_errorOutTranslation)?;
            serializer
                .serialize_field(
                    "alignWithGroundRotation",
                    &self.m_alignWithGroundRotation,
                )?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("legs", &self.m_legs)?;
            serializer.end()
        }
    }
};
