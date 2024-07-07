use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaDeltaCompressedAnimationQuantizationFormat`
/// -         version: `0`
/// -       signature: `0x724a7561`
/// -          size:  20(x86)/ 20(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaDeltaCompressedAnimationQuantizationFormat {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `maxBitWidth`(ctype: `hkUint8`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_maxBitWidth: u8,
    /// # C++ Info
    /// -          name: `preserved`(ctype: `hkUint8`)
    /// -        offset:   1(x86)/  1(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_preserved: u8,
    /// # C++ Info
    /// -          name: `numD`(ctype: `hkUint32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numD: u32,
    /// # C++ Info
    /// -          name: `offsetIdx`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offsetIdx: u32,
    /// # C++ Info
    /// -          name: `scaleIdx`(ctype: `hkUint32`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_scaleIdx: u32,
    /// # C++ Info
    /// -          name: `bitWidthIdx`(ctype: `hkUint32`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bitWidthIdx: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaDeltaCompressedAnimationQuantizationFormat {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaDeltaCompressedAnimationQuantizationFormat"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x724a7561)
        }
    }
    impl _serde::Serialize for hkaDeltaCompressedAnimationQuantizationFormat {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x724a7561)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaDeltaCompressedAnimationQuantizationFormat",
                    class_meta,
                )?;
            serializer.serialize_field("maxBitWidth", &self.m_maxBitWidth)?;
            serializer.serialize_field("preserved", &self.m_preserved)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("numD", &self.m_numD)?;
            serializer.serialize_field("offsetIdx", &self.m_offsetIdx)?;
            serializer.serialize_field("scaleIdx", &self.m_scaleIdx)?;
            serializer.serialize_field("bitWidthIdx", &self.m_bitWidthIdx)?;
            serializer.end()
        }
    }
};
