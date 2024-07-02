use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDataWheelComponentParams`
/// -         version: `0`
/// -       signature: `0x82fe40e0`
/// -          size:  40(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDataWheelComponentParams {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `radius`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_radius: f32,
    /// # C++ Info
    /// -          name: `mass`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_mass: f32,
    /// # C++ Info
    /// -          name: `width`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_width: f32,
    /// # C++ Info
    /// -          name: `friction`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_friction: f32,
    /// # C++ Info
    /// -          name: `viscosityFriction`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_viscosityFriction: f32,
    /// # C++ Info
    /// -          name: `maxFriction`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxFriction: f32,
    /// # C++ Info
    /// -          name: `slipAngle`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_slipAngle: f32,
    /// # C++ Info
    /// -          name: `forceFeedbackMultiplier`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_forceFeedbackMultiplier: f32,
    /// # C++ Info
    /// -          name: `maxContactBodyAcceleration`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxContactBodyAcceleration: f32,
    /// # C++ Info
    /// -          name: `axle`(ctype: `hkInt8`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_axle: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDataWheelComponentParams {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDataWheelComponentParams"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2197700832u32)
        }
    }
    impl _serde::Serialize for hkpVehicleDataWheelComponentParams {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2197700832u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDataWheelComponentParams", class_meta)?;
            serializer.serialize_field("radius", &self.m_radius)?;
            serializer.serialize_field("mass", &self.m_mass)?;
            serializer.serialize_field("width", &self.m_width)?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("viscosityFriction", &self.m_viscosityFriction)?;
            serializer.serialize_field("maxFriction", &self.m_maxFriction)?;
            serializer.serialize_field("slipAngle", &self.m_slipAngle)?;
            serializer
                .serialize_field(
                    "forceFeedbackMultiplier",
                    &self.m_forceFeedbackMultiplier,
                )?;
            serializer
                .serialize_field(
                    "maxContactBodyAcceleration",
                    &self.m_maxContactBodyAcceleration,
                )?;
            serializer.serialize_field("axle", &self.m_axle)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
