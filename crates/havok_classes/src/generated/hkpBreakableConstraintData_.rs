use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBreakableConstraintData`
/// -         version: `0`
/// -       signature: `0x7d6310c8`
/// -          size:  40(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBreakableConstraintData {
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
    /// -          name: `atoms`(ctype: `struct hkpBridgeAtoms`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_atoms: hkpBridgeAtoms,
    /// # C++ Info
    /// -          name: `constraintData`(ctype: `struct hkpConstraintData*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_constraintData: Pointer,
    /// # C++ Info
    /// -          name: `childRuntimeSize`(ctype: `hkUint16`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_childRuntimeSize: u16,
    /// # C++ Info
    /// -          name: `childNumSolverResults`(ctype: `hkUint16`)
    /// -        offset:  30(x86)/ 58(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_childNumSolverResults: u16,
    /// # C++ Info
    /// -          name: `solverResultLimit`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_solverResultLimit: f32,
    /// # C++ Info
    /// -          name: `removeWhenBroken`(ctype: `hkBool`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_removeWhenBroken: bool,
    /// # C++ Info
    /// -          name: `revertBackVelocityOnBreak`(ctype: `hkBool`)
    /// -        offset:  37(x86)/ 65(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_revertBackVelocityOnBreak: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpBreakableConstraintData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpBreakableConstraintData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2103644360u32)
        }
    }
    impl __serde::Serialize for hkpBreakableConstraintData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpBreakableConstraintData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.serialize_field("constraintData", &self.m_constraintData)?;
            serializer.serialize_field("childRuntimeSize", &self.m_childRuntimeSize)?;
            serializer
                .serialize_field(
                    "childNumSolverResults",
                    &self.m_childNumSolverResults,
                )?;
            serializer.serialize_field("solverResultLimit", &self.m_solverResultLimit)?;
            serializer.serialize_field("removeWhenBroken", &self.m_removeWhenBroken)?;
            serializer
                .serialize_field(
                    "revertBackVelocityOnBreak",
                    &self.m_revertBackVelocityOnBreak,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.end()
        }
    }
};
