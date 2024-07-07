use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkControlsModifierLeg`
/// -         version: `0`
/// -       signature: `0x9e17091a`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkControlsModifierLeg {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `groundPosition`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_groundPosition: Vector4,
    /// # C++ Info
    /// -          name: `ungroundedEvent`(ctype: `struct hkbEventProperty`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_ungroundedEvent: hkbEventProperty,
    /// # C++ Info
    /// -          name: `verticalError`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalError: f32,
    /// # C++ Info
    /// -          name: `hitSomething`(ctype: `hkBool`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hitSomething: bool,
    /// # C++ Info
    /// -          name: `isPlantedMS`(ctype: `hkBool`)
    /// -        offset:  29(x86)/ 37(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isPlantedMS: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbFootIkControlsModifierLeg {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkControlsModifierLeg"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9e17091a)
        }
    }
    impl _serde::Serialize for hkbFootIkControlsModifierLeg {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9e17091a)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkControlsModifierLeg", class_meta)?;
            serializer.serialize_field("groundPosition", &self.m_groundPosition)?;
            serializer.serialize_field("ungroundedEvent", &self.m_ungroundedEvent)?;
            serializer.serialize_field("verticalError", &self.m_verticalError)?;
            serializer.serialize_field("hitSomething", &self.m_hitSomething)?;
            serializer.serialize_field("isPlantedMS", &self.m_isPlantedMS)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.end()
        }
    }
};
