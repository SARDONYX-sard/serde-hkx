use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BGSGamebryoSequenceGenerator`
/// -         version: `2`
/// -       signature: `0xc8df2d77`
/// -          size:  72(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BGSGamebryoSequenceGenerator<'a> {
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
    /// -          name: `pSequence`(ctype: `char*`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_pSequence: CString<'a>,
    /// # C++ Info
    /// -          name: `eBlendModeFunction`(ctype: `enum BlendModeFunction`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_eBlendModeFunction: BlendModeFunction,
    /// # C++ Info
    /// -          name: `fPercent`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fPercent: f32,
    /// # C++ Info
    /// -          name: `events`(ctype: `hkArray<void>`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_events: Vec<()>,
    /// # C++ Info
    /// -          name: `fTime`(ctype: `hkReal`)
    /// -        offset:  64(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_fTime: f32,
    /// # C++ Info
    /// -          name: `bDelayedActivate`(ctype: `hkBool`)
    /// -        offset:  68(x86)/108(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bDelayedActivate: bool,
    /// # C++ Info
    /// -          name: `bLooping`(ctype: `hkBool`)
    /// -        offset:  69(x86)/109(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bLooping: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for BGSGamebryoSequenceGenerator<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"BGSGamebryoSequenceGenerator"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3370069367u32)
        }
    }
    impl<'a> __serde::Serialize for BGSGamebryoSequenceGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("BGSGamebryoSequenceGenerator", class_meta)?;
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
            serializer.serialize_cstring_meta_field("pSequence", &self.m_pSequence)?;
            serializer
                .serialize_field("eBlendModeFunction", &self.m_eBlendModeFunction)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("fPercent", &self.m_fPercent)?;
            serializer.skip_array_meta_field("events", &self.m_events)?;
            serializer.skip_field("fTime", &self.m_fTime)?;
            serializer.skip_field("bDelayedActivate", &self.m_bDelayedActivate)?;
            serializer.skip_field("bLooping", &self.m_bLooping)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_cstring_field("pSequence", &self.m_pSequence)?;
            serializer.serialize_array_field("events", &self.m_events)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum BlendModeFunction {
    #[default]
    BMF_NONE = 0isize,
    BMF_PERCENT = 1isize,
    BMF_ONE_MINUS_PERCENT = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BlendModeFunction {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BMF_NONE => __serializer.serialize_field("BMF_NONE", &0u64),
                Self::BMF_PERCENT => __serializer.serialize_field("BMF_PERCENT", &1u64),
                Self::BMF_ONE_MINUS_PERCENT => {
                    __serializer.serialize_field("BMF_ONE_MINUS_PERCENT", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BlendModeFunction to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
