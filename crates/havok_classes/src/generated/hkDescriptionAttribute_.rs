use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkDescriptionAttribute`
/// -         version: `0`
/// -       signature: `0xe9f9578a`
/// -          size:   4(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkDescriptionAttribute<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `string`(ctype: `char*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_string: CString<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkDescriptionAttribute<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkDescriptionAttribute"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3925432202u32)
        }
    }
    impl<'a> __serde::Serialize for hkDescriptionAttribute<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3925432202u32)));
            let mut serializer = __serializer
                .serialize_struct("hkDescriptionAttribute", class_meta)?;
            serializer.serialize_cstring_meta_field("string", &self.m_string)?;
            serializer.serialize_cstring_field("string", &self.m_string)?;
            serializer.end()
        }
    }
};
