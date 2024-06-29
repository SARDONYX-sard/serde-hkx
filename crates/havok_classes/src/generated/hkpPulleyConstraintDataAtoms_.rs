use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPulleyConstraintDataAtoms`
/// -         version: `0`
/// -       signature: `0xb149e5a`
/// -          size: 112(x86)/112(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPulleyConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `translations`(ctype: `struct hkpSetLocalTranslationsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_translations: hkpSetLocalTranslationsConstraintAtom,
    /// # C++ Info
    /// -          name: `pulley`(ctype: `struct hkpPulleyConstraintAtom`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_pulley: hkpPulleyConstraintAtom,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpPulleyConstraintDataAtoms {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpPulleyConstraintDataAtoms"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(185900634u32)
        }
    }
    impl __serde::Serialize for hkpPulleyConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpPulleyConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("translations", &self.m_translations)?;
            serializer.serialize_field("pulley", &self.m_pulley)?;
            serializer.end()
        }
    }
};
