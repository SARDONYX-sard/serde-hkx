use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkAabbUint32`
/// -         version: `0`
/// -       signature: `0x11e7c11`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkAabbUint32 {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `min`(ctype: `hkUint32[3]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_min: [u32; 3usize],
    /// # C++ Info
    /// -          name: `expansionMin`(ctype: `hkUint8[3]`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   3(x86)/  3(x86_64)
    ///
    pub m_expansionMin: [u8; 3usize],
    /// # C++ Info
    /// -          name: `expansionShift`(ctype: `hkUint8`)
    /// -        offset:  15(x86)/ 15(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_expansionShift: u8,
    /// # C++ Info
    /// -          name: `max`(ctype: `hkUint32[3]`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_max: [u32; 3usize],
    /// # C++ Info
    /// -          name: `expansionMax`(ctype: `hkUint8[3]`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   3(x86)/  3(x86_64)
    ///
    pub m_expansionMax: [u8; 3usize],
    /// # C++ Info
    /// -          name: `shapeKeyByte`(ctype: `hkUint8`)
    /// -        offset:  31(x86)/ 31(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_shapeKeyByte: u8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkAabbUint32 {
        #[inline]
        fn name(&self) -> &'static str {
            "hkAabbUint32"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(18775057u32)
        }
    }
    impl __serde::Serialize for hkAabbUint32 {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(18775057u32)));
            let mut serializer = __serializer
                .serialize_struct("hkAabbUint32", class_meta)?;
            serializer.serialize_field("min", &self.m_min.as_slice())?;
            serializer.serialize_field("expansionMin", &self.m_expansionMin.as_slice())?;
            serializer.serialize_field("expansionShift", &self.m_expansionShift)?;
            serializer.serialize_field("max", &self.m_max.as_slice())?;
            serializer.serialize_field("expansionMax", &self.m_expansionMax.as_slice())?;
            serializer.serialize_field("shapeKeyByte", &self.m_shapeKeyByte)?;
            serializer.end()
        }
    }
};
