use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMeshBoneIndexMapping`
/// -         version: `0`
/// -       signature: `0x48aceb75`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMeshBoneIndexMapping {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `mapping`(ctype: `hkArray<hkInt16>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_mapping: Vec<i16>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkMeshBoneIndexMapping {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMeshBoneIndexMapping"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x48aceb75)
        }
    }
    impl _serde::Serialize for hkMeshBoneIndexMapping {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x48aceb75)));
            let mut serializer = __serializer
                .serialize_struct("hkMeshBoneIndexMapping", class_meta)?;
            serializer.serialize_array_meta_field("mapping", &self.m_mapping)?;
            serializer.serialize_array_field("mapping", &self.m_mapping)?;
            serializer.end()
        }
    }
};
