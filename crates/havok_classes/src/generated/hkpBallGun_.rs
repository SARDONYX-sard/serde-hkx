use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBallGun`
/// -         version: `0`
/// -       signature: `0x57b06d35`
/// -          size:  96(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBallGun<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpFirstPersonGun<'a>,
    /// # C++ Info
    /// -          name: `bulletRadius`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bulletRadius: f32,
    /// # C++ Info
    /// -          name: `bulletVelocity`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bulletVelocity: f32,
    /// # C++ Info
    /// -          name: `bulletMass`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bulletMass: f32,
    /// # C++ Info
    /// -          name: `damageMultiplier`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_damageMultiplier: f32,
    /// # C++ Info
    /// -          name: `maxBulletsInWorld`(ctype: `hkInt32`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxBulletsInWorld: i32,
    /// # C++ Info
    /// -          name: `bulletOffsetFromCenter`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_bulletOffsetFromCenter: Vector4,
    /// # C++ Info
    /// -          name: `addedBodies`(ctype: `void*`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_addedBodies: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpBallGun<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBallGun"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1471180085u32)
        }
    }
    impl<'a> _serde::Serialize for hkpBallGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1471180085u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpBallGun", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("keyboardKey", &self.parent.m_keyboardKey)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_array_meta_field("listeners", &self.parent.m_listeners)?;
            serializer.serialize_field("bulletRadius", &self.m_bulletRadius)?;
            serializer.serialize_field("bulletVelocity", &self.m_bulletVelocity)?;
            serializer.serialize_field("bulletMass", &self.m_bulletMass)?;
            serializer.serialize_field("damageMultiplier", &self.m_damageMultiplier)?;
            serializer.serialize_field("maxBulletsInWorld", &self.m_maxBulletsInWorld)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "bulletOffsetFromCenter",
                    &self.m_bulletOffsetFromCenter,
                )?;
            serializer.skip_field("addedBodies", &self.m_addedBodies)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("listeners", &self.parent.m_listeners)?;
            serializer.end()
        }
    }
};
