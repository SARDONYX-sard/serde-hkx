use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPairCollisionFilter`
/// -         version: `0`
/// -       signature: `0x4abc140e`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPairCollisionFilter {
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
    /// -          name: `disabledPairs`(ctype: `struct hkpPairCollisionFilterMapPairFilterKeyOverrideType`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_disabledPairs: hkpPairCollisionFilterMapPairFilterKeyOverrideType,
    /// # C++ Info
    /// -          name: `childFilter`(ctype: `struct hkpCollisionFilter*`)
    /// -        offset:  60(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_childFilter: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPairCollisionFilter {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPairCollisionFilter"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x4abc140e)
        }
    }
    impl _serde::Serialize for hkpPairCollisionFilter {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x4abc140e)));
            let mut serializer = __serializer
                .serialize_struct("hkpPairCollisionFilter", class_meta)?;
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
            serializer.skip_field("disabledPairs", &self.m_disabledPairs)?;
            serializer.serialize_field("childFilter", &self.m_childFilter)?;
            serializer.end()
        }
    }
};
