use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaQuantizedAnimationTrackCompressionParams`
/// - version: `0`
/// - signature: `0xf7d64649`
/// - size: ` 16`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaQuantizedAnimationTrackCompressionParams {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `rotationTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_rotationTolerance: f32,
    /// # C++ Info
    /// - name: `translationTolerance`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_translationTolerance: f32,
    /// # C++ Info
    /// - name: `scaleTolerance`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_scaleTolerance: f32,
    /// # C++ Info
    /// - name: `floatingTolerance`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_floatingTolerance: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaQuantizedAnimationTrackCompressionParams {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaQuantizedAnimationTrackCompressionParams"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf7d64649)
        }
    }
    impl _serde::Serialize for hkaQuantizedAnimationTrackCompressionParams {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf7d64649)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaQuantizedAnimationTrackCompressionParams",
                    class_meta,
                )?;
            serializer.serialize_field("rotationTolerance", &self.m_rotationTolerance)?;
            serializer
                .serialize_field("translationTolerance", &self.m_translationTolerance)?;
            serializer.serialize_field("scaleTolerance", &self.m_scaleTolerance)?;
            serializer.serialize_field("floatingTolerance", &self.m_floatingTolerance)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_rotationTolerance,
    m_translationTolerance,
    m_scaleTolerance,
    m_floatingTolerance,
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
            "rotationTolerance" => Ok(__Field::m_rotationTolerance),
            "translationTolerance" => Ok(__Field::m_translationTolerance),
            "scaleTolerance" => Ok(__Field::m_scaleTolerance),
            "floatingTolerance" => Ok(__Field::m_floatingTolerance),
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
pub(super) struct __hkaQuantizedAnimationTrackCompressionParamsVisitor<'de> {
    marker: core::marker::PhantomData<hkaQuantizedAnimationTrackCompressionParams>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaQuantizedAnimationTrackCompressionParamsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkaQuantizedAnimationTrackCompressionParams,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkaQuantizedAnimationTrackCompressionParams,
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
impl<'de> _serde::de::Visitor<'de>
for __hkaQuantizedAnimationTrackCompressionParamsVisitor<'de> {
    type Value = hkaQuantizedAnimationTrackCompressionParams;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkaQuantizedAnimationTrackCompressionParams",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_rotationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_translationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_scaleTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_floatingTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_rotationTolerance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotationTolerance",
                            ),
                        );
                    }
                    m_rotationTolerance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_translationTolerance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "translationTolerance",
                            ),
                        );
                    }
                    m_translationTolerance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_scaleTolerance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "scaleTolerance",
                            ),
                        );
                    }
                    m_scaleTolerance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_floatingTolerance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "floatingTolerance",
                            ),
                        );
                    }
                    m_floatingTolerance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
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
        let m_rotationTolerance = match m_rotationTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationTolerance"),
                );
            }
        };
        let m_translationTolerance = match m_translationTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "translationTolerance",
                    ),
                );
            }
        };
        let m_scaleTolerance = match m_scaleTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("scaleTolerance"),
                );
            }
        };
        let m_floatingTolerance = match m_floatingTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatingTolerance"),
                );
            }
        };
        _serde::__private::Ok(hkaQuantizedAnimationTrackCompressionParams {
            __ptr,
            m_rotationTolerance,
            m_translationTolerance,
            m_scaleTolerance,
            m_floatingTolerance,
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
        let mut m_rotationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_translationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_scaleTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_floatingTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_rotationTolerance => {
                        if _serde::__private::Option::is_some(&m_rotationTolerance) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotationTolerance",
                                ),
                            );
                        }
                        m_rotationTolerance = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_translationTolerance => {
                        if _serde::__private::Option::is_some(&m_translationTolerance) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "translationTolerance",
                                ),
                            );
                        }
                        m_translationTolerance = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_scaleTolerance => {
                        if _serde::__private::Option::is_some(&m_scaleTolerance) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "scaleTolerance",
                                ),
                            );
                        }
                        m_scaleTolerance = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_floatingTolerance => {
                        if _serde::__private::Option::is_some(&m_floatingTolerance) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "floatingTolerance",
                                ),
                            );
                        }
                        m_floatingTolerance = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
        let m_rotationTolerance = match m_rotationTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationTolerance"),
                );
            }
        };
        let m_translationTolerance = match m_translationTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "translationTolerance",
                    ),
                );
            }
        };
        let m_scaleTolerance = match m_scaleTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("scaleTolerance"),
                );
            }
        };
        let m_floatingTolerance = match m_floatingTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("floatingTolerance"),
                );
            }
        };
        _serde::__private::Ok(hkaQuantizedAnimationTrackCompressionParams {
            __ptr,
            m_rotationTolerance,
            m_translationTolerance,
            m_scaleTolerance,
            m_floatingTolerance,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaQuantizedAnimationTrackCompressionParams {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "rotationTolerance",
                "translationTolerance",
                "scaleTolerance",
                "floatingTolerance",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaQuantizedAnimationTrackCompressionParams",
                FIELDS,
                __hkaQuantizedAnimationTrackCompressionParamsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaQuantizedAnimationTrackCompressionParams,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
