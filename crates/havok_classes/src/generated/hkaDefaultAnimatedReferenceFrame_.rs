use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaDefaultAnimatedReferenceFrame`
/// -         version: `0`
/// -       signature: `0x6d85e445`
/// -          size:  64(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaDefaultAnimatedReferenceFrame {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkaAnimatedReferenceFrame,
    /// # C++ Info
    /// -          name: `up`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_up: Vector4,
    /// # C++ Info
    /// -          name: `forward`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_forward: Vector4,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `referenceFrameSamples`(ctype: `hkArray<hkVector4>`)
    /// -        offset:  52(x86)/ 56(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_referenceFrameSamples: Vec<Vector4>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkaDefaultAnimatedReferenceFrame {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaDefaultAnimatedReferenceFrame"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1837491269u32)
        }
    }
    impl __serde::Serialize for hkaDefaultAnimatedReferenceFrame {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkaDefaultAnimatedReferenceFrame", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer.serialize_field("forward", &self.m_forward)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "referenceFrameSamples",
                    &self.m_referenceFrameSamples,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "referenceFrameSamples",
                    &self.m_referenceFrameSamples,
                )?;
            serializer.end()
        }
    }
};
