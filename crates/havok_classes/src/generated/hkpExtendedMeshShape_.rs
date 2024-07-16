use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpExtendedMeshShape`
/// - version: `3`
/// - signature: `0x177114a2`
/// - size: `240`(x86)/`336`(x86_64)
/// -  vtable: `true`
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
    /// - name: `embeddedTrianglesSubpart`(ctype: `struct hkpExtendedMeshShapeTrianglesSubpart`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: `112`(x86)/`160`(x86_64)
    pub m_embeddedTrianglesSubpart: hkpExtendedMeshShapeTrianglesSubpart,
    /// # C++ Info
    /// - name: `aabbHalfExtents`(ctype: `hkVector4`)
    /// - offset: `144`(x86)/`208`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_aabbHalfExtents: Vector4,
    /// # C++ Info
    /// - name: `aabbCenter`(ctype: `hkVector4`)
    /// - offset: `160`(x86)/`224`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_aabbCenter: Vector4,
    /// # C++ Info
    /// - name: `materialClass`(ctype: `void*`)
    /// - offset: `176`(x86)/`240`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_materialClass: Pointer,
    /// # C++ Info
    /// - name: `numBitsForSubpartIndex`(ctype: `hkInt32`)
    /// - offset: `180`(x86)/`248`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numBitsForSubpartIndex: i32,
    /// # C++ Info
    /// - name: `trianglesSubparts`(ctype: `hkArray<struct hkpExtendedMeshShapeTrianglesSubpart>`)
    /// - offset: `184`(x86)/`256`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_trianglesSubparts: Vec<hkpExtendedMeshShapeTrianglesSubpart>,
    /// # C++ Info
    /// - name: `shapesSubparts`(ctype: `hkArray<struct hkpExtendedMeshShapeShapesSubpart>`)
    /// - offset: `196`(x86)/`272`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_shapesSubparts: Vec<hkpExtendedMeshShapeShapesSubpart>,
    /// # C++ Info
    /// - name: `weldingInfo`(ctype: `hkArray<hkUint16>`)
    /// - offset: `208`(x86)/`288`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_weldingInfo: Vec<u16>,
    /// # C++ Info
    /// - name: `weldingType`(ctype: `enum WeldingType`)
    /// - offset: `220`(x86)/`304`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_weldingType: WeldingType,
    /// # C++ Info
    /// - name: `defaultCollisionFilterInfo`(ctype: `hkUint32`)
    /// - offset: `224`(x86)/`308`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_defaultCollisionFilterInfo: u32,
    /// # C++ Info
    /// - name: `cachedNumChildShapes`(ctype: `hkInt32`)
    /// - offset: `228`(x86)/`312`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_cachedNumChildShapes: i32,
    /// # C++ Info
    /// - name: `triangleRadius`(ctype: `hkReal`)
    /// - offset: `232`(x86)/`316`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_triangleRadius: f32,
    /// # C++ Info
    /// - name: `padding`(ctype: `hkInt32`)
    /// - offset: `236`(x86)/`320`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_padding: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpExtendedMeshShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpExtendedMeshShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x177114a2)
        }
    }
    impl _serde::Serialize for hkpExtendedMeshShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x177114a2)));
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
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_embeddedTrianglesSubpart,
    m_aabbHalfExtents,
    m_aabbCenter,
    m_materialClass,
    m_numBitsForSubpartIndex,
    m_trianglesSubparts,
    m_shapesSubparts,
    m_weldingInfo,
    m_weldingType,
    m_defaultCollisionFilterInfo,
    m_cachedNumChildShapes,
    m_triangleRadius,
    m_padding,
    __ignore,
}
struct __FieldVisitor;
impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
    type Value = __Field;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "field identifier")
    }
    /// Intended for use in XML.
    #[allow(clippy::match_single_binding)]
    #[allow(clippy::reversed_empty_ranges)]
    #[allow(clippy::single_match)]
    fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
    where
        __E: _serde::de::Error,
    {
        match __value {
            "embeddedTrianglesSubpart" => Ok(__Field::m_embeddedTrianglesSubpart),
            "aabbHalfExtents" => Ok(__Field::m_aabbHalfExtents),
            "aabbCenter" => Ok(__Field::m_aabbCenter),
            "numBitsForSubpartIndex" => Ok(__Field::m_numBitsForSubpartIndex),
            "trianglesSubparts" => Ok(__Field::m_trianglesSubparts),
            "shapesSubparts" => Ok(__Field::m_shapesSubparts),
            "weldingInfo" => Ok(__Field::m_weldingInfo),
            "weldingType" => Ok(__Field::m_weldingType),
            "defaultCollisionFilterInfo" => Ok(__Field::m_defaultCollisionFilterInfo),
            "cachedNumChildShapes" => Ok(__Field::m_cachedNumChildShapes),
            "triangleRadius" => Ok(__Field::m_triangleRadius),
            _ => Ok(__Field::__ignore),
        }
    }
}
impl<'de> _serde::Deserialize<'de> for __Field {
    #[inline]
    fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
    }
}
pub(super) struct __hkpExtendedMeshShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpExtendedMeshShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpExtendedMeshShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpExtendedMeshShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpExtendedMeshShape>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpExtendedMeshShapeVisitor<'de> {
    type Value = hkpExtendedMeshShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpExtendedMeshShape")
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
        let mut m_embeddedTrianglesSubpart: _serde::__private::Option<
            hkpExtendedMeshShapeTrianglesSubpart,
        > = _serde::__private::None;
        let mut m_aabbHalfExtents: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_aabbCenter: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_materialClass: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_numBitsForSubpartIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_trianglesSubparts: _serde::__private::Option<
            Vec<hkpExtendedMeshShapeTrianglesSubpart>,
        > = _serde::__private::None;
        let mut m_shapesSubparts: _serde::__private::Option<
            Vec<hkpExtendedMeshShapeShapesSubpart>,
        > = _serde::__private::None;
        let mut m_weldingInfo: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_weldingType: _serde::__private::Option<WeldingType> = _serde::__private::None;
        let mut m_defaultCollisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_cachedNumChildShapes: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_triangleRadius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_padding: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..13usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_embeddedTrianglesSubpart) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "embeddedTrianglesSubpart",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 8usize, 0usize)?;
                    m_embeddedTrianglesSubpart = _serde::__private::Some(
                        match __A::next_value::<
                            hkpExtendedMeshShapeTrianglesSubpart,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_aabbHalfExtents) {
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
                2usize => {
                    if _serde::__private::Option::is_some(&m_aabbCenter) {
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
                3usize => {
                    if _serde::__private::Option::is_some(&m_materialClass) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialClass",
                            ),
                        );
                    }
                    m_materialClass = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_numBitsForSubpartIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numBitsForSubpartIndex",
                            ),
                        );
                    }
                    m_numBitsForSubpartIndex = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_trianglesSubparts) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "trianglesSubparts",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_trianglesSubparts = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpExtendedMeshShapeTrianglesSubpart>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_shapesSubparts) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "shapesSubparts",
                            ),
                        );
                    }
                    m_shapesSubparts = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpExtendedMeshShapeShapesSubpart>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_weldingInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "weldingInfo",
                            ),
                        );
                    }
                    m_weldingInfo = _serde::__private::Some(
                        match __A::next_value::<Vec<u16>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
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
                9usize => {
                    if _serde::__private::Option::is_some(
                        &m_defaultCollisionFilterInfo,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "defaultCollisionFilterInfo",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_defaultCollisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_cachedNumChildShapes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cachedNumChildShapes",
                            ),
                        );
                    }
                    m_cachedNumChildShapes = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_triangleRadius) {
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
                12usize => {
                    if _serde::__private::Option::is_some(&m_padding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("padding"),
                        );
                    }
                    m_padding = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        __A::pad(&mut __map, 0usize, 12usize)?;
        let m_embeddedTrianglesSubpart = match m_embeddedTrianglesSubpart {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "embeddedTrianglesSubpart",
                    ),
                );
            }
        };
        let m_aabbHalfExtents = match m_aabbHalfExtents {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("aabbHalfExtents"),
                );
            }
        };
        let m_aabbCenter = match m_aabbCenter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("aabbCenter"),
                );
            }
        };
        let m_materialClass = match m_materialClass {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialClass"),
                );
            }
        };
        let m_numBitsForSubpartIndex = match m_numBitsForSubpartIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numBitsForSubpartIndex",
                    ),
                );
            }
        };
        let m_trianglesSubparts = match m_trianglesSubparts {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("trianglesSubparts"),
                );
            }
        };
        let m_shapesSubparts = match m_shapesSubparts {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shapesSubparts"),
                );
            }
        };
        let m_weldingInfo = match m_weldingInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weldingInfo"),
                );
            }
        };
        let m_weldingType = match m_weldingType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weldingType"),
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
        let m_cachedNumChildShapes = match m_cachedNumChildShapes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "cachedNumChildShapes",
                    ),
                );
            }
        };
        let m_triangleRadius = match m_triangleRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangleRadius"),
                );
            }
        };
        let m_padding = match m_padding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("padding"),
                );
            }
        };
        _serde::__private::Ok(hkpExtendedMeshShape {
            __ptr,
            parent,
            m_embeddedTrianglesSubpart,
            m_aabbHalfExtents,
            m_aabbCenter,
            m_materialClass,
            m_numBitsForSubpartIndex,
            m_trianglesSubparts,
            m_shapesSubparts,
            m_weldingInfo,
            m_weldingType,
            m_defaultCollisionFilterInfo,
            m_cachedNumChildShapes,
            m_triangleRadius,
            m_padding,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkpShapeCollectionVisitor::visit_as_parent(&mut __map)?;
        let mut m_embeddedTrianglesSubpart: _serde::__private::Option<
            hkpExtendedMeshShapeTrianglesSubpart,
        > = _serde::__private::None;
        let mut m_aabbHalfExtents: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_aabbCenter: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_numBitsForSubpartIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_trianglesSubparts: _serde::__private::Option<
            Vec<hkpExtendedMeshShapeTrianglesSubpart>,
        > = _serde::__private::None;
        let mut m_shapesSubparts: _serde::__private::Option<
            Vec<hkpExtendedMeshShapeShapesSubpart>,
        > = _serde::__private::None;
        let mut m_weldingInfo: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_weldingType: _serde::__private::Option<WeldingType> = _serde::__private::None;
        let mut m_defaultCollisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_cachedNumChildShapes: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_triangleRadius: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..11usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_embeddedTrianglesSubpart => {
                        if _serde::__private::Option::is_some(
                            &m_embeddedTrianglesSubpart,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "embeddedTrianglesSubpart",
                                ),
                            );
                        }
                        m_embeddedTrianglesSubpart = _serde::__private::Some(
                            match __A::next_value::<
                                hkpExtendedMeshShapeTrianglesSubpart,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_aabbHalfExtents => {
                        if _serde::__private::Option::is_some(&m_aabbHalfExtents) {
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
                        if _serde::__private::Option::is_some(&m_aabbCenter) {
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
                        if _serde::__private::Option::is_some(
                            &m_numBitsForSubpartIndex,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numBitsForSubpartIndex",
                                ),
                            );
                        }
                        m_numBitsForSubpartIndex = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_trianglesSubparts => {
                        if _serde::__private::Option::is_some(&m_trianglesSubparts) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "trianglesSubparts",
                                ),
                            );
                        }
                        m_trianglesSubparts = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkpExtendedMeshShapeTrianglesSubpart>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_shapesSubparts => {
                        if _serde::__private::Option::is_some(&m_shapesSubparts) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "shapesSubparts",
                                ),
                            );
                        }
                        m_shapesSubparts = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkpExtendedMeshShapeShapesSubpart>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_weldingInfo => {
                        if _serde::__private::Option::is_some(&m_weldingInfo) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "weldingInfo",
                                ),
                            );
                        }
                        m_weldingInfo = _serde::__private::Some(
                            match __A::next_value::<Vec<u16>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_weldingType => {
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
                    __Field::m_defaultCollisionFilterInfo => {
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
                    __Field::m_cachedNumChildShapes => {
                        if _serde::__private::Option::is_some(&m_cachedNumChildShapes) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "cachedNumChildShapes",
                                ),
                            );
                        }
                        m_cachedNumChildShapes = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_triangleRadius => {
                        if _serde::__private::Option::is_some(&m_triangleRadius) {
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
                    _ => {}
                }
            }
        }
        let m_embeddedTrianglesSubpart = match m_embeddedTrianglesSubpart {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "embeddedTrianglesSubpart",
                    ),
                );
            }
        };
        let m_aabbHalfExtents = match m_aabbHalfExtents {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("aabbHalfExtents"),
                );
            }
        };
        let m_aabbCenter = match m_aabbCenter {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("aabbCenter"),
                );
            }
        };
        let m_numBitsForSubpartIndex = match m_numBitsForSubpartIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numBitsForSubpartIndex",
                    ),
                );
            }
        };
        let m_trianglesSubparts = match m_trianglesSubparts {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("trianglesSubparts"),
                );
            }
        };
        let m_shapesSubparts = match m_shapesSubparts {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shapesSubparts"),
                );
            }
        };
        let m_weldingInfo = match m_weldingInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weldingInfo"),
                );
            }
        };
        let m_weldingType = match m_weldingType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weldingType"),
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
        let m_cachedNumChildShapes = match m_cachedNumChildShapes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "cachedNumChildShapes",
                    ),
                );
            }
        };
        let m_triangleRadius = match m_triangleRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangleRadius"),
                );
            }
        };
        _serde::__private::Ok(hkpExtendedMeshShape {
            __ptr,
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
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpExtendedMeshShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "embeddedTrianglesSubpart",
                "aabbHalfExtents",
                "aabbCenter",
                "materialClass",
                "numBitsForSubpartIndex",
                "trianglesSubparts",
                "shapesSubparts",
                "weldingInfo",
                "weldingType",
                "defaultCollisionFilterInfo",
                "cachedNumChildShapes",
                "triangleRadius",
                "padding",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpExtendedMeshShape",
                FIELDS,
                __hkpExtendedMeshShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpExtendedMeshShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for IndexStridingType {
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
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        4i8 => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3, 4",
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
                            v if v == "0"
                                || v.eq_ignore_ascii_case("INDICES_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("INDICES_INT8") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("INDICES_INT16") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("INDICES_INT32") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("INDICES_MAX_ID") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<IndexStridingType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = IndexStridingType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum IndexStridingType",
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
                            _serde::__private::Ok(IndexStridingType::INDICES_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(IndexStridingType::INDICES_INT8)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(IndexStridingType::INDICES_INT16)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(IndexStridingType::INDICES_INT32)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(IndexStridingType::INDICES_MAX_ID)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "INDICES_INVALID",
                "INDICES_INT8",
                "INDICES_INT16",
                "INDICES_INT32",
                "INDICES_MAX_ID",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "IndexStridingType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<IndexStridingType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for MaterialIndexStridingType {
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
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3",
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
                            v if v == "0"
                                || v.eq_ignore_ascii_case("MATERIAL_INDICES_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("MATERIAL_INDICES_INT8") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("MATERIAL_INDICES_INT16") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("MATERIAL_INDICES_MAX_ID") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MaterialIndexStridingType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MaterialIndexStridingType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum MaterialIndexStridingType",
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
                            _serde::__private::Ok(
                                MaterialIndexStridingType::MATERIAL_INDICES_INVALID,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MaterialIndexStridingType::MATERIAL_INDICES_INT8,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MaterialIndexStridingType::MATERIAL_INDICES_INT16,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MaterialIndexStridingType::MATERIAL_INDICES_MAX_ID,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "MATERIAL_INDICES_INVALID",
                "MATERIAL_INDICES_INT8",
                "MATERIAL_INDICES_INT16",
                "MATERIAL_INDICES_MAX_ID",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MaterialIndexStridingType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MaterialIndexStridingType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SubpartType {
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
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
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
                            v if v == "0"
                                || v.eq_ignore_ascii_case("SUBPART_TRIANGLES") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("SUBPART_SHAPE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SubpartType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SubpartType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum SubpartType",
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
                            _serde::__private::Ok(SubpartType::SUBPART_TRIANGLES)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(SubpartType::SUBPART_SHAPE)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "SUBPART_TRIANGLES",
                "SUBPART_SHAPE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "SubpartType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SubpartType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
