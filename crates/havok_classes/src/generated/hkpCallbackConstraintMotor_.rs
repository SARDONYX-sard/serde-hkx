use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCallbackConstraintMotor`
/// -         version: `0`
/// -       signature: `0xafcd79ad`
/// -          size:  40(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCallbackConstraintMotor {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpLimitedForceConstraintMotor,
    /// # C++ Info
    /// -          name: `callbackFunc`(ctype: `void*`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_callbackFunc: Pointer,
    /// # C++ Info
    /// -          name: `callbackType`(ctype: `enum CallbackType`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_callbackType: CallbackType,
    /// # C++ Info
    /// -          name: `userData0`(ctype: `hkUlong`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData0: u64,
    /// # C++ Info
    /// -          name: `userData1`(ctype: `hkUlong`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData1: u64,
    /// # C++ Info
    /// -          name: `userData2`(ctype: `hkUlong`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData2: u64,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCallbackConstraintMotor {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpCallbackConstraintMotor"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2949478829u32)
        }
    }
    impl __serde::Serialize for hkpCallbackConstraintMotor {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpCallbackConstraintMotor", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("minForce", &self.parent.m_minForce)?;
            serializer.serialize_field("maxForce", &self.parent.m_maxForce)?;
            serializer.skip_field("callbackFunc", &self.m_callbackFunc)?;
            serializer.serialize_field("callbackType", &self.m_callbackType)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData0", &self.m_userData0)?;
            serializer.serialize_field("userData1", &self.m_userData1)?;
            serializer.serialize_field("userData2", &self.m_userData2)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT32`
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
pub enum CallbackType {
    #[default]
    CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER = 0isize,
    CALLBACK_MOTOR_TYPE_USER_0 = 1isize,
    CALLBACK_MOTOR_TYPE_USER_1 = 2isize,
    CALLBACK_MOTOR_TYPE_USER_2 = 3isize,
    CALLBACK_MOTOR_TYPE_USER_3 = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for CallbackType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER => {
                    __serializer
                        .serialize_field(
                            "CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER",
                            &0u64,
                        )
                }
                Self::CALLBACK_MOTOR_TYPE_USER_0 => {
                    __serializer.serialize_field("CALLBACK_MOTOR_TYPE_USER_0", &1u64)
                }
                Self::CALLBACK_MOTOR_TYPE_USER_1 => {
                    __serializer.serialize_field("CALLBACK_MOTOR_TYPE_USER_1", &2u64)
                }
                Self::CALLBACK_MOTOR_TYPE_USER_2 => {
                    __serializer.serialize_field("CALLBACK_MOTOR_TYPE_USER_2", &3u64)
                }
                Self::CALLBACK_MOTOR_TYPE_USER_3 => {
                    __serializer.serialize_field("CALLBACK_MOTOR_TYPE_USER_3", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u32()
                .ok_or(S::Error::custom("Failed enum CallbackType to_u32"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
