use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRagdollLimitsDataAtoms`
/// -         version: `0`
/// -       signature: `0x82b894c3`
/// -          size: 176(x86)/176(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRagdollLimitsDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `rotations`(ctype: `struct hkpSetLocalRotationsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size: 112(x86)/112(x86_64)
    ///
    pub m_rotations: hkpSetLocalRotationsConstraintAtom,
    /// # C++ Info
    /// -          name: `twistLimit`(ctype: `struct hkpTwistLimitConstraintAtom`)
    /// -        offset: 112(x86)/112(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_twistLimit: hkpTwistLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `coneLimit`(ctype: `struct hkpConeLimitConstraintAtom`)
    /// -        offset: 132(x86)/132(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_coneLimit: hkpConeLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `planesLimit`(ctype: `struct hkpConeLimitConstraintAtom`)
    /// -        offset: 152(x86)/152(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_planesLimit: hkpConeLimitConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpRagdollLimitsDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRagdollLimitsDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x82b894c3)
        }
    }
    impl _serde::Serialize for hkpRagdollLimitsDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x82b894c3)));
            let mut serializer = __serializer
                .serialize_struct("hkpRagdollLimitsDataAtoms", class_meta)?;
            serializer.serialize_field("rotations", &self.m_rotations)?;
            serializer.serialize_field("twistLimit", &self.m_twistLimit)?;
            serializer.serialize_field("coneLimit", &self.m_coneLimit)?;
            serializer.serialize_field("planesLimit", &self.m_planesLimit)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
