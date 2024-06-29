use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStiffSpringConstraintData`
/// -         version: `0`
/// -       signature: `0xb98f66f4`
/// -          size:  80(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStiffSpringConstraintData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintData,
    /// # C++ Info
    /// -          name: `atoms`(ctype: `struct hkpStiffSpringConstraintDataAtoms`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_atoms: hkpStiffSpringConstraintDataAtoms,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpStiffSpringConstraintData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpStiffSpringConstraintData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3113182964u32)
        }
    }
    impl __serde::Serialize for hkpStiffSpringConstraintData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpStiffSpringConstraintData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.end()
        }
    }
};