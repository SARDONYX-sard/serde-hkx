use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbGeneratorSyncInfo`
/// -         version: `0`
/// -       signature: `0xa3c341f8`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbGeneratorSyncInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `syncPoints`(ctype: `struct hkbGeneratorSyncInfoSyncPoint[8]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/ 64(x86_64)
    ///
    pub m_syncPoints: [hkbGeneratorSyncInfoSyncPoint; 8usize],
    /// # C++ Info
    /// -          name: `baseFrequency`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_baseFrequency: f32,
    /// # C++ Info
    /// -          name: `localTime`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_localTime: f32,
    /// # C++ Info
    /// -          name: `playbackSpeed`(ctype: `hkReal`)
    /// -        offset:  72(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_playbackSpeed: f32,
    /// # C++ Info
    /// -          name: `numSyncPoints`(ctype: `hkInt8`)
    /// -        offset:  76(x86)/ 76(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numSyncPoints: i8,
    /// # C++ Info
    /// -          name: `isCyclic`(ctype: `hkBool`)
    /// -        offset:  77(x86)/ 77(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isCyclic: bool,
    /// # C++ Info
    /// -          name: `isMirrored`(ctype: `hkBool`)
    /// -        offset:  78(x86)/ 78(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isMirrored: bool,
    /// # C++ Info
    /// -          name: `isAdditive`(ctype: `hkBool`)
    /// -        offset:  79(x86)/ 79(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isAdditive: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbGeneratorSyncInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbGeneratorSyncInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa3c341f8)
        }
    }
    impl _serde::Serialize for hkbGeneratorSyncInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa3c341f8)));
            let mut serializer = __serializer
                .serialize_struct("hkbGeneratorSyncInfo", class_meta)?;
            serializer.serialize_field("syncPoints", &self.m_syncPoints.as_slice())?;
            serializer.pad_field([0u8; 56usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("baseFrequency", &self.m_baseFrequency)?;
            serializer.serialize_field("localTime", &self.m_localTime)?;
            serializer.serialize_field("playbackSpeed", &self.m_playbackSpeed)?;
            serializer.serialize_field("numSyncPoints", &self.m_numSyncPoints)?;
            serializer.serialize_field("isCyclic", &self.m_isCyclic)?;
            serializer.serialize_field("isMirrored", &self.m_isMirrored)?;
            serializer.serialize_field("isAdditive", &self.m_isAdditive)?;
            serializer.end()
        }
    }
};
