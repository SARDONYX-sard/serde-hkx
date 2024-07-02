use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBlenderGeneratorInternalState`
/// -         version: `0`
/// -       signature: `0x84717488`
/// -          size:  48(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlenderGeneratorInternalState {
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
    /// -          name: `childrenInternalStates`(ctype: `hkArray<struct hkbBlenderGeneratorChildInternalState>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childrenInternalStates: Vec<hkbBlenderGeneratorChildInternalState>,
    /// # C++ Info
    /// -          name: `sortedChildren`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_sortedChildren: Vec<i16>,
    /// # C++ Info
    /// -          name: `endIntervalWeight`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_endIntervalWeight: f32,
    /// # C++ Info
    /// -          name: `numActiveChildren`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numActiveChildren: i32,
    /// # C++ Info
    /// -          name: `beginIntervalIndex`(ctype: `hkInt16`)
    /// -        offset:  40(x86)/ 56(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_beginIntervalIndex: i16,
    /// # C++ Info
    /// -          name: `endIntervalIndex`(ctype: `hkInt16`)
    /// -        offset:  42(x86)/ 58(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_endIntervalIndex: i16,
    /// # C++ Info
    /// -          name: `initSync`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 60(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_initSync: bool,
    /// # C++ Info
    /// -          name: `doSubtractiveBlend`(ctype: `hkBool`)
    /// -        offset:  45(x86)/ 61(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_doSubtractiveBlend: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBlenderGeneratorInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBlenderGeneratorInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2222027912u32)
        }
    }
    impl _serde::Serialize for hkbBlenderGeneratorInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2222027912u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbBlenderGeneratorInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "childrenInternalStates",
                    &self.m_childrenInternalStates,
                )?;
            serializer
                .serialize_array_meta_field("sortedChildren", &self.m_sortedChildren)?;
            serializer.serialize_field("endIntervalWeight", &self.m_endIntervalWeight)?;
            serializer.serialize_field("numActiveChildren", &self.m_numActiveChildren)?;
            serializer
                .serialize_field("beginIntervalIndex", &self.m_beginIntervalIndex)?;
            serializer.serialize_field("endIntervalIndex", &self.m_endIntervalIndex)?;
            serializer.serialize_field("initSync", &self.m_initSync)?;
            serializer
                .serialize_field("doSubtractiveBlend", &self.m_doSubtractiveBlend)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "childrenInternalStates",
                    &self.m_childrenInternalStates,
                )?;
            serializer.serialize_array_field("sortedChildren", &self.m_sortedChildren)?;
            serializer.end()
        }
    }
};
