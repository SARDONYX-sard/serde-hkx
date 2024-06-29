use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleLinearCastWheelCollideWheelState`
/// -         version: `0`
/// -       signature: `0x2a9acf98`
/// -          size:  96(x86)/ 96(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleLinearCastWheelCollideWheelState {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `phantom`(ctype: `struct hkpAabbPhantom*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_phantom: Pointer,
    /// # C++ Info
    /// -          name: `shape`(ctype: `struct hkpShape*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_shape: Pointer,
    /// # C++ Info
    /// -          name: `transform`(ctype: `hkTransform`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_transform: Transform,
    /// # C++ Info
    /// -          name: `to`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_to: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleLinearCastWheelCollideWheelState {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpVehicleLinearCastWheelCollideWheelState"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(714788760u32)
        }
    }
    impl __serde::Serialize for hkpVehicleLinearCastWheelCollideWheelState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpVehicleLinearCastWheelCollideWheelState",
                    class_meta,
                )?;
            serializer.serialize_field("phantom", &self.m_phantom)?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer.serialize_field("to", &self.m_to)?;
            serializer.end()
        }
    }
};
