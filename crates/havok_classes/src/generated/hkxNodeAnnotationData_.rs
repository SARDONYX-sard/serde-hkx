use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxNodeAnnotationData`
/// -         version: `0`
/// -       signature: `0x433dee92`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxNodeAnnotationData<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_time: f32,
    /// # C++ Info
    /// -          name: `description`(ctype: `hkStringPtr`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_description: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkxNodeAnnotationData<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxNodeAnnotationData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1128132242u32)
        }
    }
    impl<'a> __serde::Serialize for hkxNodeAnnotationData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxNodeAnnotationData", class_meta)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field("description", &self.m_description)?;
            serializer.serialize_stringptr_field("description", &self.m_description)?;
            serializer.end()
        }
    }
};
