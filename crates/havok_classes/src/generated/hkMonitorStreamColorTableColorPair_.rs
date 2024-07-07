use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMonitorStreamColorTableColorPair`
/// -         version: `0`
/// -       signature: `0x738fca05`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMonitorStreamColorTableColorPair<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `colorName`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_colorName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `color`(ctype: `enum ExtendedColors`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_color: ExtendedColors,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkMonitorStreamColorTableColorPair<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMonitorStreamColorTableColorPair"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x738fca05)
        }
    }
    impl<'a> _serde::Serialize for hkMonitorStreamColorTableColorPair<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x738fca05)));
            let mut serializer = __serializer
                .serialize_struct("hkMonitorStreamColorTableColorPair", class_meta)?;
            serializer.serialize_stringptr_meta_field("colorName", &self.m_colorName)?;
            serializer.serialize_field("color", &self.m_color)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_field("colorName", &self.m_colorName)?;
            serializer.end()
        }
    }
};
