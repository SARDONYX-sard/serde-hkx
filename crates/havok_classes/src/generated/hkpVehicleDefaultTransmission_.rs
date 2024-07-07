use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultTransmission`
/// -         version: `0`
/// -       signature: `0x235d5d6b`
/// -          size:  52(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultTransmission {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleTransmission,
    /// # C++ Info
    /// -          name: `downshiftRPM`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_downshiftRPM: f32,
    /// # C++ Info
    /// -          name: `upshiftRPM`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_upshiftRPM: f32,
    /// # C++ Info
    /// -          name: `primaryTransmissionRatio`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_primaryTransmissionRatio: f32,
    /// # C++ Info
    /// -          name: `clutchDelayTime`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_clutchDelayTime: f32,
    /// # C++ Info
    /// -          name: `reverseGearRatio`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_reverseGearRatio: f32,
    /// # C++ Info
    /// -          name: `gearsRatio`(ctype: `hkArray<hkReal>`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_gearsRatio: Vec<f32>,
    /// # C++ Info
    /// -          name: `wheelsTorqueRatio`(ctype: `hkArray<hkReal>`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wheelsTorqueRatio: Vec<f32>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultTransmission {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultTransmission"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x235d5d6b)
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultTransmission {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x235d5d6b)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultTransmission", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("downshiftRPM", &self.m_downshiftRPM)?;
            serializer.serialize_field("upshiftRPM", &self.m_upshiftRPM)?;
            serializer
                .serialize_field(
                    "primaryTransmissionRatio",
                    &self.m_primaryTransmissionRatio,
                )?;
            serializer.serialize_field("clutchDelayTime", &self.m_clutchDelayTime)?;
            serializer.serialize_field("reverseGearRatio", &self.m_reverseGearRatio)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("gearsRatio", &self.m_gearsRatio)?;
            serializer
                .serialize_array_meta_field(
                    "wheelsTorqueRatio",
                    &self.m_wheelsTorqueRatio,
                )?;
            serializer.serialize_array_field("gearsRatio", &self.m_gearsRatio)?;
            serializer
                .serialize_array_field("wheelsTorqueRatio", &self.m_wheelsTorqueRatio)?;
            serializer.end()
        }
    }
};
