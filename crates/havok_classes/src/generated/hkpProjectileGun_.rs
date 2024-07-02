use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpProjectileGun`
/// -         version: `0`
/// -       signature: `0xb4f30148`
/// -          size:  64(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpProjectileGun<'a> {
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
    /// -          name: `maxProjectiles`(ctype: `hkInt32`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxProjectiles: i32,
    /// # C++ Info
    /// -          name: `reloadTime`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_reloadTime: f32,
    /// # C++ Info
    /// -          name: `reload`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_reload: f32,
    /// # C++ Info
    /// -          name: `projectiles`(ctype: `hkArray<void*>`)
    /// -        offset:  44(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_projectiles: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `world`(ctype: `void*`)
    /// -        offset:  56(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_world: Pointer,
    /// # C++ Info
    /// -          name: `destructionWorld`(ctype: `void*`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_destructionWorld: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpProjectileGun<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpProjectileGun"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3035824456u32)
        }
    }
    impl<'a> __serde::Serialize for hkpProjectileGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3035824456u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpProjectileGun", class_meta)?;
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
            serializer.serialize_field("maxProjectiles", &self.m_maxProjectiles)?;
            serializer.serialize_field("reloadTime", &self.m_reloadTime)?;
            serializer.skip_field("reload", &self.m_reload)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_array_meta_field("projectiles", &self.m_projectiles)?;
            serializer.skip_field("world", &self.m_world)?;
            serializer.skip_field("destructionWorld", &self.m_destructionWorld)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("listeners", &self.parent.m_listeners)?;
            serializer.serialize_array_field("projectiles", &self.m_projectiles)?;
            serializer.end()
        }
    }
};
