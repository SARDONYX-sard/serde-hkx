use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCompressedMeshShapeChunk`
/// -         version: `4`
/// -       signature: `0x5d0d67bd`
/// -          size:  80(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCompressedMeshShapeChunk {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `offset`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_offset: Vector4,
    /// # C++ Info
    /// -          name: `vertices`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vertices: Vec<u16>,
    /// # C++ Info
    /// -          name: `indices`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  28(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_indices: Vec<u16>,
    /// # C++ Info
    /// -          name: `stripLengths`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  40(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_stripLengths: Vec<u16>,
    /// # C++ Info
    /// -          name: `weldingInfo`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  52(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_weldingInfo: Vec<u16>,
    /// # C++ Info
    /// -          name: `materialInfo`(ctype: `hkUint32`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_materialInfo: u32,
    /// # C++ Info
    /// -          name: `reference`(ctype: `hkUint16`)
    /// -        offset:  68(x86)/ 84(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_reference: u16,
    /// # C++ Info
    /// -          name: `transformIndex`(ctype: `hkUint16`)
    /// -        offset:  70(x86)/ 86(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_transformIndex: u16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCompressedMeshShapeChunk {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCompressedMeshShapeChunk"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5d0d67bd)
        }
    }
    impl _serde::Serialize for hkpCompressedMeshShapeChunk {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5d0d67bd)));
            let mut serializer = __serializer
                .serialize_struct("hkpCompressedMeshShapeChunk", class_meta)?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.serialize_array_meta_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_meta_field("indices", &self.m_indices)?;
            serializer.serialize_array_meta_field("stripLengths", &self.m_stripLengths)?;
            serializer.serialize_array_meta_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.serialize_field("materialInfo", &self.m_materialInfo)?;
            serializer.serialize_field("reference", &self.m_reference)?;
            serializer.serialize_field("transformIndex", &self.m_transformIndex)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_array_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_field("indices", &self.m_indices)?;
            serializer.serialize_array_field("stripLengths", &self.m_stripLengths)?;
            serializer.serialize_array_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_offset,
    m_vertices,
    m_indices,
    m_stripLengths,
    m_weldingInfo,
    m_materialInfo,
    m_reference,
    m_transformIndex,
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
            "offset" => Ok(__Field::m_offset),
            "vertices" => Ok(__Field::m_vertices),
            "indices" => Ok(__Field::m_indices),
            "stripLengths" => Ok(__Field::m_stripLengths),
            "weldingInfo" => Ok(__Field::m_weldingInfo),
            "materialInfo" => Ok(__Field::m_materialInfo),
            "reference" => Ok(__Field::m_reference),
            "transformIndex" => Ok(__Field::m_transformIndex),
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
pub(super) struct __hkpCompressedMeshShapeChunkVisitor<'de> {
    marker: core::marker::PhantomData<hkpCompressedMeshShapeChunk>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpCompressedMeshShapeChunkVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpCompressedMeshShapeChunk, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpCompressedMeshShapeChunk>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpCompressedMeshShapeChunkVisitor<'de> {
    type Value = hkpCompressedMeshShapeChunk;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpCompressedMeshShapeChunk",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_offset: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vertices: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_indices: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_stripLengths: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_weldingInfo: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_materialInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_reference: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_transformIndex: _serde::__private::Option<u16> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_offset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                        );
                    }
                    m_offset = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_vertices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vertices",
                            ),
                        );
                    }
                    m_vertices = _serde::__private::Some(
                        match __A::next_value::<Vec<u16>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_indices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("indices"),
                        );
                    }
                    m_indices = _serde::__private::Some(
                        match __A::next_value::<Vec<u16>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_stripLengths) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "stripLengths",
                            ),
                        );
                    }
                    m_stripLengths = _serde::__private::Some(
                        match __A::next_value::<Vec<u16>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
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
                5usize => {
                    if _serde::__private::Option::is_some(&m_materialInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialInfo",
                            ),
                        );
                    }
                    m_materialInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_reference) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "reference",
                            ),
                        );
                    }
                    m_reference = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_transformIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformIndex",
                            ),
                        );
                    }
                    m_transformIndex = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
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
        __A::pad(&mut __map, 8usize, 8usize)?;
        let m_offset = match m_offset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offset"),
                );
            }
        };
        let m_vertices = match m_vertices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertices"),
                );
            }
        };
        let m_indices = match m_indices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("indices"),
                );
            }
        };
        let m_stripLengths = match m_stripLengths {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("stripLengths"),
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
        let m_materialInfo = match m_materialInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialInfo"),
                );
            }
        };
        let m_reference = match m_reference {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("reference"),
                );
            }
        };
        let m_transformIndex = match m_transformIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformIndex"),
                );
            }
        };
        _serde::__private::Ok(hkpCompressedMeshShapeChunk {
            __ptr: __A::class_ptr(&mut __map),
            m_offset,
            m_vertices,
            m_indices,
            m_stripLengths,
            m_weldingInfo,
            m_materialInfo,
            m_reference,
            m_transformIndex,
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
        let mut m_offset: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vertices: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_indices: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_stripLengths: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_weldingInfo: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_materialInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_reference: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_transformIndex: _serde::__private::Option<u16> = _serde::__private::None;
        for _ in 0..8usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_offset => {
                        if _serde::__private::Option::is_some(&m_offset) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                            );
                        }
                        m_offset = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_vertices => {
                        if _serde::__private::Option::is_some(&m_vertices) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vertices",
                                ),
                            );
                        }
                        m_vertices = _serde::__private::Some(
                            match __A::next_value::<Vec<u16>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_indices => {
                        if _serde::__private::Option::is_some(&m_indices) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "indices",
                                ),
                            );
                        }
                        m_indices = _serde::__private::Some(
                            match __A::next_value::<Vec<u16>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_stripLengths => {
                        if _serde::__private::Option::is_some(&m_stripLengths) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "stripLengths",
                                ),
                            );
                        }
                        m_stripLengths = _serde::__private::Some(
                            match __A::next_value::<Vec<u16>>(&mut __map) {
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
                    __Field::m_materialInfo => {
                        if _serde::__private::Option::is_some(&m_materialInfo) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materialInfo",
                                ),
                            );
                        }
                        m_materialInfo = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_reference => {
                        if _serde::__private::Option::is_some(&m_reference) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "reference",
                                ),
                            );
                        }
                        m_reference = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_transformIndex => {
                        if _serde::__private::Option::is_some(&m_transformIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "transformIndex",
                                ),
                            );
                        }
                        m_transformIndex = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
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
        let m_offset = match m_offset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offset"),
                );
            }
        };
        let m_vertices = match m_vertices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertices"),
                );
            }
        };
        let m_indices = match m_indices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("indices"),
                );
            }
        };
        let m_stripLengths = match m_stripLengths {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("stripLengths"),
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
        let m_materialInfo = match m_materialInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialInfo"),
                );
            }
        };
        let m_reference = match m_reference {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("reference"),
                );
            }
        };
        let m_transformIndex = match m_transformIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformIndex"),
                );
            }
        };
        _serde::__private::Ok(hkpCompressedMeshShapeChunk {
            __ptr,
            m_offset,
            m_vertices,
            m_indices,
            m_stripLengths,
            m_weldingInfo,
            m_materialInfo,
            m_reference,
            m_transformIndex,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCompressedMeshShapeChunk {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "offset",
                "vertices",
                "indices",
                "stripLengths",
                "weldingInfo",
                "materialInfo",
                "reference",
                "transformIndex",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCompressedMeshShapeChunk",
                FIELDS,
                __hkpCompressedMeshShapeChunkVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpCompressedMeshShapeChunk,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
