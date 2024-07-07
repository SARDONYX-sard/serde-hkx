use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbLookAtModifierInternalState`
/// -         version: `0`
/// -       signature: `0xa14caba6`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbLookAtModifierInternalState {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `lookAtLastTargetWS`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_lookAtLastTargetWS: Vector4,
    /// # C++ Info
    /// -          name: `lookAtWeight`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_lookAtWeight: f32,
    /// # C++ Info
    /// -          name: `isTargetInsideLimitCone`(ctype: `hkBool`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isTargetInsideLimitCone: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbLookAtModifierInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbLookAtModifierInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa14caba6)
        }
    }
    impl _serde::Serialize for hkbLookAtModifierInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa14caba6)));
            let mut serializer = __serializer
                .serialize_struct("hkbLookAtModifierInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field("lookAtLastTargetWS", &self.m_lookAtLastTargetWS)?;
            serializer.serialize_field("lookAtWeight", &self.m_lookAtWeight)?;
            serializer
                .serialize_field(
                    "isTargetInsideLimitCone",
                    &self.m_isTargetInsideLimitCone,
                )?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer.end()
        }
    }
};
