use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbVariableBindingSet`
/// -         version: `2`
/// -       signature: `0x338ad4ff`
/// -          size:  28(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbVariableBindingSet<'a> {
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
    /// -          name: `bindings`(ctype: `hkArray<struct hkbVariableBindingSetBinding>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bindings: Vec<hkbVariableBindingSetBinding<'a>>,
    /// # C++ Info
    /// -          name: `indexOfBindingToEnable`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_indexOfBindingToEnable: i32,
    /// # C++ Info
    /// -          name: `hasOutputBinding`(ctype: `hkBool`)
    /// -        offset:  24(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_hasOutputBinding: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbVariableBindingSet<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbVariableBindingSet"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(864736511u32)
        }
    }
    impl<'a> __serde::Serialize for hkbVariableBindingSet<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(864736511u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbVariableBindingSet", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("bindings", &self.m_bindings)?;
            serializer
                .serialize_field(
                    "indexOfBindingToEnable",
                    &self.m_indexOfBindingToEnable,
                )?;
            serializer.skip_field("hasOutputBinding", &self.m_hasOutputBinding)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_array_field("bindings", &self.m_bindings)?;
            serializer.end()
        }
    }
};
