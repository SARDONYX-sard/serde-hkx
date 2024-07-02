use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkDataObjectTypeAttribute`
/// -         version: `0`
/// -       signature: `0x1e3857bb`
/// -          size:   4(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkDataObjectTypeAttribute<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `typeName`(ctype: `char*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_typeName: CString<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkDataObjectTypeAttribute<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkDataObjectTypeAttribute"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(507008955u32)
        }
    }
    impl<'a> _serde::Serialize for hkDataObjectTypeAttribute<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(507008955u32)));
            let mut serializer = __serializer
                .serialize_struct("hkDataObjectTypeAttribute", class_meta)?;
            serializer.serialize_cstring_meta_field("typeName", &self.m_typeName)?;
            serializer.serialize_cstring_field("typeName", &self.m_typeName)?;
            serializer.end()
        }
    }
};
