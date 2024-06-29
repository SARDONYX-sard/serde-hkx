use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkCustomAttributes`
/// -         version: `0`
/// -       signature: `0xbff19005`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkCustomAttributes<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `attributes`(ctype: `hkSimpleArray<struct hkCustomAttributesAttribute>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/ 12(x86_64)
    ///
    pub m_attributes: Vec<hkCustomAttributesAttribute<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkCustomAttributes<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkCustomAttributes"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3220279301u32)
        }
    }
    impl<'a> __serde::Serialize for hkCustomAttributes<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkCustomAttributes", class_meta)?;
            serializer.serialize_field("attributes", &self.m_attributes)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
