use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBoneIndexArray`
/// -         version: `0`
/// -       signature: `0xaa8619`
/// -          size:  40(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBoneIndexArray {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbBindable,
    /// # C++ Info
    /// -          name: `boneIndices`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_boneIndices: Vec<i16>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbBoneIndexArray {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbBoneIndexArray"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(11175449u32)
        }
    }
    impl __serde::Serialize for hkbBoneIndexArray {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbBoneIndexArray", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field("areBindablesCached", &self.parent.m_areBindablesCached)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_meta_field("boneIndices", &self.m_boneIndices)?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                )?;
            serializer.serialize_array_field("boneIndices", &self.m_boneIndices)?;
            serializer.end()
        }
    }
};
