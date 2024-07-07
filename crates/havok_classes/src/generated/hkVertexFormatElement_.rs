use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkVertexFormatElement`
/// -         version: `0`
/// -       signature: `0x54867cbf`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkVertexFormatElement {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `dataType`(ctype: `enum ComponentType`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_dataType: ComponentType,
    /// # C++ Info
    /// -          name: `numValues`(ctype: `hkUint8`)
    /// -        offset:   1(x86)/  1(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numValues: u8,
    /// # C++ Info
    /// -          name: `usage`(ctype: `enum ComponentUsage`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_usage: ComponentUsage,
    /// # C++ Info
    /// -          name: `subUsage`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_subUsage: u8,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags HintFlags`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_flags: HintFlags,
    /// # C++ Info
    /// -          name: `pad`(ctype: `hkUint8[3]`)
    /// -        offset:   5(x86)/  5(x86_64)
    /// -     type_size:   3(x86)/  3(x86_64)
    ///
    pub m_pad: [u8; 3usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkVertexFormatElement {
        #[inline]
        fn name(&self) -> &'static str {
            "hkVertexFormatElement"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x54867cbf)
        }
    }
    impl _serde::Serialize for hkVertexFormatElement {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x54867cbf)));
            let mut serializer = __serializer
                .serialize_struct("hkVertexFormatElement", class_meta)?;
            serializer.serialize_field("dataType", &self.m_dataType)?;
            serializer.serialize_field("numValues", &self.m_numValues)?;
            serializer.serialize_field("usage", &self.m_usage)?;
            serializer.serialize_field("subUsage", &self.m_subUsage)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("pad", &self.m_pad.as_slice())?;
            serializer.end()
        }
    }
};
