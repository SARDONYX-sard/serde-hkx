use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpHingeLimitsDataAtoms`
/// -         version: `0`
/// -       signature: `0x555876ff`
/// -          size: 144(x86)/144(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpHingeLimitsDataAtoms {
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
    /// -          name: `angLimit`(ctype: `struct hkpAngLimitConstraintAtom`)
    /// -        offset: 112(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_angLimit: hkpAngLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `2dAng`(ctype: `struct hkp2dAngConstraintAtom`)
    /// -        offset: 128(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_2dAng: hkp2dAngConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpHingeLimitsDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpHingeLimitsDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x555876ff)
        }
    }
    impl _serde::Serialize for hkpHingeLimitsDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x555876ff)));
            let mut serializer = __serializer
                .serialize_struct("hkpHingeLimitsDataAtoms", class_meta)?;
            serializer.serialize_field("rotations", &self.m_rotations)?;
            serializer.serialize_field("angLimit", &self.m_angLimit)?;
            serializer.serialize_field("2dAng", &self.m_2dAng)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
