use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCompressedMeshShape`
/// -         version: `9`
/// -       signature: `0xa62d5e6e`
/// -          size: 224(x86)/304(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCompressedMeshShape<'a> {
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
    /// -          name: `bitsPerIndex`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bitsPerIndex: i32,
    /// # C++ Info
    /// -          name: `bitsPerWIndex`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bitsPerWIndex: i32,
    /// # C++ Info
    /// -          name: `wIndexMask`(ctype: `hkInt32`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wIndexMask: i32,
    /// # C++ Info
    /// -          name: `indexMask`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_indexMask: i32,
    /// # C++ Info
    /// -          name: `radius`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_radius: f32,
    /// # C++ Info
    /// -          name: `weldingType`(ctype: `enum WeldingType`)
    /// -        offset:  44(x86)/ 68(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_weldingType: WeldingType,
    /// # C++ Info
    /// -          name: `materialType`(ctype: `enum MaterialType`)
    /// -        offset:  45(x86)/ 69(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_materialType: MaterialType,
    /// # C++ Info
    /// -          name: `materials`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materials: Vec<u32>,
    /// # C++ Info
    /// -          name: `materials16`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  60(x86)/ 88(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materials16: Vec<u16>,
    /// # C++ Info
    /// -          name: `materials8`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  72(x86)/104(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_materials8: Vec<u8>,
    /// # C++ Info
    /// -          name: `transforms`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset:  84(x86)/120(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_transforms: Vec<QsTransform>,
    /// # C++ Info
    /// -          name: `bigVertices`(ctype: `hkArray<hkVector4>`)
    /// -        offset:  96(x86)/136(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bigVertices: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `bigTriangles`(ctype: `hkArray<struct hkpCompressedMeshShapeBigTriangle>`)
    /// -        offset: 108(x86)/152(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bigTriangles: Vec<hkpCompressedMeshShapeBigTriangle>,
    /// # C++ Info
    /// -          name: `chunks`(ctype: `hkArray<struct hkpCompressedMeshShapeChunk>`)
    /// -        offset: 120(x86)/168(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_chunks: Vec<hkpCompressedMeshShapeChunk>,
    /// # C++ Info
    /// -          name: `convexPieces`(ctype: `hkArray<struct hkpCompressedMeshShapeConvexPiece>`)
    /// -        offset: 132(x86)/184(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_convexPieces: Vec<hkpCompressedMeshShapeConvexPiece>,
    /// # C++ Info
    /// -          name: `error`(ctype: `hkReal`)
    /// -        offset: 144(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_error: f32,
    /// # C++ Info
    /// -          name: `bounds`(ctype: `struct hkAabb`)
    /// -        offset: 160(x86)/208(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_bounds: hkAabb,
    /// # C++ Info
    /// -          name: `defaultCollisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset: 192(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_defaultCollisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `meshMaterials`(ctype: `void*`)
    /// -        offset: 196(x86)/248(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_meshMaterials: Pointer,
    /// # C++ Info
    /// -          name: `materialStriding`(ctype: `hkUint16`)
    /// -        offset: 200(x86)/256(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_materialStriding: u16,
    /// # C++ Info
    /// -          name: `numMaterials`(ctype: `hkUint16`)
    /// -        offset: 202(x86)/258(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_numMaterials: u16,
    /// # C++ Info
    /// -          name: `namedMaterials`(ctype: `hkArray<struct hkpNamedMeshMaterial>`)
    /// -        offset: 204(x86)/264(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_namedMaterials: Vec<hkpNamedMeshMaterial<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpCompressedMeshShape<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpCompressedMeshShape"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2787991150u32)
        }
    }
    impl<'a> __serde::Serialize for hkpCompressedMeshShape<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpCompressedMeshShape", class_meta)?;
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
            serializer.serialize_field("bitsPerIndex", &self.m_bitsPerIndex)?;
            serializer.serialize_field("bitsPerWIndex", &self.m_bitsPerWIndex)?;
            serializer.serialize_field("wIndexMask", &self.m_wIndexMask)?;
            serializer.serialize_field("indexMask", &self.m_indexMask)?;
            serializer.serialize_field("radius", &self.m_radius)?;
            serializer.serialize_field("weldingType", &self.m_weldingType)?;
            serializer.serialize_field("materialType", &self.m_materialType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_array_meta_field("materials", &self.m_materials)?;
            serializer.serialize_array_meta_field("materials16", &self.m_materials16)?;
            serializer.serialize_array_meta_field("materials8", &self.m_materials8)?;
            serializer.serialize_array_meta_field("transforms", &self.m_transforms)?;
            serializer.serialize_array_meta_field("bigVertices", &self.m_bigVertices)?;
            serializer.serialize_array_meta_field("bigTriangles", &self.m_bigTriangles)?;
            serializer.serialize_array_meta_field("chunks", &self.m_chunks)?;
            serializer.serialize_array_meta_field("convexPieces", &self.m_convexPieces)?;
            serializer.serialize_field("error", &self.m_error)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("bounds", &self.m_bounds)?;
            serializer
                .serialize_field(
                    "defaultCollisionFilterInfo",
                    &self.m_defaultCollisionFilterInfo,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("meshMaterials", &self.m_meshMaterials)?;
            serializer.serialize_field("materialStriding", &self.m_materialStriding)?;
            serializer.serialize_field("numMaterials", &self.m_numMaterials)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("namedMaterials", &self.m_namedMaterials)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 24usize].as_slice())?;
            serializer.serialize_array_field("materials", &self.m_materials)?;
            serializer.serialize_array_field("materials16", &self.m_materials16)?;
            serializer.serialize_array_field("materials8", &self.m_materials8)?;
            serializer.serialize_array_field("transforms", &self.m_transforms)?;
            serializer.serialize_array_field("bigVertices", &self.m_bigVertices)?;
            serializer.serialize_array_field("bigTriangles", &self.m_bigTriangles)?;
            serializer.serialize_array_field("chunks", &self.m_chunks)?;
            serializer.serialize_array_field("convexPieces", &self.m_convexPieces)?;
            serializer.serialize_array_field("namedMaterials", &self.m_namedMaterials)?;
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
pub enum MaterialType {
    #[default]
    MATERIAL_NONE = 0isize,
    MATERIAL_SINGLE_VALUE_PER_CHUNK = 1isize,
    MATERIAL_ONE_BYTE_PER_TRIANGLE = 2isize,
    MATERIAL_TWO_BYTES_PER_TRIANGLE = 3isize,
    MATERIAL_FOUR_BYTES_PER_TRIANGLE = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MaterialType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MATERIAL_NONE => {
                    __serializer.serialize_field("MATERIAL_NONE", &0u64)
                }
                Self::MATERIAL_SINGLE_VALUE_PER_CHUNK => {
                    __serializer
                        .serialize_field("MATERIAL_SINGLE_VALUE_PER_CHUNK", &1u64)
                }
                Self::MATERIAL_ONE_BYTE_PER_TRIANGLE => {
                    __serializer.serialize_field("MATERIAL_ONE_BYTE_PER_TRIANGLE", &2u64)
                }
                Self::MATERIAL_TWO_BYTES_PER_TRIANGLE => {
                    __serializer
                        .serialize_field("MATERIAL_TWO_BYTES_PER_TRIANGLE", &3u64)
                }
                Self::MATERIAL_FOUR_BYTES_PER_TRIANGLE => {
                    __serializer
                        .serialize_field("MATERIAL_FOUR_BYTES_PER_TRIANGLE", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum MaterialType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for MaterialType {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint8<__E>(
                    self,
                    __value: u8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u8 => _serde::__private::Ok(__Field::__field0),
                        1u8 => _serde::__private::Ok(__Field::__field1),
                        2u8 => _serde::__private::Ok(__Field::__field2),
                        3u8 => _serde::__private::Ok(__Field::__field3),
                        4u8 => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0" || v.eq_ignore_ascii_case("MATERIAL_NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case("MATERIAL_SINGLE_VALUE_PER_CHUNK") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case("MATERIAL_ONE_BYTE_PER_TRIANGLE") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case("MATERIAL_TWO_BYTES_PER_TRIANGLE") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v
                                    .eq_ignore_ascii_case(
                                        "MATERIAL_FOUR_BYTES_PER_TRIANGLE",
                                    ) => _serde::__private::Ok(__Field::__field4),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MaterialType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MaterialType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum MaterialType",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MaterialType::MATERIAL_NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MaterialType::MATERIAL_SINGLE_VALUE_PER_CHUNK,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MaterialType::MATERIAL_ONE_BYTE_PER_TRIANGLE,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MaterialType::MATERIAL_TWO_BYTES_PER_TRIANGLE,
                            )
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MaterialType::MATERIAL_FOUR_BYTES_PER_TRIANGLE,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "MATERIAL_NONE",
                "MATERIAL_SINGLE_VALUE_PER_CHUNK",
                "MATERIAL_ONE_BYTE_PER_TRIANGLE",
                "MATERIAL_TWO_BYTES_PER_TRIANGLE",
                "MATERIAL_FOUR_BYTES_PER_TRIANGLE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MaterialType",
                VARIANTS,
                _serde::de::ReadEnumSize::Uint8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MaterialType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
