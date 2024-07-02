use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBindable`
/// -         version: `0`
/// -       signature: `0x2c1432d7`
/// -          size:  28(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBindable {
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
    /// -          name: `variableBindingSet`(ctype: `struct hkbVariableBindingSet*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_variableBindingSet: Pointer,
    /// # C++ Info
    /// -          name: `cachedBindables`(ctype: `hkArray<void>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_cachedBindables: Vec<()>,
    /// # C++ Info
    /// -          name: `areBindablesCached`(ctype: `hkBool`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_areBindablesCached: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbBindable {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBindable"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(739521239u32)
        }
    }
    impl __serde::Serialize for hkbBindable {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(739521239u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbBindable", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("variableBindingSet", &self.m_variableBindingSet)?;
            serializer
                .skip_array_meta_field("cachedBindables", &self.m_cachedBindables)?;
            serializer.skip_field("areBindablesCached", &self.m_areBindablesCached)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field("cachedBindables", &self.m_cachedBindables)?;
            serializer.end()
        }
    }
};
