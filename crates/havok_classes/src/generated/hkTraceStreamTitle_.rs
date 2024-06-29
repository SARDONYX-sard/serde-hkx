use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkTraceStreamTitle`
/// -         version: `0`
/// -       signature: `0x6a4ca82c`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkTraceStreamTitle {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `value`(ctype: `hkChar[32]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_value: [char; 32usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkTraceStreamTitle {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkTraceStreamTitle"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1783408684u32)
        }
    }
    impl __serde::Serialize for hkTraceStreamTitle {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkTraceStreamTitle", class_meta)?;
            serializer.serialize_field("value", &self.m_value.as_slice())?;
            serializer.end()
        }
    }
};
