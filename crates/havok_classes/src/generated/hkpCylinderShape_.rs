use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCylinderShape`
/// -         version: `0`
/// -       signature: `0x3e463c3a`
/// -          size:  96(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCylinderShape {
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
    /// -          name: `cylRadius`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cylRadius: f32,
    /// # C++ Info
    /// -          name: `cylBaseRadiusFactorForHeightFieldCollisions`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cylBaseRadiusFactorForHeightFieldCollisions: f32,
    /// # C++ Info
    /// -          name: `vertexA`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vertexA: Vector4,
    /// # C++ Info
    /// -          name: `vertexB`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vertexB: Vector4,
    /// # C++ Info
    /// -          name: `perpendicular1`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_perpendicular1: Vector4,
    /// # C++ Info
    /// -          name: `perpendicular2`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_perpendicular2: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCylinderShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCylinderShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3e463c3a)
        }
    }
    impl _serde::Serialize for hkpCylinderShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3e463c3a)));
            let mut serializer = __serializer
                .serialize_struct("hkpCylinderShape", class_meta)?;
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
            serializer.serialize_field("cylRadius", &self.m_cylRadius)?;
            serializer
                .serialize_field(
                    "cylBaseRadiusFactorForHeightFieldCollisions",
                    &self.m_cylBaseRadiusFactorForHeightFieldCollisions,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("vertexA", &self.m_vertexA)?;
            serializer.serialize_field("vertexB", &self.m_vertexB)?;
            serializer.serialize_field("perpendicular1", &self.m_perpendicular1)?;
            serializer.serialize_field("perpendicular2", &self.m_perpendicular2)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_cylRadius,
    m_cylBaseRadiusFactorForHeightFieldCollisions,
    m_vertexA,
    m_vertexB,
    m_perpendicular1,
    m_perpendicular2,
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
            "cylRadius" => Ok(__Field::m_cylRadius),
            "cylBaseRadiusFactorForHeightFieldCollisions" => {
                Ok(__Field::m_cylBaseRadiusFactorForHeightFieldCollisions)
            }
            "vertexA" => Ok(__Field::m_vertexA),
            "vertexB" => Ok(__Field::m_vertexB),
            "perpendicular1" => Ok(__Field::m_perpendicular1),
            "perpendicular2" => Ok(__Field::m_perpendicular2),
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
pub(super) struct __hkpCylinderShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpCylinderShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpCylinderShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpCylinderShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpCylinderShape>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpCylinderShapeVisitor<'de> {
    type Value = hkpCylinderShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpCylinderShape")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_cylRadius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cylBaseRadiusFactorForHeightFieldCollisions: _serde::__private::Option<
            f32,
        > = _serde::__private::None;
        let mut m_vertexA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vertexB: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_perpendicular1: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_perpendicular2: _serde::__private::Option<Vector4> = _serde::__private::None;
        for i in 0..6usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_cylRadius) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cylRadius",
                            ),
                        );
                    }
                    m_cylRadius = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(
                        &m_cylBaseRadiusFactorForHeightFieldCollisions,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cylBaseRadiusFactorForHeightFieldCollisions",
                            ),
                        );
                    }
                    m_cylBaseRadiusFactorForHeightFieldCollisions = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_vertexA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("vertexA"),
                        );
                    }
                    __A::pad(&mut __map, 4usize, 0usize)?;
                    m_vertexA = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_vertexB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("vertexB"),
                        );
                    }
                    m_vertexB = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_perpendicular1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "perpendicular1",
                            ),
                        );
                    }
                    m_perpendicular1 = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_perpendicular2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "perpendicular2",
                            ),
                        );
                    }
                    m_perpendicular2 = _serde::__private::Some(
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
        let m_cylRadius = match m_cylRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cylRadius"),
                );
            }
        };
        let m_cylBaseRadiusFactorForHeightFieldCollisions = match m_cylBaseRadiusFactorForHeightFieldCollisions {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "cylBaseRadiusFactorForHeightFieldCollisions",
                    ),
                );
            }
        };
        let m_vertexA = match m_vertexA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexA"),
                );
            }
        };
        let m_vertexB = match m_vertexB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexB"),
                );
            }
        };
        let m_perpendicular1 = match m_perpendicular1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("perpendicular1"),
                );
            }
        };
        let m_perpendicular2 = match m_perpendicular2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("perpendicular2"),
                );
            }
        };
        _serde::__private::Ok(hkpCylinderShape {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_cylRadius,
            m_cylBaseRadiusFactorForHeightFieldCollisions,
            m_vertexA,
            m_vertexB,
            m_perpendicular1,
            m_perpendicular2,
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
        let mut m_cylRadius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cylBaseRadiusFactorForHeightFieldCollisions: _serde::__private::Option<
            f32,
        > = _serde::__private::None;
        let mut m_vertexA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vertexB: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_perpendicular1: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_perpendicular2: _serde::__private::Option<Vector4> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_cylRadius => {
                    if _serde::__private::Option::is_some(&m_cylRadius) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cylRadius",
                            ),
                        );
                    }
                    m_cylRadius = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_cylBaseRadiusFactorForHeightFieldCollisions => {
                    if _serde::__private::Option::is_some(
                        &m_cylBaseRadiusFactorForHeightFieldCollisions,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cylBaseRadiusFactorForHeightFieldCollisions",
                            ),
                        );
                    }
                    m_cylBaseRadiusFactorForHeightFieldCollisions = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_vertexA => {
                    if _serde::__private::Option::is_some(&m_vertexA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("vertexA"),
                        );
                    }
                    m_vertexA = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_vertexB => {
                    if _serde::__private::Option::is_some(&m_vertexB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("vertexB"),
                        );
                    }
                    m_vertexB = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_perpendicular1 => {
                    if _serde::__private::Option::is_some(&m_perpendicular1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "perpendicular1",
                            ),
                        );
                    }
                    m_perpendicular1 = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_perpendicular2 => {
                    if _serde::__private::Option::is_some(&m_perpendicular2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "perpendicular2",
                            ),
                        );
                    }
                    m_perpendicular2 = _serde::__private::Some(
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
        let m_cylRadius = match m_cylRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cylRadius"),
                );
            }
        };
        let m_cylBaseRadiusFactorForHeightFieldCollisions = match m_cylBaseRadiusFactorForHeightFieldCollisions {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "cylBaseRadiusFactorForHeightFieldCollisions",
                    ),
                );
            }
        };
        let m_vertexA = match m_vertexA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexA"),
                );
            }
        };
        let m_vertexB = match m_vertexB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexB"),
                );
            }
        };
        let m_perpendicular1 = match m_perpendicular1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("perpendicular1"),
                );
            }
        };
        let m_perpendicular2 = match m_perpendicular2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("perpendicular2"),
                );
            }
        };
        _serde::__private::Ok(hkpCylinderShape {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_cylRadius,
            m_cylBaseRadiusFactorForHeightFieldCollisions,
            m_vertexA,
            m_vertexB,
            m_perpendicular1,
            m_perpendicular2,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCylinderShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "cylRadius",
                "cylBaseRadiusFactorForHeightFieldCollisions",
                "vertexA",
                "vertexB",
                "perpendicular1",
                "perpendicular2",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCylinderShape",
                FIELDS,
                __hkpCylinderShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpCylinderShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
