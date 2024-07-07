use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkClassEnumItem`
/// -         version: `0`
/// -       signature: `0xce6f8a6c`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkClassEnumItem<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `value`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_value: i32,
    /// # C++ Info
    /// -          name: `name`(ctype: `char*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: CString<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkClassEnumItem<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkClassEnumItem"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xce6f8a6c)
        }
    }
    impl<'a> _serde::Serialize for hkClassEnumItem<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xce6f8a6c)));
            let mut serializer = __serializer
                .serialize_struct("hkClassEnumItem", class_meta)?;
            serializer.serialize_field("value", &self.m_value)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_cstring_meta_field("name", &self.m_name)?;
            serializer.serialize_cstring_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
