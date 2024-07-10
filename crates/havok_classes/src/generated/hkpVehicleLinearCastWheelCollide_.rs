use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleLinearCastWheelCollide`
/// -         version: `0`
/// -       signature: `0xc59399d0`
/// -          size:  52(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleLinearCastWheelCollide {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleWheelCollide,
    /// # C++ Info
    /// -          name: `wheelCollisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelCollisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `wheelStates`(ctype: `hkArray<struct hkpVehicleLinearCastWheelCollideWheelState>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wheelStates: Vec<hkpVehicleLinearCastWheelCollideWheelState>,
    /// # C++ Info
    /// -          name: `rejectChassisListener`(ctype: `struct hkpRejectChassisListener`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 24(x86_64)
    ///
    pub m_rejectChassisListener: hkpRejectChassisListener,
    /// # C++ Info
    /// -          name: `maxExtraPenetration`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxExtraPenetration: f32,
    /// # C++ Info
    /// -          name: `startPointTolerance`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_startPointTolerance: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleLinearCastWheelCollide {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleLinearCastWheelCollide"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc59399d0)
        }
    }
    impl _serde::Serialize for hkpVehicleLinearCastWheelCollide {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc59399d0)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleLinearCastWheelCollide", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("alreadyUsed", &self.parent.m_alreadyUsed)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_field(
                    "wheelCollisionFilterInfo",
                    &self.m_wheelCollisionFilterInfo,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("wheelStates", &self.m_wheelStates)?;
            serializer
                .serialize_field(
                    "rejectChassisListener",
                    &self.m_rejectChassisListener,
                )?;
            serializer
                .serialize_field("maxExtraPenetration", &self.m_maxExtraPenetration)?;
            serializer
                .serialize_field("startPointTolerance", &self.m_startPointTolerance)?;
            serializer.serialize_array_field("wheelStates", &self.m_wheelStates)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_wheelCollisionFilterInfo,
    m_wheelStates,
    m_rejectChassisListener,
    m_maxExtraPenetration,
    m_startPointTolerance,
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
            "wheelCollisionFilterInfo" => Ok(__Field::m_wheelCollisionFilterInfo),
            "wheelStates" => Ok(__Field::m_wheelStates),
            "rejectChassisListener" => Ok(__Field::m_rejectChassisListener),
            "maxExtraPenetration" => Ok(__Field::m_maxExtraPenetration),
            "startPointTolerance" => Ok(__Field::m_startPointTolerance),
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
pub(super) struct __hkpVehicleLinearCastWheelCollideVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleLinearCastWheelCollide>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleLinearCastWheelCollideVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpVehicleLinearCastWheelCollide, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpVehicleLinearCastWheelCollide,
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
impl<'de> _serde::de::Visitor<'de> for __hkpVehicleLinearCastWheelCollideVisitor<'de> {
    type Value = hkpVehicleLinearCastWheelCollide;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleLinearCastWheelCollide",
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
        let mut m_wheelCollisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_wheelStates: _serde::__private::Option<
            Vec<hkpVehicleLinearCastWheelCollideWheelState>,
        > = _serde::__private::None;
        let mut m_rejectChassisListener: _serde::__private::Option<
            hkpRejectChassisListener,
        > = _serde::__private::None;
        let mut m_maxExtraPenetration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_startPointTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_wheelCollisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelCollisionFilterInfo",
                            ),
                        );
                    }
                    m_wheelCollisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_wheelStates) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelStates",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_wheelStates = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpVehicleLinearCastWheelCollideWheelState>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_rejectChassisListener) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rejectChassisListener",
                            ),
                        );
                    }
                    m_rejectChassisListener = _serde::__private::Some(
                        match __A::next_value::<hkpRejectChassisListener>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_maxExtraPenetration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxExtraPenetration",
                            ),
                        );
                    }
                    m_maxExtraPenetration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_startPointTolerance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startPointTolerance",
                            ),
                        );
                    }
                    m_startPointTolerance = _serde::__private::Some(
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
        let m_wheelCollisionFilterInfo = match m_wheelCollisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelCollisionFilterInfo",
                    ),
                );
            }
        };
        let m_wheelStates = match m_wheelStates {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelStates"),
                );
            }
        };
        let m_rejectChassisListener = match m_rejectChassisListener {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "rejectChassisListener",
                    ),
                );
            }
        };
        let m_maxExtraPenetration = match m_maxExtraPenetration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxExtraPenetration",
                    ),
                );
            }
        };
        let m_startPointTolerance = match m_startPointTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "startPointTolerance",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleLinearCastWheelCollide {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_wheelCollisionFilterInfo,
            m_wheelStates,
            m_rejectChassisListener,
            m_maxExtraPenetration,
            m_startPointTolerance,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpVehicleWheelCollideVisitor::visit_as_parent(&mut __map)?;
        let mut m_wheelCollisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_wheelStates: _serde::__private::Option<
            Vec<hkpVehicleLinearCastWheelCollideWheelState>,
        > = _serde::__private::None;
        let mut m_rejectChassisListener: _serde::__private::Option<
            hkpRejectChassisListener,
        > = _serde::__private::None;
        let mut m_maxExtraPenetration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_startPointTolerance: _serde::__private::Option<f32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_wheelCollisionFilterInfo => {
                    if _serde::__private::Option::is_some(&m_wheelCollisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelCollisionFilterInfo",
                            ),
                        );
                    }
                    m_wheelCollisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_wheelStates => {
                    if _serde::__private::Option::is_some(&m_wheelStates) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelStates",
                            ),
                        );
                    }
                    m_wheelStates = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpVehicleLinearCastWheelCollideWheelState>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_rejectChassisListener => {
                    if _serde::__private::Option::is_some(&m_rejectChassisListener) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rejectChassisListener",
                            ),
                        );
                    }
                    m_rejectChassisListener = _serde::__private::Some(
                        match __A::next_value::<hkpRejectChassisListener>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_maxExtraPenetration => {
                    if _serde::__private::Option::is_some(&m_maxExtraPenetration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxExtraPenetration",
                            ),
                        );
                    }
                    m_maxExtraPenetration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_startPointTolerance => {
                    if _serde::__private::Option::is_some(&m_startPointTolerance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startPointTolerance",
                            ),
                        );
                    }
                    m_startPointTolerance = _serde::__private::Some(
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
        let m_wheelCollisionFilterInfo = match m_wheelCollisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelCollisionFilterInfo",
                    ),
                );
            }
        };
        let m_wheelStates = match m_wheelStates {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelStates"),
                );
            }
        };
        let m_rejectChassisListener = match m_rejectChassisListener {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "rejectChassisListener",
                    ),
                );
            }
        };
        let m_maxExtraPenetration = match m_maxExtraPenetration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxExtraPenetration",
                    ),
                );
            }
        };
        let m_startPointTolerance = match m_startPointTolerance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "startPointTolerance",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleLinearCastWheelCollide {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_wheelCollisionFilterInfo,
            m_wheelStates,
            m_rejectChassisListener,
            m_maxExtraPenetration,
            m_startPointTolerance,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleLinearCastWheelCollide {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "wheelCollisionFilterInfo",
                "wheelStates",
                "rejectChassisListener",
                "maxExtraPenetration",
                "startPointTolerance",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleLinearCastWheelCollide",
                FIELDS,
                __hkpVehicleLinearCastWheelCollideVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleLinearCastWheelCollide,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
