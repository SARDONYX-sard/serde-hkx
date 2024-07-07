use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbProjectData`
/// -         version: `2`
/// -       signature: `0x13a39ba7`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbProjectData {
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
    /// -          name: `worldUpWS`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_worldUpWS: Vector4,
    /// # C++ Info
    /// -          name: `stringData`(ctype: `struct hkbProjectStringData*`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_stringData: Pointer,
    /// # C++ Info
    /// -          name: `defaultEventMode`(ctype: `enum EventMode`)
    /// -        offset:  36(x86)/ 40(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_defaultEventMode: EventMode,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbProjectData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbProjectData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x13a39ba7)
        }
    }
    impl _serde::Serialize for hkbProjectData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x13a39ba7)));
            let mut serializer = __serializer
                .serialize_struct("hkbProjectData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("worldUpWS", &self.m_worldUpWS)?;
            serializer.serialize_field("stringData", &self.m_stringData)?;
            serializer.serialize_field("defaultEventMode", &self.m_defaultEventMode)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
