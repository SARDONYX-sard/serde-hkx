use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterControllerControlData`
/// -         version: `0`
/// -       signature: `0x5b6c03d9`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterControllerControlData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `desiredVelocity`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_desiredVelocity: Vector4,
    /// # C++ Info
    /// -          name: `verticalGain`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalGain: f32,
    /// # C++ Info
    /// -          name: `horizontalCatchUpGain`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_horizontalCatchUpGain: f32,
    /// # C++ Info
    /// -          name: `maxVerticalSeparation`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxVerticalSeparation: f32,
    /// # C++ Info
    /// -          name: `maxHorizontalSeparation`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxHorizontalSeparation: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbCharacterControllerControlData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterControllerControlData"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1533805529u32)
        }
    }
    impl __serde::Serialize for hkbCharacterControllerControlData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1533805529u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterControllerControlData", class_meta)?;
            serializer.serialize_field("desiredVelocity", &self.m_desiredVelocity)?;
            serializer.serialize_field("verticalGain", &self.m_verticalGain)?;
            serializer
                .serialize_field(
                    "horizontalCatchUpGain",
                    &self.m_horizontalCatchUpGain,
                )?;
            serializer
                .serialize_field(
                    "maxVerticalSeparation",
                    &self.m_maxVerticalSeparation,
                )?;
            serializer
                .serialize_field(
                    "maxHorizontalSeparation",
                    &self.m_maxHorizontalSeparation,
                )?;
            serializer.end()
        }
    }
};
