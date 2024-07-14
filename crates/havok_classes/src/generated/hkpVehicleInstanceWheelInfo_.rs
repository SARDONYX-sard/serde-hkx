use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleInstanceWheelInfo`
/// -         version: `1`
/// -       signature: `0x99f693f0`
/// -          size: 224(x86)/224(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleInstanceWheelInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `contactPoint`(ctype: `struct hkContactPoint`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_contactPoint: hkContactPoint,
    /// # C++ Info
    /// -          name: `contactFriction`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_contactFriction: f32,
    /// # C++ Info
    /// -          name: `contactBody`(ctype: `void*`)
    /// -        offset:  36(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_contactBody: Pointer,
    /// # C++ Info
    /// -          name: `contactShapeKey`(ctype: `hkUint32[8]`)
    /// -        offset:  40(x86)/ 48(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_contactShapeKey: [u32; 8usize],
    /// # C++ Info
    /// -          name: `hardPointWs`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_hardPointWs: Vector4,
    /// # C++ Info
    /// -          name: `rayEndPointWs`(ctype: `hkVector4`)
    /// -        offset:  96(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rayEndPointWs: Vector4,
    /// # C++ Info
    /// -          name: `currentSuspensionLength`(ctype: `hkReal`)
    /// -        offset: 112(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_currentSuspensionLength: f32,
    /// # C++ Info
    /// -          name: `suspensionDirectionWs`(ctype: `hkVector4`)
    /// -        offset: 128(x86)/128(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_suspensionDirectionWs: Vector4,
    /// # C++ Info
    /// -          name: `spinAxisChassisSpace`(ctype: `hkVector4`)
    /// -        offset: 144(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_spinAxisChassisSpace: Vector4,
    /// # C++ Info
    /// -          name: `spinAxisWs`(ctype: `hkVector4`)
    /// -        offset: 160(x86)/160(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_spinAxisWs: Vector4,
    /// # C++ Info
    /// -          name: `steeringOrientationChassisSpace`(ctype: `hkQuaternion`)
    /// -        offset: 176(x86)/176(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_steeringOrientationChassisSpace: Quaternion,
    /// # C++ Info
    /// -          name: `spinVelocity`(ctype: `hkReal`)
    /// -        offset: 192(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_spinVelocity: f32,
    /// # C++ Info
    /// -          name: `spinAngle`(ctype: `hkReal`)
    /// -        offset: 196(x86)/196(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_spinAngle: f32,
    /// # C++ Info
    /// -          name: `skidEnergyDensity`(ctype: `hkReal`)
    /// -        offset: 200(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_skidEnergyDensity: f32,
    /// # C++ Info
    /// -          name: `sideForce`(ctype: `hkReal`)
    /// -        offset: 204(x86)/204(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sideForce: f32,
    /// # C++ Info
    /// -          name: `forwardSlipVelocity`(ctype: `hkReal`)
    /// -        offset: 208(x86)/208(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_forwardSlipVelocity: f32,
    /// # C++ Info
    /// -          name: `sideSlipVelocity`(ctype: `hkReal`)
    /// -        offset: 212(x86)/212(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sideSlipVelocity: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleInstanceWheelInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleInstanceWheelInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x99f693f0)
        }
    }
    impl _serde::Serialize for hkpVehicleInstanceWheelInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x99f693f0)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleInstanceWheelInfo", class_meta)?;
            serializer.serialize_field("contactPoint", &self.m_contactPoint)?;
            serializer.serialize_field("contactFriction", &self.m_contactFriction)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("contactBody", &self.m_contactBody)?;
            serializer
                .serialize_field("contactShapeKey", &self.m_contactShapeKey.as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("hardPointWs", &self.m_hardPointWs)?;
            serializer.serialize_field("rayEndPointWs", &self.m_rayEndPointWs)?;
            serializer
                .serialize_field(
                    "currentSuspensionLength",
                    &self.m_currentSuspensionLength,
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer
                .serialize_field(
                    "suspensionDirectionWs",
                    &self.m_suspensionDirectionWs,
                )?;
            serializer
                .serialize_field("spinAxisChassisSpace", &self.m_spinAxisChassisSpace)?;
            serializer.serialize_field("spinAxisWs", &self.m_spinAxisWs)?;
            serializer
                .serialize_field(
                    "steeringOrientationChassisSpace",
                    &self.m_steeringOrientationChassisSpace,
                )?;
            serializer.serialize_field("spinVelocity", &self.m_spinVelocity)?;
            serializer.serialize_field("spinAngle", &self.m_spinAngle)?;
            serializer.serialize_field("skidEnergyDensity", &self.m_skidEnergyDensity)?;
            serializer.serialize_field("sideForce", &self.m_sideForce)?;
            serializer
                .serialize_field("forwardSlipVelocity", &self.m_forwardSlipVelocity)?;
            serializer.serialize_field("sideSlipVelocity", &self.m_sideSlipVelocity)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_contactPoint,
    m_contactFriction,
    m_contactBody,
    m_contactShapeKey,
    m_hardPointWs,
    m_rayEndPointWs,
    m_currentSuspensionLength,
    m_suspensionDirectionWs,
    m_spinAxisChassisSpace,
    m_spinAxisWs,
    m_steeringOrientationChassisSpace,
    m_spinVelocity,
    m_spinAngle,
    m_skidEnergyDensity,
    m_sideForce,
    m_forwardSlipVelocity,
    m_sideSlipVelocity,
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
            "contactPoint" => Ok(__Field::m_contactPoint),
            "contactFriction" => Ok(__Field::m_contactFriction),
            "contactShapeKey" => Ok(__Field::m_contactShapeKey),
            "hardPointWs" => Ok(__Field::m_hardPointWs),
            "rayEndPointWs" => Ok(__Field::m_rayEndPointWs),
            "currentSuspensionLength" => Ok(__Field::m_currentSuspensionLength),
            "suspensionDirectionWs" => Ok(__Field::m_suspensionDirectionWs),
            "spinAxisChassisSpace" => Ok(__Field::m_spinAxisChassisSpace),
            "spinAxisWs" => Ok(__Field::m_spinAxisWs),
            "steeringOrientationChassisSpace" => {
                Ok(__Field::m_steeringOrientationChassisSpace)
            }
            "spinVelocity" => Ok(__Field::m_spinVelocity),
            "spinAngle" => Ok(__Field::m_spinAngle),
            "skidEnergyDensity" => Ok(__Field::m_skidEnergyDensity),
            "sideForce" => Ok(__Field::m_sideForce),
            "forwardSlipVelocity" => Ok(__Field::m_forwardSlipVelocity),
            "sideSlipVelocity" => Ok(__Field::m_sideSlipVelocity),
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
pub(super) struct __hkpVehicleInstanceWheelInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleInstanceWheelInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleInstanceWheelInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpVehicleInstanceWheelInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpVehicleInstanceWheelInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpVehicleInstanceWheelInfoVisitor<'de> {
    type Value = hkpVehicleInstanceWheelInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleInstanceWheelInfo",
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
        let mut m_contactPoint: _serde::__private::Option<hkContactPoint> = _serde::__private::None;
        let mut m_contactFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_contactBody: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_contactShapeKey: _serde::__private::Option<[u32; 8usize]> = _serde::__private::None;
        let mut m_hardPointWs: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rayEndPointWs: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_currentSuspensionLength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_suspensionDirectionWs: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_spinAxisChassisSpace: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_spinAxisWs: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_steeringOrientationChassisSpace: _serde::__private::Option<
            Quaternion,
        > = _serde::__private::None;
        let mut m_spinVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_spinAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_skidEnergyDensity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_sideForce: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_forwardSlipVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_sideSlipVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..17usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_contactPoint) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactPoint",
                            ),
                        );
                    }
                    m_contactPoint = _serde::__private::Some(
                        match __A::next_value::<hkContactPoint>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_contactFriction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactFriction",
                            ),
                        );
                    }
                    m_contactFriction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_contactBody) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactBody",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_contactBody = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_contactShapeKey) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactShapeKey",
                            ),
                        );
                    }
                    m_contactShapeKey = _serde::__private::Some(
                        match __A::next_value::<[u32; 8usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_hardPointWs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hardPointWs",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 8usize, 0usize)?;
                    m_hardPointWs = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_rayEndPointWs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rayEndPointWs",
                            ),
                        );
                    }
                    m_rayEndPointWs = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_currentSuspensionLength) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentSuspensionLength",
                            ),
                        );
                    }
                    m_currentSuspensionLength = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_suspensionDirectionWs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "suspensionDirectionWs",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 12usize)?;
                    m_suspensionDirectionWs = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_spinAxisChassisSpace) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "spinAxisChassisSpace",
                            ),
                        );
                    }
                    m_spinAxisChassisSpace = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_spinAxisWs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "spinAxisWs",
                            ),
                        );
                    }
                    m_spinAxisWs = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(
                        &m_steeringOrientationChassisSpace,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "steeringOrientationChassisSpace",
                            ),
                        );
                    }
                    m_steeringOrientationChassisSpace = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_spinVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "spinVelocity",
                            ),
                        );
                    }
                    m_spinVelocity = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_spinAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "spinAngle",
                            ),
                        );
                    }
                    m_spinAngle = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_skidEnergyDensity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "skidEnergyDensity",
                            ),
                        );
                    }
                    m_skidEnergyDensity = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                14usize => {
                    if _serde::__private::Option::is_some(&m_sideForce) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sideForce",
                            ),
                        );
                    }
                    m_sideForce = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_forwardSlipVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "forwardSlipVelocity",
                            ),
                        );
                    }
                    m_forwardSlipVelocity = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                16usize => {
                    if _serde::__private::Option::is_some(&m_sideSlipVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sideSlipVelocity",
                            ),
                        );
                    }
                    m_sideSlipVelocity = _serde::__private::Some(
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
        __A::pad(&mut __map, 8usize, 8usize)?;
        let m_contactPoint = match m_contactPoint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactPoint"),
                );
            }
        };
        let m_contactFriction = match m_contactFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactFriction"),
                );
            }
        };
        let m_contactBody = match m_contactBody {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactBody"),
                );
            }
        };
        let m_contactShapeKey = match m_contactShapeKey {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactShapeKey"),
                );
            }
        };
        let m_hardPointWs = match m_hardPointWs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hardPointWs"),
                );
            }
        };
        let m_rayEndPointWs = match m_rayEndPointWs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rayEndPointWs"),
                );
            }
        };
        let m_currentSuspensionLength = match m_currentSuspensionLength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "currentSuspensionLength",
                    ),
                );
            }
        };
        let m_suspensionDirectionWs = match m_suspensionDirectionWs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "suspensionDirectionWs",
                    ),
                );
            }
        };
        let m_spinAxisChassisSpace = match m_spinAxisChassisSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "spinAxisChassisSpace",
                    ),
                );
            }
        };
        let m_spinAxisWs = match m_spinAxisWs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spinAxisWs"),
                );
            }
        };
        let m_steeringOrientationChassisSpace = match m_steeringOrientationChassisSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "steeringOrientationChassisSpace",
                    ),
                );
            }
        };
        let m_spinVelocity = match m_spinVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spinVelocity"),
                );
            }
        };
        let m_spinAngle = match m_spinAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spinAngle"),
                );
            }
        };
        let m_skidEnergyDensity = match m_skidEnergyDensity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skidEnergyDensity"),
                );
            }
        };
        let m_sideForce = match m_sideForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sideForce"),
                );
            }
        };
        let m_forwardSlipVelocity = match m_forwardSlipVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "forwardSlipVelocity",
                    ),
                );
            }
        };
        let m_sideSlipVelocity = match m_sideSlipVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sideSlipVelocity"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleInstanceWheelInfo {
            __ptr,
            m_contactPoint,
            m_contactFriction,
            m_contactBody,
            m_contactShapeKey,
            m_hardPointWs,
            m_rayEndPointWs,
            m_currentSuspensionLength,
            m_suspensionDirectionWs,
            m_spinAxisChassisSpace,
            m_spinAxisWs,
            m_steeringOrientationChassisSpace,
            m_spinVelocity,
            m_spinAngle,
            m_skidEnergyDensity,
            m_sideForce,
            m_forwardSlipVelocity,
            m_sideSlipVelocity,
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
        let mut m_contactPoint: _serde::__private::Option<hkContactPoint> = _serde::__private::None;
        let mut m_contactFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_contactShapeKey: _serde::__private::Option<[u32; 8usize]> = _serde::__private::None;
        let mut m_hardPointWs: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rayEndPointWs: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_currentSuspensionLength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_suspensionDirectionWs: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_spinAxisChassisSpace: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_spinAxisWs: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_steeringOrientationChassisSpace: _serde::__private::Option<
            Quaternion,
        > = _serde::__private::None;
        let mut m_spinVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_spinAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_skidEnergyDensity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_sideForce: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_forwardSlipVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_sideSlipVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..16usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_contactPoint => {
                        if _serde::__private::Option::is_some(&m_contactPoint) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contactPoint",
                                ),
                            );
                        }
                        m_contactPoint = _serde::__private::Some(
                            match __A::next_value::<hkContactPoint>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_contactFriction => {
                        if _serde::__private::Option::is_some(&m_contactFriction) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contactFriction",
                                ),
                            );
                        }
                        m_contactFriction = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_contactShapeKey => {
                        if _serde::__private::Option::is_some(&m_contactShapeKey) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contactShapeKey",
                                ),
                            );
                        }
                        m_contactShapeKey = _serde::__private::Some(
                            match __A::next_value::<[u32; 8usize]>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_hardPointWs => {
                        if _serde::__private::Option::is_some(&m_hardPointWs) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "hardPointWs",
                                ),
                            );
                        }
                        m_hardPointWs = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rayEndPointWs => {
                        if _serde::__private::Option::is_some(&m_rayEndPointWs) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rayEndPointWs",
                                ),
                            );
                        }
                        m_rayEndPointWs = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_currentSuspensionLength => {
                        if _serde::__private::Option::is_some(
                            &m_currentSuspensionLength,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "currentSuspensionLength",
                                ),
                            );
                        }
                        m_currentSuspensionLength = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_suspensionDirectionWs => {
                        if _serde::__private::Option::is_some(&m_suspensionDirectionWs) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "suspensionDirectionWs",
                                ),
                            );
                        }
                        m_suspensionDirectionWs = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_spinAxisChassisSpace => {
                        if _serde::__private::Option::is_some(&m_spinAxisChassisSpace) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "spinAxisChassisSpace",
                                ),
                            );
                        }
                        m_spinAxisChassisSpace = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_spinAxisWs => {
                        if _serde::__private::Option::is_some(&m_spinAxisWs) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "spinAxisWs",
                                ),
                            );
                        }
                        m_spinAxisWs = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_steeringOrientationChassisSpace => {
                        if _serde::__private::Option::is_some(
                            &m_steeringOrientationChassisSpace,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "steeringOrientationChassisSpace",
                                ),
                            );
                        }
                        m_steeringOrientationChassisSpace = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_spinVelocity => {
                        if _serde::__private::Option::is_some(&m_spinVelocity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "spinVelocity",
                                ),
                            );
                        }
                        m_spinVelocity = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_spinAngle => {
                        if _serde::__private::Option::is_some(&m_spinAngle) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "spinAngle",
                                ),
                            );
                        }
                        m_spinAngle = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_skidEnergyDensity => {
                        if _serde::__private::Option::is_some(&m_skidEnergyDensity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "skidEnergyDensity",
                                ),
                            );
                        }
                        m_skidEnergyDensity = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_sideForce => {
                        if _serde::__private::Option::is_some(&m_sideForce) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sideForce",
                                ),
                            );
                        }
                        m_sideForce = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_forwardSlipVelocity => {
                        if _serde::__private::Option::is_some(&m_forwardSlipVelocity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "forwardSlipVelocity",
                                ),
                            );
                        }
                        m_forwardSlipVelocity = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_sideSlipVelocity => {
                        if _serde::__private::Option::is_some(&m_sideSlipVelocity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sideSlipVelocity",
                                ),
                            );
                        }
                        m_sideSlipVelocity = _serde::__private::Some(
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
        let m_contactPoint = match m_contactPoint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactPoint"),
                );
            }
        };
        let m_contactFriction = match m_contactFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactFriction"),
                );
            }
        };
        let m_contactShapeKey = match m_contactShapeKey {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactShapeKey"),
                );
            }
        };
        let m_hardPointWs = match m_hardPointWs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hardPointWs"),
                );
            }
        };
        let m_rayEndPointWs = match m_rayEndPointWs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rayEndPointWs"),
                );
            }
        };
        let m_currentSuspensionLength = match m_currentSuspensionLength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "currentSuspensionLength",
                    ),
                );
            }
        };
        let m_suspensionDirectionWs = match m_suspensionDirectionWs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "suspensionDirectionWs",
                    ),
                );
            }
        };
        let m_spinAxisChassisSpace = match m_spinAxisChassisSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "spinAxisChassisSpace",
                    ),
                );
            }
        };
        let m_spinAxisWs = match m_spinAxisWs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spinAxisWs"),
                );
            }
        };
        let m_steeringOrientationChassisSpace = match m_steeringOrientationChassisSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "steeringOrientationChassisSpace",
                    ),
                );
            }
        };
        let m_spinVelocity = match m_spinVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spinVelocity"),
                );
            }
        };
        let m_spinAngle = match m_spinAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spinAngle"),
                );
            }
        };
        let m_skidEnergyDensity = match m_skidEnergyDensity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skidEnergyDensity"),
                );
            }
        };
        let m_sideForce = match m_sideForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sideForce"),
                );
            }
        };
        let m_forwardSlipVelocity = match m_forwardSlipVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "forwardSlipVelocity",
                    ),
                );
            }
        };
        let m_sideSlipVelocity = match m_sideSlipVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sideSlipVelocity"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleInstanceWheelInfo {
            __ptr,
            m_contactPoint,
            m_contactFriction,
            m_contactShapeKey,
            m_hardPointWs,
            m_rayEndPointWs,
            m_currentSuspensionLength,
            m_suspensionDirectionWs,
            m_spinAxisChassisSpace,
            m_spinAxisWs,
            m_steeringOrientationChassisSpace,
            m_spinVelocity,
            m_spinAngle,
            m_skidEnergyDensity,
            m_sideForce,
            m_forwardSlipVelocity,
            m_sideSlipVelocity,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleInstanceWheelInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "contactPoint",
                "contactFriction",
                "contactBody",
                "contactShapeKey",
                "hardPointWs",
                "rayEndPointWs",
                "currentSuspensionLength",
                "suspensionDirectionWs",
                "spinAxisChassisSpace",
                "spinAxisWs",
                "steeringOrientationChassisSpace",
                "spinVelocity",
                "spinAngle",
                "skidEnergyDensity",
                "sideForce",
                "forwardSlipVelocity",
                "sideSlipVelocity",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleInstanceWheelInfo",
                FIELDS,
                __hkpVehicleInstanceWheelInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleInstanceWheelInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
