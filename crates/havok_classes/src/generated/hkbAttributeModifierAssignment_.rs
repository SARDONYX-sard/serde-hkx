use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbAttributeModifierAssignment`
/// -         version: `0`
/// -       signature: `0x48b8ad52`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbAttributeModifierAssignment {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `attributeIndex`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_attributeIndex: i32,
    /// # C++ Info
    /// -          name: `attributeValue`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_attributeValue: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbAttributeModifierAssignment {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbAttributeModifierAssignment"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x48b8ad52)
        }
    }
    impl _serde::Serialize for hkbAttributeModifierAssignment {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x48b8ad52)));
            let mut serializer = __serializer
                .serialize_struct("hkbAttributeModifierAssignment", class_meta)?;
            serializer.serialize_field("attributeIndex", &self.m_attributeIndex)?;
            serializer.serialize_field("attributeValue", &self.m_attributeValue)?;
            serializer.end()
        }
    }
};
