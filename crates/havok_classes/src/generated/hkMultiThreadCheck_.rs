use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMultiThreadCheck`
/// -         version: `0`
/// -       signature: `0x11e4408b`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMultiThreadCheck {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `threadId`(ctype: `hkUint32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_threadId: u32,
    /// # C++ Info
    /// -          name: `stackTraceId`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_stackTraceId: i32,
    /// # C++ Info
    /// -          name: `markCount`(ctype: `hkUint16`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_markCount: u16,
    /// # C++ Info
    /// -          name: `markBitStack`(ctype: `hkUint16`)
    /// -        offset:  10(x86)/ 10(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_markBitStack: u16,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkMultiThreadCheck {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMultiThreadCheck"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(300171403u32)
        }
    }
    impl __serde::Serialize for hkMultiThreadCheck {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(300171403u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMultiThreadCheck", class_meta)?;
            serializer.skip_field("threadId", &self.m_threadId)?;
            serializer.skip_field("stackTraceId", &self.m_stackTraceId)?;
            serializer.skip_field("markCount", &self.m_markCount)?;
            serializer.skip_field("markBitStack", &self.m_markBitStack)?;
            serializer.end()
        }
    }
};
