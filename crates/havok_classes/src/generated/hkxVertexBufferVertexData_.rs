use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxVertexBufferVertexData`
/// -         version: `0`
/// -       signature: `0xd72b6fd0`
/// -          size:  84(x86)/104(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxVertexBufferVertexData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `vectorData`(ctype: `hkArray<hkVector4>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vectorData: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `floatData`(ctype: `hkArray<hkReal>`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_floatData: Vec<f32>,
    /// # C++ Info
    /// -          name: `uint32Data`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_uint32Data: Vec<u32>,
    /// # C++ Info
    /// -          name: `uint16Data`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  36(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_uint16Data: Vec<u16>,
    /// # C++ Info
    /// -          name: `uint8Data`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_uint8Data: Vec<u8>,
    /// # C++ Info
    /// -          name: `numVerts`(ctype: `hkUint32`)
    /// -        offset:  60(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numVerts: u32,
    /// # C++ Info
    /// -          name: `vectorStride`(ctype: `hkUint32`)
    /// -        offset:  64(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vectorStride: u32,
    /// # C++ Info
    /// -          name: `floatStride`(ctype: `hkUint32`)
    /// -        offset:  68(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_floatStride: u32,
    /// # C++ Info
    /// -          name: `uint32Stride`(ctype: `hkUint32`)
    /// -        offset:  72(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_uint32Stride: u32,
    /// # C++ Info
    /// -          name: `uint16Stride`(ctype: `hkUint32`)
    /// -        offset:  76(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_uint16Stride: u32,
    /// # C++ Info
    /// -          name: `uint8Stride`(ctype: `hkUint32`)
    /// -        offset:  80(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_uint8Stride: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxVertexBufferVertexData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxVertexBufferVertexData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd72b6fd0)
        }
    }
    impl _serde::Serialize for hkxVertexBufferVertexData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd72b6fd0)));
            let mut serializer = __serializer
                .serialize_struct("hkxVertexBufferVertexData", class_meta)?;
            serializer.serialize_array_meta_field("vectorData", &self.m_vectorData)?;
            serializer.serialize_array_meta_field("floatData", &self.m_floatData)?;
            serializer.serialize_array_meta_field("uint32Data", &self.m_uint32Data)?;
            serializer.serialize_array_meta_field("uint16Data", &self.m_uint16Data)?;
            serializer.serialize_array_meta_field("uint8Data", &self.m_uint8Data)?;
            serializer.serialize_field("numVerts", &self.m_numVerts)?;
            serializer.serialize_field("vectorStride", &self.m_vectorStride)?;
            serializer.serialize_field("floatStride", &self.m_floatStride)?;
            serializer.serialize_field("uint32Stride", &self.m_uint32Stride)?;
            serializer.serialize_field("uint16Stride", &self.m_uint16Stride)?;
            serializer.serialize_field("uint8Stride", &self.m_uint8Stride)?;
            serializer.serialize_array_field("vectorData", &self.m_vectorData)?;
            serializer.serialize_array_field("floatData", &self.m_floatData)?;
            serializer.serialize_array_field("uint32Data", &self.m_uint32Data)?;
            serializer.serialize_array_field("uint16Data", &self.m_uint16Data)?;
            serializer.serialize_array_field("uint8Data", &self.m_uint8Data)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_vectorData,
    m_floatData,
    m_uint32Data,
    m_uint16Data,
    m_uint8Data,
    m_numVerts,
    m_vectorStride,
    m_floatStride,
    m_uint32Stride,
    m_uint16Stride,
    m_uint8Stride,
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
            "vectorData" => Ok(__Field::m_vectorData),
            "floatData" => Ok(__Field::m_floatData),
            "uint32Data" => Ok(__Field::m_uint32Data),
            "uint16Data" => Ok(__Field::m_uint16Data),
            "uint8Data" => Ok(__Field::m_uint8Data),
            "numVerts" => Ok(__Field::m_numVerts),
            "vectorStride" => Ok(__Field::m_vectorStride),
            "floatStride" => Ok(__Field::m_floatStride),
            "uint32Stride" => Ok(__Field::m_uint32Stride),
            "uint16Stride" => Ok(__Field::m_uint16Stride),
            "uint8Stride" => Ok(__Field::m_uint8Stride),
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
pub(super) struct __hkxVertexBufferVertexDataVisitor<'de> {
    marker: core::marker::PhantomData<hkxVertexBufferVertexData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxVertexBufferVertexDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxVertexBufferVertexData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxVertexBufferVertexData>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxVertexBufferVertexDataVisitor<'de> {
    type Value = hkxVertexBufferVertexData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxVertexBufferVertexData")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_vectorData: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_floatData: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_uint32Data: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_uint16Data: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_uint8Data: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_numVerts: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_vectorStride: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_floatStride: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_uint32Stride: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_uint16Stride: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_uint8Stride: _serde::__private::Option<u32> = _serde::__private::None;
        for i in 0..11usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_vectorData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vectorData",
                            ),
                        );
                    }
                    m_vectorData = _serde::__private::Some(
                        match __A::next_value::<Vec<Vector4>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_floatData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "floatData",
                            ),
                        );
                    }
                    m_floatData = _serde::__private::Some(
                        match __A::next_value::<Vec<f32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_uint32Data) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "uint32Data",
                            ),
                        );
                    }
                    m_uint32Data = _serde::__private::Some(
                        match __A::next_value::<Vec<u32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_uint16Data) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "uint16Data",
                            ),
                        );
                    }
                    m_uint16Data = _serde::__private::Some(
                        match __A::next_value::<Vec<u16>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_uint8Data) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "uint8Data",
                            ),
                        );
                    }
                    m_uint8Data = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_numVerts) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numVerts",
                            ),
                        );
                    }
                    m_numVerts = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_vectorStride) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vectorStride",
                            ),
                        );
                    }
                    m_vectorStride = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_floatStride) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "floatStride",
                            ),
                        );
                    }
                    m_floatStride = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_uint32Stride) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "uint32Stride",
                            ),
                        );
                    }
                    m_uint32Stride = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_uint16Stride) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "uint16Stride",
                            ),
                        );
                    }
                    m_uint16Stride = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_uint8Stride) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "uint8Stride",
                            ),
                        );
                    }
                    m_uint8Stride = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
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
        let m_vectorData = match m_vectorData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectorData"),
                );
            }
        };
        let m_floatData = match m_floatData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatData"),
                );
            }
        };
        let m_uint32Data = match m_uint32Data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint32Data"),
                );
            }
        };
        let m_uint16Data = match m_uint16Data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint16Data"),
                );
            }
        };
        let m_uint8Data = match m_uint8Data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint8Data"),
                );
            }
        };
        let m_numVerts = match m_numVerts {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numVerts"),
                );
            }
        };
        let m_vectorStride = match m_vectorStride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectorStride"),
                );
            }
        };
        let m_floatStride = match m_floatStride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatStride"),
                );
            }
        };
        let m_uint32Stride = match m_uint32Stride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint32Stride"),
                );
            }
        };
        let m_uint16Stride = match m_uint16Stride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint16Stride"),
                );
            }
        };
        let m_uint8Stride = match m_uint8Stride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint8Stride"),
                );
            }
        };
        _serde::__private::Ok(hkxVertexBufferVertexData {
            __ptr: __A::class_ptr(&mut __map),
            m_vectorData,
            m_floatData,
            m_uint32Data,
            m_uint16Data,
            m_uint8Data,
            m_numVerts,
            m_vectorStride,
            m_floatStride,
            m_uint32Stride,
            m_uint16Stride,
            m_uint8Stride,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_vectorData: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_floatData: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_uint32Data: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
        let mut m_uint16Data: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_uint8Data: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_numVerts: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_vectorStride: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_floatStride: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_uint32Stride: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_uint16Stride: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_uint8Stride: _serde::__private::Option<u32> = _serde::__private::None;
        for _ in 0..11usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_vectorData => {
                        if _serde::__private::Option::is_some(&m_vectorData) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vectorData",
                                ),
                            );
                        }
                        m_vectorData = _serde::__private::Some(
                            match __A::next_value::<Vec<Vector4>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_floatData => {
                        if _serde::__private::Option::is_some(&m_floatData) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "floatData",
                                ),
                            );
                        }
                        m_floatData = _serde::__private::Some(
                            match __A::next_value::<Vec<f32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_uint32Data => {
                        if _serde::__private::Option::is_some(&m_uint32Data) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "uint32Data",
                                ),
                            );
                        }
                        m_uint32Data = _serde::__private::Some(
                            match __A::next_value::<Vec<u32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_uint16Data => {
                        if _serde::__private::Option::is_some(&m_uint16Data) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "uint16Data",
                                ),
                            );
                        }
                        m_uint16Data = _serde::__private::Some(
                            match __A::next_value::<Vec<u16>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_uint8Data => {
                        if _serde::__private::Option::is_some(&m_uint8Data) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "uint8Data",
                                ),
                            );
                        }
                        m_uint8Data = _serde::__private::Some(
                            match __A::next_value::<Vec<u8>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numVerts => {
                        if _serde::__private::Option::is_some(&m_numVerts) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numVerts",
                                ),
                            );
                        }
                        m_numVerts = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_vectorStride => {
                        if _serde::__private::Option::is_some(&m_vectorStride) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vectorStride",
                                ),
                            );
                        }
                        m_vectorStride = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_floatStride => {
                        if _serde::__private::Option::is_some(&m_floatStride) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "floatStride",
                                ),
                            );
                        }
                        m_floatStride = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_uint32Stride => {
                        if _serde::__private::Option::is_some(&m_uint32Stride) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "uint32Stride",
                                ),
                            );
                        }
                        m_uint32Stride = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_uint16Stride => {
                        if _serde::__private::Option::is_some(&m_uint16Stride) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "uint16Stride",
                                ),
                            );
                        }
                        m_uint16Stride = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_uint8Stride => {
                        if _serde::__private::Option::is_some(&m_uint8Stride) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "uint8Stride",
                                ),
                            );
                        }
                        m_uint8Stride = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
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
        let m_vectorData = match m_vectorData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectorData"),
                );
            }
        };
        let m_floatData = match m_floatData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatData"),
                );
            }
        };
        let m_uint32Data = match m_uint32Data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint32Data"),
                );
            }
        };
        let m_uint16Data = match m_uint16Data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint16Data"),
                );
            }
        };
        let m_uint8Data = match m_uint8Data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint8Data"),
                );
            }
        };
        let m_numVerts = match m_numVerts {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numVerts"),
                );
            }
        };
        let m_vectorStride = match m_vectorStride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectorStride"),
                );
            }
        };
        let m_floatStride = match m_floatStride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatStride"),
                );
            }
        };
        let m_uint32Stride = match m_uint32Stride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint32Stride"),
                );
            }
        };
        let m_uint16Stride = match m_uint16Stride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint16Stride"),
                );
            }
        };
        let m_uint8Stride = match m_uint8Stride {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uint8Stride"),
                );
            }
        };
        _serde::__private::Ok(hkxVertexBufferVertexData {
            __ptr: __A::class_ptr(&mut __map),
            m_vectorData,
            m_floatData,
            m_uint32Data,
            m_uint16Data,
            m_uint8Data,
            m_numVerts,
            m_vectorStride,
            m_floatStride,
            m_uint32Stride,
            m_uint16Stride,
            m_uint8Stride,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxVertexBufferVertexData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "vectorData",
                "floatData",
                "uint32Data",
                "uint16Data",
                "uint8Data",
                "numVerts",
                "vectorStride",
                "floatStride",
                "uint32Stride",
                "uint16Stride",
                "uint8Stride",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxVertexBufferVertexData",
                FIELDS,
                __hkxVertexBufferVertexDataVisitor {
                    marker: _serde::__private::PhantomData::<hkxVertexBufferVertexData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
