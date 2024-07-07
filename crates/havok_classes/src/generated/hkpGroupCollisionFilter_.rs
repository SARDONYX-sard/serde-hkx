use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpGroupCollisionFilter`
/// -         version: `0`
/// -       signature: `0x5cc01561`
/// -          size: 180(x86)/208(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpGroupCollisionFilter {
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
    /// -          name: `noGroupCollisionEnabled`(ctype: `hkBool`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_noGroupCollisionEnabled: bool,
    /// # C++ Info
    /// -          name: `collisionGroups`(ctype: `hkUint32[32]`)
    /// -        offset:  52(x86)/ 76(x86_64)
    /// -     type_size: 128(x86)/128(x86_64)
    ///
    pub m_collisionGroups: [u32; 32usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpGroupCollisionFilter {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpGroupCollisionFilter"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5cc01561)
        }
    }
    impl _serde::Serialize for hkpGroupCollisionFilter {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5cc01561)));
            let mut serializer = __serializer
                .serialize_struct("hkpGroupCollisionFilter", class_meta)?;
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
                .serialize_field(
                    "noGroupCollisionEnabled",
                    &self.m_noGroupCollisionEnabled,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field("collisionGroups", &self.m_collisionGroups.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
