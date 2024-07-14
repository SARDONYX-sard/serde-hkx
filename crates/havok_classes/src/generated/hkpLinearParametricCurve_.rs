use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpLinearParametricCurve`
/// -         version: `0`
/// -       signature: `0xd7b3be03`
/// -          size:  64(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpLinearParametricCurve {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpParametricCurve,
    /// # C++ Info
    /// -          name: `smoothingFactor`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_smoothingFactor: f32,
    /// # C++ Info
    /// -          name: `closedLoop`(ctype: `hkBool`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_closedLoop: bool,
    /// # C++ Info
    /// -          name: `dirNotParallelToTangentAlongWholePath`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_dirNotParallelToTangentAlongWholePath: Vector4,
    /// # C++ Info
    /// -          name: `points`(ctype: `hkArray<hkVector4>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_points: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `distance`(ctype: `hkArray<hkReal>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_distance: Vec<f32>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpLinearParametricCurve {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpLinearParametricCurve"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd7b3be03)
        }
    }
    impl _serde::Serialize for hkpLinearParametricCurve {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd7b3be03)));
            let mut serializer = __serializer
                .serialize_struct("hkpLinearParametricCurve", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("smoothingFactor", &self.m_smoothingFactor)?;
            serializer.serialize_field("closedLoop", &self.m_closedLoop)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer
                .serialize_field(
                    "dirNotParallelToTangentAlongWholePath",
                    &self.m_dirNotParallelToTangentAlongWholePath,
                )?;
            serializer.serialize_array_meta_field("points", &self.m_points)?;
            serializer.serialize_array_meta_field("distance", &self.m_distance)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_array_field("points", &self.m_points)?;
            serializer.serialize_array_field("distance", &self.m_distance)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_smoothingFactor,
    m_closedLoop,
    m_dirNotParallelToTangentAlongWholePath,
    m_points,
    m_distance,
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
            "smoothingFactor" => Ok(__Field::m_smoothingFactor),
            "closedLoop" => Ok(__Field::m_closedLoop),
            "dirNotParallelToTangentAlongWholePath" => {
                Ok(__Field::m_dirNotParallelToTangentAlongWholePath)
            }
            "points" => Ok(__Field::m_points),
            "distance" => Ok(__Field::m_distance),
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
pub(super) struct __hkpLinearParametricCurveVisitor<'de> {
    marker: core::marker::PhantomData<hkpLinearParametricCurve>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpLinearParametricCurveVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpLinearParametricCurve, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpLinearParametricCurve>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpLinearParametricCurveVisitor<'de> {
    type Value = hkpLinearParametricCurve;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpLinearParametricCurve")
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
        let mut m_smoothingFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_closedLoop: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_dirNotParallelToTangentAlongWholePath: _serde::__private::Option<
            Vector4,
        > = _serde::__private::None;
        let mut m_points: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_distance: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_smoothingFactor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "smoothingFactor",
                            ),
                        );
                    }
                    m_smoothingFactor = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_closedLoop) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "closedLoop",
                            ),
                        );
                    }
                    m_closedLoop = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(
                        &m_dirNotParallelToTangentAlongWholePath,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dirNotParallelToTangentAlongWholePath",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 11usize)?;
                    m_dirNotParallelToTangentAlongWholePath = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_points) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("points"),
                        );
                    }
                    m_points = _serde::__private::Some(
                        match __A::next_value::<Vec<Vector4>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_distance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "distance",
                            ),
                        );
                    }
                    m_distance = _serde::__private::Some(
                        match __A::next_value::<Vec<f32>>(&mut __map) {
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
        let m_smoothingFactor = match m_smoothingFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("smoothingFactor"),
                );
            }
        };
        let m_closedLoop = match m_closedLoop {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("closedLoop"),
                );
            }
        };
        let m_dirNotParallelToTangentAlongWholePath = match m_dirNotParallelToTangentAlongWholePath {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "dirNotParallelToTangentAlongWholePath",
                    ),
                );
            }
        };
        let m_points = match m_points {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("points"),
                );
            }
        };
        let m_distance = match m_distance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("distance"),
                );
            }
        };
        _serde::__private::Ok(hkpLinearParametricCurve {
            __ptr,
            parent,
            m_smoothingFactor,
            m_closedLoop,
            m_dirNotParallelToTangentAlongWholePath,
            m_points,
            m_distance,
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
        let parent = __hkpParametricCurveVisitor::visit_as_parent(&mut __map)?;
        let mut m_smoothingFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_closedLoop: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_dirNotParallelToTangentAlongWholePath: _serde::__private::Option<
            Vector4,
        > = _serde::__private::None;
        let mut m_points: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_distance: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        for _ in 0..5usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_smoothingFactor => {
                        if _serde::__private::Option::is_some(&m_smoothingFactor) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "smoothingFactor",
                                ),
                            );
                        }
                        m_smoothingFactor = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_closedLoop => {
                        if _serde::__private::Option::is_some(&m_closedLoop) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "closedLoop",
                                ),
                            );
                        }
                        m_closedLoop = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_dirNotParallelToTangentAlongWholePath => {
                        if _serde::__private::Option::is_some(
                            &m_dirNotParallelToTangentAlongWholePath,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "dirNotParallelToTangentAlongWholePath",
                                ),
                            );
                        }
                        m_dirNotParallelToTangentAlongWholePath = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_points => {
                        if _serde::__private::Option::is_some(&m_points) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("points"),
                            );
                        }
                        m_points = _serde::__private::Some(
                            match __A::next_value::<Vec<Vector4>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_distance => {
                        if _serde::__private::Option::is_some(&m_distance) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "distance",
                                ),
                            );
                        }
                        m_distance = _serde::__private::Some(
                            match __A::next_value::<Vec<f32>>(&mut __map) {
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
        let m_smoothingFactor = match m_smoothingFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("smoothingFactor"),
                );
            }
        };
        let m_closedLoop = match m_closedLoop {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("closedLoop"),
                );
            }
        };
        let m_dirNotParallelToTangentAlongWholePath = match m_dirNotParallelToTangentAlongWholePath {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "dirNotParallelToTangentAlongWholePath",
                    ),
                );
            }
        };
        let m_points = match m_points {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("points"),
                );
            }
        };
        let m_distance = match m_distance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("distance"),
                );
            }
        };
        _serde::__private::Ok(hkpLinearParametricCurve {
            __ptr,
            parent,
            m_smoothingFactor,
            m_closedLoop,
            m_dirNotParallelToTangentAlongWholePath,
            m_points,
            m_distance,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpLinearParametricCurve {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "smoothingFactor",
                "closedLoop",
                "dirNotParallelToTangentAlongWholePath",
                "points",
                "distance",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpLinearParametricCurve",
                FIELDS,
                __hkpLinearParametricCurveVisitor {
                    marker: _serde::__private::PhantomData::<hkpLinearParametricCurve>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
