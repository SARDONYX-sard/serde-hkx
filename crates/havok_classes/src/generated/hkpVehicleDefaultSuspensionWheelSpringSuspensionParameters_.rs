use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters`
/// -         version: `0`
/// -       signature: `0x7be5bed1`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `strength`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_strength: f32,
    /// # C++ Info
    /// -          name: `dampingCompression`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dampingCompression: f32,
    /// # C++ Info
    /// -          name: `dampingRelaxation`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dampingRelaxation: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass
    for hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2078654161u32)
        }
    }
    impl __serde::Serialize
    for hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2078654161u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters",
                    class_meta,
                )?;
            serializer.serialize_field("strength", &self.m_strength)?;
            serializer
                .serialize_field("dampingCompression", &self.m_dampingCompression)?;
            serializer.serialize_field("dampingRelaxation", &self.m_dampingRelaxation)?;
            serializer.end()
        }
    }
};
