use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpCompressedMeshShape`
/// - version: `9`
/// - signature: `0xa62d5e6e`
/// - size: `224`(x86)/`304`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: hkpShapeCollection,
    /// # C++ Info
    /// - name: `bitsPerIndex`(ctype: `hkInt32`)
    /// - offset: ` 24`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "bitsPerIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "bitsPerIndex"))]
    pub m_bitsPerIndex: i32,
    /// # C++ Info
    /// - name: `bitsPerWIndex`(ctype: `hkInt32`)
    /// - offset: ` 28`(x86)/` 52`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "bitsPerWIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "bitsPerWIndex"))]
    pub m_bitsPerWIndex: i32,
    /// # C++ Info
    /// - name: `wIndexMask`(ctype: `hkInt32`)
    /// - offset: ` 32`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "wIndexMask"))]
    #[cfg_attr(feature = "serde", serde(rename = "wIndexMask"))]
    pub m_wIndexMask: i32,
    /// # C++ Info
    /// - name: `indexMask`(ctype: `hkInt32`)
    /// - offset: ` 36`(x86)/` 60`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "indexMask"))]
    #[cfg_attr(feature = "serde", serde(rename = "indexMask"))]
    pub m_indexMask: i32,
    /// # C++ Info
    /// - name: `radius`(ctype: `hkReal`)
    /// - offset: ` 40`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "radius"))]
    #[cfg_attr(feature = "serde", serde(rename = "radius"))]
    pub m_radius: f32,
    /// # C++ Info
    /// - name: `weldingType`(ctype: `enum WeldingType`)
    /// - offset: ` 44`(x86)/` 68`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "weldingType"))]
    #[cfg_attr(feature = "serde", serde(rename = "weldingType"))]
    pub m_weldingType: WeldingType,
    /// # C++ Info
    /// - name: `materialType`(ctype: `enum MaterialType`)
    /// - offset: ` 45`(x86)/` 69`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "materialType"))]
    #[cfg_attr(feature = "serde", serde(rename = "materialType"))]
    pub m_materialType: MaterialType,
    /// # C++ Info
    /// - name: `materials`(ctype: `hkArray<hkUint32>`)
    /// - offset: ` 48`(x86)/` 72`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "materials"))]
    #[cfg_attr(feature = "serde", serde(rename = "materials"))]
    pub m_materials: Vec<u32>,
    /// # C++ Info
    /// - name: `materials16`(ctype: `hkArray<hkUint16>`)
    /// - offset: ` 60`(x86)/` 88`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "materials16"))]
    #[cfg_attr(feature = "serde", serde(rename = "materials16"))]
    pub m_materials16: Vec<u16>,
    /// # C++ Info
    /// - name: `materials8`(ctype: `hkArray<hkUint8>`)
    /// - offset: ` 72`(x86)/`104`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "materials8"))]
    #[cfg_attr(feature = "serde", serde(rename = "materials8"))]
    pub m_materials8: Vec<u8>,
    /// # C++ Info
    /// - name: `transforms`(ctype: `hkArray<hkQsTransform>`)
    /// - offset: ` 84`(x86)/`120`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "transforms"))]
    #[cfg_attr(feature = "serde", serde(rename = "transforms"))]
    pub m_transforms: Vec<QsTransform>,
    /// # C++ Info
    /// - name: `bigVertices`(ctype: `hkArray<hkVector4>`)
    /// - offset: ` 96`(x86)/`136`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "bigVertices"))]
    #[cfg_attr(feature = "serde", serde(rename = "bigVertices"))]
    pub m_bigVertices: Vec<Vector4>,
    /// # C++ Info
    /// - name: `bigTriangles`(ctype: `hkArray<struct hkpCompressedMeshShapeBigTriangle>`)
    /// - offset: `108`(x86)/`152`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "bigTriangles"))]
    #[cfg_attr(feature = "serde", serde(rename = "bigTriangles"))]
    pub m_bigTriangles: Vec<hkpCompressedMeshShapeBigTriangle>,
    /// # C++ Info
    /// - name: `chunks`(ctype: `hkArray<struct hkpCompressedMeshShapeChunk>`)
    /// - offset: `120`(x86)/`168`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "chunks"))]
    #[cfg_attr(feature = "serde", serde(rename = "chunks"))]
    pub m_chunks: Vec<hkpCompressedMeshShapeChunk>,
    /// # C++ Info
    /// - name: `convexPieces`(ctype: `hkArray<struct hkpCompressedMeshShapeConvexPiece>`)
    /// - offset: `132`(x86)/`184`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "convexPieces"))]
    #[cfg_attr(feature = "serde", serde(rename = "convexPieces"))]
    pub m_convexPieces: Vec<hkpCompressedMeshShapeConvexPiece>,
    /// # C++ Info
    /// - name: `error`(ctype: `hkReal`)
    /// - offset: `144`(x86)/`200`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "error"))]
    #[cfg_attr(feature = "serde", serde(rename = "error"))]
    pub m_error: f32,
    /// # C++ Info
    /// - name: `bounds`(ctype: `struct hkAabb`)
    /// - offset: `160`(x86)/`208`(x86_64)
    /// - type_size: ` 32`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "bounds"))]
    #[cfg_attr(feature = "serde", serde(rename = "bounds"))]
    pub m_bounds: hkAabb,
    /// # C++ Info
    /// - name: `defaultCollisionFilterInfo`(ctype: `hkUint32`)
    /// - offset: `192`(x86)/`240`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "defaultCollisionFilterInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "defaultCollisionFilterInfo"))]
    pub m_defaultCollisionFilterInfo: u32,
    /// # C++ Info
    /// - name: `meshMaterials`(ctype: `void*`)
    /// - offset: `196`(x86)/`248`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "meshMaterials"))]
    #[cfg_attr(feature = "serde", serde(rename = "meshMaterials"))]
    pub m_meshMaterials: Pointer,
    /// # C++ Info
    /// - name: `materialStriding`(ctype: `hkUint16`)
    /// - offset: `200`(x86)/`256`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "materialStriding"))]
    #[cfg_attr(feature = "serde", serde(rename = "materialStriding"))]
    pub m_materialStriding: u16,
    /// # C++ Info
    /// - name: `numMaterials`(ctype: `hkUint16`)
    /// - offset: `202`(x86)/`258`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "numMaterials"))]
    #[cfg_attr(feature = "serde", serde(rename = "numMaterials"))]
    pub m_numMaterials: u16,
    /// # C++ Info
    /// - name: `namedMaterials`(ctype: `hkArray<struct hkpNamedMeshMaterial>`)
    /// - offset: `204`(x86)/`264`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "namedMaterials"))]
    #[cfg_attr(feature = "serde", serde(rename = "namedMaterials"))]
    pub m_namedMaterials: Vec<hkpNamedMeshMaterial<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpCompressedMeshShape<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCompressedMeshShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa62d5e6e)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_bigTriangles
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_chunks
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_convexPieces
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(self.m_bounds.deps_indexes());
            v.push(self.m_meshMaterials.get());
            v.extend(
                self
                    .m_namedMaterials
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkpCompressedMeshShape<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa62d5e6e)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpCompressedMeshShape",
                    class_meta,
                    (224u64, 304u64),
                )?;
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
            serializer
                .serialize_array_field(
                    "materials",
                    &self.m_materials,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "materials16",
                    &self.m_materials16,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "materials8",
                    &self.m_materials8,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "transforms",
                    &self.m_transforms,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "bigVertices",
                    &self.m_bigVertices,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "bigTriangles",
                    &self.m_bigTriangles,
                    TypeSize::Struct {
                        size_x86: 16u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "chunks",
                    &self.m_chunks,
                    TypeSize::Struct {
                        size_x86: 80u64,
                        size_x86_64: 96u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "convexPieces",
                    &self.m_convexPieces,
                    TypeSize::Struct {
                        size_x86: 64u64,
                        size_x86_64: 80u64,
                    },
                )?;
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
                .serialize_array_field(
                    "namedMaterials",
                    &self.m_namedMaterials,
                    TypeSize::Struct {
                        size_x86: 8u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 24usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCompressedMeshShape<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
                m_disableWelding,
                m_collectionType,
                m_bitsPerIndex,
                m_bitsPerWIndex,
                m_wIndexMask,
                m_indexMask,
                m_radius,
                m_weldingType,
                m_materialType,
                m_materials,
                m_materials16,
                m_materials8,
                m_transforms,
                m_bigVertices,
                m_bigTriangles,
                m_chunks,
                m_convexPieces,
                m_error,
                m_bounds,
                m_defaultCollisionFilterInfo,
                m_materialStriding,
                m_numMaterials,
                m_namedMaterials,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "userData" => Ok(__Field::m_userData),
                        "disableWelding" => Ok(__Field::m_disableWelding),
                        "collectionType" => Ok(__Field::m_collectionType),
                        "bitsPerIndex" => Ok(__Field::m_bitsPerIndex),
                        "bitsPerWIndex" => Ok(__Field::m_bitsPerWIndex),
                        "wIndexMask" => Ok(__Field::m_wIndexMask),
                        "indexMask" => Ok(__Field::m_indexMask),
                        "radius" => Ok(__Field::m_radius),
                        "weldingType" => Ok(__Field::m_weldingType),
                        "materialType" => Ok(__Field::m_materialType),
                        "materials" => Ok(__Field::m_materials),
                        "materials16" => Ok(__Field::m_materials16),
                        "materials8" => Ok(__Field::m_materials8),
                        "transforms" => Ok(__Field::m_transforms),
                        "bigVertices" => Ok(__Field::m_bigVertices),
                        "bigTriangles" => Ok(__Field::m_bigTriangles),
                        "chunks" => Ok(__Field::m_chunks),
                        "convexPieces" => Ok(__Field::m_convexPieces),
                        "error" => Ok(__Field::m_error),
                        "bounds" => Ok(__Field::m_bounds),
                        "defaultCollisionFilterInfo" => {
                            Ok(__Field::m_defaultCollisionFilterInfo)
                        }
                        "materialStriding" => Ok(__Field::m_materialStriding),
                        "numMaterials" => Ok(__Field::m_numMaterials),
                        "namedMaterials" => Ok(__Field::m_namedMaterials),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkpCompressedMeshShapeVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpCompressedMeshShape<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpCompressedMeshShapeVisitor<'de> {
                type Value = hkpCompressedMeshShape<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpCompressedMeshShape",
                    )
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_bitsPerIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_bitsPerWIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_wIndexMask: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_indexMask: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_weldingType: _serde::__private::Option<WeldingType> = _serde::__private::None;
                    let mut m_materialType: _serde::__private::Option<MaterialType> = _serde::__private::None;
                    let mut m_materials: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
                    let mut m_materials16: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
                    let mut m_materials8: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    let mut m_transforms: _serde::__private::Option<Vec<QsTransform>> = _serde::__private::None;
                    let mut m_bigVertices: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
                    let mut m_bigTriangles: _serde::__private::Option<
                        Vec<hkpCompressedMeshShapeBigTriangle>,
                    > = _serde::__private::None;
                    let mut m_chunks: _serde::__private::Option<
                        Vec<hkpCompressedMeshShapeChunk>,
                    > = _serde::__private::None;
                    let mut m_convexPieces: _serde::__private::Option<
                        Vec<hkpCompressedMeshShapeConvexPiece>,
                    > = _serde::__private::None;
                    let mut m_error: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bounds: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_defaultCollisionFilterInfo: _serde::__private::Option<
                        u32,
                    > = _serde::__private::None;
                    let mut m_meshMaterials: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_materialStriding: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_numMaterials: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_namedMaterials: _serde::__private::Option<
                        Vec<hkpNamedMeshMaterial<'de>>,
                    > = _serde::__private::None;
                    for i in 0..22usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_bitsPerIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bitsPerIndex",
                                        ),
                                    );
                                }
                                m_bitsPerIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_bitsPerWIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bitsPerWIndex",
                                        ),
                                    );
                                }
                                m_bitsPerWIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_wIndexMask) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wIndexMask",
                                        ),
                                    );
                                }
                                m_wIndexMask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_indexMask) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexMask",
                                        ),
                                    );
                                }
                                m_indexMask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_radius) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("radius"),
                                    );
                                }
                                m_radius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_weldingType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weldingType",
                                        ),
                                    );
                                }
                                m_weldingType = _serde::__private::Some(
                                    match __A::next_value::<WeldingType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_materialType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialType",
                                        ),
                                    );
                                }
                                m_materialType = _serde::__private::Some(
                                    match __A::next_value::<MaterialType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_materials) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materials",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_materials = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_materials16) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materials16",
                                        ),
                                    );
                                }
                                m_materials16 = _serde::__private::Some(
                                    match __A::next_value::<Vec<u16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_materials8) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materials8",
                                        ),
                                    );
                                }
                                m_materials8 = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_transforms) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transforms",
                                        ),
                                    );
                                }
                                m_transforms = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_bigVertices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bigVertices",
                                        ),
                                    );
                                }
                                m_bigVertices = _serde::__private::Some(
                                    match __A::next_value::<Vec<Vector4>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_bigTriangles) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bigTriangles",
                                        ),
                                    );
                                }
                                m_bigTriangles = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpCompressedMeshShapeBigTriangle>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_chunks) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("chunks"),
                                    );
                                }
                                m_chunks = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpCompressedMeshShapeChunk>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_convexPieces) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexPieces",
                                        ),
                                    );
                                }
                                m_convexPieces = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpCompressedMeshShapeConvexPiece>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_error) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("error"),
                                    );
                                }
                                m_error = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_bounds) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("bounds"),
                                    );
                                }
                                __A::pad(&mut __map, 12usize, 4usize)?;
                                m_bounds = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_defaultCollisionFilterInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaultCollisionFilterInfo",
                                        ),
                                    );
                                }
                                m_defaultCollisionFilterInfo = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_meshMaterials) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "meshMaterials",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_meshMaterials = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(&m_materialStriding) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialStriding",
                                        ),
                                    );
                                }
                                m_materialStriding = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(&m_numMaterials) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numMaterials",
                                        ),
                                    );
                                }
                                m_numMaterials = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(&m_namedMaterials) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "namedMaterials",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_namedMaterials = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpNamedMeshMaterial<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 8usize, 24usize)?;
                    let m_bitsPerIndex = match m_bitsPerIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bitsPerIndex",
                                ),
                            );
                        }
                    };
                    let m_bitsPerWIndex = match m_bitsPerWIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bitsPerWIndex",
                                ),
                            );
                        }
                    };
                    let m_wIndexMask = match m_wIndexMask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wIndexMask",
                                ),
                            );
                        }
                    };
                    let m_indexMask = match m_indexMask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexMask",
                                ),
                            );
                        }
                    };
                    let m_radius = match m_radius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("radius"),
                            );
                        }
                    };
                    let m_weldingType = match m_weldingType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "weldingType",
                                ),
                            );
                        }
                    };
                    let m_materialType = match m_materialType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialType",
                                ),
                            );
                        }
                    };
                    let m_materials = match m_materials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materials",
                                ),
                            );
                        }
                    };
                    let m_materials16 = match m_materials16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materials16",
                                ),
                            );
                        }
                    };
                    let m_materials8 = match m_materials8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materials8",
                                ),
                            );
                        }
                    };
                    let m_transforms = match m_transforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transforms",
                                ),
                            );
                        }
                    };
                    let m_bigVertices = match m_bigVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bigVertices",
                                ),
                            );
                        }
                    };
                    let m_bigTriangles = match m_bigTriangles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bigTriangles",
                                ),
                            );
                        }
                    };
                    let m_chunks = match m_chunks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("chunks"),
                            );
                        }
                    };
                    let m_convexPieces = match m_convexPieces {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexPieces",
                                ),
                            );
                        }
                    };
                    let m_error = match m_error {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("error"),
                            );
                        }
                    };
                    let m_bounds = match m_bounds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bounds"),
                            );
                        }
                    };
                    let m_defaultCollisionFilterInfo = match m_defaultCollisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "defaultCollisionFilterInfo",
                                ),
                            );
                        }
                    };
                    let m_meshMaterials = match m_meshMaterials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "meshMaterials",
                                ),
                            );
                        }
                    };
                    let m_materialStriding = match m_materialStriding {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialStriding",
                                ),
                            );
                        }
                    };
                    let m_numMaterials = match m_numMaterials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numMaterials",
                                ),
                            );
                        }
                    };
                    let m_namedMaterials = match m_namedMaterials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "namedMaterials",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpCompressedMeshShape {
                        __ptr,
                        parent,
                        m_bitsPerIndex,
                        m_bitsPerWIndex,
                        m_wIndexMask,
                        m_indexMask,
                        m_radius,
                        m_weldingType,
                        m_materialType,
                        m_materials,
                        m_materials16,
                        m_materials8,
                        m_transforms,
                        m_bigVertices,
                        m_bigTriangles,
                        m_chunks,
                        m_convexPieces,
                        m_error,
                        m_bounds,
                        m_defaultCollisionFilterInfo,
                        m_meshMaterials,
                        m_materialStriding,
                        m_numMaterials,
                        m_namedMaterials,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_disableWelding: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_collectionType: _serde::__private::Option<
                        CollectionType,
                    > = _serde::__private::None;
                    let mut m_bitsPerIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_bitsPerWIndex: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_wIndexMask: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_indexMask: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_weldingType: _serde::__private::Option<WeldingType> = _serde::__private::None;
                    let mut m_materialType: _serde::__private::Option<MaterialType> = _serde::__private::None;
                    let mut m_materials: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
                    let mut m_materials16: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
                    let mut m_materials8: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    let mut m_transforms: _serde::__private::Option<Vec<QsTransform>> = _serde::__private::None;
                    let mut m_bigVertices: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
                    let mut m_bigTriangles: _serde::__private::Option<
                        Vec<hkpCompressedMeshShapeBigTriangle>,
                    > = _serde::__private::None;
                    let mut m_chunks: _serde::__private::Option<
                        Vec<hkpCompressedMeshShapeChunk>,
                    > = _serde::__private::None;
                    let mut m_convexPieces: _serde::__private::Option<
                        Vec<hkpCompressedMeshShapeConvexPiece>,
                    > = _serde::__private::None;
                    let mut m_error: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bounds: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_defaultCollisionFilterInfo: _serde::__private::Option<
                        u32,
                    > = _serde::__private::None;
                    let mut m_materialStriding: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_numMaterials: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_namedMaterials: _serde::__private::Option<
                        Vec<hkpNamedMeshMaterial<'de>>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_disableWelding => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_disableWelding) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "disableWelding",
                                        ),
                                    );
                                }
                                m_disableWelding = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collectionType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_collectionType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collectionType",
                                        ),
                                    );
                                }
                                m_collectionType = _serde::__private::Some(
                                    match __A::next_value::<CollectionType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bitsPerIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bitsPerIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bitsPerIndex",
                                        ),
                                    );
                                }
                                m_bitsPerIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bitsPerWIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bitsPerWIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bitsPerWIndex",
                                        ),
                                    );
                                }
                                m_bitsPerWIndex = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wIndexMask => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_wIndexMask) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wIndexMask",
                                        ),
                                    );
                                }
                                m_wIndexMask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_indexMask => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_indexMask) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexMask",
                                        ),
                                    );
                                }
                                m_indexMask = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_radius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_radius) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("radius"),
                                    );
                                }
                                m_radius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weldingType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weldingType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weldingType",
                                        ),
                                    );
                                }
                                m_weldingType = _serde::__private::Some(
                                    match __A::next_value::<WeldingType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_materialType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_materialType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialType",
                                        ),
                                    );
                                }
                                m_materialType = _serde::__private::Some(
                                    match __A::next_value::<MaterialType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_materials => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_materials) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materials",
                                        ),
                                    );
                                }
                                m_materials = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_materials16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_materials16) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materials16",
                                        ),
                                    );
                                }
                                m_materials16 = _serde::__private::Some(
                                    match __A::next_value::<Vec<u16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_materials8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_materials8) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materials8",
                                        ),
                                    );
                                }
                                m_materials8 = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transforms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transforms) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transforms",
                                        ),
                                    );
                                }
                                m_transforms = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bigVertices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bigVertices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bigVertices",
                                        ),
                                    );
                                }
                                m_bigVertices = _serde::__private::Some(
                                    match __A::next_value::<Vec<Vector4>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bigTriangles => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bigTriangles) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bigTriangles",
                                        ),
                                    );
                                }
                                m_bigTriangles = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpCompressedMeshShapeBigTriangle>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_chunks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_chunks) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("chunks"),
                                    );
                                }
                                m_chunks = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpCompressedMeshShapeChunk>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_convexPieces => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_convexPieces) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexPieces",
                                        ),
                                    );
                                }
                                m_convexPieces = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpCompressedMeshShapeConvexPiece>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_error => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_error) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("error"),
                                    );
                                }
                                m_error = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bounds => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bounds) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("bounds"),
                                    );
                                }
                                m_bounds = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_defaultCollisionFilterInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_defaultCollisionFilterInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaultCollisionFilterInfo",
                                        ),
                                    );
                                }
                                m_defaultCollisionFilterInfo = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_materialStriding => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_materialStriding) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materialStriding",
                                        ),
                                    );
                                }
                                m_materialStriding = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numMaterials => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numMaterials) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numMaterials",
                                        ),
                                    );
                                }
                                m_numMaterials = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_namedMaterials => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_namedMaterials) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "namedMaterials",
                                        ),
                                    );
                                }
                                m_namedMaterials = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpNamedMeshMaterial<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_disableWelding = match m_disableWelding {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "disableWelding",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collectionType = match m_collectionType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collectionType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bitsPerIndex = match m_bitsPerIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bitsPerIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bitsPerWIndex = match m_bitsPerWIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bitsPerWIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wIndexMask = match m_wIndexMask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wIndexMask",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indexMask = match m_indexMask {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexMask",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_radius = match m_radius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("radius"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weldingType = match m_weldingType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "weldingType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_materialType = match m_materialType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_materials = match m_materials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materials",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_materials16 = match m_materials16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materials16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_materials8 = match m_materials8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materials8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transforms = match m_transforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transforms",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bigVertices = match m_bigVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bigVertices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bigTriangles = match m_bigTriangles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bigTriangles",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_chunks = match m_chunks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("chunks"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_convexPieces = match m_convexPieces {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexPieces",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_error = match m_error {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("error"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bounds = match m_bounds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bounds"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_defaultCollisionFilterInfo = match m_defaultCollisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "defaultCollisionFilterInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_materialStriding = match m_materialStriding {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materialStriding",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numMaterials = match m_numMaterials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numMaterials",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_namedMaterials = match m_namedMaterials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "namedMaterials",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkpShape {
                        __ptr,
                        parent,
                        m_userData,
                        ..Default::default()
                    };
                    let parent = hkpShapeCollection {
                        __ptr,
                        parent,
                        m_disableWelding,
                        m_collectionType,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpCompressedMeshShape {
                        __ptr,
                        parent,
                        m_bitsPerIndex,
                        m_bitsPerWIndex,
                        m_wIndexMask,
                        m_indexMask,
                        m_radius,
                        m_weldingType,
                        m_materialType,
                        m_materials,
                        m_materials16,
                        m_materials8,
                        m_transforms,
                        m_bigVertices,
                        m_bigTriangles,
                        m_chunks,
                        m_convexPieces,
                        m_error,
                        m_bounds,
                        m_defaultCollisionFilterInfo,
                        m_materialStriding,
                        m_numMaterials,
                        m_namedMaterials,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "bitsPerIndex",
                "bitsPerWIndex",
                "wIndexMask",
                "indexMask",
                "radius",
                "weldingType",
                "materialType",
                "materials",
                "materials16",
                "materials8",
                "transforms",
                "bigVertices",
                "bigTriangles",
                "chunks",
                "convexPieces",
                "error",
                "bounds",
                "defaultCollisionFilterInfo",
                "meshMaterials",
                "materialStriding",
                "numMaterials",
                "namedMaterials",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCompressedMeshShape",
                FIELDS,
                __hkpCompressedMeshShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpCompressedMeshShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_UINT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
                        _serde::de::ReadEnumSize::Uint8,
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
                __Visitor {
                    marker: _serde::__private::PhantomData::<MaterialType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
