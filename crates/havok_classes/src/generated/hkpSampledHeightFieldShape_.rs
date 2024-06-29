use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSampledHeightFieldShape`
/// -         version: `0`
/// -       signature: `0x11213421`
/// -          size:  96(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSampledHeightFieldShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpHeightFieldShape,
    /// # C++ Info
    /// -          name: `xRes`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_xRes: i32,
    /// # C++ Info
    /// -          name: `zRes`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_zRes: i32,
    /// # C++ Info
    /// -          name: `heightCenter`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_heightCenter: f32,
    /// # C++ Info
    /// -          name: `useProjectionBasedHeight`(ctype: `hkBool`)
    /// -        offset:  28(x86)/ 44(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useProjectionBasedHeight: bool,
    /// # C++ Info
    /// -          name: `heightfieldType`(ctype: `enum HeightFieldType`)
    /// -        offset:  29(x86)/ 45(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_heightfieldType: HeightFieldType,
    /// # C++ Info
    /// -          name: `intToFloatScale`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_intToFloatScale: Vector4,
    /// # C++ Info
    /// -          name: `floatToIntScale`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_floatToIntScale: Vector4,
    /// # C++ Info
    /// -          name: `floatToIntOffsetFloorCorrected`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_floatToIntOffsetFloorCorrected: Vector4,
    /// # C++ Info
    /// -          name: `extents`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_extents: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSampledHeightFieldShape {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSampledHeightFieldShape"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(287388705u32)
        }
    }
    impl __serde::Serialize for hkpSampledHeightFieldShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSampledHeightFieldShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("xRes", &self.m_xRes)?;
            serializer.serialize_field("zRes", &self.m_zRes)?;
            serializer.serialize_field("heightCenter", &self.m_heightCenter)?;
            serializer
                .serialize_field(
                    "useProjectionBasedHeight",
                    &self.m_useProjectionBasedHeight,
                )?;
            serializer.serialize_field("heightfieldType", &self.m_heightfieldType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("intToFloatScale", &self.m_intToFloatScale)?;
            serializer.serialize_field("floatToIntScale", &self.m_floatToIntScale)?;
            serializer
                .serialize_field(
                    "floatToIntOffsetFloorCorrected",
                    &self.m_floatToIntOffsetFloorCorrected,
                )?;
            serializer.serialize_field("extents", &self.m_extents)?;
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
pub enum HeightFieldType {
    #[default]
    HEIGHTFIELD_STORAGE = 0isize,
    HEIGHTFIELD_COMPRESSED = 1isize,
    HEIGHTFIELD_USER = 2isize,
    HEIGHTFIELD_MAX_ID = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for HeightFieldType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HEIGHTFIELD_STORAGE => {
                    __serializer.serialize_field("HEIGHTFIELD_STORAGE", &0u64)
                }
                Self::HEIGHTFIELD_COMPRESSED => {
                    __serializer.serialize_field("HEIGHTFIELD_COMPRESSED", &1u64)
                }
                Self::HEIGHTFIELD_USER => {
                    __serializer.serialize_field("HEIGHTFIELD_USER", &2u64)
                }
                Self::HEIGHTFIELD_MAX_ID => {
                    __serializer.serialize_field("HEIGHTFIELD_MAX_ID", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum HeightFieldType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
