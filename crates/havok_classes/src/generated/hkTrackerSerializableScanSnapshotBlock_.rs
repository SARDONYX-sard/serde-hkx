use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkTrackerSerializableScanSnapshotBlock`
/// -         version: `0`
/// -       signature: `0xe7f23e6d`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkTrackerSerializableScanSnapshotBlock {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `typeIndex`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_typeIndex: i32,
    /// # C++ Info
    /// -          name: `start`(ctype: `hkUlong`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_start: u64,
    /// # C++ Info
    /// -          name: `size`(ctype: `hkUlong`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_size: u64,
    /// # C++ Info
    /// -          name: `arraySize`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_arraySize: i32,
    /// # C++ Info
    /// -          name: `startReferenceIndex`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_startReferenceIndex: i32,
    /// # C++ Info
    /// -          name: `numReferences`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numReferences: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkTrackerSerializableScanSnapshotBlock {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkTrackerSerializableScanSnapshotBlock"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3891412589u32)
        }
    }
    impl __serde::Serialize for hkTrackerSerializableScanSnapshotBlock {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkTrackerSerializableScanSnapshotBlock", class_meta)?;
            serializer.serialize_field("typeIndex", &self.m_typeIndex)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("start", &self.m_start)?;
            serializer.serialize_field("size", &self.m_size)?;
            serializer.serialize_field("arraySize", &self.m_arraySize)?;
            serializer
                .serialize_field("startReferenceIndex", &self.m_startReferenceIndex)?;
            serializer.serialize_field("numReferences", &self.m_numReferences)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
