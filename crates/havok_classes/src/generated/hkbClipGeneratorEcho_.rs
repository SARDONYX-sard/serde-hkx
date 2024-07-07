use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbClipGeneratorEcho`
/// -         version: `0`
/// -       signature: `0x750edf40`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbClipGeneratorEcho {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `offsetLocalTime`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_offsetLocalTime: f32,
    /// # C++ Info
    /// -          name: `weight`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_weight: f32,
    /// # C++ Info
    /// -          name: `dwdt`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dwdt: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbClipGeneratorEcho {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbClipGeneratorEcho"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x750edf40)
        }
    }
    impl _serde::Serialize for hkbClipGeneratorEcho {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x750edf40)));
            let mut serializer = __serializer
                .serialize_struct("hkbClipGeneratorEcho", class_meta)?;
            serializer.serialize_field("offsetLocalTime", &self.m_offsetLocalTime)?;
            serializer.serialize_field("weight", &self.m_weight)?;
            serializer.serialize_field("dwdt", &self.m_dwdt)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
