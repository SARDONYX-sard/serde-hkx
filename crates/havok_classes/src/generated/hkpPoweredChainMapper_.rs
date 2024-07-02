use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPoweredChainMapper`
/// -         version: `0`
/// -       signature: `0x7a77ef5`
/// -          size:  44(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPoweredChainMapper {
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
    /// -          name: `links`(ctype: `hkArray<struct hkpPoweredChainMapperLinkInfo>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_links: Vec<hkpPoweredChainMapperLinkInfo>,
    /// # C++ Info
    /// -          name: `targets`(ctype: `hkArray<struct hkpPoweredChainMapperTarget>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_targets: Vec<hkpPoweredChainMapperTarget>,
    /// # C++ Info
    /// -          name: `chains`(ctype: `hkArray<hkpConstraintChainInstance*>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_chains: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpPoweredChainMapper {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPoweredChainMapper"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(128417525u32)
        }
    }
    impl __serde::Serialize for hkpPoweredChainMapper {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(128417525u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpPoweredChainMapper", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("links", &self.m_links)?;
            serializer.serialize_array_meta_field("targets", &self.m_targets)?;
            serializer.serialize_array_meta_field("chains", &self.m_chains)?;
            serializer.serialize_array_field("links", &self.m_links)?;
            serializer.serialize_array_field("targets", &self.m_targets)?;
            serializer.serialize_array_field("chains", &self.m_chains)?;
            serializer.end()
        }
    }
};
