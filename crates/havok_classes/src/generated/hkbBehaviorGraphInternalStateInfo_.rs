use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorGraphInternalStateInfo`
/// -         version: `1`
/// -       signature: `0x645f898b`
/// -          size:  56(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorGraphInternalStateInfo {
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
    /// -          name: `internalState`(ctype: `struct hkbBehaviorGraphInternalState*`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_internalState: Pointer,
    /// # C++ Info
    /// -          name: `auxiliaryNodeInfo`(ctype: `hkArray<hkbAuxiliaryNodeInfo*>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_auxiliaryNodeInfo: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `activeEventIds`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_activeEventIds: Vec<i16>,
    /// # C++ Info
    /// -          name: `activeVariableIds`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_activeVariableIds: Vec<i16>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBehaviorGraphInternalStateInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBehaviorGraphInternalStateInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x645f898b)
        }
    }
    impl _serde::Serialize for hkbBehaviorGraphInternalStateInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x645f898b)));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorGraphInternalStateInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("internalState", &self.m_internalState)?;
            serializer
                .serialize_array_meta_field(
                    "auxiliaryNodeInfo",
                    &self.m_auxiliaryNodeInfo,
                )?;
            serializer
                .serialize_array_meta_field("activeEventIds", &self.m_activeEventIds)?;
            serializer
                .serialize_array_meta_field(
                    "activeVariableIds",
                    &self.m_activeVariableIds,
                )?;
            serializer
                .serialize_array_field("auxiliaryNodeInfo", &self.m_auxiliaryNodeInfo)?;
            serializer.serialize_array_field("activeEventIds", &self.m_activeEventIds)?;
            serializer
                .serialize_array_field("activeVariableIds", &self.m_activeVariableIds)?;
            serializer.end()
        }
    }
};
