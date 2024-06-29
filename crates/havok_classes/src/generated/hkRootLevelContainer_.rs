use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkRootLevelContainer`
/// -         version: `0`
/// -       signature: `0x2772c11e`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkRootLevelContainer<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `namedVariants`(ctype: `hkArray<struct hkRootLevelContainerNamedVariant>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_namedVariants: Vec<hkRootLevelContainerNamedVariant<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkRootLevelContainer<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkRootLevelContainer"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(661831966u32)
        }
    }
    impl<'a> __serde::Serialize for hkRootLevelContainer<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkRootLevelContainer", class_meta)?;
            serializer
                .serialize_array_meta_field("namedVariants", &self.m_namedVariants)?;
            serializer.serialize_array_field("namedVariants", &self.m_namedVariants)?;
            serializer.end()
        }
    }
};