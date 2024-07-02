use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpGroupFilter`
/// -         version: `0`
/// -       signature: `0x65ee88e4`
/// -          size: 256(x86)/272(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpGroupFilter {
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
    /// -          name: `nextFreeSystemGroup`(ctype: `hkInt32`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_nextFreeSystemGroup: i32,
    /// # C++ Info
    /// -          name: `collisionLookupTable`(ctype: `hkUint32[32]`)
    /// -        offset:  52(x86)/ 76(x86_64)
    /// -     type_size: 128(x86)/128(x86_64)
    ///
    pub m_collisionLookupTable: [u32; 32usize],
    /// # C++ Info
    /// -          name: `pad256`(ctype: `hkVector4[4]`)
    /// -        offset: 192(x86)/208(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_pad256: [Vector4; 4usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpGroupFilter {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpGroupFilter"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1710131428u32)
        }
    }
    impl __serde::Serialize for hkpGroupFilter {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1710131428u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpGroupFilter", class_meta)?;
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
                .serialize_field("nextFreeSystemGroup", &self.m_nextFreeSystemGroup)?;
            serializer
                .serialize_field(
                    "collisionLookupTable",
                    &self.m_collisionLookupTable.as_slice(),
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("pad256", &self.m_pad256.as_slice())?;
            serializer.end()
        }
    }
};
