use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultAnalogDriverInput`
/// -         version: `0`
/// -       signature: `0x123a5d50`
/// -          size:  24(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultAnalogDriverInput {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleDriverInput,
    /// # C++ Info
    /// -          name: `slopeChangePointX`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_slopeChangePointX: f32,
    /// # C++ Info
    /// -          name: `initialSlope`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_initialSlope: f32,
    /// # C++ Info
    /// -          name: `deadZone`(ctype: `hkReal`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_deadZone: f32,
    /// # C++ Info
    /// -          name: `autoReverse`(ctype: `hkBool`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_autoReverse: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpVehicleDefaultAnalogDriverInput {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultAnalogDriverInput"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(305814864u32)
        }
    }
    impl __serde::Serialize for hkpVehicleDefaultAnalogDriverInput {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(305814864u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultAnalogDriverInput", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("slopeChangePointX", &self.m_slopeChangePointX)?;
            serializer.serialize_field("initialSlope", &self.m_initialSlope)?;
            serializer.serialize_field("deadZone", &self.m_deadZone)?;
            serializer.serialize_field("autoReverse", &self.m_autoReverse)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
