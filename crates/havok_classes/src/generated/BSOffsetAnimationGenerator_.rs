use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSOffsetAnimationGenerator`
/// -         version: `1`
/// -       signature: `0xb8571122`
/// -          size: 128(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSOffsetAnimationGenerator<'a> {
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
    /// -          name: `pDefaultGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_pDefaultGenerator: Pointer,
    /// # C++ Info
    /// -          name: `pOffsetClipGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_pOffsetClipGenerator: Pointer,
    /// # C++ Info
    /// -          name: `fOffsetVariable`(ctype: `hkReal`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fOffsetVariable: f32,
    /// # C++ Info
    /// -          name: `fOffsetRangeStart`(ctype: `hkReal`)
    /// -        offset:  72(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fOffsetRangeStart: f32,
    /// # C++ Info
    /// -          name: `fOffsetRangeEnd`(ctype: `hkReal`)
    /// -        offset:  76(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fOffsetRangeEnd: f32,
    /// # C++ Info
    /// -          name: `BoneOffsetA`(ctype: `hkArray<void>`)
    /// -        offset:  80(x86)/120(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_BoneOffsetA: Vec<()>,
    /// # C++ Info
    /// -          name: `BoneIndexA`(ctype: `hkArray<void>`)
    /// -        offset:  92(x86)/136(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_BoneIndexA: Vec<()>,
    /// # C++ Info
    /// -          name: `fCurrentPercentage`(ctype: `hkReal`)
    /// -        offset: 104(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_fCurrentPercentage: f32,
    /// # C++ Info
    /// -          name: `iCurrentFrame`(ctype: `hkUint32`)
    /// -        offset: 108(x86)/156(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_iCurrentFrame: u32,
    /// # C++ Info
    /// -          name: `bZeroOffset`(ctype: `hkBool`)
    /// -        offset: 112(x86)/160(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bZeroOffset: bool,
    /// # C++ Info
    /// -          name: `bOffsetValid`(ctype: `hkBool`)
    /// -        offset: 113(x86)/161(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bOffsetValid: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSOffsetAnimationGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSOffsetAnimationGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3092713762u32)
        }
    }
    impl<'a> _serde::Serialize for BSOffsetAnimationGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3092713762u32)));
            let mut serializer = __serializer
                .serialize_struct("BSOffsetAnimationGenerator", class_meta)?;
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
            serializer.serialize_field("pDefaultGenerator", &self.m_pDefaultGenerator)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field("pOffsetClipGenerator", &self.m_pOffsetClipGenerator)?;
            serializer.serialize_field("fOffsetVariable", &self.m_fOffsetVariable)?;
            serializer.serialize_field("fOffsetRangeStart", &self.m_fOffsetRangeStart)?;
            serializer.serialize_field("fOffsetRangeEnd", &self.m_fOffsetRangeEnd)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_array_meta_field("BoneOffsetA", &self.m_BoneOffsetA)?;
            serializer.skip_array_meta_field("BoneIndexA", &self.m_BoneIndexA)?;
            serializer.skip_field("fCurrentPercentage", &self.m_fCurrentPercentage)?;
            serializer.skip_field("iCurrentFrame", &self.m_iCurrentFrame)?;
            serializer.skip_field("bZeroOffset", &self.m_bZeroOffset)?;
            serializer.skip_field("bOffsetValid", &self.m_bOffsetValid)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("BoneOffsetA", &self.m_BoneOffsetA)?;
            serializer.serialize_array_field("BoneIndexA", &self.m_BoneIndexA)?;
            serializer.end()
        }
    }
};
