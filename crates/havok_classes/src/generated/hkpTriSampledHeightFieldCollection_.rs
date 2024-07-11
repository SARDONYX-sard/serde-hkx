use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTriSampledHeightFieldCollection`
/// -         version: `0`
/// -       signature: `0xc291ddde`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTriSampledHeightFieldCollection {
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
    /// -          name: `heightfield`(ctype: `struct hkpSampledHeightFieldShape*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_heightfield: Pointer,
    /// # C++ Info
    /// -          name: `childSize`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_childSize: i32,
    /// # C++ Info
    /// -          name: `radius`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_radius: f32,
    /// # C++ Info
    /// -          name: `weldingInfo`(ctype: `hkArray<hkUint16>`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_weldingInfo: Vec<u16>,
    /// # C++ Info
    /// -          name: `triangleExtrusion`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_triangleExtrusion: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpTriSampledHeightFieldCollection {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTriSampledHeightFieldCollection"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc291ddde)
        }
    }
    impl _serde::Serialize for hkpTriSampledHeightFieldCollection {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc291ddde)));
            let mut serializer = __serializer
                .serialize_struct("hkpTriSampledHeightFieldCollection", class_meta)?;
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
            serializer.serialize_field("heightfield", &self.m_heightfield)?;
            serializer.skip_field("childSize", &self.m_childSize)?;
            serializer.serialize_field("radius", &self.m_radius)?;
            serializer.serialize_array_meta_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.serialize_field("triangleExtrusion", &self.m_triangleExtrusion)?;
            serializer.serialize_array_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_heightfield,
    m_childSize,
    m_radius,
    m_weldingInfo,
    m_triangleExtrusion,
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
            "heightfield" => Ok(__Field::m_heightfield),
            "radius" => Ok(__Field::m_radius),
            "weldingInfo" => Ok(__Field::m_weldingInfo),
            "triangleExtrusion" => Ok(__Field::m_triangleExtrusion),
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
pub(super) struct __hkpTriSampledHeightFieldCollectionVisitor<'de> {
    marker: core::marker::PhantomData<hkpTriSampledHeightFieldCollection>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpTriSampledHeightFieldCollectionVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpTriSampledHeightFieldCollection, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpTriSampledHeightFieldCollection,
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
impl<'de> _serde::de::Visitor<'de> for __hkpTriSampledHeightFieldCollectionVisitor<'de> {
    type Value = hkpTriSampledHeightFieldCollection;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpTriSampledHeightFieldCollection",
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
        let mut m_heightfield: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_childSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_weldingInfo: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_triangleExtrusion: _serde::__private::Option<Vector4> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_heightfield) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "heightfield",
                            ),
                        );
                    }
                    m_heightfield = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_childSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childSize",
                            ),
                        );
                    }
                    m_childSize = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
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
                3usize => {
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
                4usize => {
                    if _serde::__private::Option::is_some(&m_triangleExtrusion) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "triangleExtrusion",
                            ),
                        );
                    }
                    m_triangleExtrusion = _serde::__private::Some(
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
        let m_heightfield = match m_heightfield {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("heightfield"),
                );
            }
        };
        let m_childSize = match m_childSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childSize"),
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
        let m_weldingInfo = match m_weldingInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weldingInfo"),
                );
            }
        };
        let m_triangleExtrusion = match m_triangleExtrusion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangleExtrusion"),
                );
            }
        };
        _serde::__private::Ok(hkpTriSampledHeightFieldCollection {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_heightfield,
            m_childSize,
            m_radius,
            m_weldingInfo,
            m_triangleExtrusion,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpShapeCollectionVisitor::visit_as_parent(&mut __map)?;
        let mut m_heightfield: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_weldingInfo: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_triangleExtrusion: _serde::__private::Option<Vector4> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_heightfield => {
                        if _serde::__private::Option::is_some(&m_heightfield) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "heightfield",
                                ),
                            );
                        }
                        m_heightfield = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
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
                    __Field::m_triangleExtrusion => {
                        if _serde::__private::Option::is_some(&m_triangleExtrusion) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "triangleExtrusion",
                                ),
                            );
                        }
                        m_triangleExtrusion = _serde::__private::Some(
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
        let m_heightfield = match m_heightfield {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("heightfield"),
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
        let m_weldingInfo = match m_weldingInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weldingInfo"),
                );
            }
        };
        let m_triangleExtrusion = match m_triangleExtrusion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangleExtrusion"),
                );
            }
        };
        _serde::__private::Ok(hkpTriSampledHeightFieldCollection {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_heightfield,
            m_radius,
            m_weldingInfo,
            m_triangleExtrusion,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpTriSampledHeightFieldCollection {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "heightfield",
                "childSize",
                "radius",
                "weldingInfo",
                "triangleExtrusion",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpTriSampledHeightFieldCollection",
                FIELDS,
                __hkpTriSampledHeightFieldCollectionVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpTriSampledHeightFieldCollection,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
