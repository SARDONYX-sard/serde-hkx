use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpSimpleMeshShape`
/// - version: `0`
/// - signature: `0x16b3c811`
/// - size: ` 68`(x86)/`104`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSimpleMeshShape {
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
    /// - name: `vertices`(ctype: `hkArray<hkVector4>`)
    /// - offset: ` 24`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_vertices: Vec<Vector4>,
    /// # C++ Info
    /// - name: `triangles`(ctype: `hkArray<struct hkpSimpleMeshShapeTriangle>`)
    /// - offset: ` 36`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_triangles: Vec<hkpSimpleMeshShapeTriangle>,
    /// # C++ Info
    /// - name: `materialIndices`(ctype: `hkArray<hkUint8>`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_materialIndices: Vec<u8>,
    /// # C++ Info
    /// - name: `radius`(ctype: `hkReal`)
    /// - offset: ` 60`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_radius: f32,
    /// # C++ Info
    /// - name: `weldingType`(ctype: `enum WeldingType`)
    /// - offset: ` 64`(x86)/`100`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_weldingType: WeldingType,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSimpleMeshShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSimpleMeshShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x16b3c811)
        }
    }
    impl _serde::Serialize for hkpSimpleMeshShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x16b3c811)));
            let mut serializer = __serializer
                .serialize_struct("hkpSimpleMeshShape", class_meta)?;
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
            serializer.serialize_array_meta_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_meta_field("triangles", &self.m_triangles)?;
            serializer
                .serialize_array_meta_field("materialIndices", &self.m_materialIndices)?;
            serializer.serialize_field("radius", &self.m_radius)?;
            serializer.serialize_field("weldingType", &self.m_weldingType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_array_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_field("triangles", &self.m_triangles)?;
            serializer
                .serialize_array_field("materialIndices", &self.m_materialIndices)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_vertices,
    m_triangles,
    m_materialIndices,
    m_radius,
    m_weldingType,
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
            "vertices" => Ok(__Field::m_vertices),
            "triangles" => Ok(__Field::m_triangles),
            "materialIndices" => Ok(__Field::m_materialIndices),
            "radius" => Ok(__Field::m_radius),
            "weldingType" => Ok(__Field::m_weldingType),
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
pub(super) struct __hkpSimpleMeshShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpSimpleMeshShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpSimpleMeshShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpSimpleMeshShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpSimpleMeshShape>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpSimpleMeshShapeVisitor<'de> {
    type Value = hkpSimpleMeshShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpSimpleMeshShape")
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
        let mut m_vertices: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_triangles: _serde::__private::Option<
            Vec<hkpSimpleMeshShapeTriangle>,
        > = _serde::__private::None;
        let mut m_materialIndices: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_weldingType: _serde::__private::Option<WeldingType> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_vertices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vertices",
                            ),
                        );
                    }
                    m_vertices = _serde::__private::Some(
                        match __A::next_value::<Vec<Vector4>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_triangles) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "triangles",
                            ),
                        );
                    }
                    m_triangles = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpSimpleMeshShapeTriangle>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_materialIndices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialIndices",
                            ),
                        );
                    }
                    m_materialIndices = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
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
                4usize => {
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
                _ => {}
            }
        }
        __A::pad(&mut __map, 3usize, 3usize)?;
        let m_vertices = match m_vertices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertices"),
                );
            }
        };
        let m_triangles = match m_triangles {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangles"),
                );
            }
        };
        let m_materialIndices = match m_materialIndices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialIndices"),
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
                    <__A::Error as _serde::de::Error>::missing_field("weldingType"),
                );
            }
        };
        _serde::__private::Ok(hkpSimpleMeshShape {
            __ptr,
            parent,
            m_vertices,
            m_triangles,
            m_materialIndices,
            m_radius,
            m_weldingType,
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
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkpShapeCollectionVisitor::visit_as_parent(&mut __map)?;
        let mut m_vertices: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_triangles: _serde::__private::Option<
            Vec<hkpSimpleMeshShapeTriangle>,
        > = _serde::__private::None;
        let mut m_materialIndices: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_weldingType: _serde::__private::Option<WeldingType> = _serde::__private::None;
        for _ in 0..5usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_vertices => {
                        if _serde::__private::Option::is_some(&m_vertices) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vertices",
                                ),
                            );
                        }
                        m_vertices = _serde::__private::Some(
                            match __A::next_value::<Vec<Vector4>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_triangles => {
                        if _serde::__private::Option::is_some(&m_triangles) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "triangles",
                                ),
                            );
                        }
                        m_triangles = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkpSimpleMeshShapeTriangle>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_materialIndices => {
                        if _serde::__private::Option::is_some(&m_materialIndices) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materialIndices",
                                ),
                            );
                        }
                        m_materialIndices = _serde::__private::Some(
                            match __A::next_value::<Vec<u8>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_radius => {
                        if _serde::__private::Option::is_some(&m_radius) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("radius"),
                            );
                        }
                        m_radius = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
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
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        let m_vertices = match m_vertices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertices"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_triangles = match m_triangles {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangles"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_materialIndices = match m_materialIndices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialIndices"),
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
                    <__A::Error as _serde::de::Error>::missing_field("weldingType"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpSimpleMeshShape {
            __ptr,
            parent,
            m_vertices,
            m_triangles,
            m_materialIndices,
            m_radius,
            m_weldingType,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSimpleMeshShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "vertices",
                "triangles",
                "materialIndices",
                "radius",
                "weldingType",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSimpleMeshShape",
                FIELDS,
                __hkpSimpleMeshShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpSimpleMeshShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
