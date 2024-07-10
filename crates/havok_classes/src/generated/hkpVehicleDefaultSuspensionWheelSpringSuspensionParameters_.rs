use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters`
/// -         version: `0`
/// -       signature: `0x7be5bed1`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `strength`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_strength: f32,
    /// # C++ Info
    /// -          name: `dampingCompression`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dampingCompression: f32,
    /// # C++ Info
    /// -          name: `dampingRelaxation`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dampingRelaxation: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass
    for hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7be5bed1)
        }
    }
    impl _serde::Serialize
    for hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7be5bed1)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters",
                    class_meta,
                )?;
            serializer.serialize_field("strength", &self.m_strength)?;
            serializer
                .serialize_field("dampingCompression", &self.m_dampingCompression)?;
            serializer.serialize_field("dampingRelaxation", &self.m_dampingRelaxation)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_strength,
    m_dampingCompression,
    m_dampingRelaxation,
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
            "strength" => Ok(__Field::m_strength),
            "dampingCompression" => Ok(__Field::m_dampingCompression),
            "dampingRelaxation" => Ok(__Field::m_dampingRelaxation),
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
pub(super) struct __hkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor<
    'de,
> {
    marker: core::marker::PhantomData<
        hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters,
    >,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters,
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
for __hkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor<'de> {
    type Value = hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_strength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_dampingCompression: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_dampingRelaxation: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_strength) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "strength",
                            ),
                        );
                    }
                    m_strength = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_dampingCompression) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dampingCompression",
                            ),
                        );
                    }
                    m_dampingCompression = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_dampingRelaxation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dampingRelaxation",
                            ),
                        );
                    }
                    m_dampingRelaxation = _serde::__private::Some(
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
        let m_strength = match m_strength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("strength"),
                );
            }
        };
        let m_dampingCompression = match m_dampingCompression {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "dampingCompression",
                    ),
                );
            }
        };
        let m_dampingRelaxation = match m_dampingRelaxation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dampingRelaxation"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
            __ptr: __A::class_ptr(&mut __map),
            m_strength,
            m_dampingCompression,
            m_dampingRelaxation,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_strength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_dampingCompression: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_dampingRelaxation: _serde::__private::Option<f32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_strength => {
                    if _serde::__private::Option::is_some(&m_strength) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "strength",
                            ),
                        );
                    }
                    m_strength = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_dampingCompression => {
                    if _serde::__private::Option::is_some(&m_dampingCompression) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dampingCompression",
                            ),
                        );
                    }
                    m_dampingCompression = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_dampingRelaxation => {
                    if _serde::__private::Option::is_some(&m_dampingRelaxation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dampingRelaxation",
                            ),
                        );
                    }
                    m_dampingRelaxation = _serde::__private::Some(
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
        let m_strength = match m_strength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("strength"),
                );
            }
        };
        let m_dampingCompression = match m_dampingCompression {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "dampingCompression",
                    ),
                );
            }
        };
        let m_dampingRelaxation = match m_dampingRelaxation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dampingRelaxation"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
            __ptr: __A::class_ptr(&mut __map),
            m_strength,
            m_dampingCompression,
            m_dampingRelaxation,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "strength",
                "dampingCompression",
                "dampingRelaxation",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters",
                FIELDS,
                __hkpVehicleDefaultSuspensionWheelSpringSuspensionParametersVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
