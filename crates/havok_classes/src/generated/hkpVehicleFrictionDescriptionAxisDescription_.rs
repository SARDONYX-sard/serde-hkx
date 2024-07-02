use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleFrictionDescriptionAxisDescription`
/// -         version: `0`
/// -       signature: `0x59ce153f`
/// -          size: 100(x86)/100(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleFrictionDescriptionAxisDescription {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `frictionCircleYtab`(ctype: `hkReal[16]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_frictionCircleYtab: [f32; 16usize],
    /// # C++ Info
    /// -          name: `xStep`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_xStep: f32,
    /// # C++ Info
    /// -          name: `xStart`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_xStart: f32,
    /// # C++ Info
    /// -          name: `wheelSurfaceInertia`(ctype: `hkReal`)
    /// -        offset:  72(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelSurfaceInertia: f32,
    /// # C++ Info
    /// -          name: `wheelSurfaceInertiaInv`(ctype: `hkReal`)
    /// -        offset:  76(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelSurfaceInertiaInv: f32,
    /// # C++ Info
    /// -          name: `wheelChassisMassRatio`(ctype: `hkReal`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelChassisMassRatio: f32,
    /// # C++ Info
    /// -          name: `wheelRadius`(ctype: `hkReal`)
    /// -        offset:  84(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelRadius: f32,
    /// # C++ Info
    /// -          name: `wheelRadiusInv`(ctype: `hkReal`)
    /// -        offset:  88(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelRadiusInv: f32,
    /// # C++ Info
    /// -          name: `wheelDownForceFactor`(ctype: `hkReal`)
    /// -        offset:  92(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelDownForceFactor: f32,
    /// # C++ Info
    /// -          name: `wheelDownForceSumFactor`(ctype: `hkReal`)
    /// -        offset:  96(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelDownForceSumFactor: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleFrictionDescriptionAxisDescription {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleFrictionDescriptionAxisDescription"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1506678079u32)
        }
    }
    impl __serde::Serialize for hkpVehicleFrictionDescriptionAxisDescription {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1506678079u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpVehicleFrictionDescriptionAxisDescription",
                    class_meta,
                )?;
            serializer
                .serialize_field(
                    "frictionCircleYtab",
                    &self.m_frictionCircleYtab.as_slice(),
                )?;
            serializer.serialize_field("xStep", &self.m_xStep)?;
            serializer.serialize_field("xStart", &self.m_xStart)?;
            serializer
                .serialize_field("wheelSurfaceInertia", &self.m_wheelSurfaceInertia)?;
            serializer
                .serialize_field(
                    "wheelSurfaceInertiaInv",
                    &self.m_wheelSurfaceInertiaInv,
                )?;
            serializer
                .serialize_field(
                    "wheelChassisMassRatio",
                    &self.m_wheelChassisMassRatio,
                )?;
            serializer.serialize_field("wheelRadius", &self.m_wheelRadius)?;
            serializer.serialize_field("wheelRadiusInv", &self.m_wheelRadiusInv)?;
            serializer
                .serialize_field("wheelDownForceFactor", &self.m_wheelDownForceFactor)?;
            serializer
                .serialize_field(
                    "wheelDownForceSumFactor",
                    &self.m_wheelDownForceSumFactor,
                )?;
            serializer.end()
        }
    }
};
