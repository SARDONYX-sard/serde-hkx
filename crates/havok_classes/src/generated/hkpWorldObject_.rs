use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpWorldObject`
/// -         version: `0`
/// -       signature: `0x49fb6f2e`
/// -          size: 140(x86)/208(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpWorldObject<'a> {
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
    /// -          name: `world`(ctype: `void*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_world: Pointer,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
    /// # C++ Info
    /// -          name: `collidable`(ctype: `struct hkpLinkedCollidable`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  92(x86)/128(x86_64)
    ///
    pub m_collidable: hkpLinkedCollidable,
    /// # C++ Info
    /// -          name: `multiThreadCheck`(ctype: `struct hkMultiThreadCheck`)
    /// -        offset: 108(x86)/160(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_multiThreadCheck: hkMultiThreadCheck,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset: 120(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `properties`(ctype: `hkArray<struct hkpProperty>`)
    /// -        offset: 124(x86)/184(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_properties: Vec<hkpProperty>,
    /// # C++ Info
    /// -          name: `treeData`(ctype: `void*`)
    /// -        offset: 136(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_treeData: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpWorldObject<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpWorldObject"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x49fb6f2e)
        }
    }
    impl<'a> _serde::Serialize for hkpWorldObject<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x49fb6f2e)));
            let mut serializer = __serializer
                .serialize_struct("hkpWorldObject", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.m_world)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.serialize_field("collidable", &self.m_collidable)?;
            serializer.serialize_field("multiThreadCheck", &self.m_multiThreadCheck)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_array_meta_field("properties", &self.m_properties)?;
            serializer.skip_field("treeData", &self.m_treeData)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("properties", &self.m_properties)?;
            serializer.end()
        }
    }
};
