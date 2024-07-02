use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPrismaticConstraintDataAtoms`
/// -         version: `0`
/// -       signature: `0x7f516137`
/// -          size: 192(x86)/208(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPrismaticConstraintDataAtoms {
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
    /// -          name: `motor`(ctype: `struct hkpLinMotorConstraintAtom`)
    /// -        offset: 144(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 24(x86_64)
    ///
    pub m_motor: hkpLinMotorConstraintAtom,
    /// # C++ Info
    /// -          name: `friction`(ctype: `struct hkpLinFrictionConstraintAtom`)
    /// -        offset: 160(x86)/168(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_friction: hkpLinFrictionConstraintAtom,
    /// # C++ Info
    /// -          name: `ang`(ctype: `struct hkpAngConstraintAtom`)
    /// -        offset: 168(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_ang: hkpAngConstraintAtom,
    /// # C++ Info
    /// -          name: `lin0`(ctype: `struct hkpLinConstraintAtom`)
    /// -        offset: 172(x86)/180(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lin0: hkpLinConstraintAtom,
    /// # C++ Info
    /// -          name: `lin1`(ctype: `struct hkpLinConstraintAtom`)
    /// -        offset: 176(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lin1: hkpLinConstraintAtom,
    /// # C++ Info
    /// -          name: `linLimit`(ctype: `struct hkpLinLimitConstraintAtom`)
    /// -        offset: 180(x86)/188(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_linLimit: hkpLinLimitConstraintAtom,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpPrismaticConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPrismaticConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2136039735u32)
        }
    }
    impl __serde::Serialize for hkpPrismaticConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2136039735u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpPrismaticConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer.serialize_field("motor", &self.m_motor)?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("ang", &self.m_ang)?;
            serializer.serialize_field("lin0", &self.m_lin0)?;
            serializer.serialize_field("lin1", &self.m_lin1)?;
            serializer.serialize_field("linLimit", &self.m_linLimit)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
