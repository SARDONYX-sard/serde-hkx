use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpArrayAction`
/// -         version: `0`
/// -       signature: `0x674bcd2d`
/// -          size:  36(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpArrayAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpAction<'a>,
    /// # C++ Info
    /// -          name: `entities`(ctype: `hkArray<hkpEntity*>`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_entities: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpArrayAction<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpArrayAction"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1733020973u32)
        }
    }
    impl<'a> _serde::Serialize for hkpArrayAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1733020973u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpArrayAction", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.m_world)?;
            serializer.skip_field("island", &self.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_array_meta_field("entities", &self.m_entities)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("entities", &self.m_entities)?;
            serializer.end()
        }
    }
};
