use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEvaluateHandleModifier`
/// -         version: `2`
/// -       signature: `0x79757102`
/// -          size: 176(x86)/240(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEvaluateHandleModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `handle`(ctype: `struct hkbHandle*`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_handle: Pointer,
    /// # C++ Info
    /// -          name: `handlePositionOut`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_handlePositionOut: Vector4,
    /// # C++ Info
    /// -          name: `handleRotationOut`(ctype: `hkQuaternion`)
    /// -        offset:  64(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_handleRotationOut: Quaternion,
    /// # C++ Info
    /// -          name: `isValidOut`(ctype: `hkBool`)
    /// -        offset:  80(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isValidOut: bool,
    /// # C++ Info
    /// -          name: `extrapolationTimeStep`(ctype: `hkReal`)
    /// -        offset:  84(x86)/132(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_extrapolationTimeStep: f32,
    /// # C++ Info
    /// -          name: `handleChangeSpeed`(ctype: `hkReal`)
    /// -        offset:  88(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_handleChangeSpeed: f32,
    /// # C++ Info
    /// -          name: `handleChangeMode`(ctype: `enum HandleChangeMode`)
    /// -        offset:  92(x86)/140(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_handleChangeMode: HandleChangeMode,
    /// # C++ Info
    /// -          name: `oldHandle`(ctype: `struct hkbHandle`)
    /// -        offset:  96(x86)/144(x86_64)
    /// -     type_size:  24(x86)/ 48(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldHandle: hkbHandle,
    /// # C++ Info
    /// -          name: `oldHandlePosition`(ctype: `hkVector4`)
    /// -        offset: 128(x86)/192(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldHandlePosition: Vector4,
    /// # C++ Info
    /// -          name: `oldHandleRotation`(ctype: `hkQuaternion`)
    /// -        offset: 144(x86)/208(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldHandleRotation: Quaternion,
    /// # C++ Info
    /// -          name: `timeSinceLastModify`(ctype: `hkReal`)
    /// -        offset: 160(x86)/224(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeSinceLastModify: f32,
    /// # C++ Info
    /// -          name: `smoothlyChangingHandles`(ctype: `hkBool`)
    /// -        offset: 164(x86)/228(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_smoothlyChangingHandles: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbEvaluateHandleModifier<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbEvaluateHandleModifier"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2037739778u32)
        }
    }
    impl<'a> __serde::Serialize for hkbEvaluateHandleModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbEvaluateHandleModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("handle", &self.m_handle)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("handlePositionOut", &self.m_handlePositionOut)?;
            serializer.serialize_field("handleRotationOut", &self.m_handleRotationOut)?;
            serializer.serialize_field("isValidOut", &self.m_isValidOut)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "extrapolationTimeStep",
                    &self.m_extrapolationTimeStep,
                )?;
            serializer.serialize_field("handleChangeSpeed", &self.m_handleChangeSpeed)?;
            serializer.serialize_field("handleChangeMode", &self.m_handleChangeMode)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("oldHandle", &self.m_oldHandle)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.skip_field("oldHandlePosition", &self.m_oldHandlePosition)?;
            serializer.skip_field("oldHandleRotation", &self.m_oldHandleRotation)?;
            serializer.skip_field("timeSinceLastModify", &self.m_timeSinceLastModify)?;
            serializer
                .skip_field("smoothlyChangingHandles", &self.m_smoothlyChangingHandles)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
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
pub enum HandleChangeMode {
    #[default]
    HANDLE_CHANGE_MODE_ABRUPT = 0isize,
    HANDLE_CHANGE_MODE_CONSTANT_VELOCITY = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for HandleChangeMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HANDLE_CHANGE_MODE_ABRUPT => {
                    __serializer.serialize_field("HANDLE_CHANGE_MODE_ABRUPT", &0u64)
                }
                Self::HANDLE_CHANGE_MODE_CONSTANT_VELOCITY => {
                    __serializer
                        .serialize_field("HANDLE_CHANGE_MODE_CONSTANT_VELOCITY", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum HandleChangeMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
