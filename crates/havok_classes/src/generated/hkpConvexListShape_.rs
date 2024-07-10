use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConvexListShape`
/// -         version: `0`
/// -       signature: `0x450b26e8`
/// -          size:  80(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConvexListShape {
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
    /// -          name: `minDistanceToUseConvexHullForGetClosestPoints`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minDistanceToUseConvexHullForGetClosestPoints: f32,
    /// # C++ Info
    /// -          name: `aabbHalfExtents`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_aabbHalfExtents: Vector4,
    /// # C++ Info
    /// -          name: `aabbCenter`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_aabbCenter: Vector4,
    /// # C++ Info
    /// -          name: `useCachedAabb`(ctype: `hkBool`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useCachedAabb: bool,
    /// # C++ Info
    /// -          name: `childShapes`(ctype: `hkArray<hkpConvexShape*>`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childShapes: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConvexListShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConvexListShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x450b26e8)
        }
    }
    impl _serde::Serialize for hkpConvexListShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x450b26e8)));
            let mut serializer = __serializer
                .serialize_struct("hkpConvexListShape", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field(
                    "minDistanceToUseConvexHullForGetClosestPoints",
                    &self.m_minDistanceToUseConvexHullForGetClosestPoints,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_field("aabbHalfExtents", &self.m_aabbHalfExtents)?;
            serializer.serialize_field("aabbCenter", &self.m_aabbCenter)?;
            serializer.serialize_field("useCachedAabb", &self.m_useCachedAabb)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_meta_field("childShapes", &self.m_childShapes)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_array_field("childShapes", &self.m_childShapes)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_minDistanceToUseConvexHullForGetClosestPoints,
    m_aabbHalfExtents,
    m_aabbCenter,
    m_useCachedAabb,
    m_childShapes,
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
            "minDistanceToUseConvexHullForGetClosestPoints" => {
                Ok(__Field::m_minDistanceToUseConvexHullForGetClosestPoints)
            }
            "aabbHalfExtents" => Ok(__Field::m_aabbHalfExtents),
            "aabbCenter" => Ok(__Field::m_aabbCenter),
            "useCachedAabb" => Ok(__Field::m_useCachedAabb),
            "childShapes" => Ok(__Field::m_childShapes),
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
pub(super) struct __hkpConvexListShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpConvexListShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpConvexListShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpConvexListShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpConvexListShape>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpConvexListShapeVisitor<'de> {
    type Value = hkpConvexListShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpConvexListShape")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_minDistanceToUseConvexHullForGetClosestPoints: _serde::__private::Option<
            f32,
        > = _serde::__private::None;
        let mut m_aabbHalfExtents: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_aabbCenter: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_useCachedAabb: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_childShapes: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(
                        &m_minDistanceToUseConvexHullForGetClosestPoints,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minDistanceToUseConvexHullForGetClosestPoints",
                            ),
                        );
                    }
                    m_minDistanceToUseConvexHullForGetClosestPoints = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
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
                    __A::pad(&mut __map, 4usize, 12usize)?;
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
                    if _serde::__private::Option::is_some(&m_useCachedAabb) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "useCachedAabb",
                            ),
                        );
                    }
                    m_useCachedAabb = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_childShapes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childShapes",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 7usize)?;
                    m_childShapes = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        __A::pad(&mut __map, 0usize, 8usize)?;
        let m_minDistanceToUseConvexHullForGetClosestPoints = match m_minDistanceToUseConvexHullForGetClosestPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minDistanceToUseConvexHullForGetClosestPoints",
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
        let m_useCachedAabb = match m_useCachedAabb {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("useCachedAabb"),
                );
            }
        };
        let m_childShapes = match m_childShapes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childShapes"),
                );
            }
        };
        _serde::__private::Ok(hkpConvexListShape {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_minDistanceToUseConvexHullForGetClosestPoints,
            m_aabbHalfExtents,
            m_aabbCenter,
            m_useCachedAabb,
            m_childShapes,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpConvexShapeVisitor::visit_as_parent(&mut __map)?;
        let mut m_minDistanceToUseConvexHullForGetClosestPoints: _serde::__private::Option<
            f32,
        > = _serde::__private::None;
        let mut m_aabbHalfExtents: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_aabbCenter: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_useCachedAabb: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_childShapes: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_minDistanceToUseConvexHullForGetClosestPoints => {
                    if _serde::__private::Option::is_some(
                        &m_minDistanceToUseConvexHullForGetClosestPoints,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minDistanceToUseConvexHullForGetClosestPoints",
                            ),
                        );
                    }
                    m_minDistanceToUseConvexHullForGetClosestPoints = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
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
                __Field::m_useCachedAabb => {
                    if _serde::__private::Option::is_some(&m_useCachedAabb) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "useCachedAabb",
                            ),
                        );
                    }
                    m_useCachedAabb = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
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
                _ => {}
            }
        }
        let m_minDistanceToUseConvexHullForGetClosestPoints = match m_minDistanceToUseConvexHullForGetClosestPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minDistanceToUseConvexHullForGetClosestPoints",
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
        let m_useCachedAabb = match m_useCachedAabb {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("useCachedAabb"),
                );
            }
        };
        let m_childShapes = match m_childShapes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childShapes"),
                );
            }
        };
        _serde::__private::Ok(hkpConvexListShape {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_minDistanceToUseConvexHullForGetClosestPoints,
            m_aabbHalfExtents,
            m_aabbCenter,
            m_useCachedAabb,
            m_childShapes,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConvexListShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "minDistanceToUseConvexHullForGetClosestPoints",
                "aabbHalfExtents",
                "aabbCenter",
                "useCachedAabb",
                "childShapes",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConvexListShape",
                FIELDS,
                __hkpConvexListShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpConvexListShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
