use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpLimitedHingeConstraintDataAtoms`
/// -         version: `1`
/// -       signature: `0x54c7715b`
/// -          size: 240(x86)/240(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpLimitedHingeConstraintDataAtoms {
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
    /// -          name: `angMotor`(ctype: `struct hkpAngMotorConstraintAtom`)
    /// -        offset: 160(x86)/160(x86_64)
    /// -     type_size:  20(x86)/ 24(x86_64)
    ///
    pub m_angMotor: hkpAngMotorConstraintAtom,
    /// # C++ Info
    /// -          name: `angFriction`(ctype: `struct hkpAngFrictionConstraintAtom`)
    /// -        offset: 180(x86)/184(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_angFriction: hkpAngFrictionConstraintAtom,
    /// # C++ Info
    /// -          name: `angLimit`(ctype: `struct hkpAngLimitConstraintAtom`)
    /// -        offset: 192(x86)/196(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_angLimit: hkpAngLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `2dAng`(ctype: `struct hkp2dAngConstraintAtom`)
    /// -        offset: 208(x86)/212(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_2dAng: hkp2dAngConstraintAtom,
    /// # C++ Info
    /// -          name: `ballSocket`(ctype: `struct hkpBallSocketConstraintAtom`)
    /// -        offset: 212(x86)/216(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_ballSocket: hkpBallSocketConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpLimitedHingeConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpLimitedHingeConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x54c7715b)
        }
    }
    impl _serde::Serialize for hkpLimitedHingeConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x54c7715b)));
            let mut serializer = __serializer
                .serialize_struct("hkpLimitedHingeConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer
                .serialize_field("setupStabilization", &self.m_setupStabilization)?;
            serializer.serialize_field("angMotor", &self.m_angMotor)?;
            serializer.serialize_field("angFriction", &self.m_angFriction)?;
            serializer.serialize_field("angLimit", &self.m_angLimit)?;
            serializer.serialize_field("2dAng", &self.m_2dAng)?;
            serializer.serialize_field("ballSocket", &self.m_ballSocket)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
