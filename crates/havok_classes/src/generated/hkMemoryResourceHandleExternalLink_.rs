use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryResourceHandleExternalLink`
/// -         version: `1`
/// -       signature: `0x3144d17c`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryResourceHandleExternalLink<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `memberName`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_memberName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `externalId`(ctype: `hkStringPtr`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_externalId: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkMemoryResourceHandleExternalLink<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMemoryResourceHandleExternalLink"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(826593660u32)
        }
    }
    impl<'a> __serde::Serialize for hkMemoryResourceHandleExternalLink<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(826593660u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryResourceHandleExternalLink", class_meta)?;
            serializer.serialize_stringptr_meta_field("memberName", &self.m_memberName)?;
            serializer.serialize_stringptr_meta_field("externalId", &self.m_externalId)?;
            serializer.serialize_stringptr_field("memberName", &self.m_memberName)?;
            serializer.serialize_stringptr_field("externalId", &self.m_externalId)?;
            serializer.end()
        }
    }
};
