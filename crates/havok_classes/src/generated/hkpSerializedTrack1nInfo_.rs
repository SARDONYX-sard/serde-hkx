use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSerializedTrack1nInfo`
/// -         version: `0`
/// -       signature: `0xf12d48d9`
/// -          size:  24(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSerializedTrack1nInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `sectors`(ctype: `hkArray<hkpAgent1nSector*>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_sectors: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `subTracks`(ctype: `hkArray<hkpSerializedSubTrack1nInfo*>`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_subTracks: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSerializedTrack1nInfo {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSerializedTrack1nInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4046276825u32)
        }
    }
    impl __serde::Serialize for hkpSerializedTrack1nInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSerializedTrack1nInfo", class_meta)?;
            serializer.serialize_array_meta_field("sectors", &self.m_sectors)?;
            serializer.serialize_array_meta_field("subTracks", &self.m_subTracks)?;
            serializer.serialize_array_field("sectors", &self.m_sectors)?;
            serializer.serialize_array_field("subTracks", &self.m_subTracks)?;
            serializer.end()
        }
    }
};
