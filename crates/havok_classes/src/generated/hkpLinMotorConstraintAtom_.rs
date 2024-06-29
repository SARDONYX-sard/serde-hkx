use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpLinMotorConstraintAtom`
/// -         version: `0`
/// -       signature: `0x10312464`
/// -          size:  16(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpLinMotorConstraintAtom {
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
    /// -          name: `previousTargetPositionOffset`(ctype: `hkInt16`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_previousTargetPositionOffset: i16,
    /// # C++ Info
    /// -          name: `targetPosition`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_targetPosition: f32,
    /// # C++ Info
    /// -          name: `motor`(ctype: `struct hkpConstraintMotor*`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_motor: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpLinMotorConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpLinMotorConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(271656036u32)
        }
    }
    impl __serde::Serialize for hkpLinMotorConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpLinMotorConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.serialize_field("motorAxis", &self.m_motorAxis)?;
            serializer.serialize_field("initializedOffset", &self.m_initializedOffset)?;
            serializer
                .serialize_field(
                    "previousTargetPositionOffset",
                    &self.m_previousTargetPositionOffset,
                )?;
            serializer.serialize_field("targetPosition", &self.m_targetPosition)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("motor", &self.m_motor)?;
            serializer.end()
        }
    }
};
