use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMeshShapeSubpart`
/// -         version: `0`
/// -       signature: `0x27336e5d`
/// -          size:  56(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMeshShapeSubpart {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `vertexBase`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_vertexBase: Pointer,
    /// # C++ Info
    /// -          name: `vertexStriding`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vertexStriding: i32,
    /// # C++ Info
    /// -          name: `numVertices`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numVertices: i32,
    /// # C++ Info
    /// -          name: `indexBase`(ctype: `void*`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_indexBase: Pointer,
    /// # C++ Info
    /// -          name: `stridingType`(ctype: `enum MeshShapeIndexStridingType`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_stridingType: MeshShapeIndexStridingType,
    /// # C++ Info
    /// -          name: `materialIndexStridingType`(ctype: `enum MeshShapeMaterialIndexStridingType`)
    /// -        offset:  17(x86)/ 25(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_materialIndexStridingType: MeshShapeMaterialIndexStridingType,
    /// # C++ Info
    /// -          name: `indexStriding`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_indexStriding: i32,
    /// # C++ Info
    /// -          name: `flipAlternateTriangles`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_flipAlternateTriangles: i32,
    /// # C++ Info
    /// -          name: `numTriangles`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numTriangles: i32,
    /// # C++ Info
    /// -          name: `materialIndexBase`(ctype: `void*`)
    /// -        offset:  32(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialIndexBase: Pointer,
    /// # C++ Info
    /// -          name: `materialIndexStriding`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_materialIndexStriding: i32,
    /// # C++ Info
    /// -          name: `materialBase`(ctype: `void*`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialBase: Pointer,
    /// # C++ Info
    /// -          name: `materialStriding`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_materialStriding: i32,
    /// # C++ Info
    /// -          name: `numMaterials`(ctype: `hkInt32`)
    /// -        offset:  48(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numMaterials: i32,
    /// # C++ Info
    /// -          name: `triangleOffset`(ctype: `hkInt32`)
    /// -        offset:  52(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_triangleOffset: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMeshShapeSubpart {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMeshShapeSubpart"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x27336e5d)
        }
    }
    impl _serde::Serialize for hkpMeshShapeSubpart {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x27336e5d)));
            let mut serializer = __serializer
                .serialize_struct("hkpMeshShapeSubpart", class_meta)?;
            serializer.skip_field("vertexBase", &self.m_vertexBase)?;
            serializer.serialize_field("vertexStriding", &self.m_vertexStriding)?;
            serializer.serialize_field("numVertices", &self.m_numVertices)?;
            serializer.skip_field("indexBase", &self.m_indexBase)?;
            serializer.serialize_field("stridingType", &self.m_stridingType)?;
            serializer
                .serialize_field(
                    "materialIndexStridingType",
                    &self.m_materialIndexStridingType,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("indexStriding", &self.m_indexStriding)?;
            serializer
                .serialize_field(
                    "flipAlternateTriangles",
                    &self.m_flipAlternateTriangles,
                )?;
            serializer.serialize_field("numTriangles", &self.m_numTriangles)?;
            serializer.skip_field("materialIndexBase", &self.m_materialIndexBase)?;
            serializer
                .serialize_field(
                    "materialIndexStriding",
                    &self.m_materialIndexStriding,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialBase", &self.m_materialBase)?;
            serializer.serialize_field("materialStriding", &self.m_materialStriding)?;
            serializer.serialize_field("numMaterials", &self.m_numMaterials)?;
            serializer.serialize_field("triangleOffset", &self.m_triangleOffset)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_vertexBase,
    m_vertexStriding,
    m_numVertices,
    m_indexBase,
    m_stridingType,
    m_materialIndexStridingType,
    m_indexStriding,
    m_flipAlternateTriangles,
    m_numTriangles,
    m_materialIndexBase,
    m_materialIndexStriding,
    m_materialBase,
    m_materialStriding,
    m_numMaterials,
    m_triangleOffset,
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
            "vertexStriding" => Ok(__Field::m_vertexStriding),
            "numVertices" => Ok(__Field::m_numVertices),
            "stridingType" => Ok(__Field::m_stridingType),
            "materialIndexStridingType" => Ok(__Field::m_materialIndexStridingType),
            "indexStriding" => Ok(__Field::m_indexStriding),
            "flipAlternateTriangles" => Ok(__Field::m_flipAlternateTriangles),
            "numTriangles" => Ok(__Field::m_numTriangles),
            "materialIndexStriding" => Ok(__Field::m_materialIndexStriding),
            "materialStriding" => Ok(__Field::m_materialStriding),
            "numMaterials" => Ok(__Field::m_numMaterials),
            "triangleOffset" => Ok(__Field::m_triangleOffset),
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
pub(super) struct __hkpMeshShapeSubpartVisitor<'de> {
    marker: core::marker::PhantomData<hkpMeshShapeSubpart>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpMeshShapeSubpartVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpMeshShapeSubpart, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpMeshShapeSubpart>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpMeshShapeSubpartVisitor<'de> {
    type Value = hkpMeshShapeSubpart;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpMeshShapeSubpart")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_vertexBase: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_vertexStriding: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numVertices: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_indexBase: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_stridingType: _serde::__private::Option<MeshShapeIndexStridingType> = _serde::__private::None;
        let mut m_materialIndexStridingType: _serde::__private::Option<
            MeshShapeMaterialIndexStridingType,
        > = _serde::__private::None;
        let mut m_indexStriding: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_flipAlternateTriangles: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numTriangles: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_materialIndexBase: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_materialIndexStriding: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_materialBase: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_materialStriding: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numMaterials: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_triangleOffset: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..15usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_vertexBase) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vertexBase",
                            ),
                        );
                    }
                    m_vertexBase = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_vertexStriding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vertexStriding",
                            ),
                        );
                    }
                    m_vertexStriding = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_numVertices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numVertices",
                            ),
                        );
                    }
                    m_numVertices = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_indexBase) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "indexBase",
                            ),
                        );
                    }
                    m_indexBase = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_stridingType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "stridingType",
                            ),
                        );
                    }
                    m_stridingType = _serde::__private::Some(
                        match __A::next_value::<MeshShapeIndexStridingType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_materialIndexStridingType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialIndexStridingType",
                            ),
                        );
                    }
                    m_materialIndexStridingType = _serde::__private::Some(
                        match __A::next_value::<
                            MeshShapeMaterialIndexStridingType,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_indexStriding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "indexStriding",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 2usize)?;
                    m_indexStriding = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_flipAlternateTriangles) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "flipAlternateTriangles",
                            ),
                        );
                    }
                    m_flipAlternateTriangles = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_numTriangles) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numTriangles",
                            ),
                        );
                    }
                    m_numTriangles = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_materialIndexBase) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialIndexBase",
                            ),
                        );
                    }
                    m_materialIndexBase = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_materialIndexStriding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialIndexStriding",
                            ),
                        );
                    }
                    m_materialIndexStriding = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_materialBase) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialBase",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_materialBase = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_materialStriding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialStriding",
                            ),
                        );
                    }
                    m_materialStriding = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_numMaterials) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numMaterials",
                            ),
                        );
                    }
                    m_numMaterials = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                14usize => {
                    if _serde::__private::Option::is_some(&m_triangleOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "triangleOffset",
                            ),
                        );
                    }
                    m_triangleOffset = _serde::__private::Some(
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
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_vertexBase = match m_vertexBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexBase"),
                );
            }
        };
        let m_vertexStriding = match m_vertexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexStriding"),
                );
            }
        };
        let m_numVertices = match m_numVertices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numVertices"),
                );
            }
        };
        let m_indexBase = match m_indexBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("indexBase"),
                );
            }
        };
        let m_stridingType = match m_stridingType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("stridingType"),
                );
            }
        };
        let m_materialIndexStridingType = match m_materialIndexStridingType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "materialIndexStridingType",
                    ),
                );
            }
        };
        let m_indexStriding = match m_indexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("indexStriding"),
                );
            }
        };
        let m_flipAlternateTriangles = match m_flipAlternateTriangles {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "flipAlternateTriangles",
                    ),
                );
            }
        };
        let m_numTriangles = match m_numTriangles {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numTriangles"),
                );
            }
        };
        let m_materialIndexBase = match m_materialIndexBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialIndexBase"),
                );
            }
        };
        let m_materialIndexStriding = match m_materialIndexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "materialIndexStriding",
                    ),
                );
            }
        };
        let m_materialBase = match m_materialBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialBase"),
                );
            }
        };
        let m_materialStriding = match m_materialStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialStriding"),
                );
            }
        };
        let m_numMaterials = match m_numMaterials {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numMaterials"),
                );
            }
        };
        let m_triangleOffset = match m_triangleOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangleOffset"),
                );
            }
        };
        _serde::__private::Ok(hkpMeshShapeSubpart {
            __ptr: __A::class_ptr(&mut __map),
            m_vertexBase,
            m_vertexStriding,
            m_numVertices,
            m_indexBase,
            m_stridingType,
            m_materialIndexStridingType,
            m_indexStriding,
            m_flipAlternateTriangles,
            m_numTriangles,
            m_materialIndexBase,
            m_materialIndexStriding,
            m_materialBase,
            m_materialStriding,
            m_numMaterials,
            m_triangleOffset,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_vertexStriding: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numVertices: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_stridingType: _serde::__private::Option<MeshShapeIndexStridingType> = _serde::__private::None;
        let mut m_materialIndexStridingType: _serde::__private::Option<
            MeshShapeMaterialIndexStridingType,
        > = _serde::__private::None;
        let mut m_indexStriding: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_flipAlternateTriangles: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numTriangles: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_materialIndexStriding: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_materialStriding: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numMaterials: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_triangleOffset: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..11usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_vertexStriding => {
                        if _serde::__private::Option::is_some(&m_vertexStriding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vertexStriding",
                                ),
                            );
                        }
                        m_vertexStriding = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numVertices => {
                        if _serde::__private::Option::is_some(&m_numVertices) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numVertices",
                                ),
                            );
                        }
                        m_numVertices = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_stridingType => {
                        if _serde::__private::Option::is_some(&m_stridingType) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "stridingType",
                                ),
                            );
                        }
                        m_stridingType = _serde::__private::Some(
                            match __A::next_value::<
                                MeshShapeIndexStridingType,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_materialIndexStridingType => {
                        if _serde::__private::Option::is_some(
                            &m_materialIndexStridingType,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materialIndexStridingType",
                                ),
                            );
                        }
                        m_materialIndexStridingType = _serde::__private::Some(
                            match __A::next_value::<
                                MeshShapeMaterialIndexStridingType,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_indexStriding => {
                        if _serde::__private::Option::is_some(&m_indexStriding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "indexStriding",
                                ),
                            );
                        }
                        m_indexStriding = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_flipAlternateTriangles => {
                        if _serde::__private::Option::is_some(
                            &m_flipAlternateTriangles,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "flipAlternateTriangles",
                                ),
                            );
                        }
                        m_flipAlternateTriangles = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numTriangles => {
                        if _serde::__private::Option::is_some(&m_numTriangles) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numTriangles",
                                ),
                            );
                        }
                        m_numTriangles = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_materialIndexStriding => {
                        if _serde::__private::Option::is_some(&m_materialIndexStriding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materialIndexStriding",
                                ),
                            );
                        }
                        m_materialIndexStriding = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_materialStriding => {
                        if _serde::__private::Option::is_some(&m_materialStriding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materialStriding",
                                ),
                            );
                        }
                        m_materialStriding = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numMaterials => {
                        if _serde::__private::Option::is_some(&m_numMaterials) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numMaterials",
                                ),
                            );
                        }
                        m_numMaterials = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_triangleOffset => {
                        if _serde::__private::Option::is_some(&m_triangleOffset) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "triangleOffset",
                                ),
                            );
                        }
                        m_triangleOffset = _serde::__private::Some(
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
        }
        let m_vertexStriding = match m_vertexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexStriding"),
                );
            }
        };
        let m_numVertices = match m_numVertices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numVertices"),
                );
            }
        };
        let m_stridingType = match m_stridingType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("stridingType"),
                );
            }
        };
        let m_materialIndexStridingType = match m_materialIndexStridingType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "materialIndexStridingType",
                    ),
                );
            }
        };
        let m_indexStriding = match m_indexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("indexStriding"),
                );
            }
        };
        let m_flipAlternateTriangles = match m_flipAlternateTriangles {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "flipAlternateTriangles",
                    ),
                );
            }
        };
        let m_numTriangles = match m_numTriangles {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numTriangles"),
                );
            }
        };
        let m_materialIndexStriding = match m_materialIndexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "materialIndexStriding",
                    ),
                );
            }
        };
        let m_materialStriding = match m_materialStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialStriding"),
                );
            }
        };
        let m_numMaterials = match m_numMaterials {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numMaterials"),
                );
            }
        };
        let m_triangleOffset = match m_triangleOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangleOffset"),
                );
            }
        };
        _serde::__private::Ok(hkpMeshShapeSubpart {
            __ptr: __A::class_ptr(&mut __map),
            m_vertexStriding,
            m_numVertices,
            m_stridingType,
            m_materialIndexStridingType,
            m_indexStriding,
            m_flipAlternateTriangles,
            m_numTriangles,
            m_materialIndexStriding,
            m_materialStriding,
            m_numMaterials,
            m_triangleOffset,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpMeshShapeSubpart {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "vertexBase",
                "vertexStriding",
                "numVertices",
                "indexBase",
                "stridingType",
                "materialIndexStridingType",
                "indexStriding",
                "flipAlternateTriangles",
                "numTriangles",
                "materialIndexBase",
                "materialIndexStriding",
                "materialBase",
                "materialStriding",
                "numMaterials",
                "triangleOffset",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpMeshShapeSubpart",
                FIELDS,
                __hkpMeshShapeSubpartVisitor {
                    marker: _serde::__private::PhantomData::<hkpMeshShapeSubpart>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
