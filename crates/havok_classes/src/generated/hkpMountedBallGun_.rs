use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMountedBallGun`
/// -         version: `0`
/// -       signature: `0x6791ffce`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMountedBallGun<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpBallGun<'a>,
    /// # C++ Info
    /// -          name: `position`(ctype: `hkVector4`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_position: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpMountedBallGun<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMountedBallGun"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1737621454u32)
        }
    }
    impl<'a> _serde::Serialize for hkpMountedBallGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1737621454u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpMountedBallGun", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_field("keyboardKey", &self.parent.parent.m_keyboardKey)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .skip_array_meta_field("listeners", &self.parent.parent.m_listeners)?;
            serializer.serialize_field("bulletRadius", &self.parent.m_bulletRadius)?;
            serializer.serialize_field("bulletVelocity", &self.parent.m_bulletVelocity)?;
            serializer.serialize_field("bulletMass", &self.parent.m_bulletMass)?;
            serializer
                .serialize_field("damageMultiplier", &self.parent.m_damageMultiplier)?;
            serializer
                .serialize_field("maxBulletsInWorld", &self.parent.m_maxBulletsInWorld)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "bulletOffsetFromCenter",
                    &self.parent.m_bulletOffsetFromCenter,
                )?;
            serializer.skip_field("addedBodies", &self.parent.m_addedBodies)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("position", &self.m_position)?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field("listeners", &self.parent.parent.m_listeners)?;
            serializer.end()
        }
    }
};
