use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpAngMotorConstraintAtom`
/// -         version: `0`
/// -       signature: `0x81f087ff`
/// -          size:  20(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAngMotorConstraintAtom {
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
    /// -          name: `isEnabled`(ctype: `hkBool`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isEnabled: bool,
    /// # C++ Info
    /// -          name: `motorAxis`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_motorAxis: u8,
    /// # C++ Info
    /// -          name: `initializedOffset`(ctype: `hkInt16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_initializedOffset: i16,
    /// # C++ Info
    /// -          name: `previousTargetAngleOffset`(ctype: `hkInt16`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_previousTargetAngleOffset: i16,
    /// # C++ Info
    /// -          name: `correspondingAngLimitSolverResultOffset`(ctype: `hkInt16`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_correspondingAngLimitSolverResultOffset: i16,
    /// # C++ Info
    /// -          name: `targetAngle`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_targetAngle: f32,
    /// # C++ Info
    /// -          name: `motor`(ctype: `struct hkpConstraintMotor*`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_motor: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpAngMotorConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpAngMotorConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2180024319u32)
        }
    }
    impl _serde::Serialize for hkpAngMotorConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2180024319u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpAngMotorConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.serialize_field("motorAxis", &self.m_motorAxis)?;
            serializer.serialize_field("initializedOffset", &self.m_initializedOffset)?;
            serializer
                .serialize_field(
                    "previousTargetAngleOffset",
                    &self.m_previousTargetAngleOffset,
                )?;
            serializer
                .serialize_field(
                    "correspondingAngLimitSolverResultOffset",
                    &self.m_correspondingAngLimitSolverResultOffset,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("targetAngle", &self.m_targetAngle)?;
            serializer.serialize_field("motor", &self.m_motor)?;
            serializer.end()
        }
    }
};
