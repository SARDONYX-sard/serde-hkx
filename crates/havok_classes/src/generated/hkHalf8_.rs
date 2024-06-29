use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkHalf8`
/// -         version: `1`
/// -       signature: `0x7684dc80`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkHalf8 {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `quad`(ctype: `hkHalf[8]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_quad: [f16; 8usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkHalf8 {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkHalf8"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1988418688u32)
        }
    }
    impl __serde::Serialize for hkHalf8 {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer.serialize_struct("hkHalf8", class_meta)?;
            serializer.serialize_field("quad", &self.m_quad.as_slice())?;
            serializer.end()
        }
    }
};
