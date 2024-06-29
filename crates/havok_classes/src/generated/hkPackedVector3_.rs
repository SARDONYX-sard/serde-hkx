use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkPackedVector3`
/// -         version: `0`
/// -       signature: `0x9c16df5b`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkPackedVector3 {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `values`(ctype: `hkInt16[4]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_values: [i16; 4usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkPackedVector3 {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkPackedVector3"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2618744667u32)
        }
    }
    impl __serde::Serialize for hkPackedVector3 {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkPackedVector3", class_meta)?;
            serializer.serialize_field("values", &self.m_values.as_slice())?;
            serializer.end()
        }
    }
};