use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMonitorStreamColorTable`
/// -         version: `0`
/// -       signature: `0x79e53e85`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMonitorStreamColorTable<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `colorPairs`(ctype: `hkArray<struct hkMonitorStreamColorTableColorPair>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_colorPairs: Vec<hkMonitorStreamColorTableColorPair<'a>>,
    /// # C++ Info
    /// -          name: `defaultColor`(ctype: `hkUint32`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_defaultColor: u32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkMonitorStreamColorTable<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMonitorStreamColorTable"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2045066885u32)
        }
    }
    impl<'a> __serde::Serialize for hkMonitorStreamColorTable<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMonitorStreamColorTable", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("colorPairs", &self.m_colorPairs)?;
            serializer.serialize_field("defaultColor", &self.m_defaultColor)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("colorPairs", &self.m_colorPairs)?;
            serializer.end()
        }
    }
};
