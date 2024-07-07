use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleRayCastWheelCollide`
/// -         version: `1`
/// -       signature: `0x41efd9e3`
/// -          size:  36(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleRayCastWheelCollide {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleWheelCollide,
    /// # C++ Info
    /// -          name: `wheelCollisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelCollisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `phantom`(ctype: `struct hkpAabbPhantom*`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_phantom: Pointer,
    /// # C++ Info
    /// -          name: `rejectRayChassisListener`(ctype: `struct hkpRejectChassisListener`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:  16(x86)/ 24(x86_64)
    ///
    pub m_rejectRayChassisListener: hkpRejectChassisListener,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleRayCastWheelCollide {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleRayCastWheelCollide"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x41efd9e3)
        }
    }
    impl _serde::Serialize for hkpVehicleRayCastWheelCollide {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x41efd9e3)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleRayCastWheelCollide", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("alreadyUsed", &self.parent.m_alreadyUsed)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_field(
                    "wheelCollisionFilterInfo",
                    &self.m_wheelCollisionFilterInfo,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("phantom", &self.m_phantom)?;
            serializer
                .serialize_field(
                    "rejectRayChassisListener",
                    &self.m_rejectRayChassisListener,
                )?;
            serializer.end()
        }
    }
};
