use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkSimpleLocalFrame`
/// -         version: `1`
/// -       signature: `0xe758f63c`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkSimpleLocalFrame<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkLocalFrame,
    /// # C++ Info
    /// -          name: `transform`(ctype: `hkTransform`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_transform: Transform,
    /// # C++ Info
    /// -          name: `children`(ctype: `hkArray<hkLocalFrame*>`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_children: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `parentFrame`(ctype: `struct hkLocalFrame*`)
    /// -        offset:  92(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `NOT_OWNED`
    ///
    pub m_parentFrame: Pointer,
    /// # C++ Info
    /// -          name: `group`(ctype: `struct hkLocalFrameGroup*`)
    /// -        offset:  96(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_group: Pointer,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset: 100(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkSimpleLocalFrame<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkSimpleLocalFrame"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3881367100u32)
        }
    }
    impl<'a> __serde::Serialize for hkSimpleLocalFrame<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkSimpleLocalFrame", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer.serialize_array_meta_field("children", &self.m_children)?;
            serializer.serialize_field("parentFrame", &self.m_parentFrame)?;
            serializer.serialize_field("group", &self.m_group)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_array_field("children", &self.m_children)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
