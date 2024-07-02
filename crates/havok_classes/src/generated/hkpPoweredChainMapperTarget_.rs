use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPoweredChainMapperTarget`
/// -         version: `0`
/// -       signature: `0xf651c74d`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPoweredChainMapperTarget {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `chain`(ctype: `struct hkpPoweredChainData*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_chain: Pointer,
    /// # C++ Info
    /// -          name: `infoIndex`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_infoIndex: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPoweredChainMapperTarget {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPoweredChainMapperTarget"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(4132554573u32)
        }
    }
    impl _serde::Serialize for hkpPoweredChainMapperTarget {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(4132554573u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpPoweredChainMapperTarget", class_meta)?;
            serializer.serialize_field("chain", &self.m_chain)?;
            serializer.serialize_field("infoIndex", &self.m_infoIndex)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
