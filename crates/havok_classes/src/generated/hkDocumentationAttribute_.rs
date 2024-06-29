use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkDocumentationAttribute`
/// -         version: `0`
/// -       signature: `0x630edd9e`
/// -          size:   4(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkDocumentationAttribute<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `docsSectionTag`(ctype: `char*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_docsSectionTag: CString<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkDocumentationAttribute<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkDocumentationAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1661918622u32)
        }
    }
    impl<'a> __serde::Serialize for hkDocumentationAttribute<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkDocumentationAttribute", class_meta)?;
            serializer
                .serialize_cstring_meta_field("docsSectionTag", &self.m_docsSectionTag)?;
            serializer
                .serialize_cstring_field("docsSectionTag", &self.m_docsSectionTag)?;
            serializer.end()
        }
    }
};