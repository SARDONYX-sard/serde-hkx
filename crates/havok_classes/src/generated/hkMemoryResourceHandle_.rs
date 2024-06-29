use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMemoryResourceHandle`
/// -         version: `2`
/// -       signature: `0xbffac086`
/// -          size:  28(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMemoryResourceHandle<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkResourceHandle,
    /// # C++ Info
    /// -          name: `variant`(ctype: `struct hkReferencedObject*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_variant: Pointer,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `references`(ctype: `hkArray<struct hkMemoryResourceHandleExternalLink>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_references: Vec<hkMemoryResourceHandleExternalLink<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkMemoryResourceHandle<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMemoryResourceHandle"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3220881542u32)
        }
    }
    impl<'a> __serde::Serialize for hkMemoryResourceHandle<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMemoryResourceHandle", class_meta)?;
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
            serializer.serialize_field("variant", &self.m_variant)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_array_meta_field("references", &self.m_references)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("references", &self.m_references)?;
            serializer.end()
        }
    }
};
