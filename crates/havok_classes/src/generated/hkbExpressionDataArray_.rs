use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbExpressionDataArray`
/// -         version: `0`
/// -       signature: `0x4b9ee1a2`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbExpressionDataArray<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `expressionsData`(ctype: `hkArray<struct hkbExpressionData>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_expressionsData: Vec<hkbExpressionData<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbExpressionDataArray<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbExpressionDataArray"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1268703650u32)
        }
    }
    impl<'a> _serde::Serialize for hkbExpressionDataArray<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1268703650u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbExpressionDataArray", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("expressionsData", &self.m_expressionsData)?;
            serializer
                .serialize_array_field("expressionsData", &self.m_expressionsData)?;
            serializer.end()
        }
    }
};
