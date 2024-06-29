use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpNamedMeshMaterial`
/// -         version: `0`
/// -       signature: `0x66b42df1`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpNamedMeshMaterial<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpMeshMaterial,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpNamedMeshMaterial<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpNamedMeshMaterial"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1723084273u32)
        }
    }
    impl<'a> __serde::Serialize for hkpNamedMeshMaterial<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpNamedMeshMaterial", class_meta)?;
            serializer.serialize_field("filterInfo", &self.parent.m_filterInfo)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
