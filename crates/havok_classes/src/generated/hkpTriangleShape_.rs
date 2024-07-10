use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTriangleShape`
/// -         version: `0`
/// -       signature: `0x95ad1a25`
/// -          size:  96(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTriangleShape {
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
    /// -          name: `weldingInfo`(ctype: `hkUint16`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_weldingInfo: u16,
    /// # C++ Info
    /// -          name: `weldingType`(ctype: `enum WeldingType`)
    /// -        offset:  22(x86)/ 42(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_weldingType: WeldingType,
    /// # C++ Info
    /// -          name: `isExtruded`(ctype: `hkUint8`)
    /// -        offset:  23(x86)/ 43(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isExtruded: u8,
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
    /// -          name: `vertexC`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vertexC: Vector4,
    /// # C++ Info
    /// -          name: `extrusion`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_extrusion: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpTriangleShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTriangleShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x95ad1a25)
        }
    }
    impl _serde::Serialize for hkpTriangleShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x95ad1a25)));
            let mut serializer = __serializer
                .serialize_struct("hkpTriangleShape", class_meta)?;
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
            serializer.serialize_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.serialize_field("weldingType", &self.m_weldingType)?;
            serializer.serialize_field("isExtruded", &self.m_isExtruded)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("vertexA", &self.m_vertexA)?;
            serializer.serialize_field("vertexB", &self.m_vertexB)?;
            serializer.serialize_field("vertexC", &self.m_vertexC)?;
            serializer.serialize_field("extrusion", &self.m_extrusion)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_weldingInfo,
    m_weldingType,
    m_isExtruded,
    m_vertexA,
    m_vertexB,
    m_vertexC,
    m_extrusion,
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
            "weldingInfo" => Ok(__Field::m_weldingInfo),
            "weldingType" => Ok(__Field::m_weldingType),
            "isExtruded" => Ok(__Field::m_isExtruded),
            "vertexA" => Ok(__Field::m_vertexA),
            "vertexB" => Ok(__Field::m_vertexB),
            "vertexC" => Ok(__Field::m_vertexC),
            "extrusion" => Ok(__Field::m_extrusion),
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
pub(super) struct __hkpTriangleShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpTriangleShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpTriangleShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpTriangleShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpTriangleShape>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpTriangleShapeVisitor<'de> {
    type Value = hkpTriangleShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpTriangleShape")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_weldingInfo: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_weldingType: _serde::__private::Option<WeldingType> = _serde::__private::None;
        let mut m_isExtruded: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_vertexA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vertexB: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vertexC: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_extrusion: _serde::__private::Option<Vector4> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_weldingInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "weldingInfo",
                            ),
                        );
                    }
                    m_weldingInfo = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
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
                2usize => {
                    if _serde::__private::Option::is_some(&m_isExtruded) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isExtruded",
                            ),
                        );
                    }
                    m_isExtruded = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_vertexA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("vertexA"),
                        );
                    }
                    __A::pad(&mut __map, 8usize, 4usize)?;
                    m_vertexA = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
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
                5usize => {
                    if _serde::__private::Option::is_some(&m_vertexC) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("vertexC"),
                        );
                    }
                    m_vertexC = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
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
                _ => {}
            }
        }
        let m_weldingInfo = match m_weldingInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weldingInfo"),
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
        let m_isExtruded = match m_isExtruded {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isExtruded"),
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
        let m_vertexC = match m_vertexC {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexC"),
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
        _serde::__private::Ok(hkpTriangleShape {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_weldingInfo,
            m_weldingType,
            m_isExtruded,
            m_vertexA,
            m_vertexB,
            m_vertexC,
            m_extrusion,
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
        let mut m_weldingInfo: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_weldingType: _serde::__private::Option<WeldingType> = _serde::__private::None;
        let mut m_isExtruded: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_vertexA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vertexB: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vertexC: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_extrusion: _serde::__private::Option<Vector4> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_weldingInfo => {
                    if _serde::__private::Option::is_some(&m_weldingInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "weldingInfo",
                            ),
                        );
                    }
                    m_weldingInfo = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
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
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_isExtruded => {
                    if _serde::__private::Option::is_some(&m_isExtruded) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isExtruded",
                            ),
                        );
                    }
                    m_isExtruded = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
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
                __Field::m_vertexC => {
                    if _serde::__private::Option::is_some(&m_vertexC) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("vertexC"),
                        );
                    }
                    m_vertexC = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
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
                _ => {}
            }
        }
        let m_weldingInfo = match m_weldingInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weldingInfo"),
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
        let m_isExtruded = match m_isExtruded {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isExtruded"),
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
        let m_vertexC = match m_vertexC {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexC"),
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
        _serde::__private::Ok(hkpTriangleShape {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_weldingInfo,
            m_weldingType,
            m_isExtruded,
            m_vertexA,
            m_vertexB,
            m_vertexC,
            m_extrusion,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpTriangleShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "weldingInfo",
                "weldingType",
                "isExtruded",
                "vertexA",
                "vertexB",
                "vertexC",
                "extrusion",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpTriangleShape",
                FIELDS,
                __hkpTriangleShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpTriangleShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
