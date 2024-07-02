use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkCustomAttributesAttribute`
/// -         version: `0`
/// -       signature: `0x1388d601`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkCustomAttributesAttribute<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `name`(ctype: `char*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: CString<'a>,
    /// # C++ Info
    /// -          name: `value`(ctype: `hkVariant`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_value: Variant,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkCustomAttributesAttribute<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkCustomAttributesAttribute"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(327734785u32)
        }
    }
    impl<'a> __serde::Serialize for hkCustomAttributesAttribute<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(327734785u32)));
            let mut serializer = __serializer
                .serialize_struct("hkCustomAttributesAttribute", class_meta)?;
            serializer.serialize_cstring_meta_field("name", &self.m_name)?;
            serializer.serialize_field("value", &self.m_value)?;
            serializer.serialize_cstring_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
