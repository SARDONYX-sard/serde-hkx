use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbNodeInternalStateInfo`
/// -         version: `0`
/// -       signature: `0x7db9971d`
/// -          size: 100(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbNodeInternalStateInfo<'a> {
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
    /// -          name: `syncInfo`(ctype: `struct hkbGeneratorSyncInfo`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  80(x86)/ 80(x86_64)
    ///
    pub m_syncInfo: hkbGeneratorSyncInfo,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  88(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `internalState`(ctype: `struct hkReferencedObject*`)
    /// -        offset:  92(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_internalState: Pointer,
    /// # C++ Info
    /// -          name: `nodeId`(ctype: `hkInt16`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_nodeId: i16,
    /// # C++ Info
    /// -          name: `hasActivateBeenCalled`(ctype: `hkBool`)
    /// -        offset:  98(x86)/114(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hasActivateBeenCalled: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbNodeInternalStateInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbNodeInternalStateInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2109314845u32)
        }
    }
    impl<'a> _serde::Serialize for hkbNodeInternalStateInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2109314845u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbNodeInternalStateInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("syncInfo", &self.m_syncInfo)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("internalState", &self.m_internalState)?;
            serializer.serialize_field("nodeId", &self.m_nodeId)?;
            serializer
                .serialize_field(
                    "hasActivateBeenCalled",
                    &self.m_hasActivateBeenCalled,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
