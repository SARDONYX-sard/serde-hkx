use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxAttributeGroup`
/// -         version: `0`
/// -       signature: `0x345ca95d`
/// -          size:  16(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxAttributeGroup<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `attributes`(ctype: `hkArray<struct hkxAttribute>`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_attributes: Vec<hkxAttribute<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkxAttributeGroup<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxAttributeGroup"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(878487901u32)
        }
    }
    impl<'a> __serde::Serialize for hkxAttributeGroup<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxAttributeGroup", class_meta)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_array_meta_field("attributes", &self.m_attributes)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("attributes", &self.m_attributes)?;
            serializer.end()
        }
    }
};
