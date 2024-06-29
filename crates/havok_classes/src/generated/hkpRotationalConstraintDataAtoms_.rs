use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRotationalConstraintDataAtoms`
/// -         version: `0`
/// -       signature: `0xa0c64586`
/// -          size: 128(x86)/128(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRotationalConstraintDataAtoms {
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
    /// -          name: `ang`(ctype: `struct hkpAngConstraintAtom`)
    /// -        offset: 112(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_ang: hkpAngConstraintAtom,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpRotationalConstraintDataAtoms {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpRotationalConstraintDataAtoms"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2697348486u32)
        }
    }
    impl __serde::Serialize for hkpRotationalConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpRotationalConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("rotations", &self.m_rotations)?;
            serializer.serialize_field("ang", &self.m_ang)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
