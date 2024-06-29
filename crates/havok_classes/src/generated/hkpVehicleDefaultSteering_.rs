use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultSteering`
/// -         version: `0`
/// -       signature: `0x8f0411c8`
/// -          size:  28(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultSteering {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleSteering,
    /// # C++ Info
    /// -          name: `maxSteeringAngle`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSteeringAngle: f32,
    /// # C++ Info
    /// -          name: `maxSpeedFullSteeringAngle`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSpeedFullSteeringAngle: f32,
    /// # C++ Info
    /// -          name: `doesWheelSteer`(ctype: `hkArray<hkBool>`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_doesWheelSteer: Vec<bool>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleDefaultSteering {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpVehicleDefaultSteering"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2399408584u32)
        }
    }
    impl __serde::Serialize for hkpVehicleDefaultSteering {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultSteering", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("maxSteeringAngle", &self.m_maxSteeringAngle)?;
            serializer
                .serialize_field(
                    "maxSpeedFullSteeringAngle",
                    &self.m_maxSpeedFullSteeringAngle,
                )?;
            serializer
                .serialize_array_meta_field("doesWheelSteer", &self.m_doesWheelSteer)?;
            serializer.serialize_array_field("doesWheelSteer", &self.m_doesWheelSteer)?;
            serializer.end()
        }
    }
};
