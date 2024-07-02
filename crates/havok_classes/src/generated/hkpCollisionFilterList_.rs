use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCollisionFilterList`
/// -         version: `0`
/// -       signature: `0x2603bf04`
/// -          size:  60(x86)/ 88(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCollisionFilterList {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpCollisionFilter,
    /// # C++ Info
    /// -          name: `collisionFilters`(ctype: `hkArray<hkpCollisionFilter*>`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_collisionFilters: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCollisionFilterList {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCollisionFilterList"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(637779716u32)
        }
    }
    impl __serde::Serialize for hkpCollisionFilterList {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(637779716u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpCollisionFilterList", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 16usize].as_slice(), [0u8; 32usize].as_slice())?;
            serializer.serialize_field("prepad", &self.parent.m_prepad.as_slice())?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("postpad", &self.parent.m_postpad.as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "collisionFilters",
                    &self.m_collisionFilters,
                )?;
            serializer
                .serialize_array_field("collisionFilters", &self.m_collisionFilters)?;
            serializer.end()
        }
    }
};
