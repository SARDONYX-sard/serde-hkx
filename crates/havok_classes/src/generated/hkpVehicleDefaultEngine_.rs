use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultEngine`
/// -         version: `0`
/// -       signature: `0x56f8ca24`
/// -          size:  48(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultEngine {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleEngine,
    /// # C++ Info
    /// -          name: `minRPM`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minRPM: f32,
    /// # C++ Info
    /// -          name: `optRPM`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_optRPM: f32,
    /// # C++ Info
    /// -          name: `maxRPM`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxRPM: f32,
    /// # C++ Info
    /// -          name: `maxTorque`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxTorque: f32,
    /// # C++ Info
    /// -          name: `torqueFactorAtMinRPM`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_torqueFactorAtMinRPM: f32,
    /// # C++ Info
    /// -          name: `torqueFactorAtMaxRPM`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_torqueFactorAtMaxRPM: f32,
    /// # C++ Info
    /// -          name: `resistanceFactorAtMinRPM`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_resistanceFactorAtMinRPM: f32,
    /// # C++ Info
    /// -          name: `resistanceFactorAtOptRPM`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_resistanceFactorAtOptRPM: f32,
    /// # C++ Info
    /// -          name: `resistanceFactorAtMaxRPM`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_resistanceFactorAtMaxRPM: f32,
    /// # C++ Info
    /// -          name: `clutchSlipRPM`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_clutchSlipRPM: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleDefaultEngine {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpVehicleDefaultEngine"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1459145252u32)
        }
    }
    impl __serde::Serialize for hkpVehicleDefaultEngine {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultEngine", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("minRPM", &self.m_minRPM)?;
            serializer.serialize_field("optRPM", &self.m_optRPM)?;
            serializer.serialize_field("maxRPM", &self.m_maxRPM)?;
            serializer.serialize_field("maxTorque", &self.m_maxTorque)?;
            serializer
                .serialize_field("torqueFactorAtMinRPM", &self.m_torqueFactorAtMinRPM)?;
            serializer
                .serialize_field("torqueFactorAtMaxRPM", &self.m_torqueFactorAtMaxRPM)?;
            serializer
                .serialize_field(
                    "resistanceFactorAtMinRPM",
                    &self.m_resistanceFactorAtMinRPM,
                )?;
            serializer
                .serialize_field(
                    "resistanceFactorAtOptRPM",
                    &self.m_resistanceFactorAtOptRPM,
                )?;
            serializer
                .serialize_field(
                    "resistanceFactorAtMaxRPM",
                    &self.m_resistanceFactorAtMaxRPM,
                )?;
            serializer.serialize_field("clutchSlipRPM", &self.m_clutchSlipRPM)?;
            serializer.end()
        }
    }
};
