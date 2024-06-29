use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkControlData`
/// -         version: `0`
/// -       signature: `0xa111b704`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkControlData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `gains`(ctype: `struct hkbFootIkGains`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_gains: hkbFootIkGains,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbFootIkControlData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbFootIkControlData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2702292740u32)
        }
    }
    impl __serde::Serialize for hkbFootIkControlData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkControlData", class_meta)?;
            serializer.serialize_field("gains", &self.m_gains)?;
            serializer.end()
        }
    }
};
