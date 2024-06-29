use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkRootLevelContainerNamedVariant`
/// -         version: `0`
/// -       signature: `0xb103a2cd`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkRootLevelContainerNamedVariant<'a> {
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
    /// -          name: `className`(ctype: `hkStringPtr`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_className: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `variant`(ctype: `struct hkReferencedObject*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_variant: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkRootLevelContainerNamedVariant<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkRootLevelContainerNamedVariant"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2969805517u32)
        }
    }
    impl<'a> __serde::Serialize for hkRootLevelContainerNamedVariant<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkRootLevelContainerNamedVariant", class_meta)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_stringptr_meta_field("className", &self.m_className)?;
            serializer.serialize_field("variant", &self.m_variant)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_stringptr_field("className", &self.m_className)?;
            serializer.end()
        }
    }
};