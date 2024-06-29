use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkBitField`
/// -         version: `0`
/// -       signature: `0xda41bd9b`
/// -          size:  16(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkBitField {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `words`(ctype: `hkArray<hkUint32>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_words: Vec<u32>,
    /// # C++ Info
    /// -          name: `numBits`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numBits: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkBitField {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkBitField"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3661741467u32)
        }
    }
    impl __serde::Serialize for hkBitField {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkBitField", class_meta)?;
            serializer.serialize_array_meta_field("words", &self.m_words)?;
            serializer.serialize_field("numBits", &self.m_numBits)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("words", &self.m_words)?;
            serializer.end()
        }
    }
};
