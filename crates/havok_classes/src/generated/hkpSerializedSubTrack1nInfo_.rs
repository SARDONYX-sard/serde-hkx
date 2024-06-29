use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSerializedSubTrack1nInfo`
/// -         version: `0`
/// -       signature: `0x10155a`
/// -          size:  32(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSerializedSubTrack1nInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpSerializedTrack1nInfo,
    /// # C++ Info
    /// -          name: `sectorIndex`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sectorIndex: i32,
    /// # C++ Info
    /// -          name: `offsetInSector`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offsetInSector: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSerializedSubTrack1nInfo {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSerializedSubTrack1nInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1054042u32)
        }
    }
    impl __serde::Serialize for hkpSerializedSubTrack1nInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSerializedSubTrack1nInfo", class_meta)?;
            serializer.serialize_array_meta_field("sectors", &self.parent.m_sectors)?;
            serializer
                .serialize_array_meta_field("subTracks", &self.parent.m_subTracks)?;
            serializer.serialize_field("sectorIndex", &self.m_sectorIndex)?;
            serializer.serialize_field("offsetInSector", &self.m_offsetInSector)?;
            serializer.serialize_array_field("sectors", &self.parent.m_sectors)?;
            serializer.serialize_array_field("subTracks", &self.parent.m_subTracks)?;
            serializer.end()
        }
    }
};
