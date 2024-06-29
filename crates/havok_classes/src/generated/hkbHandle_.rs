use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbHandle`
/// -         version: `1`
/// -       signature: `0xd8b6401c`
/// -          size:  24(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbHandle {
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
    /// -          name: `frame`(ctype: `struct hkLocalFrame*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_frame: Pointer,
    /// # C++ Info
    /// -          name: `rigidBody`(ctype: `struct hkpRigidBody*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rigidBody: Pointer,
    /// # C++ Info
    /// -          name: `character`(ctype: `struct hkbCharacter*`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_character: Pointer,
    /// # C++ Info
    /// -          name: `animationBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_animationBoneIndex: i16,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbHandle {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbHandle"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3635822620u32)
        }
    }
    impl __serde::Serialize for hkbHandle {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer.serialize_struct("hkbHandle", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("frame", &self.m_frame)?;
            serializer.serialize_field("rigidBody", &self.m_rigidBody)?;
            serializer.serialize_field("character", &self.m_character)?;
            serializer
                .serialize_field("animationBoneIndex", &self.m_animationBoneIndex)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.end()
        }
    }
};
