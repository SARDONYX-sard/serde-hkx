use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRackAndPinionConstraintAtom`
/// -         version: `0`
/// -       signature: `0x30cae006`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRackAndPinionConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// -          name: `pinionRadiusOrScrewPitch`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_pinionRadiusOrScrewPitch: f32,
    /// # C++ Info
    /// -          name: `isScrew`(ctype: `hkBool`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isScrew: bool,
    /// # C++ Info
    /// -          name: `memOffsetToInitialAngleOffset`(ctype: `hkInt8`)
    /// -        offset:   9(x86)/  9(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_memOffsetToInitialAngleOffset: i8,
    /// # C++ Info
    /// -          name: `memOffsetToPrevAngle`(ctype: `hkInt8`)
    /// -        offset:  10(x86)/ 10(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_memOffsetToPrevAngle: i8,
    /// # C++ Info
    /// -          name: `memOffsetToRevolutionCounter`(ctype: `hkInt8`)
    /// -        offset:  11(x86)/ 11(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_memOffsetToRevolutionCounter: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpRackAndPinionConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRackAndPinionConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x30cae006)
        }
    }
    impl _serde::Serialize for hkpRackAndPinionConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x30cae006)));
            let mut serializer = __serializer
                .serialize_struct("hkpRackAndPinionConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field(
                    "pinionRadiusOrScrewPitch",
                    &self.m_pinionRadiusOrScrewPitch,
                )?;
            serializer.serialize_field("isScrew", &self.m_isScrew)?;
            serializer
                .serialize_field(
                    "memOffsetToInitialAngleOffset",
                    &self.m_memOffsetToInitialAngleOffset,
                )?;
            serializer
                .serialize_field("memOffsetToPrevAngle", &self.m_memOffsetToPrevAngle)?;
            serializer
                .serialize_field(
                    "memOffsetToRevolutionCounter",
                    &self.m_memOffsetToRevolutionCounter,
                )?;
            serializer.end()
        }
    }
};
