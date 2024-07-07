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
