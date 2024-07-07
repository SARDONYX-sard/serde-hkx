use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRagdollMotorConstraintAtom`
/// -         version: `0`
/// -       signature: `0x71013826`
/// -          size:  80(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRagdollMotorConstraintAtom {
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
    /// -          name: `initializedOffset`(ctype: `hkInt16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_initializedOffset: i16,
    /// # C++ Info
    /// -          name: `previousTargetAnglesOffset`(ctype: `hkInt16`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_previousTargetAnglesOffset: i16,
    /// # C++ Info
    /// -          name: `target_bRca`(ctype: `hkMatrix3`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_target_bRca: Matrix3,
    /// # C++ Info
    /// -          name: `motors`(ctype: `struct hkpConstraintMotor*`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_motors: [Pointer; 3usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpRagdollMotorConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRagdollMotorConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x71013826)
        }
    }
    impl _serde::Serialize for hkpRagdollMotorConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x71013826)));
            let mut serializer = __serializer
                .serialize_struct("hkpRagdollMotorConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("initializedOffset", &self.m_initializedOffset)?;
            serializer
                .serialize_field(
                    "previousTargetAnglesOffset",
                    &self.m_previousTargetAnglesOffset,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("target_bRca", &self.m_target_bRca)?;
            serializer.serialize_field("motors", &self.m_motors.as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
