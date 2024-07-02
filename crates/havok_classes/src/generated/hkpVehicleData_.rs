use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleData`
/// -         version: `1`
/// -       signature: `0x173feb43`
/// -          size: 416(x86)/416(x86_64)
/// -          vtable: true
///
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
    /// -          name: `gravity`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_gravity: Vector4,
    /// # C++ Info
    /// -          name: `numWheels`(ctype: `hkInt8`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numWheels: i8,
    /// # C++ Info
    /// -          name: `chassisOrientation`(ctype: `hkRotation`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_chassisOrientation: Rotation,
    /// # C++ Info
    /// -          name: `torqueRollFactor`(ctype: `hkReal`)
    /// -        offset:  96(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_torqueRollFactor: f32,
    /// # C++ Info
    /// -          name: `torquePitchFactor`(ctype: `hkReal`)
    /// -        offset: 100(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_torquePitchFactor: f32,
    /// # C++ Info
    /// -          name: `torqueYawFactor`(ctype: `hkReal`)
    /// -        offset: 104(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_torqueYawFactor: f32,
    /// # C++ Info
    /// -          name: `extraTorqueFactor`(ctype: `hkReal`)
    /// -        offset: 108(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_extraTorqueFactor: f32,
    /// # C++ Info
    /// -          name: `maxVelocityForPositionalFriction`(ctype: `hkReal`)
    /// -        offset: 112(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxVelocityForPositionalFriction: f32,
    /// # C++ Info
    /// -          name: `chassisUnitInertiaYaw`(ctype: `hkReal`)
    /// -        offset: 116(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_chassisUnitInertiaYaw: f32,
    /// # C++ Info
    /// -          name: `chassisUnitInertiaRoll`(ctype: `hkReal`)
    /// -        offset: 120(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_chassisUnitInertiaRoll: f32,
    /// # C++ Info
    /// -          name: `chassisUnitInertiaPitch`(ctype: `hkReal`)
    /// -        offset: 124(x86)/124(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_chassisUnitInertiaPitch: f32,
    /// # C++ Info
    /// -          name: `frictionEqualizer`(ctype: `hkReal`)
    /// -        offset: 128(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_frictionEqualizer: f32,
    /// # C++ Info
    /// -          name: `normalClippingAngleCos`(ctype: `hkReal`)
    /// -        offset: 132(x86)/132(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_normalClippingAngleCos: f32,
    /// # C++ Info
    /// -          name: `maxFrictionSolverMassRatio`(ctype: `hkReal`)
    /// -        offset: 136(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxFrictionSolverMassRatio: f32,
    /// # C++ Info
    /// -          name: `wheelParams`(ctype: `hkArray<struct hkpVehicleDataWheelComponentParams>`)
    /// -        offset: 140(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wheelParams: Vec<hkpVehicleDataWheelComponentParams>,
    /// # C++ Info
    /// -          name: `numWheelsPerAxle`(ctype: `hkArray<hkInt8>`)
    /// -        offset: 152(x86)/160(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_numWheelsPerAxle: Vec<i8>,
    /// # C++ Info
    /// -          name: `frictionDescription`(ctype: `struct hkpVehicleFrictionDescription`)
    /// -        offset: 164(x86)/176(x86_64)
    /// -     type_size: 208(x86)/208(x86_64)
    ///
    pub m_frictionDescription: hkpVehicleFrictionDescription,
    /// # C++ Info
    /// -          name: `chassisFrictionInertiaInvDiag`(ctype: `hkVector4`)
    /// -        offset: 384(x86)/384(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_chassisFrictionInertiaInvDiag: Vector4,
    /// # C++ Info
    /// -          name: `alreadyInitialised`(ctype: `hkBool`)
    /// -        offset: 400(x86)/400(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_alreadyInitialised: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleData"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(390064963u32)
        }
    }
    impl __serde::Serialize for hkpVehicleData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(390064963u32)));
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
