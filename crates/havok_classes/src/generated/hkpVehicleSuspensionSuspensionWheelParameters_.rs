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
