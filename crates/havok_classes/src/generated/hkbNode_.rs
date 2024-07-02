use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbNode`
/// -         version: `1`
/// -       signature: `0x6d26f61d`
/// -          size:  40(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbNode<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbBindable,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `id`(ctype: `hkInt16`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_id: i16,
    /// # C++ Info
    /// -          name: `cloneState`(ctype: `enum unknown`)
    /// -        offset:  38(x86)/ 66(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_cloneState: i8,
    /// # C++ Info
    /// -          name: `padNode`(ctype: `hkBool[1]`)
    /// -        offset:  39(x86)/ 67(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_padNode: [bool; 1usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbNode<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbNode"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1831269917u32)
        }
    }
    impl<'a> _serde::Serialize for hkbNode<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1831269917u32)));
            let mut serializer = __serializer.serialize_struct("hkbNode", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field("areBindablesCached", &self.parent.m_areBindablesCached)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.skip_field("id", &self.m_id)?;
            serializer.skip_field("cloneState", &self.m_cloneState)?;
            serializer.skip_field("padNode", &self.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
