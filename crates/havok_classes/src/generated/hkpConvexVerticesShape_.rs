use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpConvexVerticesShape`
/// - version: `3`
/// - signature: `0x28726ad8`
/// - size: `112`(x86)/`144`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConvexVerticesShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConvexShape,
    /// # C++ Info
    /// - name: `aabbHalfExtents`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_aabbHalfExtents: Vector4,
    /// # C++ Info
    /// - name: `aabbCenter`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 64`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_aabbCenter: Vector4,
    /// # C++ Info
    /// - name: `rotatedVertices`(ctype: `hkArray<struct hkpConvexVerticesShapeFourVectors>`)
    /// - offset: ` 64`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_rotatedVertices: Vec<hkpConvexVerticesShapeFourVectors>,
    /// # C++ Info
    /// - name: `numVertices`(ctype: `hkInt32`)
    /// - offset: ` 76`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numVertices: i32,
    /// # C++ Info
    /// - name: `externalObject`(ctype: `void*`)
    /// - offset: ` 80`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_externalObject: Pointer,
    /// # C++ Info
    /// - name: `getFaceNormals`(ctype: `void*`)
    /// - offset: ` 84`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_getFaceNormals: Pointer,
    /// # C++ Info
    /// - name: `planeEquations`(ctype: `hkArray<hkVector4>`)
    /// - offset: ` 88`(x86)/`120`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_planeEquations: Vec<Vector4>,
    /// # C++ Info
    /// - name: `connectivity`(ctype: `struct hkpConvexVerticesConnectivity*`)
    /// - offset: `100`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_connectivity: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConvexVerticesShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConvexVerticesShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x28726ad8)
        }
    }
    impl _serde::Serialize for hkpConvexVerticesShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x28726ad8)));
            let mut serializer = __serializer
                .serialize_struct("hkpConvexVerticesShape", class_meta)?;
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
            serializer.serialize_field("radius", &self.parent.m_radius)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("aabbHalfExtents", &self.m_aabbHalfExtents)?;
            serializer.serialize_field("aabbCenter", &self.m_aabbCenter)?;
            serializer
                .serialize_array_meta_field("rotatedVertices", &self.m_rotatedVertices)?;
            serializer.serialize_field("numVertices", &self.m_numVertices)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("externalObject", &self.m_externalObject)?;
            serializer.skip_field("getFaceNormals", &self.m_getFaceNormals)?;
            serializer
                .serialize_array_meta_field("planeEquations", &self.m_planeEquations)?;
            serializer.serialize_field("connectivity", &self.m_connectivity)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field("rotatedVertices", &self.m_rotatedVertices)?;
            serializer.serialize_array_field("planeEquations", &self.m_planeEquations)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_aabbHalfExtents,
    m_aabbCenter,
    m_rotatedVertices,
    m_numVertices,
    m_externalObject,
    m_getFaceNormals,
    m_planeEquations,
    m_connectivity,
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
            "aabbHalfExtents" => Ok(__Field::m_aabbHalfExtents),
            "aabbCenter" => Ok(__Field::m_aabbCenter),
            "rotatedVertices" => Ok(__Field::m_rotatedVertices),
            "numVertices" => Ok(__Field::m_numVertices),
            "planeEquations" => Ok(__Field::m_planeEquations),
            "connectivity" => Ok(__Field::m_connectivity),
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
pub(super) struct __hkpConvexVerticesShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpConvexVerticesShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpConvexVerticesShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpConvexVerticesShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpConvexVerticesShape>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpConvexVerticesShapeVisitor<'de> {
    type Value = hkpConvexVerticesShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpConvexVerticesShape")
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
        let mut m_aabbHalfExtents: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_aabbCenter: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rotatedVertices: _serde::__private::Option<
            Vec<hkpConvexVerticesShapeFourVectors>,
        > = _serde::__private::None;
        let mut m_numVertices: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_externalObject: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_getFaceNormals: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_planeEquations: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_connectivity: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_aabbHalfExtents) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "aabbHalfExtents",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 8usize)?;
                    m_aabbHalfExtents = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
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
                2usize => {
                    if _serde::__private::Option::is_some(&m_rotatedVertices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotatedVertices",
                            ),
                        );
                    }
                    m_rotatedVertices = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpConvexVerticesShapeFourVectors>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
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
                4usize => {
                    if _serde::__private::Option::is_some(&m_externalObject) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "externalObject",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_externalObject = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_getFaceNormals) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "getFaceNormals",
                            ),
                        );
                    }
                    m_getFaceNormals = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_planeEquations) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "planeEquations",
                            ),
                        );
                    }
                    m_planeEquations = _serde::__private::Some(
                        match __A::next_value::<Vec<Vector4>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_connectivity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "connectivity",
                            ),
                        );
                    }
                    m_connectivity = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
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
        let m_rotatedVertices = match m_rotatedVertices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotatedVertices"),
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
        let m_externalObject = match m_externalObject {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("externalObject"),
                );
            }
        };
        let m_getFaceNormals = match m_getFaceNormals {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("getFaceNormals"),
                );
            }
        };
        let m_planeEquations = match m_planeEquations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("planeEquations"),
                );
            }
        };
        let m_connectivity = match m_connectivity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("connectivity"),
                );
            }
        };
        _serde::__private::Ok(hkpConvexVerticesShape {
            __ptr,
            parent,
            m_aabbHalfExtents,
            m_aabbCenter,
            m_rotatedVertices,
            m_numVertices,
            m_externalObject,
            m_getFaceNormals,
            m_planeEquations,
            m_connectivity,
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
        let parent = __hkpConvexShapeVisitor::visit_as_parent(&mut __map)?;
        let mut m_aabbHalfExtents: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_aabbCenter: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rotatedVertices: _serde::__private::Option<
            Vec<hkpConvexVerticesShapeFourVectors>,
        > = _serde::__private::None;
        let mut m_numVertices: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_planeEquations: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_connectivity: _serde::__private::Option<Pointer> = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
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
                    __Field::m_rotatedVertices => {
                        if _serde::__private::Option::is_some(&m_rotatedVertices) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotatedVertices",
                                ),
                            );
                        }
                        m_rotatedVertices = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkpConvexVerticesShapeFourVectors>,
                            >(&mut __map) {
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
                    __Field::m_planeEquations => {
                        if _serde::__private::Option::is_some(&m_planeEquations) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "planeEquations",
                                ),
                            );
                        }
                        m_planeEquations = _serde::__private::Some(
                            match __A::next_value::<Vec<Vector4>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_connectivity => {
                        if _serde::__private::Option::is_some(&m_connectivity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "connectivity",
                                ),
                            );
                        }
                        m_connectivity = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
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
        let m_rotatedVertices = match m_rotatedVertices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotatedVertices"),
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
        let m_planeEquations = match m_planeEquations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("planeEquations"),
                );
            }
        };
        let m_connectivity = match m_connectivity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("connectivity"),
                );
            }
        };
        _serde::__private::Ok(hkpConvexVerticesShape {
            __ptr,
            parent,
            m_aabbHalfExtents,
            m_aabbCenter,
            m_rotatedVertices,
            m_numVertices,
            m_planeEquations,
            m_connectivity,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConvexVerticesShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "aabbHalfExtents",
                "aabbCenter",
                "rotatedVertices",
                "numVertices",
                "externalObject",
                "getFaceNormals",
                "planeEquations",
                "connectivity",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConvexVerticesShape",
                FIELDS,
                __hkpConvexVerticesShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpConvexVerticesShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
