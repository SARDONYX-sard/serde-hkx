use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCachingShapePhantom`
/// -         version: `0`
/// -       signature: `0xcf227f58`
/// -          size: 368(x86)/448(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCachingShapePhantom<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShapePhantom<'a>,
    /// # C++ Info
    /// -          name: `collisionDetails`(ctype: `hkArray<void>`)
    /// -        offset: 352(x86)/416(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_collisionDetails: Vec<()>,
    /// # C++ Info
    /// -          name: `orderDirty`(ctype: `hkBool`)
    /// -        offset: 364(x86)/432(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_orderDirty: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpCachingShapePhantom<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCachingShapePhantom"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3475144536u32)
        }
    }
    impl<'a> __serde::Serialize for hkpCachingShapePhantom<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3475144536u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpCachingShapePhantom", class_meta)?;
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
            serializer.skip_field("world", &self.parent.parent.parent.m_world)?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer
                .serialize_field("collidable", &self.parent.parent.parent.m_collidable)?;
            serializer
                .serialize_field(
                    "multiThreadCheck",
                    &self.parent.parent.parent.m_multiThreadCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "name",
                    &self.parent.parent.parent.m_name,
                )?;
            serializer
                .serialize_array_meta_field(
                    "properties",
                    &self.parent.parent.parent.m_properties,
                )?;
            serializer.skip_field("treeData", &self.parent.parent.parent.m_treeData)?;
            serializer
                .skip_array_meta_field(
                    "overlapListeners",
                    &self.parent.parent.m_overlapListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "phantomListeners",
                    &self.parent.parent.m_phantomListeners,
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("motionState", &self.parent.m_motionState)?;
            serializer
                .skip_array_meta_field("collisionDetails", &self.m_collisionDetails)?;
            serializer.skip_field("orderDirty", &self.m_orderDirty)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer
                .serialize_stringptr_field("name", &self.parent.parent.parent.m_name)?;
            serializer
                .serialize_array_field(
                    "properties",
                    &self.parent.parent.parent.m_properties,
                )?;
            serializer
                .serialize_array_field(
                    "overlapListeners",
                    &self.parent.parent.m_overlapListeners,
                )?;
            serializer
                .serialize_array_field(
                    "phantomListeners",
                    &self.parent.parent.m_phantomListeners,
                )?;
            serializer
                .serialize_array_field("collisionDetails", &self.m_collisionDetails)?;
            serializer.end()
        }
    }
};
