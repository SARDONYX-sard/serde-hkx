use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpEntitySpuCollisionCallback`
/// -         version: `0`
/// -       signature: `0x81147f05`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpEntitySpuCollisionCallback {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `util`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_util: Pointer,
    /// # C++ Info
    /// -          name: `capacity`(ctype: `hkUint16`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_capacity: u16,
    /// # C++ Info
    /// -          name: `eventFilter`(ctype: `hkUint8`)
    /// -        offset:   6(x86)/ 10(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_eventFilter: u8,
    /// # C++ Info
    /// -          name: `userFilter`(ctype: `hkUint8`)
    /// -        offset:   7(x86)/ 11(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_userFilter: u8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpEntitySpuCollisionCallback {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpEntitySpuCollisionCallback"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2165604101u32)
        }
    }
    impl __serde::Serialize for hkpEntitySpuCollisionCallback {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpEntitySpuCollisionCallback", class_meta)?;
            serializer.skip_field("util", &self.m_util)?;
            serializer.skip_field("capacity", &self.m_capacity)?;
            serializer.serialize_field("eventFilter", &self.m_eventFilter)?;
            serializer.serialize_field("userFilter", &self.m_userFilter)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
