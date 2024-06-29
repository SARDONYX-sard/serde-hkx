use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbRigidBodyRagdollControlData`
/// -         version: `1`
/// -       signature: `0x1e0bc068`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbRigidBodyRagdollControlData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `keyFrameHierarchyControlData`(ctype: `struct hkaKeyFrameHierarchyUtilityControlData`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_keyFrameHierarchyControlData: hkaKeyFrameHierarchyUtilityControlData,
    /// # C++ Info
    /// -          name: `durationToBlend`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_durationToBlend: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbRigidBodyRagdollControlData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbRigidBodyRagdollControlData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(504086632u32)
        }
    }
    impl __serde::Serialize for hkbRigidBodyRagdollControlData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbRigidBodyRagdollControlData", class_meta)?;
            serializer
                .serialize_field(
                    "keyFrameHierarchyControlData",
                    &self.m_keyFrameHierarchyControlData,
                )?;
            serializer.serialize_field("durationToBlend", &self.m_durationToBlend)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
