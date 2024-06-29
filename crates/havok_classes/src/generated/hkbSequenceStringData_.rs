use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSequenceStringData`
/// -         version: `0`
/// -       signature: `0x6a5094e3`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSequenceStringData<'a> {
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
    /// -          name: `eventNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_eventNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `variableNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_variableNames: Vec<StringPtr<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbSequenceStringData<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbSequenceStringData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1783665891u32)
        }
    }
    impl<'a> __serde::Serialize for hkbSequenceStringData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbSequenceStringData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("eventNames", &self.m_eventNames)?;
            serializer
                .serialize_array_meta_field("variableNames", &self.m_variableNames)?;
            serializer.serialize_array_field("eventNames", &self.m_eventNames)?;
            serializer.serialize_array_field("variableNames", &self.m_variableNames)?;
            serializer.end()
        }
    }
};