use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCameraShakeEventPayload`
/// -         version: `0`
/// -       signature: `0x64136982`
/// -          size:  16(x86)/ 24(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCameraShakeEventPayload {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbEventPayload,
    /// # C++ Info
    /// -          name: `amplitude`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_amplitude: f32,
    /// # C++ Info
    /// -          name: `halfLife`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_halfLife: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCameraShakeEventPayload {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCameraShakeEventPayload"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1678993794u32)
        }
    }
    impl _serde::Serialize for hkbCameraShakeEventPayload {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1678993794u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbCameraShakeEventPayload", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("amplitude", &self.m_amplitude)?;
            serializer.serialize_field("halfLife", &self.m_halfLife)?;
            serializer.end()
        }
    }
};
