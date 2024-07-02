use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorGraphData`
/// -         version: `2`
/// -       signature: `0x95aca5d`
/// -          size:  88(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorGraphData {
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
    /// -          name: `attributeDefaults`(ctype: `hkArray<hkReal>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_attributeDefaults: Vec<f32>,
    /// # C++ Info
    /// -          name: `variableInfos`(ctype: `hkArray<struct hkbVariableInfo>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_variableInfos: Vec<hkbVariableInfo>,
    /// # C++ Info
    /// -          name: `characterPropertyInfos`(ctype: `hkArray<struct hkbVariableInfo>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_characterPropertyInfos: Vec<hkbVariableInfo>,
    /// # C++ Info
    /// -          name: `eventInfos`(ctype: `hkArray<struct hkbEventInfo>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_eventInfos: Vec<hkbEventInfo>,
    /// # C++ Info
    /// -          name: `wordMinVariableValues`(ctype: `hkArray<struct hkbVariableValue>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wordMinVariableValues: Vec<hkbVariableValue>,
    /// # C++ Info
    /// -          name: `wordMaxVariableValues`(ctype: `hkArray<struct hkbVariableValue>`)
    /// -        offset:  68(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wordMaxVariableValues: Vec<hkbVariableValue>,
    /// # C++ Info
    /// -          name: `variableInitialValues`(ctype: `struct hkbVariableValueSet*`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_variableInitialValues: Pointer,
    /// # C++ Info
    /// -          name: `stringData`(ctype: `struct hkbBehaviorGraphStringData*`)
    /// -        offset:  84(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_stringData: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBehaviorGraphData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBehaviorGraphData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(156944989u32)
        }
    }
    impl _serde::Serialize for hkbBehaviorGraphData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(156944989u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorGraphData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "attributeDefaults",
                    &self.m_attributeDefaults,
                )?;
            serializer
                .serialize_array_meta_field("variableInfos", &self.m_variableInfos)?;
            serializer
                .serialize_array_meta_field(
                    "characterPropertyInfos",
                    &self.m_characterPropertyInfos,
                )?;
            serializer.serialize_array_meta_field("eventInfos", &self.m_eventInfos)?;
            serializer
                .serialize_array_meta_field(
                    "wordMinVariableValues",
                    &self.m_wordMinVariableValues,
                )?;
            serializer
                .serialize_array_meta_field(
                    "wordMaxVariableValues",
                    &self.m_wordMaxVariableValues,
                )?;
            serializer
                .serialize_field(
                    "variableInitialValues",
                    &self.m_variableInitialValues,
                )?;
            serializer.serialize_field("stringData", &self.m_stringData)?;
            serializer
                .serialize_array_field("attributeDefaults", &self.m_attributeDefaults)?;
            serializer.serialize_array_field("variableInfos", &self.m_variableInfos)?;
            serializer
                .serialize_array_field(
                    "characterPropertyInfos",
                    &self.m_characterPropertyInfos,
                )?;
            serializer.serialize_array_field("eventInfos", &self.m_eventInfos)?;
            serializer
                .serialize_array_field(
                    "wordMinVariableValues",
                    &self.m_wordMinVariableValues,
                )?;
            serializer
                .serialize_array_field(
                    "wordMaxVariableValues",
                    &self.m_wordMaxVariableValues,
                )?;
            serializer.end()
        }
    }
};
