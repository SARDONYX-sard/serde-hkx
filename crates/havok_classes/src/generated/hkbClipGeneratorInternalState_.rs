use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbClipGeneratorInternalState`
/// -         version: `0`
/// -       signature: `0x26ce5bf3`
/// -          size: 112(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbClipGeneratorInternalState {
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
    /// -          name: `extractedMotion`(ctype: `hkQsTransform`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_extractedMotion: QsTransform,
    /// # C++ Info
    /// -          name: `echos`(ctype: `hkArray<struct hkbClipGeneratorEcho>`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_echos: Vec<hkbClipGeneratorEcho>,
    /// # C++ Info
    /// -          name: `localTime`(ctype: `hkReal`)
    /// -        offset:  76(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_localTime: f32,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset:  80(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_time: f32,
    /// # C++ Info
    /// -          name: `previousUserControlledTimeFraction`(ctype: `hkReal`)
    /// -        offset:  84(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_previousUserControlledTimeFraction: f32,
    /// # C++ Info
    /// -          name: `bufferSize`(ctype: `hkInt32`)
    /// -        offset:  88(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_bufferSize: i32,
    /// # C++ Info
    /// -          name: `echoBufferSize`(ctype: `hkInt32`)
    /// -        offset:  92(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_echoBufferSize: i32,
    /// # C++ Info
    /// -          name: `atEnd`(ctype: `hkBool`)
    /// -        offset:  96(x86)/100(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_atEnd: bool,
    /// # C++ Info
    /// -          name: `ignoreStartTime`(ctype: `hkBool`)
    /// -        offset:  97(x86)/101(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_ignoreStartTime: bool,
    /// # C++ Info
    /// -          name: `pingPongBackward`(ctype: `hkBool`)
    /// -        offset:  98(x86)/102(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_pingPongBackward: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbClipGeneratorInternalState {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbClipGeneratorInternalState"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(651058163u32)
        }
    }
    impl __serde::Serialize for hkbClipGeneratorInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbClipGeneratorInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("extractedMotion", &self.m_extractedMotion)?;
            serializer.serialize_array_meta_field("echos", &self.m_echos)?;
            serializer.serialize_field("localTime", &self.m_localTime)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer
                .serialize_field(
                    "previousUserControlledTimeFraction",
                    &self.m_previousUserControlledTimeFraction,
                )?;
            serializer.serialize_field("bufferSize", &self.m_bufferSize)?;
            serializer.serialize_field("echoBufferSize", &self.m_echoBufferSize)?;
            serializer.serialize_field("atEnd", &self.m_atEnd)?;
            serializer.serialize_field("ignoreStartTime", &self.m_ignoreStartTime)?;
            serializer.serialize_field("pingPongBackward", &self.m_pingPongBackward)?;
            serializer.pad_field([0u8; 13usize].as_slice(), [0u8; 9usize].as_slice())?;
            serializer.serialize_array_field("echos", &self.m_echos)?;
            serializer.end()
        }
    }
};
