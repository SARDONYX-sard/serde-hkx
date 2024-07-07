use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpEntitySmallArraySerializeOverrideType`
/// -         version: `1`
/// -       signature: `0xee3c2aec`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpEntitySmallArraySerializeOverrideType {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `data`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_data: Pointer,
    /// # C++ Info
    /// -          name: `size`(ctype: `hkUint16`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_size: u16,
    /// # C++ Info
    /// -          name: `capacityAndFlags`(ctype: `hkUint16`)
    /// -        offset:   6(x86)/ 10(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_capacityAndFlags: u16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpEntitySmallArraySerializeOverrideType {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpEntitySmallArraySerializeOverrideType"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xee3c2aec)
        }
    }
    impl _serde::Serialize for hkpEntitySmallArraySerializeOverrideType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xee3c2aec)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpEntitySmallArraySerializeOverrideType",
                    class_meta,
                )?;
            serializer.skip_field("data", &self.m_data)?;
            serializer.serialize_field("size", &self.m_size)?;
            serializer.serialize_field("capacityAndFlags", &self.m_capacityAndFlags)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
