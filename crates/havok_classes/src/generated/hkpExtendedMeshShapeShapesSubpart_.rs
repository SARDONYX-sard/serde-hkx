use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpExtendedMeshShapeShapesSubpart`
/// -         version: `1`
/// -       signature: `0xf204b155`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpExtendedMeshShapeShapesSubpart {
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
    /// -          name: `childShapes`(ctype: `hkArray<hkpConvexShape*>`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childShapes: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `rotation`(ctype: `hkQuaternion`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotation: Quaternion,
    /// # C++ Info
    /// -          name: `translation`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_translation: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpExtendedMeshShapeShapesSubpart {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpExtendedMeshShapeShapesSubpart"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf204b155)
        }
    }
    impl _serde::Serialize for hkpExtendedMeshShapeShapesSubpart {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf204b155)));
            let mut serializer = __serializer
                .serialize_struct("hkpExtendedMeshShapeShapesSubpart", class_meta)?;
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
            serializer.serialize_array_meta_field("childShapes", &self.m_childShapes)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("rotation", &self.m_rotation)?;
            serializer.serialize_field("translation", &self.m_translation)?;
            serializer.serialize_array_field("childShapes", &self.m_childShapes)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_childShapes,
    m_rotation,
    m_translation,
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
            "childShapes" => Ok(__Field::m_childShapes),
            "rotation" => Ok(__Field::m_rotation),
            "translation" => Ok(__Field::m_translation),
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
pub(super) struct __hkpExtendedMeshShapeShapesSubpartVisitor<'de> {
    marker: core::marker::PhantomData<hkpExtendedMeshShapeShapesSubpart>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpExtendedMeshShapeShapesSubpartVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpExtendedMeshShapeShapesSubpart, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpExtendedMeshShapeShapesSubpart,
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
impl<'de> _serde::de::Visitor<'de> for __hkpExtendedMeshShapeShapesSubpartVisitor<'de> {
    type Value = hkpExtendedMeshShapeShapesSubpart;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpExtendedMeshShapeShapesSubpart",
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
        let mut m_childShapes: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_rotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_translation: _serde::__private::Option<Vector4> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_childShapes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childShapes",
                            ),
                        );
                    }
                    m_childShapes = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_rotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotation",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 8usize)?;
                    m_rotation = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_translation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "translation",
                            ),
                        );
                    }
                    m_translation = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
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
        let m_childShapes = match m_childShapes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childShapes"),
                );
            }
        };
        let m_rotation = match m_rotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotation"),
                );
            }
        };
        let m_translation = match m_translation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("translation"),
                );
            }
        };
        _serde::__private::Ok(hkpExtendedMeshShapeShapesSubpart {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_childShapes,
            m_rotation,
            m_translation,
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
        let parent = __hkpExtendedMeshShapeSubpartVisitor::visit_as_parent(&mut __map)?;
        let mut m_childShapes: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_rotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_translation: _serde::__private::Option<Vector4> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_childShapes => {
                        if _serde::__private::Option::is_some(&m_childShapes) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "childShapes",
                                ),
                            );
                        }
                        m_childShapes = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rotation => {
                        if _serde::__private::Option::is_some(&m_rotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotation",
                                ),
                            );
                        }
                        m_rotation = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_translation => {
                        if _serde::__private::Option::is_some(&m_translation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "translation",
                                ),
                            );
                        }
                        m_translation = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
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
        let m_childShapes = match m_childShapes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childShapes"),
                );
            }
        };
        let m_rotation = match m_rotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotation"),
                );
            }
        };
        let m_translation = match m_translation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("translation"),
                );
            }
        };
        _serde::__private::Ok(hkpExtendedMeshShapeShapesSubpart {
            __ptr,
            parent,
            m_childShapes,
            m_rotation,
            m_translation,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpExtendedMeshShapeShapesSubpart {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["childShapes", "rotation", "translation"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpExtendedMeshShapeShapesSubpart",
                FIELDS,
                __hkpExtendedMeshShapeShapesSubpartVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpExtendedMeshShapeShapesSubpart,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
