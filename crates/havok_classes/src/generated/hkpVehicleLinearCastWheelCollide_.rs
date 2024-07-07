use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleLinearCastWheelCollide`
/// -         version: `0`
/// -       signature: `0xc59399d0`
/// -          size:  52(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleLinearCastWheelCollide {
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
    /// -          name: `wheelStates`(ctype: `hkArray<struct hkpVehicleLinearCastWheelCollideWheelState>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wheelStates: Vec<hkpVehicleLinearCastWheelCollideWheelState>,
    /// # C++ Info
    /// -          name: `rejectChassisListener`(ctype: `struct hkpRejectChassisListener`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 24(x86_64)
    ///
    pub m_rejectChassisListener: hkpRejectChassisListener,
    /// # C++ Info
    /// -          name: `maxExtraPenetration`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxExtraPenetration: f32,
    /// # C++ Info
    /// -          name: `startPointTolerance`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_startPointTolerance: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleLinearCastWheelCollide {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleLinearCastWheelCollide"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc59399d0)
        }
    }
    impl _serde::Serialize for hkpVehicleLinearCastWheelCollide {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc59399d0)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleLinearCastWheelCollide", class_meta)?;
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
            serializer.serialize_array_meta_field("wheelStates", &self.m_wheelStates)?;
            serializer
                .serialize_field(
                    "rejectChassisListener",
                    &self.m_rejectChassisListener,
                )?;
            serializer
                .serialize_field("maxExtraPenetration", &self.m_maxExtraPenetration)?;
            serializer
                .serialize_field("startPointTolerance", &self.m_startPointTolerance)?;
            serializer.serialize_array_field("wheelStates", &self.m_wheelStates)?;
            serializer.end()
        }
    }
};
