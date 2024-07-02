use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkTrackerSerializableScanSnapshotAllocation`
/// -         version: `0`
/// -       signature: `0x9ab3a6ac`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkTrackerSerializableScanSnapshotAllocation {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `start`(ctype: `hkUlong`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_start: u64,
    /// # C++ Info
    /// -          name: `size`(ctype: `hkUlong`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_size: u64,
    /// # C++ Info
    /// -          name: `traceId`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_traceId: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkTrackerSerializableScanSnapshotAllocation {
        #[inline]
        fn name(&self) -> &'static str {
            "hkTrackerSerializableScanSnapshotAllocation"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2595464876u32)
        }
    }
    impl __serde::Serialize for hkTrackerSerializableScanSnapshotAllocation {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2595464876u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkTrackerSerializableScanSnapshotAllocation",
                    class_meta,
                )?;
            serializer.serialize_field("start", &self.m_start)?;
            serializer.serialize_field("size", &self.m_size)?;
            serializer.serialize_field("traceId", &self.m_traceId)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
