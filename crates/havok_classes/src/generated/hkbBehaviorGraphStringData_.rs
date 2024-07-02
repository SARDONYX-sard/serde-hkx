use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorGraphStringData`
/// -         version: `1`
/// -       signature: `0xc713064e`
/// -          size:  56(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorGraphStringData<'a> {
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
    /// -          name: `attributeNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_attributeNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `variableNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_variableNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `characterPropertyNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_characterPropertyNames: Vec<StringPtr<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbBehaviorGraphStringData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBehaviorGraphStringData"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3339912782u32)
        }
    }
    impl<'a> __serde::Serialize for hkbBehaviorGraphStringData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3339912782u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorGraphStringData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("eventNames", &self.m_eventNames)?;
            serializer
                .serialize_array_meta_field("attributeNames", &self.m_attributeNames)?;
            serializer
                .serialize_array_meta_field("variableNames", &self.m_variableNames)?;
            serializer
                .serialize_array_meta_field(
                    "characterPropertyNames",
                    &self.m_characterPropertyNames,
                )?;
            serializer.serialize_array_field("eventNames", &self.m_eventNames)?;
            serializer.serialize_array_field("attributeNames", &self.m_attributeNames)?;
            serializer.serialize_array_field("variableNames", &self.m_variableNames)?;
            serializer
                .serialize_array_field(
                    "characterPropertyNames",
                    &self.m_characterPropertyNames,
                )?;
            serializer.end()
        }
    }
};
