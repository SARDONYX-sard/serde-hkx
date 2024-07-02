use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBlenderGeneratorChild`
/// -         version: `2`
/// -       signature: `0xe2b384b0`
/// -          size:  48(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlenderGeneratorChild {
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
    /// -          name: `generator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_generator: Pointer,
    /// # C++ Info
    /// -          name: `boneWeights`(ctype: `struct hkbBoneWeightArray*`)
    /// -        offset:  36(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_boneWeights: Pointer,
    /// # C++ Info
    /// -          name: `weight`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_weight: f32,
    /// # C++ Info
    /// -          name: `worldFromModelWeight`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_worldFromModelWeight: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBlenderGeneratorChild {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBlenderGeneratorChild"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3803415728u32)
        }
    }
    impl _serde::Serialize for hkbBlenderGeneratorChild {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3803415728u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbBlenderGeneratorChild", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("generator", &self.m_generator)?;
            serializer.serialize_field("boneWeights", &self.m_boneWeights)?;
            serializer.serialize_field("weight", &self.m_weight)?;
            serializer
                .serialize_field("worldFromModelWeight", &self.m_worldFromModelWeight)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                )?;
            serializer.end()
        }
    }
};
