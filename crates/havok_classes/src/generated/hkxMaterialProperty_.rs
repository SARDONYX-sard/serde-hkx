use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxMaterialProperty`
/// -         version: `0`
/// -       signature: `0xd295234d`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMaterialProperty {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `key`(ctype: `hkUint32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_key: u32,
    /// # C++ Info
    /// -          name: `value`(ctype: `hkUint32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_value: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxMaterialProperty {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxMaterialProperty"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd295234d)
        }
    }
    impl _serde::Serialize for hkxMaterialProperty {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd295234d)));
            let mut serializer = __serializer
                .serialize_struct("hkxMaterialProperty", class_meta)?;
            serializer.serialize_field("key", &self.m_key)?;
            serializer.serialize_field("value", &self.m_value)?;
            serializer.end()
        }
    }
};
