use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxSparselyAnimatedEnum`
/// -         version: `1`
/// -       signature: `0x68a47b64`
/// -          size:  36(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxSparselyAnimatedEnum {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkxSparselyAnimatedInt,
    /// # C++ Info
    /// -          name: `enum`(ctype: `struct hkxEnum*`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_enum: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxSparselyAnimatedEnum {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxSparselyAnimatedEnum"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1755609956u32)
        }
    }
    impl __serde::Serialize for hkxSparselyAnimatedEnum {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxSparselyAnimatedEnum", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("ints", &self.parent.m_ints)?;
            serializer.serialize_array_meta_field("times", &self.parent.m_times)?;
            serializer.serialize_field("enum", &self.m_enum)?;
            serializer.serialize_array_field("ints", &self.parent.m_ints)?;
            serializer.serialize_array_field("times", &self.parent.m_times)?;
            serializer.end()
        }
    }
};
