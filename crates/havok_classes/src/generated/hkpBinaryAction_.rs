use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBinaryAction`
/// -         version: `0`
/// -       signature: `0xc00f3403`
/// -          size:  32(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBinaryAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpAction<'a>,
    /// # C++ Info
    /// -          name: `entityA`(ctype: `struct hkpEntity*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_entityA: Pointer,
    /// # C++ Info
    /// -          name: `entityB`(ctype: `struct hkpEntity*`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_entityB: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpBinaryAction<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpBinaryAction"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3222221827u32)
        }
    }
    impl<'a> __serde::Serialize for hkpBinaryAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpBinaryAction", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.m_world)?;
            serializer.skip_field("island", &self.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("entityA", &self.m_entityA)?;
            serializer.serialize_field("entityB", &self.m_entityB)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.end()
        }
    }
};
