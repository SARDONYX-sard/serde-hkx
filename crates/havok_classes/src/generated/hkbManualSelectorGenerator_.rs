use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbManualSelectorGenerator`
/// -         version: `0`
/// -       signature: `0xd932fab8`
/// -          size:  56(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbManualSelectorGenerator<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// -          name: `generators`(ctype: `hkArray<hkbGenerator*>`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_generators: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `selectedGeneratorIndex`(ctype: `hkInt8`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_selectedGeneratorIndex: i8,
    /// # C++ Info
    /// -          name: `currentGeneratorIndex`(ctype: `hkInt8`)
    /// -        offset:  53(x86)/ 89(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_currentGeneratorIndex: i8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbManualSelectorGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbManualSelectorGenerator"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3643996856u32)
        }
    }
    impl<'a> __serde::Serialize for hkbManualSelectorGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3643996856u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbManualSelectorGenerator", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("generators", &self.m_generators)?;
            serializer
                .serialize_field(
                    "selectedGeneratorIndex",
                    &self.m_selectedGeneratorIndex,
                )?;
            serializer
                .serialize_field(
                    "currentGeneratorIndex",
                    &self.m_currentGeneratorIndex,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("generators", &self.m_generators)?;
            serializer.end()
        }
    }
};
