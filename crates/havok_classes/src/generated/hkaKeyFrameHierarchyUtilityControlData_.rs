use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaKeyFrameHierarchyUtilityControlData`
/// -         version: `0`
/// -       signature: `0xa3d0ac71`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaKeyFrameHierarchyUtilityControlData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `hierarchyGain`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_hierarchyGain: f32,
    /// # C++ Info
    /// -          name: `velocityDamping`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_velocityDamping: f32,
    /// # C++ Info
    /// -          name: `accelerationGain`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_accelerationGain: f32,
    /// # C++ Info
    /// -          name: `velocityGain`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_velocityGain: f32,
    /// # C++ Info
    /// -          name: `positionGain`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_positionGain: f32,
    /// # C++ Info
    /// -          name: `positionMaxLinearVelocity`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_positionMaxLinearVelocity: f32,
    /// # C++ Info
    /// -          name: `positionMaxAngularVelocity`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_positionMaxAngularVelocity: f32,
    /// # C++ Info
    /// -          name: `snapGain`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_snapGain: f32,
    /// # C++ Info
    /// -          name: `snapMaxLinearVelocity`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_snapMaxLinearVelocity: f32,
    /// # C++ Info
    /// -          name: `snapMaxAngularVelocity`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_snapMaxAngularVelocity: f32,
    /// # C++ Info
    /// -          name: `snapMaxLinearDistance`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_snapMaxLinearDistance: f32,
    /// # C++ Info
    /// -          name: `snapMaxAngularDistance`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_snapMaxAngularDistance: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkaKeyFrameHierarchyUtilityControlData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaKeyFrameHierarchyUtilityControlData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2748361841u32)
        }
    }
    impl __serde::Serialize for hkaKeyFrameHierarchyUtilityControlData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkaKeyFrameHierarchyUtilityControlData", class_meta)?;
            serializer.serialize_field("hierarchyGain", &self.m_hierarchyGain)?;
            serializer.serialize_field("velocityDamping", &self.m_velocityDamping)?;
            serializer.serialize_field("accelerationGain", &self.m_accelerationGain)?;
            serializer.serialize_field("velocityGain", &self.m_velocityGain)?;
            serializer.serialize_field("positionGain", &self.m_positionGain)?;
            serializer
                .serialize_field(
                    "positionMaxLinearVelocity",
                    &self.m_positionMaxLinearVelocity,
                )?;
            serializer
                .serialize_field(
                    "positionMaxAngularVelocity",
                    &self.m_positionMaxAngularVelocity,
                )?;
            serializer.serialize_field("snapGain", &self.m_snapGain)?;
            serializer
                .serialize_field(
                    "snapMaxLinearVelocity",
                    &self.m_snapMaxLinearVelocity,
                )?;
            serializer
                .serialize_field(
                    "snapMaxAngularVelocity",
                    &self.m_snapMaxAngularVelocity,
                )?;
            serializer
                .serialize_field(
                    "snapMaxLinearDistance",
                    &self.m_snapMaxLinearDistance,
                )?;
            serializer
                .serialize_field(
                    "snapMaxAngularDistance",
                    &self.m_snapMaxAngularDistance,
                )?;
            serializer.end()
        }
    }
};
