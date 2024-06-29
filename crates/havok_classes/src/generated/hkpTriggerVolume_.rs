use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTriggerVolume`
/// -         version: `0`
/// -       signature: `0xa29a8d1a`
/// -          size:  52(x86)/ 88(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTriggerVolume {
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
    /// -          name: `overlappingBodies`(ctype: `hkArray<hkpRigidBody*>`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_overlappingBodies: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `eventQueue`(ctype: `hkArray<struct hkpTriggerVolumeEventInfo>`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_eventQueue: Vec<hkpTriggerVolumeEventInfo>,
    /// # C++ Info
    /// -          name: `triggerBody`(ctype: `struct hkpRigidBody*`)
    /// -        offset:  44(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_triggerBody: Pointer,
    /// # C++ Info
    /// -          name: `sequenceNumber`(ctype: `hkUint32`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sequenceNumber: u32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpTriggerVolume {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpTriggerVolume"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2728037658u32)
        }
    }
    impl __serde::Serialize for hkpTriggerVolume {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpTriggerVolume", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 24usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "overlappingBodies",
                    &self.m_overlappingBodies,
                )?;
            serializer.serialize_array_meta_field("eventQueue", &self.m_eventQueue)?;
            serializer.serialize_field("triggerBody", &self.m_triggerBody)?;
            serializer.serialize_field("sequenceNumber", &self.m_sequenceNumber)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field("overlappingBodies", &self.m_overlappingBodies)?;
            serializer.serialize_array_field("eventQueue", &self.m_eventQueue)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT32`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum Operation {
    #[default]
    ADDED_OP = 0isize,
    REMOVED_OP = 1isize,
    CONTACT_OP = 2isize,
    TOI_OP = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Operation {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ADDED_OP => __serializer.serialize_field("ADDED_OP", &0u64),
                Self::REMOVED_OP => __serializer.serialize_field("REMOVED_OP", &1u64),
                Self::CONTACT_OP => __serializer.serialize_field("CONTACT_OP", &2u64),
                Self::TOI_OP => __serializer.serialize_field("TOI_OP", &3u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i32()
                .ok_or(S::Error::custom("Failed enum Operation to_i32"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
