use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkAlignSceneToNodeOptions`
/// -         version: `2`
/// -       signature: `0x207cb01`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkAlignSceneToNodeOptions<'a> {
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
    /// -          name: `invert`(ctype: `hkBool`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_invert: bool,
    /// # C++ Info
    /// -          name: `transformPositionX`(ctype: `hkBool`)
    /// -        offset:   9(x86)/ 17(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformPositionX: bool,
    /// # C++ Info
    /// -          name: `transformPositionY`(ctype: `hkBool`)
    /// -        offset:  10(x86)/ 18(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformPositionY: bool,
    /// # C++ Info
    /// -          name: `transformPositionZ`(ctype: `hkBool`)
    /// -        offset:  11(x86)/ 19(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformPositionZ: bool,
    /// # C++ Info
    /// -          name: `transformRotation`(ctype: `hkBool`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformRotation: bool,
    /// # C++ Info
    /// -          name: `transformScale`(ctype: `hkBool`)
    /// -        offset:  13(x86)/ 21(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformScale: bool,
    /// # C++ Info
    /// -          name: `transformSkew`(ctype: `hkBool`)
    /// -        offset:  14(x86)/ 22(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformSkew: bool,
    /// # C++ Info
    /// -          name: `keyframe`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_keyframe: i32,
    /// # C++ Info
    /// -          name: `nodeName`(ctype: `hkStringPtr`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_nodeName: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkAlignSceneToNodeOptions<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkAlignSceneToNodeOptions"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(34065153u32)
        }
    }
    impl<'a> __serde::Serialize for hkAlignSceneToNodeOptions<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkAlignSceneToNodeOptions", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("invert", &self.m_invert)?;
            serializer
                .serialize_field("transformPositionX", &self.m_transformPositionX)?;
            serializer
                .serialize_field("transformPositionY", &self.m_transformPositionY)?;
            serializer
                .serialize_field("transformPositionZ", &self.m_transformPositionZ)?;
            serializer.serialize_field("transformRotation", &self.m_transformRotation)?;
            serializer.serialize_field("transformScale", &self.m_transformScale)?;
            serializer.serialize_field("transformSkew", &self.m_transformSkew)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("keyframe", &self.m_keyframe)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("nodeName", &self.m_nodeName)?;
            serializer.serialize_stringptr_field("nodeName", &self.m_nodeName)?;
            serializer.end()
        }
    }
};
