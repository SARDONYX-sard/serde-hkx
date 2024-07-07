use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbRealVariableSequencedData`
/// -         version: `0`
/// -       signature: `0xe2862d02`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbRealVariableSequencedData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbSequencedData,
    /// # C++ Info
    /// -          name: `samples`(ctype: `hkArray<struct hkbRealVariableSequencedDataSample>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_samples: Vec<hkbRealVariableSequencedDataSample>,
    /// # C++ Info
    /// -          name: `variableIndex`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_variableIndex: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbRealVariableSequencedData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbRealVariableSequencedData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe2862d02)
        }
    }
    impl _serde::Serialize for hkbRealVariableSequencedData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe2862d02)));
            let mut serializer = __serializer
                .serialize_struct("hkbRealVariableSequencedData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("samples", &self.m_samples)?;
            serializer.serialize_field("variableIndex", &self.m_variableIndex)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("samples", &self.m_samples)?;
            serializer.end()
        }
    }
};
