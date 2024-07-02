use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPhantom`
/// -         version: `0`
/// -       signature: `0x9b7e6f86`
/// -          size: 164(x86)/240(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPhantom<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpWorldObject<'a>,
    /// # C++ Info
    /// -          name: `overlapListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 140(x86)/208(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_overlapListeners: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `phantomListeners`(ctype: `hkArray<void*>`)
    /// -        offset: 152(x86)/224(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_phantomListeners: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpPhantom<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPhantom"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2608754566u32)
        }
    }
    impl<'a> __serde::Serialize for hkpPhantom<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2608754566u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpPhantom", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.m_world)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("collidable", &self.parent.m_collidable)?;
            serializer
                .serialize_field("multiThreadCheck", &self.parent.m_multiThreadCheck)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer
                .serialize_array_meta_field("properties", &self.parent.m_properties)?;
            serializer.skip_field("treeData", &self.parent.m_treeData)?;
            serializer
                .skip_array_meta_field("overlapListeners", &self.m_overlapListeners)?;
            serializer
                .skip_array_meta_field("phantomListeners", &self.m_phantomListeners)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("properties", &self.parent.m_properties)?;
            serializer
                .serialize_array_field("overlapListeners", &self.m_overlapListeners)?;
            serializer
                .serialize_array_field("phantomListeners", &self.m_phantomListeners)?;
            serializer.end()
        }
    }
};
