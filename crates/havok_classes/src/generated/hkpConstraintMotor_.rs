use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConstraintMotor`
/// -         version: `0`
/// -       signature: `0x6a44c317`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintMotor {
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
    /// -          name: `type`(ctype: `enum MotorType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: MotorType,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpConstraintMotor {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpConstraintMotor"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1782891287u32)
        }
    }
    impl __serde::Serialize for hkpConstraintMotor {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintMotor", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
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
pub enum MotorType {
    #[default]
    TYPE_INVALID = 0isize,
    TYPE_POSITION = 1isize,
    TYPE_VELOCITY = 2isize,
    TYPE_SPRING_DAMPER = 3isize,
    TYPE_CALLBACK = 4isize,
    TYPE_MAX = 5isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MotorType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TYPE_INVALID => __serializer.serialize_field("TYPE_INVALID", &0u64),
                Self::TYPE_POSITION => {
                    __serializer.serialize_field("TYPE_POSITION", &1u64)
                }
                Self::TYPE_VELOCITY => {
                    __serializer.serialize_field("TYPE_VELOCITY", &2u64)
                }
                Self::TYPE_SPRING_DAMPER => {
                    __serializer.serialize_field("TYPE_SPRING_DAMPER", &3u64)
                }
                Self::TYPE_CALLBACK => {
                    __serializer.serialize_field("TYPE_CALLBACK", &4u64)
                }
                Self::TYPE_MAX => __serializer.serialize_field("TYPE_MAX", &5u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum MotorType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
