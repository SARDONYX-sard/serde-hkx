use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConeLimitConstraintAtom`
/// -         version: `0`
/// -       signature: `0xf19443c8`
/// -          size:  20(x86)/ 20(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConeLimitConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// -          name: `isEnabled`(ctype: `hkUint8`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isEnabled: u8,
    /// # C++ Info
    /// -          name: `twistAxisInA`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_twistAxisInA: u8,
    /// # C++ Info
    /// -          name: `refAxisInB`(ctype: `hkUint8`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_refAxisInB: u8,
    /// # C++ Info
    /// -          name: `angleMeasurementMode`(ctype: `enum MeasurementMode`)
    /// -        offset:   5(x86)/  5(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_angleMeasurementMode: MeasurementMode,
    /// # C++ Info
    /// -          name: `memOffsetToAngleOffset`(ctype: `hkUint8`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_memOffsetToAngleOffset: u8,
    /// # C++ Info
    /// -          name: `minAngle`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minAngle: f32,
    /// # C++ Info
    /// -          name: `maxAngle`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAngle: f32,
    /// # C++ Info
    /// -          name: `angularLimitsTauFactor`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_angularLimitsTauFactor: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpConeLimitConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpConeLimitConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4053025736u32)
        }
    }
    impl __serde::Serialize for hkpConeLimitConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpConeLimitConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.serialize_field("twistAxisInA", &self.m_twistAxisInA)?;
            serializer.serialize_field("refAxisInB", &self.m_refAxisInB)?;
            serializer
                .serialize_field("angleMeasurementMode", &self.m_angleMeasurementMode)?;
            serializer
                .serialize_field(
                    "memOffsetToAngleOffset",
                    &self.m_memOffsetToAngleOffset,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("minAngle", &self.m_minAngle)?;
            serializer.serialize_field("maxAngle", &self.m_maxAngle)?;
            serializer
                .serialize_field(
                    "angularLimitsTauFactor",
                    &self.m_angularLimitsTauFactor,
                )?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT8`
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
pub enum MeasurementMode {
    #[default]
    ZERO_WHEN_VECTORS_ALIGNED = 0isize,
    ZERO_WHEN_VECTORS_PERPENDICULAR = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MeasurementMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ZERO_WHEN_VECTORS_ALIGNED => {
                    __serializer.serialize_field("ZERO_WHEN_VECTORS_ALIGNED", &0u64)
                }
                Self::ZERO_WHEN_VECTORS_PERPENDICULAR => {
                    __serializer
                        .serialize_field("ZERO_WHEN_VECTORS_PERPENDICULAR", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum MeasurementMode to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
