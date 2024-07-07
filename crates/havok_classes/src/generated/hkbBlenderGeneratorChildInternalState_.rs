use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBlenderGeneratorChildInternalState`
/// -         version: `0`
/// -       signature: `0xff7327c0`
/// -          size:   2(x86)/  2(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlenderGeneratorChildInternalState {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `isActive`(ctype: `hkBool`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isActive: bool,
    /// # C++ Info
    /// -          name: `syncNextFrame`(ctype: `hkBool`)
    /// -        offset:   1(x86)/  1(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_syncNextFrame: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBlenderGeneratorChildInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBlenderGeneratorChildInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xff7327c0)
        }
    }
    impl _serde::Serialize for hkbBlenderGeneratorChildInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xff7327c0)));
            let mut serializer = __serializer
                .serialize_struct("hkbBlenderGeneratorChildInternalState", class_meta)?;
            serializer.serialize_field("isActive", &self.m_isActive)?;
            serializer.serialize_field("syncNextFrame", &self.m_syncNextFrame)?;
            serializer.end()
        }
    }
};
