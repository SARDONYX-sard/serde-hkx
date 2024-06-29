use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbAttachmentSetup`
/// -         version: `1`
/// -       signature: `0x774632b`
/// -          size:  40(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbAttachmentSetup {
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
    /// -          name: `blendInTime`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blendInTime: f32,
    /// # C++ Info
    /// -          name: `moveAttacherFraction`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_moveAttacherFraction: f32,
    /// # C++ Info
    /// -          name: `gain`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_gain: f32,
    /// # C++ Info
    /// -          name: `extrapolationTimeStep`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_extrapolationTimeStep: f32,
    /// # C++ Info
    /// -          name: `fixUpGain`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fixUpGain: f32,
    /// # C++ Info
    /// -          name: `maxLinearDistance`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxLinearDistance: f32,
    /// # C++ Info
    /// -          name: `maxAngularDistance`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAngularDistance: f32,
    /// # C++ Info
    /// -          name: `attachmentType`(ctype: `enum AttachmentType`)
    /// -        offset:  36(x86)/ 44(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_attachmentType: AttachmentType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbAttachmentSetup {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbAttachmentSetup"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(125068075u32)
        }
    }
    impl __serde::Serialize for hkbAttachmentSetup {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbAttachmentSetup", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("blendInTime", &self.m_blendInTime)?;
            serializer
                .serialize_field("moveAttacherFraction", &self.m_moveAttacherFraction)?;
            serializer.serialize_field("gain", &self.m_gain)?;
            serializer
                .serialize_field(
                    "extrapolationTimeStep",
                    &self.m_extrapolationTimeStep,
                )?;
            serializer.serialize_field("fixUpGain", &self.m_fixUpGain)?;
            serializer.serialize_field("maxLinearDistance", &self.m_maxLinearDistance)?;
            serializer
                .serialize_field("maxAngularDistance", &self.m_maxAngularDistance)?;
            serializer.serialize_field("attachmentType", &self.m_attachmentType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
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
pub enum AttachmentType {
    #[default]
    ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY = 0isize,
    ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT = 1isize,
    ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT = 2isize,
    ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL = 3isize,
    ATTACHMENT_TYPE_NONE = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for AttachmentType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY => {
                    __serializer
                        .serialize_field("ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY", &0u64)
                }
                Self::ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT => {
                    __serializer
                        .serialize_field("ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT", &1u64)
                }
                Self::ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT => {
                    __serializer
                        .serialize_field("ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT", &2u64)
                }
                Self::ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL => {
                    __serializer
                        .serialize_field("ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL", &3u64)
                }
                Self::ATTACHMENT_TYPE_NONE => {
                    __serializer.serialize_field("ATTACHMENT_TYPE_NONE", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum AttachmentType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
