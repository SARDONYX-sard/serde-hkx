use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaAnimationBinding`
/// -         version: `1`
/// -       signature: `0x66eac971`
/// -          size:  44(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnimationBinding<'a> {
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
    /// -          name: `originalSkeletonName`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_originalSkeletonName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `animation`(ctype: `struct hkaAnimation*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_animation: Pointer,
    /// # C++ Info
    /// -          name: `transformTrackToBoneIndices`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_transformTrackToBoneIndices: Vec<i16>,
    /// # C++ Info
    /// -          name: `floatTrackToFloatSlotIndices`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_floatTrackToFloatSlotIndices: Vec<i16>,
    /// # C++ Info
    /// -          name: `blendHint`(ctype: `enum BlendHint`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_blendHint: BlendHint,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkaAnimationBinding<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaAnimationBinding"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1726663025u32)
        }
    }
    impl<'a> __serde::Serialize for hkaAnimationBinding<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkaAnimationBinding", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer.serialize_field("animation", &self.m_animation)?;
            serializer
                .serialize_array_meta_field(
                    "transformTrackToBoneIndices",
                    &self.m_transformTrackToBoneIndices,
                )?;
            serializer
                .serialize_array_meta_field(
                    "floatTrackToFloatSlotIndices",
                    &self.m_floatTrackToFloatSlotIndices,
                )?;
            serializer.serialize_field("blendHint", &self.m_blendHint)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_stringptr_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer
                .serialize_array_field(
                    "transformTrackToBoneIndices",
                    &self.m_transformTrackToBoneIndices,
                )?;
            serializer
                .serialize_array_field(
                    "floatTrackToFloatSlotIndices",
                    &self.m_floatTrackToFloatSlotIndices,
                )?;
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
pub enum BlendHint {
    #[default]
    NORMAL = 0isize,
    ADDITIVE = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BlendHint {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NORMAL => __serializer.serialize_field("NORMAL", &0u64),
                Self::ADDITIVE => __serializer.serialize_field("ADDITIVE", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BlendHint to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
