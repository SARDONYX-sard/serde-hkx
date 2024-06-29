use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaAnimationContainer`
/// -         version: `1`
/// -       signature: `0x8dc20333`
/// -          size:  68(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnimationContainer {
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
    /// -          name: `skeletons`(ctype: `hkArray<hkaSkeleton*>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_skeletons: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `animations`(ctype: `hkArray<hkaAnimation*>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_animations: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `bindings`(ctype: `hkArray<hkaAnimationBinding*>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bindings: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `attachments`(ctype: `hkArray<hkaBoneAttachment*>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_attachments: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `skins`(ctype: `hkArray<hkaMeshBinding*>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_skins: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkaAnimationContainer {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkaAnimationContainer"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2378302259u32)
        }
    }
    impl __serde::Serialize for hkaAnimationContainer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkaAnimationContainer", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("skeletons", &self.m_skeletons)?;
            serializer.serialize_array_meta_field("animations", &self.m_animations)?;
            serializer.serialize_array_meta_field("bindings", &self.m_bindings)?;
            serializer.serialize_array_meta_field("attachments", &self.m_attachments)?;
            serializer.serialize_array_meta_field("skins", &self.m_skins)?;
            serializer.serialize_array_field("skeletons", &self.m_skeletons)?;
            serializer.serialize_array_field("animations", &self.m_animations)?;
            serializer.serialize_array_field("bindings", &self.m_bindings)?;
            serializer.serialize_array_field("attachments", &self.m_attachments)?;
            serializer.serialize_array_field("skins", &self.m_skins)?;
            serializer.end()
        }
    }
};
