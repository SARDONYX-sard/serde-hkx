use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbMessageLog`
/// -         version: `0`
/// -       signature: `0x26a196c5`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbMessageLog {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `messages`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_messages: Pointer,
    /// # C++ Info
    /// -          name: `maxMessages`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_maxMessages: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbMessageLog {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbMessageLog"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x26a196c5)
        }
    }
    impl _serde::Serialize for hkbMessageLog {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x26a196c5)));
            let mut serializer = __serializer
                .serialize_struct("hkbMessageLog", class_meta)?;
            serializer.skip_field("messages", &self.m_messages)?;
            serializer.skip_field("maxMessages", &self.m_maxMessages)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
