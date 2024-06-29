use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPoweredChainMapperLinkInfo`
/// -         version: `0`
/// -       signature: `0xcf071a1b`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPoweredChainMapperLinkInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `firstTargetIdx`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_firstTargetIdx: i32,
    /// # C++ Info
    /// -          name: `numTargets`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numTargets: i32,
    /// # C++ Info
    /// -          name: `limitConstraint`(ctype: `struct hkpConstraintInstance*`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_limitConstraint: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpPoweredChainMapperLinkInfo {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpPoweredChainMapperLinkInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3473349147u32)
        }
    }
    impl __serde::Serialize for hkpPoweredChainMapperLinkInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpPoweredChainMapperLinkInfo", class_meta)?;
            serializer.serialize_field("firstTargetIdx", &self.m_firstTargetIdx)?;
            serializer.serialize_field("numTargets", &self.m_numTargets)?;
            serializer.serialize_field("limitConstraint", &self.m_limitConstraint)?;
            serializer.end()
        }
    }
};
