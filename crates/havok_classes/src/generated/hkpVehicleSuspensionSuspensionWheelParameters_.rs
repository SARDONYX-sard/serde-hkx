use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleSuspensionSuspensionWheelParameters`
/// -         version: `0`
/// -       signature: `0x358bfe9c`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleSuspensionSuspensionWheelParameters {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `hardpointChassisSpace`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_hardpointChassisSpace: Vector4,
    /// # C++ Info
    /// -          name: `directionChassisSpace`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_directionChassisSpace: Vector4,
    /// # C++ Info
    /// -          name: `length`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_length: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleSuspensionSuspensionWheelParameters {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleSuspensionSuspensionWheelParameters"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x358bfe9c)
        }
    }
    impl _serde::Serialize for hkpVehicleSuspensionSuspensionWheelParameters {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x358bfe9c)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpVehicleSuspensionSuspensionWheelParameters",
                    class_meta,
                )?;
            serializer
                .serialize_field(
                    "hardpointChassisSpace",
                    &self.m_hardpointChassisSpace,
                )?;
            serializer
                .serialize_field(
                    "directionChassisSpace",
                    &self.m_directionChassisSpace,
                )?;
            serializer.serialize_field("length", &self.m_length)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_hardpointChassisSpace,
    m_directionChassisSpace,
    m_length,
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
            "hardpointChassisSpace" => Ok(__Field::m_hardpointChassisSpace),
            "directionChassisSpace" => Ok(__Field::m_directionChassisSpace),
            "length" => Ok(__Field::m_length),
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
pub(super) struct __hkpVehicleSuspensionSuspensionWheelParametersVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleSuspensionSuspensionWheelParameters>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleSuspensionSuspensionWheelParametersVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkpVehicleSuspensionSuspensionWheelParameters,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpVehicleSuspensionSuspensionWheelParameters,
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
for __hkpVehicleSuspensionSuspensionWheelParametersVisitor<'de> {
    type Value = hkpVehicleSuspensionSuspensionWheelParameters;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleSuspensionSuspensionWheelParameters",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_hardpointChassisSpace: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_directionChassisSpace: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_length: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_hardpointChassisSpace) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hardpointChassisSpace",
                            ),
                        );
                    }
                    m_hardpointChassisSpace = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_directionChassisSpace) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directionChassisSpace",
                            ),
                        );
                    }
                    m_directionChassisSpace = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_length) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("length"),
                        );
                    }
                    m_length = _serde::__private::Some(
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
        __A::pad(&mut __map, 12usize, 12usize)?;
        let m_hardpointChassisSpace = match m_hardpointChassisSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hardpointChassisSpace",
                    ),
                );
            }
        };
        let m_directionChassisSpace = match m_directionChassisSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "directionChassisSpace",
                    ),
                );
            }
        };
        let m_length = match m_length {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("length"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleSuspensionSuspensionWheelParameters {
            __ptr: __A::class_ptr(&mut __map),
            m_hardpointChassisSpace,
            m_directionChassisSpace,
            m_length,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_hardpointChassisSpace: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_directionChassisSpace: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_length: _serde::__private::Option<f32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_hardpointChassisSpace => {
                    if _serde::__private::Option::is_some(&m_hardpointChassisSpace) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hardpointChassisSpace",
                            ),
                        );
                    }
                    m_hardpointChassisSpace = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_directionChassisSpace => {
                    if _serde::__private::Option::is_some(&m_directionChassisSpace) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "directionChassisSpace",
                            ),
                        );
                    }
                    m_directionChassisSpace = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_length => {
                    if _serde::__private::Option::is_some(&m_length) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("length"),
                        );
                    }
                    m_length = _serde::__private::Some(
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
        let m_hardpointChassisSpace = match m_hardpointChassisSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hardpointChassisSpace",
                    ),
                );
            }
        };
        let m_directionChassisSpace = match m_directionChassisSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "directionChassisSpace",
                    ),
                );
            }
        };
        let m_length = match m_length {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("length"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleSuspensionSuspensionWheelParameters {
            __ptr: __A::class_ptr(&mut __map),
            m_hardpointChassisSpace,
            m_directionChassisSpace,
            m_length,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkpVehicleSuspensionSuspensionWheelParameters {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "hardpointChassisSpace",
                "directionChassisSpace",
                "length",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleSuspensionSuspensionWheelParameters",
                FIELDS,
                __hkpVehicleSuspensionSuspensionWheelParametersVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleSuspensionSuspensionWheelParameters,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
