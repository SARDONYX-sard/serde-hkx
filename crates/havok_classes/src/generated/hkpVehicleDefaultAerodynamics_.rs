use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultAerodynamics`
/// -         version: `0`
/// -       signature: `0x42fc5bbd`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultAerodynamics {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleAerodynamics,
    /// # C++ Info
    /// -          name: `airDensity`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_airDensity: f32,
    /// # C++ Info
    /// -          name: `frontalArea`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_frontalArea: f32,
    /// # C++ Info
    /// -          name: `dragCoefficient`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dragCoefficient: f32,
    /// # C++ Info
    /// -          name: `liftCoefficient`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_liftCoefficient: f32,
    /// # C++ Info
    /// -          name: `extraGravityws`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_extraGravityws: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultAerodynamics {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultAerodynamics"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1123834813u32)
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultAerodynamics {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1123834813u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultAerodynamics", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("airDensity", &self.m_airDensity)?;
            serializer.serialize_field("frontalArea", &self.m_frontalArea)?;
            serializer.serialize_field("dragCoefficient", &self.m_dragCoefficient)?;
            serializer.serialize_field("liftCoefficient", &self.m_liftCoefficient)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("extraGravityws", &self.m_extraGravityws)?;
            serializer.end()
        }
    }
};
