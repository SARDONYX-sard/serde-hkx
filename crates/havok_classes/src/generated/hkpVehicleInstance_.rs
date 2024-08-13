use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleInstance`
/// - version: `0`
/// - signature: `0x877bb579`
/// - size: `212`(x86)/`304`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleInstance<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpUnaryAction<'a>,
    /// # C++ Info
    /// - name: `data`(ctype: `struct hkpVehicleData*`)
    /// - offset: ` 28`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_data: Pointer,
    /// # C++ Info
    /// - name: `driverInput`(ctype: `struct hkpVehicleDriverInput*`)
    /// - offset: ` 32`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_driverInput: Pointer,
    /// # C++ Info
    /// - name: `steering`(ctype: `struct hkpVehicleSteering*`)
    /// - offset: ` 36`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_steering: Pointer,
    /// # C++ Info
    /// - name: `engine`(ctype: `struct hkpVehicleEngine*`)
    /// - offset: ` 40`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_engine: Pointer,
    /// # C++ Info
    /// - name: `transmission`(ctype: `struct hkpVehicleTransmission*`)
    /// - offset: ` 44`(x86)/` 88`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_transmission: Pointer,
    /// # C++ Info
    /// - name: `brake`(ctype: `struct hkpVehicleBrake*`)
    /// - offset: ` 48`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_brake: Pointer,
    /// # C++ Info
    /// - name: `suspension`(ctype: `struct hkpVehicleSuspension*`)
    /// - offset: ` 52`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_suspension: Pointer,
    /// # C++ Info
    /// - name: `aerodynamics`(ctype: `struct hkpVehicleAerodynamics*`)
    /// - offset: ` 56`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_aerodynamics: Pointer,
    /// # C++ Info
    /// - name: `wheelCollide`(ctype: `struct hkpVehicleWheelCollide*`)
    /// - offset: ` 60`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_wheelCollide: Pointer,
    /// # C++ Info
    /// - name: `tyreMarks`(ctype: `struct hkpTyremarksInfo*`)
    /// - offset: ` 64`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_tyreMarks: Pointer,
    /// # C++ Info
    /// - name: `velocityDamper`(ctype: `struct hkpVehicleVelocityDamper*`)
    /// - offset: ` 68`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_velocityDamper: Pointer,
    /// # C++ Info
    /// - name: `wheelsInfo`(ctype: `hkArray<struct hkpVehicleInstanceWheelInfo>`)
    /// - offset: ` 72`(x86)/`144`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_wheelsInfo: Vec<hkpVehicleInstanceWheelInfo>,
    /// # C++ Info
    /// - name: `frictionStatus`(ctype: `struct hkpVehicleFrictionStatus`)
    /// - offset: ` 84`(x86)/`160`(x86_64)
    /// - type_size: ` 72`(x86)/` 72`(x86_64)
    pub m_frictionStatus: hkpVehicleFrictionStatus,
    /// # C++ Info
    /// - name: `deviceStatus`(ctype: `struct hkpVehicleDriverInputStatus*`)
    /// - offset: `156`(x86)/`232`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_deviceStatus: Pointer,
    /// # C++ Info
    /// - name: `isFixed`(ctype: `hkArray<hkBool>`)
    /// - offset: `160`(x86)/`240`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_isFixed: Vec<bool>,
    /// # C++ Info
    /// - name: `wheelsTimeSinceMaxPedalInput`(ctype: `hkReal`)
    /// - offset: `172`(x86)/`256`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_wheelsTimeSinceMaxPedalInput: f32,
    /// # C++ Info
    /// - name: `tryingToReverse`(ctype: `hkBool`)
    /// - offset: `176`(x86)/`260`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_tryingToReverse: bool,
    /// # C++ Info
    /// - name: `torque`(ctype: `hkReal`)
    /// - offset: `180`(x86)/`264`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_torque: f32,
    /// # C++ Info
    /// - name: `rpm`(ctype: `hkReal`)
    /// - offset: `184`(x86)/`268`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_rpm: f32,
    /// # C++ Info
    /// - name: `mainSteeringAngle`(ctype: `hkReal`)
    /// - offset: `188`(x86)/`272`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_mainSteeringAngle: f32,
    /// # C++ Info
    /// - name: `wheelsSteeringAngle`(ctype: `hkArray<hkReal>`)
    /// - offset: `192`(x86)/`280`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_wheelsSteeringAngle: Vec<f32>,
    /// # C++ Info
    /// - name: `isReversing`(ctype: `hkBool`)
    /// - offset: `204`(x86)/`296`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isReversing: bool,
    /// # C++ Info
    /// - name: `currentGear`(ctype: `hkInt8`)
    /// - offset: `205`(x86)/`297`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_currentGear: i8,
    /// # C++ Info
    /// - name: `delayed`(ctype: `hkBool`)
    /// - offset: `206`(x86)/`298`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_delayed: bool,
    /// # C++ Info
    /// - name: `clutchDelayCountdown`(ctype: `hkReal`)
    /// - offset: `208`(x86)/`300`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_clutchDelayCountdown: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpVehicleInstance<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleInstance"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x877bb579)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.m_world.get());
            v.push(self.parent.parent.m_island.get());
            v.push(self.parent.m_entity.get());
            v.push(self.m_data.get());
            v.push(self.m_driverInput.get());
            v.push(self.m_steering.get());
            v.push(self.m_engine.get());
            v.push(self.m_transmission.get());
            v.push(self.m_brake.get());
            v.push(self.m_suspension.get());
            v.push(self.m_aerodynamics.get());
            v.push(self.m_wheelCollide.get());
            v.push(self.m_tyreMarks.get());
            v.push(self.m_velocityDamper.get());
            v.extend(
                self
                    .m_wheelsInfo
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(self.m_frictionStatus.deps_indexes());
            v.push(self.m_deviceStatus.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkpVehicleInstance<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x877bb579)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleInstance", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.skip_field("island", &self.parent.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_field("entity", &self.parent.m_entity)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer.serialize_field("driverInput", &self.m_driverInput)?;
            serializer.serialize_field("steering", &self.m_steering)?;
            serializer.serialize_field("engine", &self.m_engine)?;
            serializer.serialize_field("transmission", &self.m_transmission)?;
            serializer.serialize_field("brake", &self.m_brake)?;
            serializer.serialize_field("suspension", &self.m_suspension)?;
            serializer.serialize_field("aerodynamics", &self.m_aerodynamics)?;
            serializer.serialize_field("wheelCollide", &self.m_wheelCollide)?;
            serializer.serialize_field("tyreMarks", &self.m_tyreMarks)?;
            serializer.serialize_field("velocityDamper", &self.m_velocityDamper)?;
            serializer.serialize_array_meta_field("wheelsInfo", &self.m_wheelsInfo)?;
            serializer.serialize_field("frictionStatus", &self.m_frictionStatus)?;
            serializer.serialize_field("deviceStatus", &self.m_deviceStatus)?;
            serializer.serialize_array_meta_field("isFixed", &self.m_isFixed)?;
            serializer
                .serialize_field(
                    "wheelsTimeSinceMaxPedalInput",
                    &self.m_wheelsTimeSinceMaxPedalInput,
                )?;
            serializer.serialize_field("tryingToReverse", &self.m_tryingToReverse)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("torque", &self.m_torque)?;
            serializer.serialize_field("rpm", &self.m_rpm)?;
            serializer.serialize_field("mainSteeringAngle", &self.m_mainSteeringAngle)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "wheelsSteeringAngle",
                    &self.m_wheelsSteeringAngle,
                )?;
            serializer.serialize_field("isReversing", &self.m_isReversing)?;
            serializer.serialize_field("currentGear", &self.m_currentGear)?;
            serializer.serialize_field("delayed", &self.m_delayed)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field("clutchDelayCountdown", &self.m_clutchDelayCountdown)?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("wheelsInfo", &self.m_wheelsInfo)?;
            serializer.serialize_array_field("isFixed", &self.m_isFixed)?;
            serializer
                .serialize_array_field(
                    "wheelsSteeringAngle",
                    &self.m_wheelsSteeringAngle,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleInstance<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
                m_name,
                m_entity,
                m_data,
                m_driverInput,
                m_steering,
                m_engine,
                m_transmission,
                m_brake,
                m_suspension,
                m_aerodynamics,
                m_wheelCollide,
                m_tyreMarks,
                m_velocityDamper,
                m_wheelsInfo,
                m_frictionStatus,
                m_deviceStatus,
                m_isFixed,
                m_wheelsTimeSinceMaxPedalInput,
                m_tryingToReverse,
                m_torque,
                m_rpm,
                m_mainSteeringAngle,
                m_wheelsSteeringAngle,
                m_isReversing,
                m_currentGear,
                m_delayed,
                m_clutchDelayCountdown,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "entity" => Ok(__Field::m_entity),
                        "data" => Ok(__Field::m_data),
                        "driverInput" => Ok(__Field::m_driverInput),
                        "steering" => Ok(__Field::m_steering),
                        "engine" => Ok(__Field::m_engine),
                        "transmission" => Ok(__Field::m_transmission),
                        "brake" => Ok(__Field::m_brake),
                        "suspension" => Ok(__Field::m_suspension),
                        "aerodynamics" => Ok(__Field::m_aerodynamics),
                        "wheelCollide" => Ok(__Field::m_wheelCollide),
                        "tyreMarks" => Ok(__Field::m_tyreMarks),
                        "velocityDamper" => Ok(__Field::m_velocityDamper),
                        "wheelsInfo" => Ok(__Field::m_wheelsInfo),
                        "frictionStatus" => Ok(__Field::m_frictionStatus),
                        "deviceStatus" => Ok(__Field::m_deviceStatus),
                        "isFixed" => Ok(__Field::m_isFixed),
                        "wheelsTimeSinceMaxPedalInput" => {
                            Ok(__Field::m_wheelsTimeSinceMaxPedalInput)
                        }
                        "tryingToReverse" => Ok(__Field::m_tryingToReverse),
                        "torque" => Ok(__Field::m_torque),
                        "rpm" => Ok(__Field::m_rpm),
                        "mainSteeringAngle" => Ok(__Field::m_mainSteeringAngle),
                        "wheelsSteeringAngle" => Ok(__Field::m_wheelsSteeringAngle),
                        "isReversing" => Ok(__Field::m_isReversing),
                        "currentGear" => Ok(__Field::m_currentGear),
                        "delayed" => Ok(__Field::m_delayed),
                        "clutchDelayCountdown" => Ok(__Field::m_clutchDelayCountdown),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkpVehicleInstanceVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpVehicleInstance<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpVehicleInstanceVisitor<'de> {
                type Value = hkpVehicleInstance<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpVehicleInstance",
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
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_data: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_driverInput: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_steering: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_engine: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_transmission: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_brake: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_suspension: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_aerodynamics: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_wheelCollide: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_tyreMarks: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_velocityDamper: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_wheelsInfo: _serde::__private::Option<
                        Vec<hkpVehicleInstanceWheelInfo>,
                    > = _serde::__private::None;
                    let mut m_frictionStatus: _serde::__private::Option<
                        hkpVehicleFrictionStatus,
                    > = _serde::__private::None;
                    let mut m_deviceStatus: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_isFixed: _serde::__private::Option<Vec<bool>> = _serde::__private::None;
                    let mut m_wheelsTimeSinceMaxPedalInput: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_tryingToReverse: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_torque: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_rpm: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_mainSteeringAngle: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_wheelsSteeringAngle: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_isReversing: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_currentGear: _serde::__private::Option<i8> = _serde::__private::None;
                    let mut m_delayed: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_clutchDelayCountdown: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..25usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_data) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_driverInput) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "driverInput",
                                        ),
                                    );
                                }
                                m_driverInput = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_steering) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "steering",
                                        ),
                                    );
                                }
                                m_steering = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_engine) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("engine"),
                                    );
                                }
                                m_engine = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_transmission) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transmission",
                                        ),
                                    );
                                }
                                m_transmission = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_brake) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("brake"),
                                    );
                                }
                                m_brake = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_suspension) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "suspension",
                                        ),
                                    );
                                }
                                m_suspension = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_aerodynamics) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "aerodynamics",
                                        ),
                                    );
                                }
                                m_aerodynamics = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_wheelCollide) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelCollide",
                                        ),
                                    );
                                }
                                m_wheelCollide = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_tyreMarks) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "tyreMarks",
                                        ),
                                    );
                                }
                                m_tyreMarks = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_velocityDamper) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "velocityDamper",
                                        ),
                                    );
                                }
                                m_velocityDamper = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_wheelsInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsInfo",
                                        ),
                                    );
                                }
                                m_wheelsInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpVehicleInstanceWheelInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_frictionStatus) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frictionStatus",
                                        ),
                                    );
                                }
                                m_frictionStatus = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpVehicleFrictionStatus,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_deviceStatus) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deviceStatus",
                                        ),
                                    );
                                }
                                m_deviceStatus = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_isFixed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isFixed",
                                        ),
                                    );
                                }
                                m_isFixed = _serde::__private::Some(
                                    match __A::next_value::<Vec<bool>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wheelsTimeSinceMaxPedalInput,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsTimeSinceMaxPedalInput",
                                        ),
                                    );
                                }
                                m_wheelsTimeSinceMaxPedalInput = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_tryingToReverse) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "tryingToReverse",
                                        ),
                                    );
                                }
                                m_tryingToReverse = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_torque) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("torque"),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_torque = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_rpm) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rpm"),
                                    );
                                }
                                m_rpm = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_mainSteeringAngle,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mainSteeringAngle",
                                        ),
                                    );
                                }
                                m_mainSteeringAngle = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wheelsSteeringAngle,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsSteeringAngle",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_wheelsSteeringAngle = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(&m_isReversing) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isReversing",
                                        ),
                                    );
                                }
                                m_isReversing = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(&m_currentGear) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentGear",
                                        ),
                                    );
                                }
                                m_currentGear = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(&m_delayed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "delayed",
                                        ),
                                    );
                                }
                                m_delayed = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_clutchDelayCountdown,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clutchDelayCountdown",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 1usize, 1usize)?;
                                m_clutchDelayCountdown = _serde::__private::Some(
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
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
                            );
                        }
                    };
                    let m_driverInput = match m_driverInput {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "driverInput",
                                ),
                            );
                        }
                    };
                    let m_steering = match m_steering {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("steering"),
                            );
                        }
                    };
                    let m_engine = match m_engine {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("engine"),
                            );
                        }
                    };
                    let m_transmission = match m_transmission {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transmission",
                                ),
                            );
                        }
                    };
                    let m_brake = match m_brake {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("brake"),
                            );
                        }
                    };
                    let m_suspension = match m_suspension {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "suspension",
                                ),
                            );
                        }
                    };
                    let m_aerodynamics = match m_aerodynamics {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "aerodynamics",
                                ),
                            );
                        }
                    };
                    let m_wheelCollide = match m_wheelCollide {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelCollide",
                                ),
                            );
                        }
                    };
                    let m_tyreMarks = match m_tyreMarks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "tyreMarks",
                                ),
                            );
                        }
                    };
                    let m_velocityDamper = match m_velocityDamper {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "velocityDamper",
                                ),
                            );
                        }
                    };
                    let m_wheelsInfo = match m_wheelsInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsInfo",
                                ),
                            );
                        }
                    };
                    let m_frictionStatus = match m_frictionStatus {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frictionStatus",
                                ),
                            );
                        }
                    };
                    let m_deviceStatus = match m_deviceStatus {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deviceStatus",
                                ),
                            );
                        }
                    };
                    let m_isFixed = match m_isFixed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isFixed"),
                            );
                        }
                    };
                    let m_wheelsTimeSinceMaxPedalInput = match m_wheelsTimeSinceMaxPedalInput {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsTimeSinceMaxPedalInput",
                                ),
                            );
                        }
                    };
                    let m_tryingToReverse = match m_tryingToReverse {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "tryingToReverse",
                                ),
                            );
                        }
                    };
                    let m_torque = match m_torque {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("torque"),
                            );
                        }
                    };
                    let m_rpm = match m_rpm {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("rpm"),
                            );
                        }
                    };
                    let m_mainSteeringAngle = match m_mainSteeringAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mainSteeringAngle",
                                ),
                            );
                        }
                    };
                    let m_wheelsSteeringAngle = match m_wheelsSteeringAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsSteeringAngle",
                                ),
                            );
                        }
                    };
                    let m_isReversing = match m_isReversing {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isReversing",
                                ),
                            );
                        }
                    };
                    let m_currentGear = match m_currentGear {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentGear",
                                ),
                            );
                        }
                    };
                    let m_delayed = match m_delayed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("delayed"),
                            );
                        }
                    };
                    let m_clutchDelayCountdown = match m_clutchDelayCountdown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clutchDelayCountdown",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpVehicleInstance {
                        __ptr,
                        parent,
                        m_data,
                        m_driverInput,
                        m_steering,
                        m_engine,
                        m_transmission,
                        m_brake,
                        m_suspension,
                        m_aerodynamics,
                        m_wheelCollide,
                        m_tyreMarks,
                        m_velocityDamper,
                        m_wheelsInfo,
                        m_frictionStatus,
                        m_deviceStatus,
                        m_isFixed,
                        m_wheelsTimeSinceMaxPedalInput,
                        m_tryingToReverse,
                        m_torque,
                        m_rpm,
                        m_mainSteeringAngle,
                        m_wheelsSteeringAngle,
                        m_isReversing,
                        m_currentGear,
                        m_delayed,
                        m_clutchDelayCountdown,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_entity: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_driverInput: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_steering: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_engine: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_transmission: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_brake: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_suspension: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_aerodynamics: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_wheelCollide: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_tyreMarks: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_velocityDamper: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_wheelsInfo: _serde::__private::Option<
                        Vec<hkpVehicleInstanceWheelInfo>,
                    > = _serde::__private::None;
                    let mut m_frictionStatus: _serde::__private::Option<
                        hkpVehicleFrictionStatus,
                    > = _serde::__private::None;
                    let mut m_deviceStatus: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_isFixed: _serde::__private::Option<Vec<bool>> = _serde::__private::None;
                    let mut m_wheelsTimeSinceMaxPedalInput: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_tryingToReverse: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_torque: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_rpm: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_mainSteeringAngle: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_wheelsSteeringAngle: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_isReversing: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_currentGear: _serde::__private::Option<i8> = _serde::__private::None;
                    let mut m_delayed: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_clutchDelayCountdown: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_entity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_entity) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("entity"),
                                    );
                                }
                                m_entity = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_data => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_data) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_driverInput => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_driverInput) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "driverInput",
                                        ),
                                    );
                                }
                                m_driverInput = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_steering => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_steering) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "steering",
                                        ),
                                    );
                                }
                                m_steering = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_engine => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_engine) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("engine"),
                                    );
                                }
                                m_engine = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transmission => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transmission) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transmission",
                                        ),
                                    );
                                }
                                m_transmission = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_brake => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_brake) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("brake"),
                                    );
                                }
                                m_brake = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_suspension => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_suspension) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "suspension",
                                        ),
                                    );
                                }
                                m_suspension = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_aerodynamics => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_aerodynamics) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "aerodynamics",
                                        ),
                                    );
                                }
                                m_aerodynamics = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wheelCollide => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_wheelCollide) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelCollide",
                                        ),
                                    );
                                }
                                m_wheelCollide = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_tyreMarks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_tyreMarks) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "tyreMarks",
                                        ),
                                    );
                                }
                                m_tyreMarks = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_velocityDamper => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_velocityDamper) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "velocityDamper",
                                        ),
                                    );
                                }
                                m_velocityDamper = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wheelsInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_wheelsInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsInfo",
                                        ),
                                    );
                                }
                                m_wheelsInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpVehicleInstanceWheelInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_frictionStatus => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_frictionStatus) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frictionStatus",
                                        ),
                                    );
                                }
                                m_frictionStatus = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpVehicleFrictionStatus,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_deviceStatus => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_deviceStatus) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deviceStatus",
                                        ),
                                    );
                                }
                                m_deviceStatus = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isFixed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isFixed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isFixed",
                                        ),
                                    );
                                }
                                m_isFixed = _serde::__private::Some(
                                    match __A::next_value::<Vec<bool>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wheelsTimeSinceMaxPedalInput => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wheelsTimeSinceMaxPedalInput,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsTimeSinceMaxPedalInput",
                                        ),
                                    );
                                }
                                m_wheelsTimeSinceMaxPedalInput = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_tryingToReverse => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_tryingToReverse) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "tryingToReverse",
                                        ),
                                    );
                                }
                                m_tryingToReverse = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_torque => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_torque) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("torque"),
                                    );
                                }
                                m_torque = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rpm => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rpm) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rpm"),
                                    );
                                }
                                m_rpm = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mainSteeringAngle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_mainSteeringAngle,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mainSteeringAngle",
                                        ),
                                    );
                                }
                                m_mainSteeringAngle = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wheelsSteeringAngle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wheelsSteeringAngle,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsSteeringAngle",
                                        ),
                                    );
                                }
                                m_wheelsSteeringAngle = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isReversing => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isReversing) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isReversing",
                                        ),
                                    );
                                }
                                m_isReversing = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_currentGear => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_currentGear) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentGear",
                                        ),
                                    );
                                }
                                m_currentGear = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_delayed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_delayed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "delayed",
                                        ),
                                    );
                                }
                                m_delayed = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_clutchDelayCountdown => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_clutchDelayCountdown,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clutchDelayCountdown",
                                        ),
                                    );
                                }
                                m_clutchDelayCountdown = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_entity = match m_entity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("entity"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_driverInput = match m_driverInput {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "driverInput",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_steering = match m_steering {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("steering"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_engine = match m_engine {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("engine"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transmission = match m_transmission {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transmission",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_brake = match m_brake {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("brake"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_suspension = match m_suspension {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "suspension",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_aerodynamics = match m_aerodynamics {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "aerodynamics",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wheelCollide = match m_wheelCollide {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelCollide",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_tyreMarks = match m_tyreMarks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "tyreMarks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_velocityDamper = match m_velocityDamper {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "velocityDamper",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wheelsInfo = match m_wheelsInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_frictionStatus = match m_frictionStatus {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frictionStatus",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deviceStatus = match m_deviceStatus {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deviceStatus",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isFixed = match m_isFixed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isFixed"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wheelsTimeSinceMaxPedalInput = match m_wheelsTimeSinceMaxPedalInput {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsTimeSinceMaxPedalInput",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_tryingToReverse = match m_tryingToReverse {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "tryingToReverse",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_torque = match m_torque {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("torque"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rpm = match m_rpm {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("rpm"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mainSteeringAngle = match m_mainSteeringAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mainSteeringAngle",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wheelsSteeringAngle = match m_wheelsSteeringAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsSteeringAngle",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isReversing = match m_isReversing {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isReversing",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_currentGear = match m_currentGear {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentGear",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_delayed = match m_delayed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("delayed"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_clutchDelayCountdown = match m_clutchDelayCountdown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clutchDelayCountdown",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkpAction {
                        __ptr,
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkpUnaryAction {
                        __ptr,
                        parent,
                        m_entity,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpVehicleInstance {
                        __ptr,
                        parent,
                        m_data,
                        m_driverInput,
                        m_steering,
                        m_engine,
                        m_transmission,
                        m_brake,
                        m_suspension,
                        m_aerodynamics,
                        m_wheelCollide,
                        m_tyreMarks,
                        m_velocityDamper,
                        m_wheelsInfo,
                        m_frictionStatus,
                        m_deviceStatus,
                        m_isFixed,
                        m_wheelsTimeSinceMaxPedalInput,
                        m_tryingToReverse,
                        m_torque,
                        m_rpm,
                        m_mainSteeringAngle,
                        m_wheelsSteeringAngle,
                        m_isReversing,
                        m_currentGear,
                        m_delayed,
                        m_clutchDelayCountdown,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "data",
                "driverInput",
                "steering",
                "engine",
                "transmission",
                "brake",
                "suspension",
                "aerodynamics",
                "wheelCollide",
                "tyreMarks",
                "velocityDamper",
                "wheelsInfo",
                "frictionStatus",
                "deviceStatus",
                "isFixed",
                "wheelsTimeSinceMaxPedalInput",
                "tryingToReverse",
                "torque",
                "rpm",
                "mainSteeringAngle",
                "wheelsSteeringAngle",
                "isReversing",
                "currentGear",
                "delayed",
                "clutchDelayCountdown",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleInstance",
                FIELDS,
                __hkpVehicleInstanceVisitor {
                    marker: _serde::__private::PhantomData::<hkpVehicleInstance>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
