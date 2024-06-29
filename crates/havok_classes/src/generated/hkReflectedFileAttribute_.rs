use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkReflectedFileAttribute`
/// -         version: `0`
/// -       signature: `0xedb6b8f7`
/// -          size:   4(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkReflectedFileAttribute<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `value`(ctype: `char*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_value: CString<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkReflectedFileAttribute<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkReflectedFileAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3988175095u32)
        }
    }
    impl<'a> __serde::Serialize for hkReflectedFileAttribute<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkReflectedFileAttribute", class_meta)?;
            serializer.serialize_cstring_meta_field("value", &self.m_value)?;
            serializer.serialize_cstring_field("value", &self.m_value)?;
            serializer.end()
        }
    }
};
