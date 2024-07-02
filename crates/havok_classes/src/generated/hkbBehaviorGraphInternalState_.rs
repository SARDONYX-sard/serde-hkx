use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorGraphInternalState`
/// -         version: `0`
/// -       signature: `0x8699b6eb`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorGraphInternalState {
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
    /// -          name: `nodeInternalStateInfos`(ctype: `hkArray<hkbNodeInternalStateInfo*>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nodeInternalStateInfos: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `variableValueSet`(ctype: `struct hkbVariableValueSet*`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_variableValueSet: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBehaviorGraphInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBehaviorGraphInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2258220779u32)
        }
    }
    impl _serde::Serialize for hkbBehaviorGraphInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2258220779u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorGraphInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "nodeInternalStateInfos",
                    &self.m_nodeInternalStateInfos,
                )?;
            serializer.serialize_field("variableValueSet", &self.m_variableValueSet)?;
            serializer
                .serialize_array_field(
                    "nodeInternalStateInfos",
                    &self.m_nodeInternalStateInfos,
                )?;
            serializer.end()
        }
    }
};
