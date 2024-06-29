use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVelocityConstraintMotor`
/// -         version: `0`
/// -       signature: `0xfca2fcc3`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVelocityConstraintMotor {
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
    /// -          name: `velocityTarget`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_velocityTarget: f32,
    /// # C++ Info
    /// -          name: `useVelocityTargetFromConstraintTargets`(ctype: `hkBool`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useVelocityTargetFromConstraintTargets: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVelocityConstraintMotor {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpVelocityConstraintMotor"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4238539971u32)
        }
    }
    impl __serde::Serialize for hkpVelocityConstraintMotor {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpVelocityConstraintMotor", class_meta)?;
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
            serializer.serialize_field("velocityTarget", &self.m_velocityTarget)?;
            serializer
                .serialize_field(
                    "useVelocityTargetFromConstraintTargets",
                    &self.m_useVelocityTargetFromConstraintTargets,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
