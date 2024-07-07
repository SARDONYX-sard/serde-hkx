use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStiffSpringConstraintDataAtoms`
/// -         version: `0`
/// -       signature: `0x207eb376`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStiffSpringConstraintDataAtoms {
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
    /// -          name: `spring`(ctype: `struct hkpStiffSpringConstraintAtom`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_spring: hkpStiffSpringConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpStiffSpringConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStiffSpringConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x207eb376)
        }
    }
    impl _serde::Serialize for hkpStiffSpringConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x207eb376)));
            let mut serializer = __serializer
                .serialize_struct("hkpStiffSpringConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("pivots", &self.m_pivots)?;
            serializer.serialize_field("spring", &self.m_spring)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
