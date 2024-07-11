use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultAnalogDriverInput`
/// -         version: `0`
/// -       signature: `0x123a5d50`
/// -          size:  24(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultAnalogDriverInput {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleDriverInput,
    /// # C++ Info
    /// -          name: `slopeChangePointX`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_slopeChangePointX: f32,
    /// # C++ Info
    /// -          name: `initialSlope`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_initialSlope: f32,
    /// # C++ Info
    /// -          name: `deadZone`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_deadZone: f32,
    /// # C++ Info
    /// -          name: `autoReverse`(ctype: `hkBool`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_autoReverse: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultAnalogDriverInput {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultAnalogDriverInput"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x123a5d50)
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultAnalogDriverInput {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x123a5d50)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultAnalogDriverInput", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("slopeChangePointX", &self.m_slopeChangePointX)?;
            serializer.serialize_field("initialSlope", &self.m_initialSlope)?;
            serializer.serialize_field("deadZone", &self.m_deadZone)?;
            serializer.serialize_field("autoReverse", &self.m_autoReverse)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_slopeChangePointX,
    m_initialSlope,
    m_deadZone,
    m_autoReverse,
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
            "slopeChangePointX" => Ok(__Field::m_slopeChangePointX),
            "initialSlope" => Ok(__Field::m_initialSlope),
            "deadZone" => Ok(__Field::m_deadZone),
            "autoReverse" => Ok(__Field::m_autoReverse),
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
pub(super) struct __hkpVehicleDefaultAnalogDriverInputVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleDefaultAnalogDriverInput>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleDefaultAnalogDriverInputVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpVehicleDefaultAnalogDriverInput, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpVehicleDefaultAnalogDriverInput,
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
impl<'de> _serde::de::Visitor<'de> for __hkpVehicleDefaultAnalogDriverInputVisitor<'de> {
    type Value = hkpVehicleDefaultAnalogDriverInput;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleDefaultAnalogDriverInput",
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
        let mut m_slopeChangePointX: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_initialSlope: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_deadZone: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_autoReverse: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_slopeChangePointX) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "slopeChangePointX",
                            ),
                        );
                    }
                    m_slopeChangePointX = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_initialSlope) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "initialSlope",
                            ),
                        );
                    }
                    m_initialSlope = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_deadZone) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "deadZone",
                            ),
                        );
                    }
                    m_deadZone = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_autoReverse) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "autoReverse",
                            ),
                        );
                    }
                    m_autoReverse = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
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
        __A::pad(&mut __map, 3usize, 3usize)?;
        let m_slopeChangePointX = match m_slopeChangePointX {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("slopeChangePointX"),
                );
            }
        };
        let m_initialSlope = match m_initialSlope {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initialSlope"),
                );
            }
        };
        let m_deadZone = match m_deadZone {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("deadZone"),
                );
            }
        };
        let m_autoReverse = match m_autoReverse {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("autoReverse"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultAnalogDriverInput {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_slopeChangePointX,
            m_initialSlope,
            m_deadZone,
            m_autoReverse,
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
        let parent = __hkpVehicleDriverInputVisitor::visit_as_parent(&mut __map)?;
        let mut m_slopeChangePointX: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_initialSlope: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_deadZone: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_autoReverse: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_slopeChangePointX => {
                        if _serde::__private::Option::is_some(&m_slopeChangePointX) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "slopeChangePointX",
                                ),
                            );
                        }
                        m_slopeChangePointX = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_initialSlope => {
                        if _serde::__private::Option::is_some(&m_initialSlope) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "initialSlope",
                                ),
                            );
                        }
                        m_initialSlope = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_deadZone => {
                        if _serde::__private::Option::is_some(&m_deadZone) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "deadZone",
                                ),
                            );
                        }
                        m_deadZone = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_autoReverse => {
                        if _serde::__private::Option::is_some(&m_autoReverse) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "autoReverse",
                                ),
                            );
                        }
                        m_autoReverse = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
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
        let m_slopeChangePointX = match m_slopeChangePointX {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("slopeChangePointX"),
                );
            }
        };
        let m_initialSlope = match m_initialSlope {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initialSlope"),
                );
            }
        };
        let m_deadZone = match m_deadZone {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("deadZone"),
                );
            }
        };
        let m_autoReverse = match m_autoReverse {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("autoReverse"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultAnalogDriverInput {
            __ptr,
            parent,
            m_slopeChangePointX,
            m_initialSlope,
            m_deadZone,
            m_autoReverse,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleDefaultAnalogDriverInput {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "slopeChangePointX",
                "initialSlope",
                "deadZone",
                "autoReverse",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDefaultAnalogDriverInput",
                FIELDS,
                __hkpVehicleDefaultAnalogDriverInputVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleDefaultAnalogDriverInput,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
