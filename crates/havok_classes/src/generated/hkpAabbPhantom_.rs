use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpAabbPhantom`
/// -         version: `0`
/// -       signature: `0x2c5189dd`
/// -          size: 224(x86)/304(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAabbPhantom<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpPhantom<'a>,
    /// # C++ Info
    /// -          name: `aabb`(ctype: `struct hkAabb`)
    /// -        offset: 176(x86)/240(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_aabb: hkAabb,
    /// # C++ Info
    /// -          name: `overlappingCollidables`(ctype: `hkArray<void*>`)
    /// -        offset: 208(x86)/272(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_overlappingCollidables: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `orderDirty`(ctype: `hkBool`)
    /// -        offset: 220(x86)/288(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_orderDirty: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpAabbPhantom<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpAabbPhantom"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(743541213u32)
        }
    }
    impl<'a> __serde::Serialize for hkpAabbPhantom<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(743541213u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpAabbPhantom", class_meta)?;
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
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("collidable", &self.parent.parent.m_collidable)?;
            serializer
                .serialize_field(
                    "multiThreadCheck",
                    &self.parent.parent.m_multiThreadCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_meta_field(
                    "properties",
                    &self.parent.parent.m_properties,
                )?;
            serializer.skip_field("treeData", &self.parent.parent.m_treeData)?;
            serializer
                .skip_array_meta_field(
                    "overlapListeners",
                    &self.parent.m_overlapListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "phantomListeners",
                    &self.parent.m_phantomListeners,
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("aabb", &self.m_aabb)?;
            serializer
                .skip_array_meta_field(
                    "overlappingCollidables",
                    &self.m_overlappingCollidables,
                )?;
            serializer.skip_field("orderDirty", &self.m_orderDirty)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field("properties", &self.parent.parent.m_properties)?;
            serializer
                .serialize_array_field(
                    "overlapListeners",
                    &self.parent.m_overlapListeners,
                )?;
            serializer
                .serialize_array_field(
                    "phantomListeners",
                    &self.parent.m_phantomListeners,
                )?;
            serializer
                .serialize_array_field(
                    "overlappingCollidables",
                    &self.m_overlappingCollidables,
                )?;
            serializer.end()
        }
    }
};
