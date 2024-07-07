use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbAttachmentModifier`
/// -         version: `1`
/// -       signature: `0xcc0aab32`
/// -          size: 108(x86)/200(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbAttachmentModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `sendToAttacherOnAttach`(ctype: `struct hkbEventProperty`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_sendToAttacherOnAttach: hkbEventProperty,
    /// # C++ Info
    /// -          name: `sendToAttacheeOnAttach`(ctype: `struct hkbEventProperty`)
    /// -        offset:  52(x86)/ 96(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_sendToAttacheeOnAttach: hkbEventProperty,
    /// # C++ Info
    /// -          name: `sendToAttacherOnDetach`(ctype: `struct hkbEventProperty`)
    /// -        offset:  60(x86)/112(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_sendToAttacherOnDetach: hkbEventProperty,
    /// # C++ Info
    /// -          name: `sendToAttacheeOnDetach`(ctype: `struct hkbEventProperty`)
    /// -        offset:  68(x86)/128(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_sendToAttacheeOnDetach: hkbEventProperty,
    /// # C++ Info
    /// -          name: `attachmentSetup`(ctype: `struct hkbAttachmentSetup*`)
    /// -        offset:  76(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_attachmentSetup: Pointer,
    /// # C++ Info
    /// -          name: `attacherHandle`(ctype: `struct hkbHandle*`)
    /// -        offset:  80(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_attacherHandle: Pointer,
    /// # C++ Info
    /// -          name: `attacheeHandle`(ctype: `struct hkbHandle*`)
    /// -        offset:  84(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_attacheeHandle: Pointer,
    /// # C++ Info
    /// -          name: `attacheeLayer`(ctype: `hkInt32`)
    /// -        offset:  88(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_attacheeLayer: i32,
    /// # C++ Info
    /// -          name: `attacheeRB`(ctype: `void*`)
    /// -        offset:  92(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_attacheeRB: Pointer,
    /// # C++ Info
    /// -          name: `oldMotionType`(ctype: `enum unknown`)
    /// -        offset:  96(x86)/184(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldMotionType: u8,
    /// # C++ Info
    /// -          name: `oldFilterInfo`(ctype: `hkInt32`)
    /// -        offset: 100(x86)/188(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldFilterInfo: i32,
    /// # C++ Info
    /// -          name: `attachment`(ctype: `void*`)
    /// -        offset: 104(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_attachment: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbAttachmentModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbAttachmentModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xcc0aab32)
        }
    }
    impl<'a> _serde::Serialize for hkbAttachmentModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xcc0aab32)));
            let mut serializer = __serializer
                .serialize_struct("hkbAttachmentModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "sendToAttacherOnAttach",
                    &self.m_sendToAttacherOnAttach,
                )?;
            serializer
                .serialize_field(
                    "sendToAttacheeOnAttach",
                    &self.m_sendToAttacheeOnAttach,
                )?;
            serializer
                .serialize_field(
                    "sendToAttacherOnDetach",
                    &self.m_sendToAttacherOnDetach,
                )?;
            serializer
                .serialize_field(
                    "sendToAttacheeOnDetach",
                    &self.m_sendToAttacheeOnDetach,
                )?;
            serializer.serialize_field("attachmentSetup", &self.m_attachmentSetup)?;
            serializer.serialize_field("attacherHandle", &self.m_attacherHandle)?;
            serializer.serialize_field("attacheeHandle", &self.m_attacheeHandle)?;
            serializer.serialize_field("attacheeLayer", &self.m_attacheeLayer)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("attacheeRB", &self.m_attacheeRB)?;
            serializer.skip_field("oldMotionType", &self.m_oldMotionType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("oldFilterInfo", &self.m_oldFilterInfo)?;
            serializer.skip_field("attachment", &self.m_attachment)?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
