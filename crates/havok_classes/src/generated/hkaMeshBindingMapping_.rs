use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaMeshBindingMapping`
/// -         version: `0`
/// -       signature: `0x48aceb75`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaMeshBindingMapping {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `mapping`(ctype: `hkArray<hkInt16>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_mapping: Vec<i16>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkaMeshBindingMapping {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaMeshBindingMapping"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1219292021u32)
        }
    }
    impl __serde::Serialize for hkaMeshBindingMapping {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1219292021u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaMeshBindingMapping", class_meta)?;
            serializer.serialize_array_meta_field("mapping", &self.m_mapping)?;
            serializer.serialize_array_field("mapping", &self.m_mapping)?;
            serializer.end()
        }
    }
};
