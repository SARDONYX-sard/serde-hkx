use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCogWheelConstraintAtom`
/// -         version: `0`
/// -       signature: `0xf2b1f399`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCogWheelConstraintAtom {
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
    /// -          name: `cogWheelRadiusA`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cogWheelRadiusA: f32,
    /// # C++ Info
    /// -          name: `cogWheelRadiusB`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cogWheelRadiusB: f32,
    /// # C++ Info
    /// -          name: `isScrew`(ctype: `hkBool`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isScrew: bool,
    /// # C++ Info
    /// -          name: `memOffsetToInitialAngleOffset`(ctype: `hkInt8`)
    /// -        offset:  13(x86)/ 13(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_memOffsetToInitialAngleOffset: i8,
    /// # C++ Info
    /// -          name: `memOffsetToPrevAngle`(ctype: `hkInt8`)
    /// -        offset:  14(x86)/ 14(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_memOffsetToPrevAngle: i8,
    /// # C++ Info
    /// -          name: `memOffsetToRevolutionCounter`(ctype: `hkInt8`)
    /// -        offset:  15(x86)/ 15(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_memOffsetToRevolutionCounter: i8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCogWheelConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpCogWheelConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4071748505u32)
        }
    }
    impl __serde::Serialize for hkpCogWheelConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpCogWheelConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("cogWheelRadiusA", &self.m_cogWheelRadiusA)?;
            serializer.serialize_field("cogWheelRadiusB", &self.m_cogWheelRadiusB)?;
            serializer.serialize_field("isScrew", &self.m_isScrew)?;
            serializer
                .serialize_field(
                    "memOffsetToInitialAngleOffset",
                    &self.m_memOffsetToInitialAngleOffset,
                )?;
            serializer
                .serialize_field("memOffsetToPrevAngle", &self.m_memOffsetToPrevAngle)?;
            serializer
                .serialize_field(
                    "memOffsetToRevolutionCounter",
                    &self.m_memOffsetToRevolutionCounter,
                )?;
            serializer.end()
        }
    }
};
