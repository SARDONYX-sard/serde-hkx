use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCompressedMeshShapeBigTriangle`
/// -         version: `2`
/// -       signature: `0xcbfc95a4`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCompressedMeshShapeBigTriangle {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `a`(ctype: `hkUint16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_a: u16,
    /// # C++ Info
    /// -          name: `b`(ctype: `hkUint16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_b: u16,
    /// # C++ Info
    /// -          name: `c`(ctype: `hkUint16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_c: u16,
    /// # C++ Info
    /// -          name: `material`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_material: u32,
    /// # C++ Info
    /// -          name: `weldingInfo`(ctype: `hkUint16`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_weldingInfo: u16,
    /// # C++ Info
    /// -          name: `transformIndex`(ctype: `hkUint16`)
    /// -        offset:  14(x86)/ 14(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_transformIndex: u16,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCompressedMeshShapeBigTriangle {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCompressedMeshShapeBigTriangle"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3422328228u32)
        }
    }
    impl __serde::Serialize for hkpCompressedMeshShapeBigTriangle {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3422328228u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpCompressedMeshShapeBigTriangle", class_meta)?;
            serializer.serialize_field("a", &self.m_a)?;
            serializer.serialize_field("b", &self.m_b)?;
            serializer.serialize_field("c", &self.m_c)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.serialize_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.serialize_field("transformIndex", &self.m_transformIndex)?;
            serializer.end()
        }
    }
};
