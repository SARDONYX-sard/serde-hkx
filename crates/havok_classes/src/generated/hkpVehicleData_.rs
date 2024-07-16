use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleData`
/// - version: `1`
/// - signature: `0x173feb43`
/// - size: `416`(x86)/`416`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// - name: `gravity`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_gravity: Vector4,
    /// # C++ Info
    /// - name: `numWheels`(ctype: `hkInt8`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_numWheels: i8,
    /// # C++ Info
    /// - name: `chassisOrientation`(ctype: `hkRotation`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_chassisOrientation: Rotation,
    /// # C++ Info
    /// - name: `torqueRollFactor`(ctype: `hkReal`)
    /// - offset: ` 96`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_torqueRollFactor: f32,
    /// # C++ Info
    /// - name: `torquePitchFactor`(ctype: `hkReal`)
    /// - offset: `100`(x86)/`100`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_torquePitchFactor: f32,
    /// # C++ Info
    /// - name: `torqueYawFactor`(ctype: `hkReal`)
    /// - offset: `104`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_torqueYawFactor: f32,
    /// # C++ Info
    /// - name: `extraTorqueFactor`(ctype: `hkReal`)
    /// - offset: `108`(x86)/`108`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_extraTorqueFactor: f32,
    /// # C++ Info
    /// - name: `maxVelocityForPositionalFriction`(ctype: `hkReal`)
    /// - offset: `112`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxVelocityForPositionalFriction: f32,
    /// # C++ Info
    /// - name: `chassisUnitInertiaYaw`(ctype: `hkReal`)
    /// - offset: `116`(x86)/`116`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_chassisUnitInertiaYaw: f32,
    /// # C++ Info
    /// - name: `chassisUnitInertiaRoll`(ctype: `hkReal`)
    /// - offset: `120`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_chassisUnitInertiaRoll: f32,
    /// # C++ Info
    /// - name: `chassisUnitInertiaPitch`(ctype: `hkReal`)
    /// - offset: `124`(x86)/`124`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_chassisUnitInertiaPitch: f32,
    /// # C++ Info
    /// - name: `frictionEqualizer`(ctype: `hkReal`)
    /// - offset: `128`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_frictionEqualizer: f32,
    /// # C++ Info
    /// - name: `normalClippingAngleCos`(ctype: `hkReal`)
    /// - offset: `132`(x86)/`132`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_normalClippingAngleCos: f32,
    /// # C++ Info
    /// - name: `maxFrictionSolverMassRatio`(ctype: `hkReal`)
    /// - offset: `136`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxFrictionSolverMassRatio: f32,
    /// # C++ Info
    /// - name: `wheelParams`(ctype: `hkArray<struct hkpVehicleDataWheelComponentParams>`)
    /// - offset: `140`(x86)/`144`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_wheelParams: Vec<hkpVehicleDataWheelComponentParams>,
    /// # C++ Info
    /// - name: `numWheelsPerAxle`(ctype: `hkArray<hkInt8>`)
    /// - offset: `152`(x86)/`160`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_numWheelsPerAxle: Vec<i8>,
    /// # C++ Info
    /// - name: `frictionDescription`(ctype: `struct hkpVehicleFrictionDescription`)
    /// - offset: `164`(x86)/`176`(x86_64)
    /// - type_size: `208`(x86)/`208`(x86_64)
    pub m_frictionDescription: hkpVehicleFrictionDescription,
    /// # C++ Info
    /// - name: `chassisFrictionInertiaInvDiag`(ctype: `hkVector4`)
    /// - offset: `384`(x86)/`384`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_chassisFrictionInertiaInvDiag: Vector4,
    /// # C++ Info
    /// - name: `alreadyInitialised`(ctype: `hkBool`)
    /// - offset: `400`(x86)/`400`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_alreadyInitialised: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x173feb43)
        }
    }
    impl _serde::Serialize for hkpVehicleData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x173feb43)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("gravity", &self.m_gravity)?;
            serializer.serialize_field("numWheels", &self.m_numWheels)?;
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer
                .serialize_field("chassisOrientation", &self.m_chassisOrientation)?;
            serializer.serialize_field("torqueRollFactor", &self.m_torqueRollFactor)?;
            serializer.serialize_field("torquePitchFactor", &self.m_torquePitchFactor)?;
            serializer.serialize_field("torqueYawFactor", &self.m_torqueYawFactor)?;
            serializer.serialize_field("extraTorqueFactor", &self.m_extraTorqueFactor)?;
            serializer
                .serialize_field(
                    "maxVelocityForPositionalFriction",
                    &self.m_maxVelocityForPositionalFriction,
                )?;
            serializer
                .serialize_field(
                    "chassisUnitInertiaYaw",
                    &self.m_chassisUnitInertiaYaw,
                )?;
            serializer
                .serialize_field(
                    "chassisUnitInertiaRoll",
                    &self.m_chassisUnitInertiaRoll,
                )?;
            serializer
                .serialize_field(
                    "chassisUnitInertiaPitch",
                    &self.m_chassisUnitInertiaPitch,
                )?;
            serializer.serialize_field("frictionEqualizer", &self.m_frictionEqualizer)?;
            serializer
                .serialize_field(
                    "normalClippingAngleCos",
                    &self.m_normalClippingAngleCos,
                )?;
            serializer
                .serialize_field(
                    "maxFrictionSolverMassRatio",
                    &self.m_maxFrictionSolverMassRatio,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("wheelParams", &self.m_wheelParams)?;
            serializer
                .serialize_array_meta_field(
                    "numWheelsPerAxle",
                    &self.m_numWheelsPerAxle,
                )?;
            serializer
                .serialize_field("frictionDescription", &self.m_frictionDescription)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field(
                    "chassisFrictionInertiaInvDiag",
                    &self.m_chassisFrictionInertiaInvDiag,
                )?;
            serializer
                .serialize_field("alreadyInitialised", &self.m_alreadyInitialised)?;
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer.serialize_array_field("wheelParams", &self.m_wheelParams)?;
            serializer
                .serialize_array_field("numWheelsPerAxle", &self.m_numWheelsPerAxle)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_gravity,
    m_numWheels,
    m_chassisOrientation,
    m_torqueRollFactor,
    m_torquePitchFactor,
    m_torqueYawFactor,
    m_extraTorqueFactor,
    m_maxVelocityForPositionalFriction,
    m_chassisUnitInertiaYaw,
    m_chassisUnitInertiaRoll,
    m_chassisUnitInertiaPitch,
    m_frictionEqualizer,
    m_normalClippingAngleCos,
    m_maxFrictionSolverMassRatio,
    m_wheelParams,
    m_numWheelsPerAxle,
    m_frictionDescription,
    m_chassisFrictionInertiaInvDiag,
    m_alreadyInitialised,
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
            "gravity" => Ok(__Field::m_gravity),
            "numWheels" => Ok(__Field::m_numWheels),
            "chassisOrientation" => Ok(__Field::m_chassisOrientation),
            "torqueRollFactor" => Ok(__Field::m_torqueRollFactor),
            "torquePitchFactor" => Ok(__Field::m_torquePitchFactor),
            "torqueYawFactor" => Ok(__Field::m_torqueYawFactor),
            "extraTorqueFactor" => Ok(__Field::m_extraTorqueFactor),
            "maxVelocityForPositionalFriction" => {
                Ok(__Field::m_maxVelocityForPositionalFriction)
            }
            "chassisUnitInertiaYaw" => Ok(__Field::m_chassisUnitInertiaYaw),
            "chassisUnitInertiaRoll" => Ok(__Field::m_chassisUnitInertiaRoll),
            "chassisUnitInertiaPitch" => Ok(__Field::m_chassisUnitInertiaPitch),
            "frictionEqualizer" => Ok(__Field::m_frictionEqualizer),
            "normalClippingAngleCos" => Ok(__Field::m_normalClippingAngleCos),
            "maxFrictionSolverMassRatio" => Ok(__Field::m_maxFrictionSolverMassRatio),
            "wheelParams" => Ok(__Field::m_wheelParams),
            "numWheelsPerAxle" => Ok(__Field::m_numWheelsPerAxle),
            "frictionDescription" => Ok(__Field::m_frictionDescription),
            "chassisFrictionInertiaInvDiag" => {
                Ok(__Field::m_chassisFrictionInertiaInvDiag)
            }
            "alreadyInitialised" => Ok(__Field::m_alreadyInitialised),
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
pub(super) struct __hkpVehicleDataVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpVehicleData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpVehicleData>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpVehicleDataVisitor<'de> {
    type Value = hkpVehicleData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpVehicleData")
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
        let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_numWheels: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_chassisOrientation: _serde::__private::Option<Rotation> = _serde::__private::None;
        let mut m_torqueRollFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_torquePitchFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_torqueYawFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_extraTorqueFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxVelocityForPositionalFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_chassisUnitInertiaYaw: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_chassisUnitInertiaRoll: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_chassisUnitInertiaPitch: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_frictionEqualizer: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_normalClippingAngleCos: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxFrictionSolverMassRatio: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelParams: _serde::__private::Option<
            Vec<hkpVehicleDataWheelComponentParams>,
        > = _serde::__private::None;
        let mut m_numWheelsPerAxle: _serde::__private::Option<Vec<i8>> = _serde::__private::None;
        let mut m_frictionDescription: _serde::__private::Option<
            hkpVehicleFrictionDescription,
        > = _serde::__private::None;
        let mut m_chassisFrictionInertiaInvDiag: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_alreadyInitialised: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..19usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_gravity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("gravity"),
                        );
                    }
                    __A::pad(&mut __map, 8usize, 0usize)?;
                    m_gravity = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_numWheels) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numWheels",
                            ),
                        );
                    }
                    m_numWheels = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_chassisOrientation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "chassisOrientation",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 15usize, 15usize)?;
                    m_chassisOrientation = _serde::__private::Some(
                        match __A::next_value::<Rotation>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_torqueRollFactor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "torqueRollFactor",
                            ),
                        );
                    }
                    m_torqueRollFactor = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_torquePitchFactor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "torquePitchFactor",
                            ),
                        );
                    }
                    m_torquePitchFactor = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_torqueYawFactor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "torqueYawFactor",
                            ),
                        );
                    }
                    m_torqueYawFactor = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_extraTorqueFactor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "extraTorqueFactor",
                            ),
                        );
                    }
                    m_extraTorqueFactor = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(
                        &m_maxVelocityForPositionalFriction,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxVelocityForPositionalFriction",
                            ),
                        );
                    }
                    m_maxVelocityForPositionalFriction = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_chassisUnitInertiaYaw) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "chassisUnitInertiaYaw",
                            ),
                        );
                    }
                    m_chassisUnitInertiaYaw = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_chassisUnitInertiaRoll) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "chassisUnitInertiaRoll",
                            ),
                        );
                    }
                    m_chassisUnitInertiaRoll = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_chassisUnitInertiaPitch) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "chassisUnitInertiaPitch",
                            ),
                        );
                    }
                    m_chassisUnitInertiaPitch = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_frictionEqualizer) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "frictionEqualizer",
                            ),
                        );
                    }
                    m_frictionEqualizer = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_normalClippingAngleCos) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "normalClippingAngleCos",
                            ),
                        );
                    }
                    m_normalClippingAngleCos = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(
                        &m_maxFrictionSolverMassRatio,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxFrictionSolverMassRatio",
                            ),
                        );
                    }
                    m_maxFrictionSolverMassRatio = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                14usize => {
                    if _serde::__private::Option::is_some(&m_wheelParams) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelParams",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_wheelParams = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpVehicleDataWheelComponentParams>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_numWheelsPerAxle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numWheelsPerAxle",
                            ),
                        );
                    }
                    m_numWheelsPerAxle = _serde::__private::Some(
                        match __A::next_value::<Vec<i8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                16usize => {
                    if _serde::__private::Option::is_some(&m_frictionDescription) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "frictionDescription",
                            ),
                        );
                    }
                    m_frictionDescription = _serde::__private::Some(
                        match __A::next_value::<
                            hkpVehicleFrictionDescription,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                17usize => {
                    if _serde::__private::Option::is_some(
                        &m_chassisFrictionInertiaInvDiag,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "chassisFrictionInertiaInvDiag",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 0usize)?;
                    m_chassisFrictionInertiaInvDiag = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                18usize => {
                    if _serde::__private::Option::is_some(&m_alreadyInitialised) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "alreadyInitialised",
                            ),
                        );
                    }
                    m_alreadyInitialised = _serde::__private::Some(
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
        __A::pad(&mut __map, 15usize, 15usize)?;
        let m_gravity = match m_gravity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("gravity"),
                );
            }
        };
        let m_numWheels = match m_numWheels {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numWheels"),
                );
            }
        };
        let m_chassisOrientation = match m_chassisOrientation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisOrientation",
                    ),
                );
            }
        };
        let m_torqueRollFactor = match m_torqueRollFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("torqueRollFactor"),
                );
            }
        };
        let m_torquePitchFactor = match m_torquePitchFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("torquePitchFactor"),
                );
            }
        };
        let m_torqueYawFactor = match m_torqueYawFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("torqueYawFactor"),
                );
            }
        };
        let m_extraTorqueFactor = match m_extraTorqueFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("extraTorqueFactor"),
                );
            }
        };
        let m_maxVelocityForPositionalFriction = match m_maxVelocityForPositionalFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxVelocityForPositionalFriction",
                    ),
                );
            }
        };
        let m_chassisUnitInertiaYaw = match m_chassisUnitInertiaYaw {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisUnitInertiaYaw",
                    ),
                );
            }
        };
        let m_chassisUnitInertiaRoll = match m_chassisUnitInertiaRoll {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisUnitInertiaRoll",
                    ),
                );
            }
        };
        let m_chassisUnitInertiaPitch = match m_chassisUnitInertiaPitch {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisUnitInertiaPitch",
                    ),
                );
            }
        };
        let m_frictionEqualizer = match m_frictionEqualizer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("frictionEqualizer"),
                );
            }
        };
        let m_normalClippingAngleCos = match m_normalClippingAngleCos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "normalClippingAngleCos",
                    ),
                );
            }
        };
        let m_maxFrictionSolverMassRatio = match m_maxFrictionSolverMassRatio {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxFrictionSolverMassRatio",
                    ),
                );
            }
        };
        let m_wheelParams = match m_wheelParams {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelParams"),
                );
            }
        };
        let m_numWheelsPerAxle = match m_numWheelsPerAxle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numWheelsPerAxle"),
                );
            }
        };
        let m_frictionDescription = match m_frictionDescription {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "frictionDescription",
                    ),
                );
            }
        };
        let m_chassisFrictionInertiaInvDiag = match m_chassisFrictionInertiaInvDiag {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisFrictionInertiaInvDiag",
                    ),
                );
            }
        };
        let m_alreadyInitialised = match m_alreadyInitialised {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "alreadyInitialised",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleData {
            __ptr,
            parent,
            m_gravity,
            m_numWheels,
            m_chassisOrientation,
            m_torqueRollFactor,
            m_torquePitchFactor,
            m_torqueYawFactor,
            m_extraTorqueFactor,
            m_maxVelocityForPositionalFriction,
            m_chassisUnitInertiaYaw,
            m_chassisUnitInertiaRoll,
            m_chassisUnitInertiaPitch,
            m_frictionEqualizer,
            m_normalClippingAngleCos,
            m_maxFrictionSolverMassRatio,
            m_wheelParams,
            m_numWheelsPerAxle,
            m_frictionDescription,
            m_chassisFrictionInertiaInvDiag,
            m_alreadyInitialised,
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
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_numWheels: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_chassisOrientation: _serde::__private::Option<Rotation> = _serde::__private::None;
        let mut m_torqueRollFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_torquePitchFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_torqueYawFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_extraTorqueFactor: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxVelocityForPositionalFriction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_chassisUnitInertiaYaw: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_chassisUnitInertiaRoll: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_chassisUnitInertiaPitch: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_frictionEqualizer: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_normalClippingAngleCos: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxFrictionSolverMassRatio: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_wheelParams: _serde::__private::Option<
            Vec<hkpVehicleDataWheelComponentParams>,
        > = _serde::__private::None;
        let mut m_numWheelsPerAxle: _serde::__private::Option<Vec<i8>> = _serde::__private::None;
        let mut m_frictionDescription: _serde::__private::Option<
            hkpVehicleFrictionDescription,
        > = _serde::__private::None;
        let mut m_chassisFrictionInertiaInvDiag: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_alreadyInitialised: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..19usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_gravity => {
                        if _serde::__private::Option::is_some(&m_gravity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "gravity",
                                ),
                            );
                        }
                        m_gravity = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numWheels => {
                        if _serde::__private::Option::is_some(&m_numWheels) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numWheels",
                                ),
                            );
                        }
                        m_numWheels = _serde::__private::Some(
                            match __A::next_value::<i8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_chassisOrientation => {
                        if _serde::__private::Option::is_some(&m_chassisOrientation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "chassisOrientation",
                                ),
                            );
                        }
                        m_chassisOrientation = _serde::__private::Some(
                            match __A::next_value::<Rotation>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_torqueRollFactor => {
                        if _serde::__private::Option::is_some(&m_torqueRollFactor) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "torqueRollFactor",
                                ),
                            );
                        }
                        m_torqueRollFactor = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_torquePitchFactor => {
                        if _serde::__private::Option::is_some(&m_torquePitchFactor) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "torquePitchFactor",
                                ),
                            );
                        }
                        m_torquePitchFactor = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_torqueYawFactor => {
                        if _serde::__private::Option::is_some(&m_torqueYawFactor) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "torqueYawFactor",
                                ),
                            );
                        }
                        m_torqueYawFactor = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_extraTorqueFactor => {
                        if _serde::__private::Option::is_some(&m_extraTorqueFactor) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "extraTorqueFactor",
                                ),
                            );
                        }
                        m_extraTorqueFactor = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxVelocityForPositionalFriction => {
                        if _serde::__private::Option::is_some(
                            &m_maxVelocityForPositionalFriction,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxVelocityForPositionalFriction",
                                ),
                            );
                        }
                        m_maxVelocityForPositionalFriction = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_chassisUnitInertiaYaw => {
                        if _serde::__private::Option::is_some(&m_chassisUnitInertiaYaw) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "chassisUnitInertiaYaw",
                                ),
                            );
                        }
                        m_chassisUnitInertiaYaw = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_chassisUnitInertiaRoll => {
                        if _serde::__private::Option::is_some(
                            &m_chassisUnitInertiaRoll,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "chassisUnitInertiaRoll",
                                ),
                            );
                        }
                        m_chassisUnitInertiaRoll = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_chassisUnitInertiaPitch => {
                        if _serde::__private::Option::is_some(
                            &m_chassisUnitInertiaPitch,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "chassisUnitInertiaPitch",
                                ),
                            );
                        }
                        m_chassisUnitInertiaPitch = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_frictionEqualizer => {
                        if _serde::__private::Option::is_some(&m_frictionEqualizer) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "frictionEqualizer",
                                ),
                            );
                        }
                        m_frictionEqualizer = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_normalClippingAngleCos => {
                        if _serde::__private::Option::is_some(
                            &m_normalClippingAngleCos,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "normalClippingAngleCos",
                                ),
                            );
                        }
                        m_normalClippingAngleCos = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxFrictionSolverMassRatio => {
                        if _serde::__private::Option::is_some(
                            &m_maxFrictionSolverMassRatio,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxFrictionSolverMassRatio",
                                ),
                            );
                        }
                        m_maxFrictionSolverMassRatio = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_wheelParams => {
                        if _serde::__private::Option::is_some(&m_wheelParams) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "wheelParams",
                                ),
                            );
                        }
                        m_wheelParams = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkpVehicleDataWheelComponentParams>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numWheelsPerAxle => {
                        if _serde::__private::Option::is_some(&m_numWheelsPerAxle) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numWheelsPerAxle",
                                ),
                            );
                        }
                        m_numWheelsPerAxle = _serde::__private::Some(
                            match __A::next_value::<Vec<i8>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_frictionDescription => {
                        if _serde::__private::Option::is_some(&m_frictionDescription) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "frictionDescription",
                                ),
                            );
                        }
                        m_frictionDescription = _serde::__private::Some(
                            match __A::next_value::<
                                hkpVehicleFrictionDescription,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_chassisFrictionInertiaInvDiag => {
                        if _serde::__private::Option::is_some(
                            &m_chassisFrictionInertiaInvDiag,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "chassisFrictionInertiaInvDiag",
                                ),
                            );
                        }
                        m_chassisFrictionInertiaInvDiag = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_alreadyInitialised => {
                        if _serde::__private::Option::is_some(&m_alreadyInitialised) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "alreadyInitialised",
                                ),
                            );
                        }
                        m_alreadyInitialised = _serde::__private::Some(
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
        let m_gravity = match m_gravity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("gravity"),
                );
            }
        };
        let m_numWheels = match m_numWheels {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numWheels"),
                );
            }
        };
        let m_chassisOrientation = match m_chassisOrientation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisOrientation",
                    ),
                );
            }
        };
        let m_torqueRollFactor = match m_torqueRollFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("torqueRollFactor"),
                );
            }
        };
        let m_torquePitchFactor = match m_torquePitchFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("torquePitchFactor"),
                );
            }
        };
        let m_torqueYawFactor = match m_torqueYawFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("torqueYawFactor"),
                );
            }
        };
        let m_extraTorqueFactor = match m_extraTorqueFactor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("extraTorqueFactor"),
                );
            }
        };
        let m_maxVelocityForPositionalFriction = match m_maxVelocityForPositionalFriction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxVelocityForPositionalFriction",
                    ),
                );
            }
        };
        let m_chassisUnitInertiaYaw = match m_chassisUnitInertiaYaw {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisUnitInertiaYaw",
                    ),
                );
            }
        };
        let m_chassisUnitInertiaRoll = match m_chassisUnitInertiaRoll {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisUnitInertiaRoll",
                    ),
                );
            }
        };
        let m_chassisUnitInertiaPitch = match m_chassisUnitInertiaPitch {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisUnitInertiaPitch",
                    ),
                );
            }
        };
        let m_frictionEqualizer = match m_frictionEqualizer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("frictionEqualizer"),
                );
            }
        };
        let m_normalClippingAngleCos = match m_normalClippingAngleCos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "normalClippingAngleCos",
                    ),
                );
            }
        };
        let m_maxFrictionSolverMassRatio = match m_maxFrictionSolverMassRatio {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxFrictionSolverMassRatio",
                    ),
                );
            }
        };
        let m_wheelParams = match m_wheelParams {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelParams"),
                );
            }
        };
        let m_numWheelsPerAxle = match m_numWheelsPerAxle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numWheelsPerAxle"),
                );
            }
        };
        let m_frictionDescription = match m_frictionDescription {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "frictionDescription",
                    ),
                );
            }
        };
        let m_chassisFrictionInertiaInvDiag = match m_chassisFrictionInertiaInvDiag {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "chassisFrictionInertiaInvDiag",
                    ),
                );
            }
        };
        let m_alreadyInitialised = match m_alreadyInitialised {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "alreadyInitialised",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleData {
            __ptr,
            parent,
            m_gravity,
            m_numWheels,
            m_chassisOrientation,
            m_torqueRollFactor,
            m_torquePitchFactor,
            m_torqueYawFactor,
            m_extraTorqueFactor,
            m_maxVelocityForPositionalFriction,
            m_chassisUnitInertiaYaw,
            m_chassisUnitInertiaRoll,
            m_chassisUnitInertiaPitch,
            m_frictionEqualizer,
            m_normalClippingAngleCos,
            m_maxFrictionSolverMassRatio,
            m_wheelParams,
            m_numWheelsPerAxle,
            m_frictionDescription,
            m_chassisFrictionInertiaInvDiag,
            m_alreadyInitialised,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "gravity",
                "numWheels",
                "chassisOrientation",
                "torqueRollFactor",
                "torquePitchFactor",
                "torqueYawFactor",
                "extraTorqueFactor",
                "maxVelocityForPositionalFriction",
                "chassisUnitInertiaYaw",
                "chassisUnitInertiaRoll",
                "chassisUnitInertiaPitch",
                "frictionEqualizer",
                "normalClippingAngleCos",
                "maxFrictionSolverMassRatio",
                "wheelParams",
                "numWheelsPerAxle",
                "frictionDescription",
                "chassisFrictionInertiaInvDiag",
                "alreadyInitialised",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleData",
                FIELDS,
                __hkpVehicleDataVisitor {
                    marker: _serde::__private::PhantomData::<hkpVehicleData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
