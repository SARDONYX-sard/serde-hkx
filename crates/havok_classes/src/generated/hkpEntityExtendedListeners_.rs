use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpEntityExtendedListeners`
/// -         version: `0`
/// -       signature: `0xf557023c`
/// -          size:  16(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpEntityExtendedListeners {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `activationListeners`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_activationListeners: hkpEntitySmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `entityListeners`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_entityListeners: hkpEntitySmallArraySerializeOverrideType,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpEntityExtendedListeners {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpEntityExtendedListeners"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf557023c)
        }
    }
    impl _serde::Serialize for hkpEntityExtendedListeners {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf557023c)));
            let mut serializer = __serializer
                .serialize_struct("hkpEntityExtendedListeners", class_meta)?;
            serializer.skip_field("activationListeners", &self.m_activationListeners)?;
            serializer.skip_field("entityListeners", &self.m_entityListeners)?;
            serializer.end()
        }
    }
};
