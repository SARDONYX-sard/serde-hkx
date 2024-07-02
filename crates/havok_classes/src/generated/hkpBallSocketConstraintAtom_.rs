use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBallSocketConstraintAtom`
/// -         version: `3`
/// -       signature: `0xe70e4dfa`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBallSocketConstraintAtom {
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
    /// -          name: `solvingMethod`(ctype: `enum SolvingMethod`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_solvingMethod: SolvingMethod,
    /// # C++ Info
    /// -          name: `bodiesToNotify`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bodiesToNotify: u8,
    /// # C++ Info
    /// -          name: `velocityStabilizationFactor`(ctype: `hkUint8`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_velocityStabilizationFactor: u8,
    /// # C++ Info
    /// -          name: `maxImpulse`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxImpulse: f32,
    /// # C++ Info
    /// -          name: `inertiaStabilizationFactor`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_inertiaStabilizationFactor: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpBallSocketConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBallSocketConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3876474362u32)
        }
    }
    impl __serde::Serialize for hkpBallSocketConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3876474362u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpBallSocketConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("solvingMethod", &self.m_solvingMethod)?;
            serializer.serialize_field("bodiesToNotify", &self.m_bodiesToNotify)?;
            serializer
                .serialize_field(
                    "velocityStabilizationFactor",
                    &self.m_velocityStabilizationFactor,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("maxImpulse", &self.m_maxImpulse)?;
            serializer
                .serialize_field(
                    "inertiaStabilizationFactor",
                    &self.m_inertiaStabilizationFactor,
                )?;
            serializer.end()
        }
    }
};
