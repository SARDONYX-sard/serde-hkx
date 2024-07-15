use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCharacterControllerControlData`
/// - version: `0`
/// - signature: `0x5b6c03d9`
/// - size: ` 32`(x86)/` 32`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterControllerControlData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `desiredVelocity`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_desiredVelocity: Vector4,
    /// # C++ Info
    /// - name: `verticalGain`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_verticalGain: f32,
    /// # C++ Info
    /// - name: `horizontalCatchUpGain`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_horizontalCatchUpGain: f32,
    /// # C++ Info
    /// - name: `maxVerticalSeparation`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxVerticalSeparation: f32,
    /// # C++ Info
    /// - name: `maxHorizontalSeparation`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxHorizontalSeparation: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCharacterControllerControlData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterControllerControlData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5b6c03d9)
        }
    }
    impl _serde::Serialize for hkbCharacterControllerControlData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5b6c03d9)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterControllerControlData", class_meta)?;
            serializer.serialize_field("desiredVelocity", &self.m_desiredVelocity)?;
            serializer.serialize_field("verticalGain", &self.m_verticalGain)?;
            serializer
                .serialize_field(
                    "horizontalCatchUpGain",
                    &self.m_horizontalCatchUpGain,
                )?;
            serializer
                .serialize_field(
                    "maxVerticalSeparation",
                    &self.m_maxVerticalSeparation,
                )?;
            serializer
                .serialize_field(
                    "maxHorizontalSeparation",
                    &self.m_maxHorizontalSeparation,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_desiredVelocity,
    m_verticalGain,
    m_horizontalCatchUpGain,
    m_maxVerticalSeparation,
    m_maxHorizontalSeparation,
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
            "desiredVelocity" => Ok(__Field::m_desiredVelocity),
            "verticalGain" => Ok(__Field::m_verticalGain),
            "horizontalCatchUpGain" => Ok(__Field::m_horizontalCatchUpGain),
            "maxVerticalSeparation" => Ok(__Field::m_maxVerticalSeparation),
            "maxHorizontalSeparation" => Ok(__Field::m_maxHorizontalSeparation),
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
pub(super) struct __hkbCharacterControllerControlDataVisitor<'de> {
    marker: core::marker::PhantomData<hkbCharacterControllerControlData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbCharacterControllerControlDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbCharacterControllerControlData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbCharacterControllerControlData,
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
impl<'de> _serde::de::Visitor<'de> for __hkbCharacterControllerControlDataVisitor<'de> {
    type Value = hkbCharacterControllerControlData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbCharacterControllerControlData",
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
        let mut m_desiredVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_verticalGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_horizontalCatchUpGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxVerticalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxHorizontalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_desiredVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "desiredVelocity",
                            ),
                        );
                    }
                    m_desiredVelocity = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_verticalGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "verticalGain",
                            ),
                        );
                    }
                    m_verticalGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_horizontalCatchUpGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "horizontalCatchUpGain",
                            ),
                        );
                    }
                    m_horizontalCatchUpGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_maxVerticalSeparation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxVerticalSeparation",
                            ),
                        );
                    }
                    m_maxVerticalSeparation = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_maxHorizontalSeparation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxHorizontalSeparation",
                            ),
                        );
                    }
                    m_maxHorizontalSeparation = _serde::__private::Some(
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
        let m_desiredVelocity = match m_desiredVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("desiredVelocity"),
                );
            }
        };
        let m_verticalGain = match m_verticalGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("verticalGain"),
                );
            }
        };
        let m_horizontalCatchUpGain = match m_horizontalCatchUpGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "horizontalCatchUpGain",
                    ),
                );
            }
        };
        let m_maxVerticalSeparation = match m_maxVerticalSeparation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxVerticalSeparation",
                    ),
                );
            }
        };
        let m_maxHorizontalSeparation = match m_maxHorizontalSeparation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxHorizontalSeparation",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterControllerControlData {
            __ptr,
            m_desiredVelocity,
            m_verticalGain,
            m_horizontalCatchUpGain,
            m_maxVerticalSeparation,
            m_maxHorizontalSeparation,
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
        let mut m_desiredVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_verticalGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_horizontalCatchUpGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxVerticalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxHorizontalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..5usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_desiredVelocity => {
                        if _serde::__private::Option::is_some(&m_desiredVelocity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "desiredVelocity",
                                ),
                            );
                        }
                        m_desiredVelocity = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_verticalGain => {
                        if _serde::__private::Option::is_some(&m_verticalGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "verticalGain",
                                ),
                            );
                        }
                        m_verticalGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_horizontalCatchUpGain => {
                        if _serde::__private::Option::is_some(&m_horizontalCatchUpGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "horizontalCatchUpGain",
                                ),
                            );
                        }
                        m_horizontalCatchUpGain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxVerticalSeparation => {
                        if _serde::__private::Option::is_some(&m_maxVerticalSeparation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxVerticalSeparation",
                                ),
                            );
                        }
                        m_maxVerticalSeparation = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxHorizontalSeparation => {
                        if _serde::__private::Option::is_some(
                            &m_maxHorizontalSeparation,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxHorizontalSeparation",
                                ),
                            );
                        }
                        m_maxHorizontalSeparation = _serde::__private::Some(
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
        let m_desiredVelocity = match m_desiredVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("desiredVelocity"),
                );
            }
        };
        let m_verticalGain = match m_verticalGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("verticalGain"),
                );
            }
        };
        let m_horizontalCatchUpGain = match m_horizontalCatchUpGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "horizontalCatchUpGain",
                    ),
                );
            }
        };
        let m_maxVerticalSeparation = match m_maxVerticalSeparation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxVerticalSeparation",
                    ),
                );
            }
        };
        let m_maxHorizontalSeparation = match m_maxHorizontalSeparation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxHorizontalSeparation",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterControllerControlData {
            __ptr,
            m_desiredVelocity,
            m_verticalGain,
            m_horizontalCatchUpGain,
            m_maxVerticalSeparation,
            m_maxHorizontalSeparation,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCharacterControllerControlData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "desiredVelocity",
                "verticalGain",
                "horizontalCatchUpGain",
                "maxVerticalSeparation",
                "maxHorizontalSeparation",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterControllerControlData",
                FIELDS,
                __hkbCharacterControllerControlDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbCharacterControllerControlData,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
