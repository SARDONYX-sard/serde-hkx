use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSimulation`
/// -         version: `0`
/// -       signature: `0x97aba922`
/// -          size:  44(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSimulation {
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
    /// -          name: `determinismCheckFrameCounter`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_determinismCheckFrameCounter: u32,
    /// # C++ Info
    /// -          name: `world`(ctype: `struct hkpWorld*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_world: Pointer,
    /// # C++ Info
    /// -          name: `lastProcessingStep`(ctype: `enum LastProcessingStep`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_lastProcessingStep: LastProcessingStep,
    /// # C++ Info
    /// -          name: `currentTime`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_currentTime: f32,
    /// # C++ Info
    /// -          name: `currentPsiTime`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_currentPsiTime: f32,
    /// # C++ Info
    /// -          name: `physicsDeltaTime`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_physicsDeltaTime: f32,
    /// # C++ Info
    /// -          name: `simulateUntilTime`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_simulateUntilTime: f32,
    /// # C++ Info
    /// -          name: `frameMarkerPsiSnap`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_frameMarkerPsiSnap: f32,
    /// # C++ Info
    /// -          name: `previousStepResult`(ctype: `hkUint32`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_previousStepResult: u32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSimulation {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSimulation"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2544609570u32)
        }
    }
    impl __serde::Serialize for hkpSimulation {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSimulation", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "determinismCheckFrameCounter",
                    &self.m_determinismCheckFrameCounter,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("world", &self.m_world)?;
            serializer
                .serialize_field("lastProcessingStep", &self.m_lastProcessingStep)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("currentTime", &self.m_currentTime)?;
            serializer.serialize_field("currentPsiTime", &self.m_currentPsiTime)?;
            serializer.serialize_field("physicsDeltaTime", &self.m_physicsDeltaTime)?;
            serializer.serialize_field("simulateUntilTime", &self.m_simulateUntilTime)?;
            serializer
                .serialize_field("frameMarkerPsiSnap", &self.m_frameMarkerPsiSnap)?;
            serializer
                .serialize_field("previousStepResult", &self.m_previousStepResult)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
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
pub enum LastProcessingStep {
    #[default]
    INTEGRATE = 0isize,
    COLLIDE = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for LastProcessingStep {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::INTEGRATE => __serializer.serialize_field("INTEGRATE", &0u64),
                Self::COLLIDE => __serializer.serialize_field("COLLIDE", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum LastProcessingStep to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
