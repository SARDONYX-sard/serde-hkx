use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterSkinInfo`
/// -         version: `2`
/// -       signature: `0x180d900d`
/// -          size:  40(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterSkinInfo {
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
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `deformableSkins`(ctype: `hkArray<hkUint64>`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_deformableSkins: Vec<u64>,
    /// # C++ Info
    /// -          name: `rigidSkins`(ctype: `hkArray<hkUint64>`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rigidSkins: Vec<u64>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCharacterSkinInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterSkinInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x180d900d)
        }
    }
    impl _serde::Serialize for hkbCharacterSkinInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x180d900d)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterSkinInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer
                .serialize_array_meta_field("deformableSkins", &self.m_deformableSkins)?;
            serializer.serialize_array_meta_field("rigidSkins", &self.m_rigidSkins)?;
            serializer
                .serialize_array_field("deformableSkins", &self.m_deformableSkins)?;
            serializer.serialize_array_field("rigidSkins", &self.m_rigidSkins)?;
            serializer.end()
        }
    }
};
