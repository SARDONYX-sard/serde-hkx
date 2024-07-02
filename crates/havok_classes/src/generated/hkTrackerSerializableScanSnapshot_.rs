use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkTrackerSerializableScanSnapshot`
/// -         version: `0`
/// -       signature: `0x875af1d9`
/// -          size:  92(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkTrackerSerializableScanSnapshot {
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
    /// -          name: `allocations`(ctype: `hkArray<struct hkTrackerSerializableScanSnapshotAllocation>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_allocations: Vec<hkTrackerSerializableScanSnapshotAllocation>,
    /// # C++ Info
    /// -          name: `blocks`(ctype: `hkArray<struct hkTrackerSerializableScanSnapshotBlock>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_blocks: Vec<hkTrackerSerializableScanSnapshotBlock>,
    /// # C++ Info
    /// -          name: `refs`(ctype: `hkArray<hkInt32>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_refs: Vec<i32>,
    /// # C++ Info
    /// -          name: `typeNames`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_typeNames: Vec<u8>,
    /// # C++ Info
    /// -          name: `traceText`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_traceText: Vec<u8>,
    /// # C++ Info
    /// -          name: `traceAddrs`(ctype: `hkArray<hkUint64>`)
    /// -        offset:  68(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_traceAddrs: Vec<u64>,
    /// # C++ Info
    /// -          name: `traceParents`(ctype: `hkArray<hkInt32>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_traceParents: Vec<i32>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkTrackerSerializableScanSnapshot {
        #[inline]
        fn name(&self) -> &'static str {
            "hkTrackerSerializableScanSnapshot"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2270884313u32)
        }
    }
    impl _serde::Serialize for hkTrackerSerializableScanSnapshot {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2270884313u32)));
            let mut serializer = __serializer
                .serialize_struct("hkTrackerSerializableScanSnapshot", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("allocations", &self.m_allocations)?;
            serializer.serialize_array_meta_field("blocks", &self.m_blocks)?;
            serializer.serialize_array_meta_field("refs", &self.m_refs)?;
            serializer.serialize_array_meta_field("typeNames", &self.m_typeNames)?;
            serializer.serialize_array_meta_field("traceText", &self.m_traceText)?;
            serializer.serialize_array_meta_field("traceAddrs", &self.m_traceAddrs)?;
            serializer.serialize_array_meta_field("traceParents", &self.m_traceParents)?;
            serializer.serialize_array_field("allocations", &self.m_allocations)?;
            serializer.serialize_array_field("blocks", &self.m_blocks)?;
            serializer.serialize_array_field("refs", &self.m_refs)?;
            serializer.serialize_array_field("typeNames", &self.m_typeNames)?;
            serializer.serialize_array_field("traceText", &self.m_traceText)?;
            serializer.serialize_array_field("traceAddrs", &self.m_traceAddrs)?;
            serializer.serialize_array_field("traceParents", &self.m_traceParents)?;
            serializer.end()
        }
    }
};
