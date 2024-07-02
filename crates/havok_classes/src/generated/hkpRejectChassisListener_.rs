use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRejectChassisListener`
/// -         version: `1`
/// -       signature: `0xc4fa16c9`
/// -          size:  16(x86)/ 24(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRejectChassisListener {
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
    /// -          name: `chassis`(ctype: `void*`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_chassis: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpRejectChassisListener {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRejectChassisListener"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3304724169u32)
        }
    }
    impl _serde::Serialize for hkpRejectChassisListener {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3304724169u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpRejectChassisListener", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.skip_field("chassis", &self.m_chassis)?;
            serializer.end()
        }
    }
};
