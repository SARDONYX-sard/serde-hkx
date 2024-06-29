use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbStateMachineTimeInterval`
/// -         version: `0`
/// -       signature: `0x60a881e5`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineTimeInterval {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `enterEventId`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_enterEventId: i32,
    /// # C++ Info
    /// -          name: `exitEventId`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_exitEventId: i32,
    /// # C++ Info
    /// -          name: `enterTime`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_enterTime: f32,
    /// # C++ Info
    /// -          name: `exitTime`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_exitTime: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbStateMachineTimeInterval {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbStateMachineTimeInterval"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1621656037u32)
        }
    }
    impl __serde::Serialize for hkbStateMachineTimeInterval {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineTimeInterval", class_meta)?;
            serializer.serialize_field("enterEventId", &self.m_enterEventId)?;
            serializer.serialize_field("exitEventId", &self.m_exitEventId)?;
            serializer.serialize_field("enterTime", &self.m_enterTime)?;
            serializer.serialize_field("exitTime", &self.m_exitTime)?;
            serializer.end()
        }
    }
};
