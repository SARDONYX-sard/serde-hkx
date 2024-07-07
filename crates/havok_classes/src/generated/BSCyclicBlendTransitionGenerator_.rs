use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSCyclicBlendTransitionGenerator`
/// -         version: `1`
/// -       signature: `0x5119eb06`
/// -          size: 112(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSCyclicBlendTransitionGenerator<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// -          name: `pBlenderGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_pBlenderGenerator: Pointer,
    /// # C++ Info
    /// -          name: `EventToFreezeBlendValue`(ctype: `struct hkbEventProperty`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_EventToFreezeBlendValue: hkbEventProperty,
    /// # C++ Info
    /// -          name: `EventToCrossBlend`(ctype: `struct hkbEventProperty`)
    /// -        offset:  60(x86)/104(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_EventToCrossBlend: hkbEventProperty,
    /// # C++ Info
    /// -          name: `fBlendParameter`(ctype: `hkReal`)
    /// -        offset:  68(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fBlendParameter: f32,
    /// # C++ Info
    /// -          name: `fTransitionDuration`(ctype: `hkReal`)
    /// -        offset:  72(x86)/124(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fTransitionDuration: f32,
    /// # C++ Info
    /// -          name: `eBlendCurve`(ctype: `enum BlendCurve`)
    /// -        offset:  76(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_eBlendCurve: BlendCurve,
    /// # C++ Info
    /// -          name: `pTransitionBlenderGenerator`(ctype: `void*`)
    /// -        offset:  80(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16|SERIALIZE_IGNORED`
    ///
    pub m_pTransitionBlenderGenerator: Pointer,
    /// # C++ Info
    /// -          name: `pTransitionEffect`(ctype: `void*`)
    /// -        offset:  96(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16|SERIALIZE_IGNORED`
    ///
    pub m_pTransitionEffect: Pointer,
    /// # C++ Info
    /// -          name: `currentMode`(ctype: `enum unknown`)
    /// -        offset: 100(x86)/168(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_currentMode: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSCyclicBlendTransitionGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSCyclicBlendTransitionGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5119eb06)
        }
    }
    impl<'a> _serde::Serialize for BSCyclicBlendTransitionGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5119eb06)));
            let mut serializer = __serializer
                .serialize_struct("BSCyclicBlendTransitionGenerator", class_meta)?;
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
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("pBlenderGenerator", &self.m_pBlenderGenerator)?;
            serializer
                .serialize_field(
                    "EventToFreezeBlendValue",
                    &self.m_EventToFreezeBlendValue,
                )?;
            serializer.serialize_field("EventToCrossBlend", &self.m_EventToCrossBlend)?;
            serializer.serialize_field("fBlendParameter", &self.m_fBlendParameter)?;
            serializer
                .serialize_field("fTransitionDuration", &self.m_fTransitionDuration)?;
            serializer.serialize_field("eBlendCurve", &self.m_eBlendCurve)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer
                .skip_field(
                    "pTransitionBlenderGenerator",
                    &self.m_pTransitionBlenderGenerator,
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("pTransitionEffect", &self.m_pTransitionEffect)?;
            serializer.skip_field("currentMode", &self.m_currentMode)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 7usize].as_slice())?;
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
