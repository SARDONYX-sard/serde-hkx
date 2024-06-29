use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMeshTexture`
/// -         version: `0`
/// -       signature: `0xc9887918`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMeshTexture {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkMeshTexture {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMeshTexture"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3381164312u32)
        }
    }
    impl __serde::Serialize for hkMeshTexture {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMeshTexture", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
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
pub enum Format {
    #[default]
    Unknown = 0isize,
    PNG = 1isize,
    TGA = 2isize,
    BMP = 3isize,
    DDS = 4isize,
}
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
pub enum FilterMode {
    #[default]
    POINT = 0isize,
    LINEAR = 1isize,
    ANISOTROPIC = 2isize,
}
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
pub enum TextureUsageType {
    #[default]
    UNKNOWN = 0isize,
    DIFFUSE = 1isize,
    REFLECTION = 2isize,
    BUMP = 3isize,
    NORMAL = 4isize,
    DISPLACEMENT = 5isize,
    SPECULAR = 6isize,
    SPECULARANDGLOSS = 7isize,
    OPACITY = 8isize,
    EMISSIVE = 9isize,
    REFRACTION = 10isize,
    GLOSS = 11isize,
    NOTEXPORTED = 12isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Format {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::Unknown => __serializer.serialize_field("Unknown", &0u64),
                Self::PNG => __serializer.serialize_field("PNG", &1u64),
                Self::TGA => __serializer.serialize_field("TGA", &2u64),
                Self::BMP => __serializer.serialize_field("BMP", &3u64),
                Self::DDS => __serializer.serialize_field("DDS", &4u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i8().ok_or(S::Error::custom("Failed enum Format to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for FilterMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::POINT => __serializer.serialize_field("POINT", &0u64),
                Self::LINEAR => __serializer.serialize_field("LINEAR", &1u64),
                Self::ANISOTROPIC => __serializer.serialize_field("ANISOTROPIC", &2u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum FilterMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for TextureUsageType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::UNKNOWN => __serializer.serialize_field("UNKNOWN", &0u64),
                Self::DIFFUSE => __serializer.serialize_field("DIFFUSE", &1u64),
                Self::REFLECTION => __serializer.serialize_field("REFLECTION", &2u64),
                Self::BUMP => __serializer.serialize_field("BUMP", &3u64),
                Self::NORMAL => __serializer.serialize_field("NORMAL", &4u64),
                Self::DISPLACEMENT => __serializer.serialize_field("DISPLACEMENT", &5u64),
                Self::SPECULAR => __serializer.serialize_field("SPECULAR", &6u64),
                Self::SPECULARANDGLOSS => {
                    __serializer.serialize_field("SPECULARANDGLOSS", &7u64)
                }
                Self::OPACITY => __serializer.serialize_field("OPACITY", &8u64),
                Self::EMISSIVE => __serializer.serialize_field("EMISSIVE", &9u64),
                Self::REFRACTION => __serializer.serialize_field("REFRACTION", &10u64),
                Self::GLOSS => __serializer.serialize_field("GLOSS", &11u64),
                Self::NOTEXPORTED => __serializer.serialize_field("NOTEXPORTED", &12u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum TextureUsageType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
