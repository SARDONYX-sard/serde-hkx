use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEventProperty`
/// -         version: `1`
/// -       signature: `0xdb38a15`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEventProperty {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbEventBase,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbEventProperty {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbEventProperty"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(229870101u32)
        }
    }
    impl __serde::Serialize for hkbEventProperty {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbEventProperty", class_meta)?;
            serializer.serialize_field("id", &self.parent.m_id)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("payload", &self.parent.m_payload)?;
            serializer.end()
        }
    }
};
