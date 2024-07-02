use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpWheelConstraintDataAtoms`
/// -         version: `0`
/// -       signature: `0x1188cbe1`
/// -          size: 304(x86)/304(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpWheelConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `suspensionBase`(ctype: `struct hkpSetLocalTransformsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size: 144(x86)/144(x86_64)
    ///
    pub m_suspensionBase: hkpSetLocalTransformsConstraintAtom,
    /// # C++ Info
    /// -          name: `lin0Limit`(ctype: `struct hkpLinLimitConstraintAtom`)
    /// -        offset: 144(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_lin0Limit: hkpLinLimitConstraintAtom,
    /// # C++ Info
    /// -          name: `lin0Soft`(ctype: `struct hkpLinSoftConstraintAtom`)
    /// -        offset: 156(x86)/156(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_lin0Soft: hkpLinSoftConstraintAtom,
    /// # C++ Info
    /// -          name: `lin1`(ctype: `struct hkpLinConstraintAtom`)
    /// -        offset: 168(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lin1: hkpLinConstraintAtom,
    /// # C++ Info
    /// -          name: `lin2`(ctype: `struct hkpLinConstraintAtom`)
    /// -        offset: 172(x86)/172(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lin2: hkpLinConstraintAtom,
    /// # C++ Info
    /// -          name: `steeringBase`(ctype: `struct hkpSetLocalRotationsConstraintAtom`)
    /// -        offset: 176(x86)/176(x86_64)
    /// -     type_size: 112(x86)/112(x86_64)
    ///
    pub m_steeringBase: hkpSetLocalRotationsConstraintAtom,
    /// # C++ Info
    /// -          name: `2dAng`(ctype: `struct hkp2dAngConstraintAtom`)
    /// -        offset: 288(x86)/288(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_2dAng: hkp2dAngConstraintAtom,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpWheelConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpWheelConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(294177761u32)
        }
    }
    impl __serde::Serialize for hkpWheelConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(294177761u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpWheelConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("suspensionBase", &self.m_suspensionBase)?;
            serializer.serialize_field("lin0Limit", &self.m_lin0Limit)?;
            serializer.serialize_field("lin0Soft", &self.m_lin0Soft)?;
            serializer.serialize_field("lin1", &self.m_lin1)?;
            serializer.serialize_field("lin2", &self.m_lin2)?;
            serializer.serialize_field("steeringBase", &self.m_steeringBase)?;
            serializer.serialize_field("2dAng", &self.m_2dAng)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
