use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTriggerVolumeEventInfo`
/// -         version: `0`
/// -       signature: `0xeb60f431`
/// -          size:  16(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTriggerVolumeEventInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `sortValue`(ctype: `hkUint64`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_sortValue: u64,
    /// # C++ Info
    /// -          name: `body`(ctype: `struct hkpRigidBody*`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_body: Pointer,
    /// # C++ Info
    /// -          name: `operation`(ctype: `enum Operation`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_operation: Operation,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpTriggerVolumeEventInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTriggerVolumeEventInfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3948999729u32)
        }
    }
    impl __serde::Serialize for hkpTriggerVolumeEventInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3948999729u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpTriggerVolumeEventInfo", class_meta)?;
            serializer.serialize_field("sortValue", &self.m_sortValue)?;
            serializer.serialize_field("body", &self.m_body)?;
            serializer.serialize_field("operation", &self.m_operation)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
