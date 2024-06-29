use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMonitorStreamStringMapStringMap`
/// -         version: `0`
/// -       signature: `0x2c76ce16`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMonitorStreamStringMapStringMap<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `id`(ctype: `hkUint64`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    /// -         flags: `ALIGN_8`
    ///
    pub m_id: u64,
    /// # C++ Info
    /// -          name: `string`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_string: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkMonitorStreamStringMapStringMap<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMonitorStreamStringMapStringMap"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(745983510u32)
        }
    }
    impl<'a> __serde::Serialize for hkMonitorStreamStringMapStringMap<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMonitorStreamStringMapStringMap", class_meta)?;
            serializer.serialize_field("id", &self.m_id)?;
            serializer.serialize_stringptr_meta_field("string", &self.m_string)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_stringptr_field("string", &self.m_string)?;
            serializer.end()
        }
    }
};
