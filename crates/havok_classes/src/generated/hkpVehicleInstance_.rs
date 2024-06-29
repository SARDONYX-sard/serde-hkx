use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleInstance`
/// -         version: `0`
/// -       signature: `0x877bb579`
/// -          size: 212(x86)/304(x86_64)
/// -          vtable: true
///
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
    /// -          name: `data`(ctype: `struct hkpVehicleData*`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_data: Pointer,
    /// # C++ Info
    /// -          name: `driverInput`(ctype: `struct hkpVehicleDriverInput*`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_driverInput: Pointer,
    /// # C++ Info
    /// -          name: `steering`(ctype: `struct hkpVehicleSteering*`)
    /// -        offset:  36(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_steering: Pointer,
    /// # C++ Info
    /// -          name: `engine`(ctype: `struct hkpVehicleEngine*`)
    /// -        offset:  40(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_engine: Pointer,
    /// # C++ Info
    /// -          name: `transmission`(ctype: `struct hkpVehicleTransmission*`)
    /// -        offset:  44(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_transmission: Pointer,
    /// # C++ Info
    /// -          name: `brake`(ctype: `struct hkpVehicleBrake*`)
    /// -        offset:  48(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_brake: Pointer,
    /// # C++ Info
    /// -          name: `suspension`(ctype: `struct hkpVehicleSuspension*`)
    /// -        offset:  52(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_suspension: Pointer,
    /// # C++ Info
    /// -          name: `aerodynamics`(ctype: `struct hkpVehicleAerodynamics*`)
    /// -        offset:  56(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_aerodynamics: Pointer,
    /// # C++ Info
    /// -          name: `wheelCollide`(ctype: `struct hkpVehicleWheelCollide*`)
    /// -        offset:  60(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_wheelCollide: Pointer,
    /// # C++ Info
    /// -          name: `tyreMarks`(ctype: `struct hkpTyremarksInfo*`)
    /// -        offset:  64(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_tyreMarks: Pointer,
    /// # C++ Info
    /// -          name: `velocityDamper`(ctype: `struct hkpVehicleVelocityDamper*`)
    /// -        offset:  68(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_velocityDamper: Pointer,
    /// # C++ Info
    /// -          name: `wheelsInfo`(ctype: `hkArray<struct hkpVehicleInstanceWheelInfo>`)
    /// -        offset:  72(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wheelsInfo: Vec<hkpVehicleInstanceWheelInfo>,
    /// # C++ Info
    /// -          name: `frictionStatus`(ctype: `struct hkpVehicleFrictionStatus`)
    /// -        offset:  84(x86)/160(x86_64)
    /// -     type_size:  72(x86)/ 72(x86_64)
    ///
    pub m_frictionStatus: hkpVehicleFrictionStatus,
    /// # C++ Info
    /// -          name: `deviceStatus`(ctype: `struct hkpVehicleDriverInputStatus*`)
    /// -        offset: 156(x86)/232(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_deviceStatus: Pointer,
    /// # C++ Info
    /// -          name: `isFixed`(ctype: `hkArray<hkBool>`)
    /// -        offset: 160(x86)/240(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_isFixed: Vec<bool>,
    /// # C++ Info
    /// -          name: `wheelsTimeSinceMaxPedalInput`(ctype: `hkReal`)
    /// -        offset: 172(x86)/256(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelsTimeSinceMaxPedalInput: f32,
    /// # C++ Info
    /// -          name: `tryingToReverse`(ctype: `hkBool`)
    /// -        offset: 176(x86)/260(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_tryingToReverse: bool,
    /// # C++ Info
    /// -          name: `torque`(ctype: `hkReal`)
    /// -        offset: 180(x86)/264(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_torque: f32,
    /// # C++ Info
    /// -          name: `rpm`(ctype: `hkReal`)
    /// -        offset: 184(x86)/268(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_rpm: f32,
    /// # C++ Info
    /// -          name: `mainSteeringAngle`(ctype: `hkReal`)
    /// -        offset: 188(x86)/272(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_mainSteeringAngle: f32,
    /// # C++ Info
    /// -          name: `wheelsSteeringAngle`(ctype: `hkArray<hkReal>`)
    /// -        offset: 192(x86)/280(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wheelsSteeringAngle: Vec<f32>,
    /// # C++ Info
    /// -          name: `isReversing`(ctype: `hkBool`)
    /// -        offset: 204(x86)/296(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isReversing: bool,
    /// # C++ Info
    /// -          name: `currentGear`(ctype: `hkInt8`)
    /// -        offset: 205(x86)/297(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_currentGear: i8,
    /// # C++ Info
    /// -          name: `delayed`(ctype: `hkBool`)
    /// -        offset: 206(x86)/298(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_delayed: bool,
    /// # C++ Info
    /// -          name: `clutchDelayCountdown`(ctype: `hkReal`)
    /// -        offset: 208(x86)/300(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_clutchDelayCountdown: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpVehicleInstance<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpVehicleInstance"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2273031545u32)
        }
    }
    impl<'a> __serde::Serialize for hkpVehicleInstance<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
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
