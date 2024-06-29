use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxMaterialTextureStage`
/// -         version: `0`
/// -       signature: `0xfa6facb2`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMaterialTextureStage {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `texture`(ctype: `struct hkReferencedObject*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_texture: Pointer,
    /// # C++ Info
    /// -          name: `usageHint`(ctype: `enum TextureType`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_usageHint: TextureType,
    /// # C++ Info
    /// -          name: `tcoordChannel`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_tcoordChannel: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxMaterialTextureStage {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxMaterialTextureStage"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4201622706u32)
        }
    }
    impl __serde::Serialize for hkxMaterialTextureStage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxMaterialTextureStage", class_meta)?;
            serializer.serialize_field("texture", &self.m_texture)?;
            serializer.serialize_field("usageHint", &self.m_usageHint)?;
            serializer.serialize_field("tcoordChannel", &self.m_tcoordChannel)?;
            serializer.end()
        }
    }
};
