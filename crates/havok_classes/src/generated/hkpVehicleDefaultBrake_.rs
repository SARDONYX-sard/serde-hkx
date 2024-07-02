use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultBrake`
/// -         version: `0`
/// -       signature: `0x4b4f8816`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultBrake {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleBrake,
    /// # C++ Info
    /// -          name: `wheelBrakingProperties`(ctype: `hkArray<struct hkpVehicleDefaultBrakeWheelBrakingProperties>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wheelBrakingProperties: Vec<hkpVehicleDefaultBrakeWheelBrakingProperties>,
    /// # C++ Info
    /// -          name: `wheelsMinTimeToBlock`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelsMinTimeToBlock: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultBrake {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultBrake"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1263503382u32)
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultBrake {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1263503382u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultBrake", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "wheelBrakingProperties",
                    &self.m_wheelBrakingProperties,
                )?;
            serializer
                .serialize_field("wheelsMinTimeToBlock", &self.m_wheelsMinTimeToBlock)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "wheelBrakingProperties",
                    &self.m_wheelBrakingProperties,
                )?;
            serializer.end()
        }
    }
};
