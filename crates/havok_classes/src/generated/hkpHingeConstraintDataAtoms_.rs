use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpHingeConstraintDataAtoms`
/// -         version: `1`
/// -       signature: `0x6958371c`
/// -          size: 192(x86)/192(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpHingeConstraintDataAtoms {
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
    /// -          name: `2dAng`(ctype: `struct hkp2dAngConstraintAtom`)
    /// -        offset: 160(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_2dAng: hkp2dAngConstraintAtom,
    /// # C++ Info
    /// -          name: `ballSocket`(ctype: `struct hkpBallSocketConstraintAtom`)
    /// -        offset: 164(x86)/164(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_ballSocket: hkpBallSocketConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpHingeConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpHingeConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6958371c)
        }
    }
    impl _serde::Serialize for hkpHingeConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6958371c)));
            let mut serializer = __serializer
                .serialize_struct("hkpHingeConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer
                .serialize_field("setupStabilization", &self.m_setupStabilization)?;
            serializer.serialize_field("2dAng", &self.m_2dAng)?;
            serializer.serialize_field("ballSocket", &self.m_ballSocket)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
