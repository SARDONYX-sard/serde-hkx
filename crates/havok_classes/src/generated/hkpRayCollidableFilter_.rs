use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRayCollidableFilter`
/// -         version: `0`
/// -       signature: `0xe0708a00`
/// -          size:   4(x86)/  8(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRayCollidableFilter {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpRayCollidableFilter {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpRayCollidableFilter"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3765471744u32)
        }
    }
    impl __serde::Serialize for hkpRayCollidableFilter {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpRayCollidableFilter", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
