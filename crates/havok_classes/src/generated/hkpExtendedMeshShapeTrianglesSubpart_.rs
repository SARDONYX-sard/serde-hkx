use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpExtendedMeshShapeTrianglesSubpart`
/// -         version: `3`
/// -       signature: `0x44c32df6`
/// -          size: 112(x86)/160(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpExtendedMeshShapeTrianglesSubpart {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpExtendedMeshShapeSubpart,
    /// # C++ Info
    /// -          name: `numTriangleShapes`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numTriangleShapes: i32,
    /// # C++ Info
    /// -          name: `vertexBase`(ctype: `void*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_vertexBase: Pointer,
    /// # C++ Info
    /// -          name: `numVertices`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numVertices: i32,
    /// # C++ Info
    /// -          name: `indexBase`(ctype: `void*`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_indexBase: Pointer,
    /// # C++ Info
    /// -          name: `vertexStriding`(ctype: `hkUint16`)
    /// -        offset:  36(x86)/ 72(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_vertexStriding: u16,
    /// # C++ Info
    /// -          name: `triangleOffset`(ctype: `hkInt32`)
    /// -        offset:  40(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_triangleOffset: i32,
    /// # C++ Info
    /// -          name: `indexStriding`(ctype: `hkUint16`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_indexStriding: u16,
    /// # C++ Info
    /// -          name: `stridingType`(ctype: `enum IndexStridingType`)
    /// -        offset:  46(x86)/ 82(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_stridingType: IndexStridingType,
    /// # C++ Info
    /// -          name: `flipAlternateTriangles`(ctype: `hkInt8`)
    /// -        offset:  47(x86)/ 83(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_flipAlternateTriangles: i8,
    /// # C++ Info
    /// -          name: `extrusion`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_extrusion: Vector4,
    /// # C++ Info
    /// -          name: `transform`(ctype: `hkQsTransform`)
    /// -        offset:  64(x86)/112(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_transform: QsTransform,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpExtendedMeshShapeTrianglesSubpart {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpExtendedMeshShapeTrianglesSubpart"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x44c32df6)
        }
    }
    impl _serde::Serialize for hkpExtendedMeshShapeTrianglesSubpart {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x44c32df6)));
            let mut serializer = __serializer
                .serialize_struct("hkpExtendedMeshShapeTrianglesSubpart", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer
                .serialize_field(
                    "materialIndexStridingType",
                    &self.parent.m_materialIndexStridingType,
                )?;
            serializer.skip_field("materialStriding", &self.parent.m_materialStriding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field("materialIndexBase", &self.parent.m_materialIndexBase)?;
            serializer
                .serialize_field(
                    "materialIndexStriding",
                    &self.parent.m_materialIndexStriding,
                )?;
            serializer.serialize_field("numMaterials", &self.parent.m_numMaterials)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialBase", &self.parent.m_materialBase)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("numTriangleShapes", &self.m_numTriangleShapes)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("vertexBase", &self.m_vertexBase)?;
            serializer.serialize_field("numVertices", &self.m_numVertices)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("indexBase", &self.m_indexBase)?;
            serializer.serialize_field("vertexStriding", &self.m_vertexStriding)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("triangleOffset", &self.m_triangleOffset)?;
            serializer.serialize_field("indexStriding", &self.m_indexStriding)?;
            serializer.serialize_field("stridingType", &self.m_stridingType)?;
            serializer
                .serialize_field(
                    "flipAlternateTriangles",
                    &self.m_flipAlternateTriangles,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_field("extrusion", &self.m_extrusion)?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_numTriangleShapes,
    m_vertexBase,
    m_numVertices,
    m_indexBase,
    m_vertexStriding,
    m_triangleOffset,
    m_indexStriding,
    m_stridingType,
    m_flipAlternateTriangles,
    m_extrusion,
    m_transform,
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
            "numTriangleShapes" => Ok(__Field::m_numTriangleShapes),
            "numVertices" => Ok(__Field::m_numVertices),
            "vertexStriding" => Ok(__Field::m_vertexStriding),
            "triangleOffset" => Ok(__Field::m_triangleOffset),
            "indexStriding" => Ok(__Field::m_indexStriding),
            "stridingType" => Ok(__Field::m_stridingType),
            "flipAlternateTriangles" => Ok(__Field::m_flipAlternateTriangles),
            "extrusion" => Ok(__Field::m_extrusion),
            "transform" => Ok(__Field::m_transform),
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
pub(super) struct __hkpExtendedMeshShapeTrianglesSubpartVisitor<'de> {
    marker: core::marker::PhantomData<hkpExtendedMeshShapeTrianglesSubpart>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpExtendedMeshShapeTrianglesSubpartVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpExtendedMeshShapeTrianglesSubpart, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpExtendedMeshShapeTrianglesSubpart,
                >,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de>
for __hkpExtendedMeshShapeTrianglesSubpartVisitor<'de> {
    type Value = hkpExtendedMeshShapeTrianglesSubpart;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpExtendedMeshShapeTrianglesSubpart",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_numTriangleShapes: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_vertexBase: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_numVertices: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_indexBase: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_vertexStriding: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_triangleOffset: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_indexStriding: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_stridingType: _serde::__private::Option<IndexStridingType> = _serde::__private::None;
        let mut m_flipAlternateTriangles: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_extrusion: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_transform: _serde::__private::Option<QsTransform> = _serde::__private::None;
        for i in 0..11usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_numTriangleShapes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numTriangleShapes",
                            ),
                        );
                    }
                    m_numTriangleShapes = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_vertexBase) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vertexBase",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_vertexBase = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 4usize)?;
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
                    if _serde::__private::Option::is_some(&m_vertexStriding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vertexStriding",
                            ),
                        );
                    }
                    m_vertexStriding = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_triangleOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "triangleOffset",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 2usize)?;
                    m_triangleOffset = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
                    m_indexStriding = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_stridingType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "stridingType",
                            ),
                        );
                    }
                    m_stridingType = _serde::__private::Some(
                        match __A::next_value::<IndexStridingType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_flipAlternateTriangles) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "flipAlternateTriangles",
                            ),
                        );
                    }
                    m_flipAlternateTriangles = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_extrusion) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "extrusion",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 12usize)?;
                    m_extrusion = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_transform) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transform",
                            ),
                        );
                    }
                    m_transform = _serde::__private::Some(
                        match __A::next_value::<QsTransform>(&mut __map) {
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
        let m_numTriangleShapes = match m_numTriangleShapes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numTriangleShapes"),
                );
            }
        };
        let m_vertexBase = match m_vertexBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexBase"),
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
        let m_vertexStriding = match m_vertexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexStriding"),
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
        let m_indexStriding = match m_indexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("indexStriding"),
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
        let m_extrusion = match m_extrusion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("extrusion"),
                );
            }
        };
        let m_transform = match m_transform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transform"),
                );
            }
        };
        _serde::__private::Ok(hkpExtendedMeshShapeTrianglesSubpart {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_numTriangleShapes,
            m_vertexBase,
            m_numVertices,
            m_indexBase,
            m_vertexStriding,
            m_triangleOffset,
            m_indexStriding,
            m_stridingType,
            m_flipAlternateTriangles,
            m_extrusion,
            m_transform,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpExtendedMeshShapeSubpartVisitor::visit_as_parent(&mut __map)?;
        let mut m_numTriangleShapes: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numVertices: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_vertexStriding: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_triangleOffset: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_indexStriding: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_stridingType: _serde::__private::Option<IndexStridingType> = _serde::__private::None;
        let mut m_flipAlternateTriangles: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_extrusion: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_transform: _serde::__private::Option<QsTransform> = _serde::__private::None;
        for _ in 0..9usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_numTriangleShapes => {
                        if _serde::__private::Option::is_some(&m_numTriangleShapes) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numTriangleShapes",
                                ),
                            );
                        }
                        m_numTriangleShapes = _serde::__private::Some(
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
                    __Field::m_vertexStriding => {
                        if _serde::__private::Option::is_some(&m_vertexStriding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vertexStriding",
                                ),
                            );
                        }
                        m_vertexStriding = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
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
                    __Field::m_indexStriding => {
                        if _serde::__private::Option::is_some(&m_indexStriding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "indexStriding",
                                ),
                            );
                        }
                        m_indexStriding = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
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
                            match __A::next_value::<IndexStridingType>(&mut __map) {
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
                            match __A::next_value::<i8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_extrusion => {
                        if _serde::__private::Option::is_some(&m_extrusion) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "extrusion",
                                ),
                            );
                        }
                        m_extrusion = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_transform => {
                        if _serde::__private::Option::is_some(&m_transform) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "transform",
                                ),
                            );
                        }
                        m_transform = _serde::__private::Some(
                            match __A::next_value::<QsTransform>(&mut __map) {
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
        let m_numTriangleShapes = match m_numTriangleShapes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numTriangleShapes"),
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
        let m_vertexStriding = match m_vertexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexStriding"),
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
        let m_indexStriding = match m_indexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("indexStriding"),
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
        let m_extrusion = match m_extrusion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("extrusion"),
                );
            }
        };
        let m_transform = match m_transform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transform"),
                );
            }
        };
        _serde::__private::Ok(hkpExtendedMeshShapeTrianglesSubpart {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_numTriangleShapes,
            m_numVertices,
            m_vertexStriding,
            m_triangleOffset,
            m_indexStriding,
            m_stridingType,
            m_flipAlternateTriangles,
            m_extrusion,
            m_transform,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpExtendedMeshShapeTrianglesSubpart {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "numTriangleShapes",
                "vertexBase",
                "numVertices",
                "indexBase",
                "vertexStriding",
                "triangleOffset",
                "indexStriding",
                "stridingType",
                "flipAlternateTriangles",
                "extrusion",
                "transform",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpExtendedMeshShapeTrianglesSubpart",
                FIELDS,
                __hkpExtendedMeshShapeTrianglesSubpartVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpExtendedMeshShapeTrianglesSubpart,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
