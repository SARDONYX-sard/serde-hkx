use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaBoneAttachment`
/// -         version: `1`
/// -       signature: `0xa8ccd5cf`
/// -          size:  96(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaBoneAttachment<'a> {
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
    /// -          name: `originalSkeletonName`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_originalSkeletonName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `boneFromAttachment`(ctype: `hkMatrix4`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_boneFromAttachment: Matrix4,
    /// # C++ Info
    /// -          name: `attachment`(ctype: `struct hkReferencedObject*`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_attachment: Pointer,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  84(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `boneIndex`(ctype: `hkInt16`)
    /// -        offset:  88(x86)/112(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_boneIndex: i16,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaBoneAttachment<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaBoneAttachment"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2831996367u32)
        }
    }
    impl<'a> _serde::Serialize for hkaBoneAttachment<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2831996367u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaBoneAttachment", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field("boneFromAttachment", &self.m_boneFromAttachment)?;
            serializer.serialize_field("attachment", &self.m_attachment)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("boneIndex", &self.m_boneIndex)?;
            serializer.pad_field([0u8; 6usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer
                .serialize_stringptr_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
