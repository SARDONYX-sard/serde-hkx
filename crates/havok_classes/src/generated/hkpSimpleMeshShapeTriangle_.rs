use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSimpleMeshShapeTriangle`
/// -         version: `0`
/// -       signature: `0xd38738c1`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSimpleMeshShapeTriangle {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `a`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_a: i32,
    /// # C++ Info
    /// -          name: `b`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_b: i32,
    /// # C++ Info
    /// -          name: `c`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_c: i32,
    /// # C++ Info
    /// -          name: `weldingInfo`(ctype: `hkUint16`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_weldingInfo: u16,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSimpleMeshShapeTriangle {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSimpleMeshShapeTriangle"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3548854465u32)
        }
    }
    impl __serde::Serialize for hkpSimpleMeshShapeTriangle {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSimpleMeshShapeTriangle", class_meta)?;
            serializer.serialize_field("a", &self.m_a)?;
            serializer.serialize_field("b", &self.m_b)?;
            serializer.serialize_field("c", &self.m_c)?;
            serializer.serialize_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.end()
        }
    }
};
