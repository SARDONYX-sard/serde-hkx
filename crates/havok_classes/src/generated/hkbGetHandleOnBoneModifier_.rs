use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbGetHandleOnBoneModifier`
/// -         version: `0`
/// -       signature: `0x50c34a17`
/// -          size:  56(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbGetHandleOnBoneModifier<'a> {
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
    /// -          name: `handleOut`(ctype: `struct hkbHandle*`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_handleOut: Pointer,
    /// # C++ Info
    /// -          name: `localFrameName`(ctype: `hkStringPtr`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_localFrameName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `ragdollBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  52(x86)/ 96(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_ragdollBoneIndex: i16,
    /// # C++ Info
    /// -          name: `animationBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  54(x86)/ 98(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_animationBoneIndex: i16,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbGetHandleOnBoneModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbGetHandleOnBoneModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x50c34a17)
        }
    }
    impl<'a> _serde::Serialize for hkbGetHandleOnBoneModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x50c34a17)));
            let mut serializer = __serializer
                .serialize_struct("hkbGetHandleOnBoneModifier", class_meta)?;
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
            serializer.serialize_field("handleOut", &self.m_handleOut)?;
            serializer
                .serialize_stringptr_meta_field(
                    "localFrameName",
                    &self.m_localFrameName,
                )?;
            serializer.serialize_field("ragdollBoneIndex", &self.m_ragdollBoneIndex)?;
            serializer
                .serialize_field("animationBoneIndex", &self.m_animationBoneIndex)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_stringptr_field("localFrameName", &self.m_localFrameName)?;
            serializer.end()
        }
    }
};
