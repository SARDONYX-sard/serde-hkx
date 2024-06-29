use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbPoweredRagdollControlData`
/// -         version: `3`
/// -       signature: `0xf5ba21b`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbPoweredRagdollControlData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `maxForce`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_maxForce: f32,
    /// # C++ Info
    /// -          name: `tau`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_tau: f32,
    /// # C++ Info
    /// -          name: `damping`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_damping: f32,
    /// # C++ Info
    /// -          name: `proportionalRecoveryVelocity`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_proportionalRecoveryVelocity: f32,
    /// # C++ Info
    /// -          name: `constantRecoveryVelocity`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_constantRecoveryVelocity: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbPoweredRagdollControlData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbPoweredRagdollControlData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(257663515u32)
        }
    }
    impl __serde::Serialize for hkbPoweredRagdollControlData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbPoweredRagdollControlData", class_meta)?;
            serializer.serialize_field("maxForce", &self.m_maxForce)?;
            serializer.serialize_field("tau", &self.m_tau)?;
            serializer.serialize_field("damping", &self.m_damping)?;
            serializer
                .serialize_field(
                    "proportionalRecoveryVelocity",
                    &self.m_proportionalRecoveryVelocity,
                )?;
            serializer
                .serialize_field(
                    "constantRecoveryVelocity",
                    &self.m_constantRecoveryVelocity,
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
