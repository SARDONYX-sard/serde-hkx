use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryResourceContainer`
/// -         version: `1`
/// -       signature: `0x4762f92a`
/// -          size:  40(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryResourceContainer<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkResourceContainer,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `parent`(ctype: `struct hkMemoryResourceContainer*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_parent: Pointer,
    /// # C++ Info
    /// -          name: `resourceHandles`(ctype: `hkArray<hkMemoryResourceHandle*>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_resourceHandles: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `children`(ctype: `hkArray<hkMemoryResourceContainer*>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_children: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkMemoryResourceContainer<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMemoryResourceContainer"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1197668650u32)
        }
    }
    impl<'a> _serde::Serialize for hkMemoryResourceContainer<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1197668650u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryResourceContainer", class_meta)?;
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
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.skip_field("parent", &self.m_parent)?;
            serializer
                .serialize_array_meta_field("resourceHandles", &self.m_resourceHandles)?;
            serializer.serialize_array_meta_field("children", &self.m_children)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer
                .serialize_array_field("resourceHandles", &self.m_resourceHandles)?;
            serializer.serialize_array_field("children", &self.m_children)?;
            serializer.end()
        }
    }
};
