use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMassProperties`
/// -         version: `0`
/// -       signature: `0x68a56834`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMassProperties {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `volume`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_volume: f32,
    /// # C++ Info
    /// -          name: `mass`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_mass: f32,
    /// # C++ Info
    /// -          name: `centerOfMass`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_centerOfMass: Vector4,
    /// # C++ Info
    /// -          name: `inertiaTensor`(ctype: `hkMatrix3`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_inertiaTensor: Matrix3,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpMassProperties {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMassProperties"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1755670580u32)
        }
    }
    impl __serde::Serialize for hkpMassProperties {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1755670580u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpMassProperties", class_meta)?;
            serializer.serialize_field("volume", &self.m_volume)?;
            serializer.serialize_field("mass", &self.m_mass)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("centerOfMass", &self.m_centerOfMass)?;
            serializer.serialize_field("inertiaTensor", &self.m_inertiaTensor)?;
            serializer.end()
        }
    }
};
