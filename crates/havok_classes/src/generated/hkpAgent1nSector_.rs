use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpAgent1nSector`
/// -         version: `0`
/// -       signature: `0x626e55a`
/// -          size: 512(x86)/512(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAgent1nSector {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `bytesAllocated`(ctype: `hkUint32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bytesAllocated: u32,
    /// # C++ Info
    /// -          name: `pad0`(ctype: `hkUint32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_pad0: u32,
    /// # C++ Info
    /// -          name: `pad1`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_pad1: u32,
    /// # C++ Info
    /// -          name: `pad2`(ctype: `hkUint32`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_pad2: u32,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkUint8[496]`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size: 496(x86)/496(x86_64)
    ///
    #[cfg_attr(feature = "serde", serde_as(as = "[_; 496]"))]
    #[educe(Default = [0;496usize])]
    pub m_data: [u8; 496usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpAgent1nSector {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpAgent1nSector"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(103212378u32)
        }
    }
    impl __serde::Serialize for hkpAgent1nSector {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(103212378u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpAgent1nSector", class_meta)?;
            serializer.serialize_field("bytesAllocated", &self.m_bytesAllocated)?;
            serializer.serialize_field("pad0", &self.m_pad0)?;
            serializer.serialize_field("pad1", &self.m_pad1)?;
            serializer.serialize_field("pad2", &self.m_pad2)?;
            serializer.serialize_field("data", &self.m_data.as_slice())?;
            serializer.end()
        }
    }
};
