use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbNamedStringEventPayload`
/// -         version: `0`
/// -       signature: `0x6caa9113`
/// -          size:  16(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbNamedStringEventPayload<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbNamedEventPayload<'a>,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_data: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbNamedStringEventPayload<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbNamedStringEventPayload"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6caa9113)
        }
    }
    impl<'a> _serde::Serialize for hkbNamedStringEventPayload<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6caa9113)));
            let mut serializer = __serializer
                .serialize_struct("hkbNamedStringEventPayload", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_stringptr_meta_field("data", &self.m_data)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_stringptr_field("data", &self.m_data)?;
            serializer.end()
        }
    }
};
