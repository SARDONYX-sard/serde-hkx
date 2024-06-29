use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxSparselyAnimatedBool`
/// -         version: `1`
/// -       signature: `0x7a894596`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxSparselyAnimatedBool {
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
    /// -          name: `bools`(ctype: `hkArray<hkBool>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bools: Vec<bool>,
    /// # C++ Info
    /// -          name: `times`(ctype: `hkArray<hkReal>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_times: Vec<f32>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxSparselyAnimatedBool {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxSparselyAnimatedBool"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2055816598u32)
        }
    }
    impl __serde::Serialize for hkxSparselyAnimatedBool {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxSparselyAnimatedBool", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("bools", &self.m_bools)?;
            serializer.serialize_array_meta_field("times", &self.m_times)?;
            serializer.serialize_array_field("bools", &self.m_bools)?;
            serializer.serialize_array_field("times", &self.m_times)?;
            serializer.end()
        }
    }
};