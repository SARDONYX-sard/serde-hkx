use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkGeometryTriangle`
/// -         version: `0`
/// -       signature: `0x9687513b`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkGeometryTriangle {
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
    /// -          name: `material`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_material: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkGeometryTriangle {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkGeometryTriangle"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2525450555u32)
        }
    }
    impl __serde::Serialize for hkGeometryTriangle {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkGeometryTriangle", class_meta)?;
            serializer.serialize_field("a", &self.m_a)?;
            serializer.serialize_field("b", &self.m_b)?;
            serializer.serialize_field("c", &self.m_c)?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.end()
        }
    }
};
