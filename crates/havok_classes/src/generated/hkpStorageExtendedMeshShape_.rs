use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpStorageExtendedMeshShape`
/// - version: `0`
/// - signature: `0xb469efbc`
/// - size: `272`(x86)/`368`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStorageExtendedMeshShape<'a> {
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
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hkpExtendedMeshShape<'a>,
    /// # C++ Info
    /// - name: `meshstorage`(ctype: `hkArray<hkpStorageExtendedMeshShapeMeshSubpartStorage*>`)
    /// - offset: `240`(x86)/`336`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "meshstorage"))]
    #[cfg_attr(feature = "serde", serde(rename = "meshstorage"))]
    pub m_meshstorage: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `shapestorage`(ctype: `hkArray<hkpStorageExtendedMeshShapeShapeSubpartStorage*>`)
    /// - offset: `252`(x86)/`352`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "shapestorage"))]
    #[cfg_attr(feature = "serde", serde(rename = "shapestorage"))]
    pub m_shapestorage: Vec<Pointer<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpStorageExtendedMeshShape<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStorageExtendedMeshShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb469efbc)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.parent.m_embeddedTrianglesSubpart.deps_indexes());
            v.push(&self.parent.m_materialClass);
            v.extend(
                self
                    .parent
                    .m_trianglesSubparts
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .parent
                    .m_shapesSubparts
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(self.m_meshstorage.iter());
            v.extend(self.m_shapestorage.iter());
            v
        }
    }
    impl<'a> _serde::Serialize for hkpStorageExtendedMeshShape<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xb469efbc)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpStorageExtendedMeshShape",
                    class_meta,
                    (272u64, 368u64),
                )?;
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
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field(
                    "disableWelding",
                    &self.parent.parent.m_disableWelding,
                )?;
            serializer
                .serialize_field(
                    "collectionType",
                    &self.parent.parent.m_collectionType,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field(
                    "embeddedTrianglesSubpart",
                    &self.parent.m_embeddedTrianglesSubpart,
                )?;
            serializer
                .serialize_field("aabbHalfExtents", &self.parent.m_aabbHalfExtents)?;
            serializer.serialize_field("aabbCenter", &self.parent.m_aabbCenter)?;
            serializer.skip_field("materialClass", &self.parent.m_materialClass)?;
            serializer
                .serialize_field(
                    "numBitsForSubpartIndex",
                    &self.parent.m_numBitsForSubpartIndex,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "trianglesSubparts",
                    &self.parent.m_trianglesSubparts,
                    TypeSize::Struct {
                        size_x86: 112u64,
                        size_x86_64: 160u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "shapesSubparts",
                    &self.parent.m_shapesSubparts,
                    TypeSize::Struct {
                        size_x86: 64u64,
                        size_x86_64: 96u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "weldingInfo",
                    &self.parent.m_weldingInfo,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("weldingType", &self.parent.m_weldingType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "defaultCollisionFilterInfo",
                    &self.parent.m_defaultCollisionFilterInfo,
                )?;
            serializer
                .serialize_field(
                    "cachedNumChildShapes",
                    &self.parent.m_cachedNumChildShapes,
                )?;
            serializer.serialize_field("triangleRadius", &self.parent.m_triangleRadius)?;
            serializer.skip_field("padding", &self.parent.m_padding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "meshstorage",
                    &self.m_meshstorage,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "shapestorage",
                    &self.m_shapestorage,
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpStorageExtendedMeshShape<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
                m_disableWelding,
                m_collectionType,
                m_embeddedTrianglesSubpart,
                m_aabbHalfExtents,
                m_aabbCenter,
                m_numBitsForSubpartIndex,
                m_trianglesSubparts,
                m_shapesSubparts,
                m_weldingInfo,
                m_weldingType,
                m_defaultCollisionFilterInfo,
                m_cachedNumChildShapes,
                m_triangleRadius,
                m_meshstorage,
                m_shapestorage,
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
                        "embeddedTrianglesSubpart" => {
                            Ok(__Field::m_embeddedTrianglesSubpart)
                        }
                        "aabbHalfExtents" => Ok(__Field::m_aabbHalfExtents),
                        "aabbCenter" => Ok(__Field::m_aabbCenter),
                        "numBitsForSubpartIndex" => Ok(__Field::m_numBitsForSubpartIndex),
                        "trianglesSubparts" => Ok(__Field::m_trianglesSubparts),
                        "shapesSubparts" => Ok(__Field::m_shapesSubparts),
                        "weldingInfo" => Ok(__Field::m_weldingInfo),
                        "weldingType" => Ok(__Field::m_weldingType),
                        "defaultCollisionFilterInfo" => {
                            Ok(__Field::m_defaultCollisionFilterInfo)
                        }
                        "cachedNumChildShapes" => Ok(__Field::m_cachedNumChildShapes),
                        "triangleRadius" => Ok(__Field::m_triangleRadius),
                        "meshstorage" => Ok(__Field::m_meshstorage),
                        "shapestorage" => Ok(__Field::m_shapestorage),
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
            struct __hkpStorageExtendedMeshShapeVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpStorageExtendedMeshShape<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpStorageExtendedMeshShapeVisitor<'de> {
                type Value = hkpStorageExtendedMeshShape<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpStorageExtendedMeshShape",
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
                    let mut m_meshstorage: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_shapestorage: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    for i in 0..2usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_meshstorage) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "meshstorage",
                                        ),
                                    );
                                }
                                m_meshstorage = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_shapestorage) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapestorage",
                                        ),
                                    );
                                }
                                m_shapestorage = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
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
                    __A::pad(&mut __map, 8usize, 0usize)?;
                    let m_meshstorage = match m_meshstorage {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "meshstorage",
                                ),
                            );
                        }
                    };
                    let m_shapestorage = match m_shapestorage {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapestorage",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpStorageExtendedMeshShape {
                        __ptr,
                        parent,
                        m_meshstorage,
                        m_shapestorage,
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
                    let mut m_embeddedTrianglesSubpart: _serde::__private::Option<
                        hkpExtendedMeshShapeTrianglesSubpart<'de>,
                    > = _serde::__private::None;
                    let mut m_aabbHalfExtents: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_aabbCenter: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_numBitsForSubpartIndex: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_trianglesSubparts: _serde::__private::Option<
                        Vec<hkpExtendedMeshShapeTrianglesSubpart<'de>>,
                    > = _serde::__private::None;
                    let mut m_shapesSubparts: _serde::__private::Option<
                        Vec<hkpExtendedMeshShapeShapesSubpart<'de>>,
                    > = _serde::__private::None;
                    let mut m_weldingInfo: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_weldingType: _serde::__private::Option<WeldingType> = _serde::__private::None;
                    let mut m_defaultCollisionFilterInfo: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_cachedNumChildShapes: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_triangleRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_meshstorage: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_shapestorage: _serde::__private::Option<
                        Vec<Pointer<'de>>,
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
                            __Field::m_embeddedTrianglesSubpart => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_embeddedTrianglesSubpart,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "embeddedTrianglesSubpart",
                                        ),
                                    );
                                }
                                m_embeddedTrianglesSubpart = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpExtendedMeshShapeTrianglesSubpart<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_aabbHalfExtents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_aabbHalfExtents) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "aabbHalfExtents",
                                        ),
                                    );
                                }
                                m_aabbHalfExtents = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_aabbCenter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_aabbCenter) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "aabbCenter",
                                        ),
                                    );
                                }
                                m_aabbCenter = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numBitsForSubpartIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numBitsForSubpartIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numBitsForSubpartIndex",
                                        ),
                                    );
                                }
                                m_numBitsForSubpartIndex = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_trianglesSubparts => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_trianglesSubparts,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "trianglesSubparts",
                                        ),
                                    );
                                }
                                m_trianglesSubparts = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpExtendedMeshShapeTrianglesSubpart<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_shapesSubparts => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_shapesSubparts) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapesSubparts",
                                        ),
                                    );
                                }
                                m_shapesSubparts = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpExtendedMeshShapeShapesSubpart<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weldingInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weldingInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weldingInfo",
                                        ),
                                    );
                                }
                                m_weldingInfo = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
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
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_cachedNumChildShapes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_cachedNumChildShapes,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cachedNumChildShapes",
                                        ),
                                    );
                                }
                                m_cachedNumChildShapes = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_triangleRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_triangleRadius) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleRadius",
                                        ),
                                    );
                                }
                                m_triangleRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_meshstorage => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_meshstorage) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "meshstorage",
                                        ),
                                    );
                                }
                                m_meshstorage = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_shapestorage => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_shapestorage) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapestorage",
                                        ),
                                    );
                                }
                                m_shapestorage = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
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
                    let m_embeddedTrianglesSubpart = match m_embeddedTrianglesSubpart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "embeddedTrianglesSubpart",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_aabbHalfExtents = match m_aabbHalfExtents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "aabbHalfExtents",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_aabbCenter = match m_aabbCenter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "aabbCenter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numBitsForSubpartIndex = match m_numBitsForSubpartIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numBitsForSubpartIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_trianglesSubparts = match m_trianglesSubparts {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "trianglesSubparts",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_shapesSubparts = match m_shapesSubparts {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapesSubparts",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weldingInfo = match m_weldingInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "weldingInfo",
                                ),
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
                    let m_cachedNumChildShapes = match m_cachedNumChildShapes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cachedNumChildShapes",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_triangleRadius = match m_triangleRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_meshstorage = match m_meshstorage {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "meshstorage",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_shapestorage = match m_shapestorage {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapestorage",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hkpShape {
                        __ptr: __ptr.clone(),
                        parent,
                        m_userData,
                        ..Default::default()
                    };
                    let parent = hkpShapeCollection {
                        __ptr: __ptr.clone(),
                        parent,
                        m_disableWelding,
                        m_collectionType,
                    };
                    let parent = hkpExtendedMeshShape {
                        __ptr: __ptr.clone(),
                        parent,
                        m_embeddedTrianglesSubpart,
                        m_aabbHalfExtents,
                        m_aabbCenter,
                        m_numBitsForSubpartIndex,
                        m_trianglesSubparts,
                        m_shapesSubparts,
                        m_weldingInfo,
                        m_weldingType,
                        m_defaultCollisionFilterInfo,
                        m_cachedNumChildShapes,
                        m_triangleRadius,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpStorageExtendedMeshShape {
                        __ptr: __ptr.clone(),
                        parent,
                        m_meshstorage,
                        m_shapestorage,
                    })
                }
            }
            const FIELDS: &[&str] = &["meshstorage", "shapestorage"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpStorageExtendedMeshShape",
                FIELDS,
                __hkpStorageExtendedMeshShapeVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpStorageExtendedMeshShape,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
