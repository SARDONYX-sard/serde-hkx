use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBallAndSocketConstraintDataAtoms`
/// -         version: `1`
/// -       signature: `0xc73dcaf9`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBallAndSocketConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `pivots`(ctype: `struct hkpSetLocalTranslationsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_pivots: hkpSetLocalTranslationsConstraintAtom,
    /// # C++ Info
    /// -          name: `setupStabilization`(ctype: `struct hkpSetupStabilizationAtom`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_setupStabilization: hkpSetupStabilizationAtom,
    /// # C++ Info
    /// -          name: `ballSocket`(ctype: `struct hkpBallSocketConstraintAtom`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_ballSocket: hkpBallSocketConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpBallAndSocketConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBallAndSocketConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3342715641u32)
        }
    }
    impl _serde::Serialize for hkpBallAndSocketConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3342715641u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpBallAndSocketConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("pivots", &self.m_pivots)?;
            serializer
                .serialize_field("setupStabilization", &self.m_setupStabilization)?;
            serializer.serialize_field("ballSocket", &self.m_ballSocket)?;
            serializer.end()
        }
    }
};
