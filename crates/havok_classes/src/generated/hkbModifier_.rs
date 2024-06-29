use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbModifier`
/// -         version: `0`
/// -       signature: `0x96ec5ced`
/// -          size:  44(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbNode<'a>,
    /// # C++ Info
    /// -          name: `enable`(ctype: `hkBool`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enable: bool,
    /// # C++ Info
    /// -          name: `padModifier`(ctype: `hkBool[3]`)
    /// -        offset:  41(x86)/ 73(x86_64)
    /// -     type_size:   3(x86)/  3(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_padModifier: [bool; 3usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbModifier<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbModifier"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2532072685u32)
        }
    }
    impl<'a> __serde::Serialize for hkbModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.skip_field("id", &self.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.m_enable)?;
            serializer.skip_field("padModifier", &self.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.end()
        }
    }
};
