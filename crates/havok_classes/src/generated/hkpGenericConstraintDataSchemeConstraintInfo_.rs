use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpGenericConstraintDataSchemeConstraintInfo`
/// -         version: `0`
/// -       signature: `0xd6421f19`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpGenericConstraintDataSchemeConstraintInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `maxSizeOfSchema`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSizeOfSchema: i32,
    /// # C++ Info
    /// -          name: `sizeOfSchemas`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sizeOfSchemas: i32,
    /// # C++ Info
    /// -          name: `numSolverResults`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numSolverResults: i32,
    /// # C++ Info
    /// -          name: `numSolverElemTemps`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numSolverElemTemps: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpGenericConstraintDataSchemeConstraintInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpGenericConstraintDataSchemeConstraintInfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3594657561u32)
        }
    }
    impl __serde::Serialize for hkpGenericConstraintDataSchemeConstraintInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3594657561u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpGenericConstraintDataSchemeConstraintInfo",
                    class_meta,
                )?;
            serializer.serialize_field("maxSizeOfSchema", &self.m_maxSizeOfSchema)?;
            serializer.serialize_field("sizeOfSchemas", &self.m_sizeOfSchemas)?;
            serializer.serialize_field("numSolverResults", &self.m_numSolverResults)?;
            serializer
                .serialize_field("numSolverElemTemps", &self.m_numSolverElemTemps)?;
            serializer.end()
        }
    }
};
