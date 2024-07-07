use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleFrictionStatusAxisStatus`
/// -         version: `0`
/// -       signature: `0xe70e2bb4`
/// -          size:  36(x86)/ 36(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleFrictionStatusAxisStatus {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `forward_slip_velocity`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_forward_slip_velocity: f32,
    /// # C++ Info
    /// -          name: `side_slip_velocity`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_side_slip_velocity: f32,
    /// # C++ Info
    /// -          name: `skid_energy_density`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_skid_energy_density: f32,
    /// # C++ Info
    /// -          name: `side_force`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_side_force: f32,
    /// # C++ Info
    /// -          name: `delayed_forward_impulse`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_delayed_forward_impulse: f32,
    /// # C++ Info
    /// -          name: `sideRhs`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sideRhs: f32,
    /// # C++ Info
    /// -          name: `forwardRhs`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_forwardRhs: f32,
    /// # C++ Info
    /// -          name: `relativeSideForce`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_relativeSideForce: f32,
    /// # C++ Info
    /// -          name: `relativeForwardForce`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_relativeForwardForce: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleFrictionStatusAxisStatus {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleFrictionStatusAxisStatus"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe70e2bb4)
        }
    }
    impl _serde::Serialize for hkpVehicleFrictionStatusAxisStatus {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe70e2bb4)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleFrictionStatusAxisStatus", class_meta)?;
            serializer
                .serialize_field(
                    "forward_slip_velocity",
                    &self.m_forward_slip_velocity,
                )?;
            serializer
                .serialize_field("side_slip_velocity", &self.m_side_slip_velocity)?;
            serializer
                .serialize_field("skid_energy_density", &self.m_skid_energy_density)?;
            serializer.serialize_field("side_force", &self.m_side_force)?;
            serializer
                .serialize_field(
                    "delayed_forward_impulse",
                    &self.m_delayed_forward_impulse,
                )?;
            serializer.serialize_field("sideRhs", &self.m_sideRhs)?;
            serializer.serialize_field("forwardRhs", &self.m_forwardRhs)?;
            serializer.serialize_field("relativeSideForce", &self.m_relativeSideForce)?;
            serializer
                .serialize_field("relativeForwardForce", &self.m_relativeForwardForce)?;
            serializer.end()
        }
    }
};
