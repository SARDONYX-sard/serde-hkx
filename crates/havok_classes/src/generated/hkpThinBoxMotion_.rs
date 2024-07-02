use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpThinBoxMotion`
/// -         version: `0`
/// -       signature: `0x64abf85c`
/// -          size: 288(x86)/320(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpThinBoxMotion {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpBoxMotion,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpThinBoxMotion {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpThinBoxMotion"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1688991836u32)
        }
    }
    impl __serde::Serialize for hkpThinBoxMotion {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1688991836u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpThinBoxMotion", class_meta)?;
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
            serializer
                .serialize_field(
                    "deactivationIntegrateCounter",
                    &self.parent.parent.m_deactivationIntegrateCounter,
                )?;
            serializer
                .serialize_field(
                    "deactivationNumInactiveFrames",
                    &self.parent.parent.m_deactivationNumInactiveFrames.as_slice(),
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer
                .serialize_field("motionState", &self.parent.parent.m_motionState)?;
            serializer
                .serialize_field(
                    "inertiaAndMassInv",
                    &self.parent.parent.m_inertiaAndMassInv,
                )?;
            serializer
                .serialize_field(
                    "linearVelocity",
                    &self.parent.parent.m_linearVelocity,
                )?;
            serializer
                .serialize_field(
                    "angularVelocity",
                    &self.parent.parent.m_angularVelocity,
                )?;
            serializer
                .serialize_field(
                    "deactivationRefPosition",
                    &self.parent.parent.m_deactivationRefPosition.as_slice(),
                )?;
            serializer
                .serialize_field(
                    "deactivationRefOrientation",
                    &self.parent.parent.m_deactivationRefOrientation.as_slice(),
                )?;
            serializer
                .serialize_field("savedMotion", &self.parent.parent.m_savedMotion)?;
            serializer
                .serialize_field(
                    "savedQualityTypeIndex",
                    &self.parent.parent.m_savedQualityTypeIndex,
                )?;
            serializer
                .serialize_field("gravityFactor", &self.parent.parent.m_gravityFactor)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
