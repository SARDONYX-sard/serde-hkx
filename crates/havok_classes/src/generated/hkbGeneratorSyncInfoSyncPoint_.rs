use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbGeneratorSyncInfoSyncPoint`
/// -         version: `0`
/// -       signature: `0xb597cf92`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbGeneratorSyncInfoSyncPoint {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `id`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_id: i32,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_time: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbGeneratorSyncInfoSyncPoint {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbGeneratorSyncInfoSyncPoint"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3046625170u32)
        }
    }
    impl __serde::Serialize for hkbGeneratorSyncInfoSyncPoint {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbGeneratorSyncInfoSyncPoint", class_meta)?;
            serializer.serialize_field("id", &self.m_id)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer.end()
        }
    }
};
