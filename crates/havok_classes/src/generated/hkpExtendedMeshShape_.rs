use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpExtendedMeshShape`
/// -         version: `3`
/// -       signature: `0x177114a2`
/// -          size: 240(x86)/336(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpExtendedMeshShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShapeCollection,
    /// # C++ Info
    /// -          name: `embeddedTrianglesSubpart`(ctype: `struct hkpExtendedMeshShapeTrianglesSubpart`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size: 112(x86)/160(x86_64)
    ///
    pub m_embeddedTrianglesSubpart: hkpExtendedMeshShapeTrianglesSubpart,
    /// # C++ Info
    /// -          name: `aabbHalfExtents`(ctype: `hkVector4`)
    /// -        offset: 144(x86)/208(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_aabbHalfExtents: Vector4,
    /// # C++ Info
    /// -          name: `aabbCenter`(ctype: `hkVector4`)
    /// -        offset: 160(x86)/224(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_aabbCenter: Vector4,
    /// # C++ Info
    /// -          name: `materialClass`(ctype: `void*`)
    /// -        offset: 176(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialClass: Pointer,
    /// # C++ Info
    /// -          name: `numBitsForSubpartIndex`(ctype: `hkInt32`)
    /// -        offset: 180(x86)/248(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numBitsForSubpartIndex: i32,
    /// # C++ Info
    /// -          name: `trianglesSubparts`(ctype: `hkArray<struct hkpExtendedMeshShapeTrianglesSubpart>`)
    /// -        offset: 184(x86)/256(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_trianglesSubparts: Vec<hkpExtendedMeshShapeTrianglesSubpart>,
    /// # C++ Info
    /// -          name: `shapesSubparts`(ctype: `hkArray<struct hkpExtendedMeshShapeShapesSubpart>`)
    /// -        offset: 196(x86)/272(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_shapesSubparts: Vec<hkpExtendedMeshShapeShapesSubpart>,
    /// # C++ Info
    /// -          name: `weldingInfo`(ctype: `hkArray<hkUint16>`)
    /// -        offset: 208(x86)/288(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_weldingInfo: Vec<u16>,
    /// # C++ Info
    /// -          name: `weldingType`(ctype: `enum WeldingType`)
    /// -        offset: 220(x86)/304(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_weldingType: WeldingType,
    /// # C++ Info
    /// -          name: `defaultCollisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset: 224(x86)/308(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_defaultCollisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `cachedNumChildShapes`(ctype: `hkInt32`)
    /// -        offset: 228(x86)/312(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cachedNumChildShapes: i32,
    /// # C++ Info
    /// -          name: `triangleRadius`(ctype: `hkReal`)
    /// -        offset: 232(x86)/316(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_triangleRadius: f32,
    /// # C++ Info
    /// -          name: `padding`(ctype: `hkInt32`)
    /// -        offset: 236(x86)/320(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_padding: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpExtendedMeshShape {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpExtendedMeshShape"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(393286818u32)
        }
    }
    impl __serde::Serialize for hkpExtendedMeshShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpExtendedMeshShape", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("disableWelding", &self.parent.m_disableWelding)?;
            serializer.serialize_field("collectionType", &self.parent.m_collectionType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field(
                    "embeddedTrianglesSubpart",
                    &self.m_embeddedTrianglesSubpart,
                )?;
            serializer.serialize_field("aabbHalfExtents", &self.m_aabbHalfExtents)?;
            serializer.serialize_field("aabbCenter", &self.m_aabbCenter)?;
            serializer.skip_field("materialClass", &self.m_materialClass)?;
            serializer
                .serialize_field(
                    "numBitsForSubpartIndex",
                    &self.m_numBitsForSubpartIndex,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "trianglesSubparts",
                    &self.m_trianglesSubparts,
                )?;
            serializer
                .serialize_array_meta_field("shapesSubparts", &self.m_shapesSubparts)?;
            serializer.serialize_array_meta_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.serialize_field("weldingType", &self.m_weldingType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "defaultCollisionFilterInfo",
                    &self.m_defaultCollisionFilterInfo,
                )?;
            serializer
                .serialize_field("cachedNumChildShapes", &self.m_cachedNumChildShapes)?;
            serializer.serialize_field("triangleRadius", &self.m_triangleRadius)?;
            serializer.skip_field("padding", &self.m_padding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer
                .serialize_array_field("trianglesSubparts", &self.m_trianglesSubparts)?;
            serializer.serialize_array_field("shapesSubparts", &self.m_shapesSubparts)?;
            serializer.serialize_array_field("weldingInfo", &self.m_weldingInfo)?;
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
pub enum IndexStridingType {
    #[default]
    INDICES_INVALID = 0isize,
    INDICES_INT8 = 1isize,
    INDICES_INT16 = 2isize,
    INDICES_INT32 = 3isize,
    INDICES_MAX_ID = 4isize,
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
pub enum MaterialIndexStridingType {
    #[default]
    MATERIAL_INDICES_INVALID = 0isize,
    MATERIAL_INDICES_INT8 = 1isize,
    MATERIAL_INDICES_INT16 = 2isize,
    MATERIAL_INDICES_MAX_ID = 3isize,
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
pub enum SubpartType {
    #[default]
    SUBPART_TRIANGLES = 0isize,
    SUBPART_SHAPE = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for IndexStridingType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::INDICES_INVALID => {
                    __serializer.serialize_field("INDICES_INVALID", &0u64)
                }
                Self::INDICES_INT8 => __serializer.serialize_field("INDICES_INT8", &1u64),
                Self::INDICES_INT16 => {
                    __serializer.serialize_field("INDICES_INT16", &2u64)
                }
                Self::INDICES_INT32 => {
                    __serializer.serialize_field("INDICES_INT32", &3u64)
                }
                Self::INDICES_MAX_ID => {
                    __serializer.serialize_field("INDICES_MAX_ID", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum IndexStridingType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MaterialIndexStridingType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MATERIAL_INDICES_INVALID => {
                    __serializer.serialize_field("MATERIAL_INDICES_INVALID", &0u64)
                }
                Self::MATERIAL_INDICES_INT8 => {
                    __serializer.serialize_field("MATERIAL_INDICES_INT8", &1u64)
                }
                Self::MATERIAL_INDICES_INT16 => {
                    __serializer.serialize_field("MATERIAL_INDICES_INT16", &2u64)
                }
                Self::MATERIAL_INDICES_MAX_ID => {
                    __serializer.serialize_field("MATERIAL_INDICES_MAX_ID", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum MaterialIndexStridingType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SubpartType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::SUBPART_TRIANGLES => {
                    __serializer.serialize_field("SUBPART_TRIANGLES", &0u64)
                }
                Self::SUBPART_SHAPE => {
                    __serializer.serialize_field("SUBPART_SHAPE", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum SubpartType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
