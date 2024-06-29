use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbHandIkControlData`
/// -         version: `2`
/// -       signature: `0xd72b8d17`
/// -          size:  80(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbHandIkControlData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `targetPosition`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_targetPosition: Vector4,
    /// # C++ Info
    /// -          name: `targetRotation`(ctype: `hkQuaternion`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_targetRotation: Quaternion,
    /// # C++ Info
    /// -          name: `targetNormal`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_targetNormal: Vector4,
    /// # C++ Info
    /// -          name: `targetHandle`(ctype: `struct hkbHandle*`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_targetHandle: Pointer,
    /// # C++ Info
    /// -          name: `transformOnFraction`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_transformOnFraction: f32,
    /// # C++ Info
    /// -          name: `normalOnFraction`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_normalOnFraction: f32,
    /// # C++ Info
    /// -          name: `fadeInDuration`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fadeInDuration: f32,
    /// # C++ Info
    /// -          name: `fadeOutDuration`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fadeOutDuration: f32,
    /// # C++ Info
    /// -          name: `extrapolationTimeStep`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_extrapolationTimeStep: f32,
    /// # C++ Info
    /// -          name: `handleChangeSpeed`(ctype: `hkReal`)
    /// -        offset:  72(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_handleChangeSpeed: f32,
    /// # C++ Info
    /// -          name: `handleChangeMode`(ctype: `enum HandleChangeMode`)
    /// -        offset:  76(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_handleChangeMode: HandleChangeMode,
    /// # C++ Info
    /// -          name: `fixUp`(ctype: `hkBool`)
    /// -        offset:  77(x86)/ 81(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_fixUp: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbHandIkControlData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbHandIkControlData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3609955607u32)
        }
    }
    impl __serde::Serialize for hkbHandIkControlData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbHandIkControlData", class_meta)?;
            serializer.serialize_field("targetPosition", &self.m_targetPosition)?;
            serializer.serialize_field("targetRotation", &self.m_targetRotation)?;
            serializer.serialize_field("targetNormal", &self.m_targetNormal)?;
            serializer.serialize_field("targetHandle", &self.m_targetHandle)?;
            serializer
                .serialize_field("transformOnFraction", &self.m_transformOnFraction)?;
            serializer.serialize_field("normalOnFraction", &self.m_normalOnFraction)?;
            serializer.serialize_field("fadeInDuration", &self.m_fadeInDuration)?;
            serializer.serialize_field("fadeOutDuration", &self.m_fadeOutDuration)?;
            serializer
                .serialize_field(
                    "extrapolationTimeStep",
                    &self.m_extrapolationTimeStep,
                )?;
            serializer.serialize_field("handleChangeSpeed", &self.m_handleChangeSpeed)?;
            serializer.serialize_field("handleChangeMode", &self.m_handleChangeMode)?;
            serializer.serialize_field("fixUp", &self.m_fixUp)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 14usize].as_slice())?;
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
