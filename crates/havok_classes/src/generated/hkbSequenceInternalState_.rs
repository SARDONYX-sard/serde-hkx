use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSequenceInternalState`
/// -         version: `0`
/// -       signature: `0x419b9a05`
/// -          size:  64(x86)/ 88(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSequenceInternalState {
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
    /// -          name: `nextSampleEvents`(ctype: `hkArray<hkInt32>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nextSampleEvents: Vec<i32>,
    /// # C++ Info
    /// -          name: `nextSampleReals`(ctype: `hkArray<hkInt32>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nextSampleReals: Vec<i32>,
    /// # C++ Info
    /// -          name: `nextSampleBools`(ctype: `hkArray<hkInt32>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nextSampleBools: Vec<i32>,
    /// # C++ Info
    /// -          name: `nextSampleInts`(ctype: `hkArray<hkInt32>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nextSampleInts: Vec<i32>,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_time: f32,
    /// # C++ Info
    /// -          name: `isEnabled`(ctype: `hkBool`)
    /// -        offset:  60(x86)/ 84(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isEnabled: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbSequenceInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSequenceInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1100716549u32)
        }
    }
    impl _serde::Serialize for hkbSequenceInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1100716549u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbSequenceInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "nextSampleEvents",
                    &self.m_nextSampleEvents,
                )?;
            serializer
                .serialize_array_meta_field("nextSampleReals", &self.m_nextSampleReals)?;
            serializer
                .serialize_array_meta_field("nextSampleBools", &self.m_nextSampleBools)?;
            serializer
                .serialize_array_meta_field("nextSampleInts", &self.m_nextSampleInts)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_array_field("nextSampleEvents", &self.m_nextSampleEvents)?;
            serializer
                .serialize_array_field("nextSampleReals", &self.m_nextSampleReals)?;
            serializer
                .serialize_array_field("nextSampleBools", &self.m_nextSampleBools)?;
            serializer.serialize_array_field("nextSampleInts", &self.m_nextSampleInts)?;
            serializer.end()
        }
    }
};
