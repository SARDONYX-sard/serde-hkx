use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDriverInputAnalogStatus`
/// -         version: `0`
/// -       signature: `0x2b4a5803`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDriverInputAnalogStatus {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleDriverInputStatus,
    /// # C++ Info
    /// -          name: `positionX`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_positionX: f32,
    /// # C++ Info
    /// -          name: `positionY`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_positionY: f32,
    /// # C++ Info
    /// -          name: `handbrakeButtonPressed`(ctype: `hkBool`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_handbrakeButtonPressed: bool,
    /// # C++ Info
    /// -          name: `reverseButtonPressed`(ctype: `hkBool`)
    /// -        offset:  17(x86)/ 25(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_reverseButtonPressed: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDriverInputAnalogStatus {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDriverInputAnalogStatus"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(726292483u32)
        }
    }
    impl _serde::Serialize for hkpVehicleDriverInputAnalogStatus {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(726292483u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDriverInputAnalogStatus", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("positionX", &self.m_positionX)?;
            serializer.serialize_field("positionY", &self.m_positionY)?;
            serializer
                .serialize_field(
                    "handbrakeButtonPressed",
                    &self.m_handbrakeButtonPressed,
                )?;
            serializer
                .serialize_field("reverseButtonPressed", &self.m_reverseButtonPressed)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.end()
        }
    }
};
