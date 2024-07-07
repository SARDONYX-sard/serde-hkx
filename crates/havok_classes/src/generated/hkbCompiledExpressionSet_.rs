use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCompiledExpressionSet`
/// -         version: `1`
/// -       signature: `0x3a7d76cc`
/// -          size:  36(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCompiledExpressionSet {
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
    /// -          name: `rpn`(ctype: `hkArray<struct hkbCompiledExpressionSetToken>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rpn: Vec<hkbCompiledExpressionSetToken>,
    /// # C++ Info
    /// -          name: `expressionToRpnIndex`(ctype: `hkArray<hkInt32>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_expressionToRpnIndex: Vec<i32>,
    /// # C++ Info
    /// -          name: `numExpressions`(ctype: `hkInt8`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numExpressions: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCompiledExpressionSet {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCompiledExpressionSet"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3a7d76cc)
        }
    }
    impl _serde::Serialize for hkbCompiledExpressionSet {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3a7d76cc)));
            let mut serializer = __serializer
                .serialize_struct("hkbCompiledExpressionSet", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("rpn", &self.m_rpn)?;
            serializer
                .serialize_array_meta_field(
                    "expressionToRpnIndex",
                    &self.m_expressionToRpnIndex,
                )?;
            serializer.serialize_field("numExpressions", &self.m_numExpressions)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_field("rpn", &self.m_rpn)?;
            serializer
                .serialize_array_field(
                    "expressionToRpnIndex",
                    &self.m_expressionToRpnIndex,
                )?;
            serializer.end()
        }
    }
};
