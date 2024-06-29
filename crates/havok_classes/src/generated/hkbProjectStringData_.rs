use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbProjectStringData`
/// -         version: `1`
/// -       signature: `0x76ad60a`
/// -          size:  76(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbProjectStringData<'a> {
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
    /// -          name: `animationFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_animationFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `behaviorFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_behaviorFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `characterFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_characterFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `eventNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_eventNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `animationPath`(ctype: `hkStringPtr`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_animationPath: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `behaviorPath`(ctype: `hkStringPtr`)
    /// -        offset:  60(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behaviorPath: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `characterPath`(ctype: `hkStringPtr`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_characterPath: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `fullPathToSource`(ctype: `hkStringPtr`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_fullPathToSource: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `rootPath`(ctype: `hkStringPtr`)
    /// -        offset:  72(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_rootPath: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbProjectStringData<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbProjectStringData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(124442122u32)
        }
    }
    impl<'a> __serde::Serialize for hkbProjectStringData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbProjectStringData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "animationFilenames",
                    &self.m_animationFilenames,
                )?;
            serializer
                .serialize_array_meta_field(
                    "behaviorFilenames",
                    &self.m_behaviorFilenames,
                )?;
            serializer
                .serialize_array_meta_field(
                    "characterFilenames",
                    &self.m_characterFilenames,
                )?;
            serializer.serialize_array_meta_field("eventNames", &self.m_eventNames)?;
            serializer
                .serialize_stringptr_meta_field("animationPath", &self.m_animationPath)?;
            serializer
                .serialize_stringptr_meta_field("behaviorPath", &self.m_behaviorPath)?;
            serializer
                .serialize_stringptr_meta_field("characterPath", &self.m_characterPath)?;
            serializer
                .serialize_stringptr_meta_field(
                    "fullPathToSource",
                    &self.m_fullPathToSource,
                )?;
            serializer.skip_stringptr_meta_field("rootPath", &self.m_rootPath)?;
            serializer
                .serialize_array_field(
                    "animationFilenames",
                    &self.m_animationFilenames,
                )?;
            serializer
                .serialize_array_field("behaviorFilenames", &self.m_behaviorFilenames)?;
            serializer
                .serialize_array_field(
                    "characterFilenames",
                    &self.m_characterFilenames,
                )?;
            serializer.serialize_array_field("eventNames", &self.m_eventNames)?;
            serializer
                .serialize_stringptr_field("animationPath", &self.m_animationPath)?;
            serializer.serialize_stringptr_field("behaviorPath", &self.m_behaviorPath)?;
            serializer
                .serialize_stringptr_field("characterPath", &self.m_characterPath)?;
            serializer
                .serialize_stringptr_field(
                    "fullPathToSource",
                    &self.m_fullPathToSource,
                )?;
            serializer.serialize_stringptr_field("rootPath", &self.m_rootPath)?;
            serializer.end()
        }
    }
};
