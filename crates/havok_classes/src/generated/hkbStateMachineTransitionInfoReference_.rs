use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineTransitionInfoReference`
/// -         version: `1`
/// -       signature: `0x9810c2d0`
/// -          size:   6(x86)/  6(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineTransitionInfoReference {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `fromStateIndex`(ctype: `hkInt16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_fromStateIndex: i16,
    /// # C++ Info
    /// -          name: `transitionIndex`(ctype: `hkInt16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_transitionIndex: i16,
    /// # C++ Info
    /// -          name: `stateMachineId`(ctype: `hkInt16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_stateMachineId: i16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineTransitionInfoReference {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineTransitionInfoReference"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2551235280u32)
        }
    }
    impl _serde::Serialize for hkbStateMachineTransitionInfoReference {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2551235280u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineTransitionInfoReference", class_meta)?;
            serializer.serialize_field("fromStateIndex", &self.m_fromStateIndex)?;
            serializer.serialize_field("transitionIndex", &self.m_transitionIndex)?;
            serializer.serialize_field("stateMachineId", &self.m_stateMachineId)?;
            serializer.end()
        }
    }
};
