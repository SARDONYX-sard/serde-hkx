use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultBrakeWheelBrakingProperties`
/// -         version: `0`
/// -       signature: `0x1ffad971`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultBrakeWheelBrakingProperties {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `maxBreakingTorque`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxBreakingTorque: f32,
    /// # C++ Info
    /// -          name: `minPedalInputToBlock`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minPedalInputToBlock: f32,
    /// # C++ Info
    /// -          name: `isConnectedToHandbrake`(ctype: `hkBool`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isConnectedToHandbrake: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleDefaultBrakeWheelBrakingProperties {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultBrakeWheelBrakingProperties"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(536533361u32)
        }
    }
    impl __serde::Serialize for hkpVehicleDefaultBrakeWheelBrakingProperties {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(536533361u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpVehicleDefaultBrakeWheelBrakingProperties",
                    class_meta,
                )?;
            serializer.serialize_field("maxBreakingTorque", &self.m_maxBreakingTorque)?;
            serializer
                .serialize_field("minPedalInputToBlock", &self.m_minPedalInputToBlock)?;
            serializer
                .serialize_field(
                    "isConnectedToHandbrake",
                    &self.m_isConnectedToHandbrake,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
