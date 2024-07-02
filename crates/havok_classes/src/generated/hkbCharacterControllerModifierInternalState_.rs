use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterControllerModifierInternalState`
/// -         version: `0`
/// -       signature: `0xf8dfec0d`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterControllerModifierInternalState {
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
    /// -          name: `gravity`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_gravity: Vector4,
    /// # C++ Info
    /// -          name: `timestep`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_timestep: f32,
    /// # C++ Info
    /// -          name: `isInitialVelocityAdded`(ctype: `hkBool`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isInitialVelocityAdded: bool,
    /// # C++ Info
    /// -          name: `isTouchingGround`(ctype: `hkBool`)
    /// -        offset:  37(x86)/ 37(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isTouchingGround: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCharacterControllerModifierInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterControllerModifierInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(4175424525u32)
        }
    }
    impl _serde::Serialize for hkbCharacterControllerModifierInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(4175424525u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbCharacterControllerModifierInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("gravity", &self.m_gravity)?;
            serializer.serialize_field("timestep", &self.m_timestep)?;
            serializer
                .serialize_field(
                    "isInitialVelocityAdded",
                    &self.m_isInitialVelocityAdded,
                )?;
            serializer.serialize_field("isTouchingGround", &self.m_isTouchingGround)?;
            serializer.pad_field([0u8; 10usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.end()
        }
    }
};
