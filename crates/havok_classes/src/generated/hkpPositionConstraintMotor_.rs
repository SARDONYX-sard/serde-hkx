use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPositionConstraintMotor`
/// -         version: `0`
/// -       signature: `0x748fb303`
/// -          size:  36(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPositionConstraintMotor {
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
    /// -          name: `tau`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_tau: f32,
    /// # C++ Info
    /// -          name: `damping`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_damping: f32,
    /// # C++ Info
    /// -          name: `proportionalRecoveryVelocity`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_proportionalRecoveryVelocity: f32,
    /// # C++ Info
    /// -          name: `constantRecoveryVelocity`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_constantRecoveryVelocity: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPositionConstraintMotor {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPositionConstraintMotor"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x748fb303)
        }
    }
    impl _serde::Serialize for hkpPositionConstraintMotor {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x748fb303)));
            let mut serializer = __serializer
                .serialize_struct("hkpPositionConstraintMotor", class_meta)?;
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
            serializer.serialize_field("tau", &self.m_tau)?;
            serializer.serialize_field("damping", &self.m_damping)?;
            serializer
                .serialize_field(
                    "proportionalRecoveryVelocity",
                    &self.m_proportionalRecoveryVelocity,
                )?;
            serializer
                .serialize_field(
                    "constantRecoveryVelocity",
                    &self.m_constantRecoveryVelocity,
                )?;
            serializer.end()
        }
    }
};
