use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBroadPhaseHandle`
/// -         version: `0`
/// -       signature: `0x940569dc`
/// -          size:   4(x86)/  4(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBroadPhaseHandle {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `id`(ctype: `hkUint32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_id: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpBroadPhaseHandle {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBroadPhaseHandle"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x940569dc)
        }
    }
    impl _serde::Serialize for hkpBroadPhaseHandle {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x940569dc)));
            let mut serializer = __serializer
                .serialize_struct("hkpBroadPhaseHandle", class_meta)?;
            serializer.skip_field("id", &self.m_id)?;
            serializer.end()
        }
    }
};
