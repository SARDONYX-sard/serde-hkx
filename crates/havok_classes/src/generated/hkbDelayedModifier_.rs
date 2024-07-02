use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbDelayedModifier`
/// -         version: `0`
/// -       signature: `0x8e101a7a`
/// -          size:  64(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbDelayedModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifierWrapper<'a>,
    /// # C++ Info
    /// -          name: `delaySeconds`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_delaySeconds: f32,
    /// # C++ Info
    /// -          name: `durationSeconds`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_durationSeconds: f32,
    /// # C++ Info
    /// -          name: `secondsElapsed`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_secondsElapsed: f32,
    /// # C++ Info
    /// -          name: `isActive`(ctype: `hkBool`)
    /// -        offset:  60(x86)/100(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isActive: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbDelayedModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbDelayedModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2383420026u32)
        }
    }
    impl<'a> _serde::Serialize for hkbDelayedModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2383420026u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbDelayedModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field(
                    "name",
                    &self.parent.parent.parent.m_name,
                )?;
            serializer.skip_field("id", &self.parent.parent.parent.m_id)?;
            serializer
                .skip_field("cloneState", &self.parent.parent.parent.m_cloneState)?;
            serializer
                .skip_field("padNode", &self.parent.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.parent.m_enable)?;
            serializer
                .skip_field(
                    "padModifier",
                    &self.parent.parent.m_padModifier.as_slice(),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("modifier", &self.parent.m_modifier)?;
            serializer.serialize_field("delaySeconds", &self.m_delaySeconds)?;
            serializer.serialize_field("durationSeconds", &self.m_durationSeconds)?;
            serializer.skip_field("secondsElapsed", &self.m_secondsElapsed)?;
            serializer.skip_field("isActive", &self.m_isActive)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .serialize_stringptr_field("name", &self.parent.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
