use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxMaterialShader`
/// -         version: `1`
/// -       signature: `0x28515eff`
/// -          size:  40(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMaterialShader<'a> {
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
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum ShaderType`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: ShaderType,
    /// # C++ Info
    /// -          name: `vertexEntryName`(ctype: `hkStringPtr`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_vertexEntryName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `geomEntryName`(ctype: `hkStringPtr`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_geomEntryName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `pixelEntryName`(ctype: `hkStringPtr`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_pixelEntryName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_data: Vec<u8>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkxMaterialShader<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxMaterialShader"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(676421375u32)
        }
    }
    impl<'a> __serde::Serialize for hkxMaterialShader<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxMaterialShader", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "vertexEntryName",
                    &self.m_vertexEntryName,
                )?;
            serializer
                .serialize_stringptr_meta_field("geomEntryName", &self.m_geomEntryName)?;
            serializer
                .serialize_stringptr_meta_field(
                    "pixelEntryName",
                    &self.m_pixelEntryName,
                )?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer
                .serialize_stringptr_field("vertexEntryName", &self.m_vertexEntryName)?;
            serializer
                .serialize_stringptr_field("geomEntryName", &self.m_geomEntryName)?;
            serializer
                .serialize_stringptr_field("pixelEntryName", &self.m_pixelEntryName)?;
            serializer.serialize_array_field("data", &self.m_data)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT8`
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
pub enum ShaderType {
    #[default]
    EFFECT_TYPE_INVALID = 0isize,
    EFFECT_TYPE_UNKNOWN = 1isize,
    EFFECT_TYPE_HLSL_INLINE = 2isize,
    EFFECT_TYPE_CG_INLINE = 3isize,
    EFFECT_TYPE_HLSL_FILENAME = 4isize,
    EFFECT_TYPE_CG_FILENAME = 5isize,
    EFFECT_TYPE_MAX_ID = 6isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ShaderType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::EFFECT_TYPE_INVALID => {
                    __serializer.serialize_field("EFFECT_TYPE_INVALID", &0u64)
                }
                Self::EFFECT_TYPE_UNKNOWN => {
                    __serializer.serialize_field("EFFECT_TYPE_UNKNOWN", &1u64)
                }
                Self::EFFECT_TYPE_HLSL_INLINE => {
                    __serializer.serialize_field("EFFECT_TYPE_HLSL_INLINE", &2u64)
                }
                Self::EFFECT_TYPE_CG_INLINE => {
                    __serializer.serialize_field("EFFECT_TYPE_CG_INLINE", &3u64)
                }
                Self::EFFECT_TYPE_HLSL_FILENAME => {
                    __serializer.serialize_field("EFFECT_TYPE_HLSL_FILENAME", &4u64)
                }
                Self::EFFECT_TYPE_CG_FILENAME => {
                    __serializer.serialize_field("EFFECT_TYPE_CG_FILENAME", &5u64)
                }
                Self::EFFECT_TYPE_MAX_ID => {
                    __serializer.serialize_field("EFFECT_TYPE_MAX_ID", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ShaderType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
