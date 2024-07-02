use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRagdollConstraintDataAtoms`
/// -         version: `1`
/// -       signature: `0xeed76b00`
/// -          size: 336(x86)/352(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRagdollConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `transforms`(ctype: `struct hkpSetLocalTransformsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size: 144(x86)/144(x86_64)
    ///
    pub m_transforms: hkpSetLocalTransformsConstraintAtom,
    /// # C++ Info
    /// -          name: `setupStabilization`(ctype: `struct hkpSetupStabilizationAtom`)
    /// -        offset: 144(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_setupStabilization: hkpSetupStabilizationAtom,
    /// # C++ Info
    /// -          name: `ragdollMotors`(ctype: `struct hkpRagdollMotorConstraintAtom`)
    /// -        offset: 160(x86)/160(x86_64)
    /// -     type_size:  80(x86)/ 96(x86_64)
    ///
    pub m_ragdollMotors: hkpRagdollMotorConstraintAtom,
    /// # C++ Info
    /// -          name: `angFriction`(ctype: `struct hkpAngFrictionConstraintAtom`)
    /// -        offset: 240(x86)/256(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_angFriction: hkpAngFrictionConstraintAtom,
    /// # C++ Info
    /// -          name: `twistLimit`(ctype: `struct hkpTwistLimitConstraintAtom`)
    /// -        offset: 252(x86)/268(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_twistLimit: hkpTwistLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `coneLimit`(ctype: `struct hkpConeLimitConstraintAtom`)
    /// -        offset: 272(x86)/288(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_coneLimit: hkpConeLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `planesLimit`(ctype: `struct hkpConeLimitConstraintAtom`)
    /// -        offset: 292(x86)/308(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_planesLimit: hkpConeLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `ballSocket`(ctype: `struct hkpBallSocketConstraintAtom`)
    /// -        offset: 312(x86)/328(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_ballSocket: hkpBallSocketConstraintAtom,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpRagdollConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRagdollConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4007095040u32)
        }
    }
    impl __serde::Serialize for hkpRagdollConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(4007095040u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpRagdollConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer
                .serialize_field("setupStabilization", &self.m_setupStabilization)?;
            serializer.serialize_field("ragdollMotors", &self.m_ragdollMotors)?;
            serializer.serialize_field("angFriction", &self.m_angFriction)?;
            serializer.serialize_field("twistLimit", &self.m_twistLimit)?;
            serializer.serialize_field("coneLimit", &self.m_coneLimit)?;
            serializer.serialize_field("planesLimit", &self.m_planesLimit)?;
            serializer.serialize_field("ballSocket", &self.m_ballSocket)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
