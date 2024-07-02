use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbMirroredSkeletonInfo`
/// -         version: `0`
/// -       signature: `0xc6c2da4f`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbMirroredSkeletonInfo {
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
    /// -          name: `mirrorAxis`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_mirrorAxis: Vector4,
    /// # C++ Info
    /// -          name: `bonePairMap`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bonePairMap: Vec<i16>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbMirroredSkeletonInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbMirroredSkeletonInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3334658639u32)
        }
    }
    impl _serde::Serialize for hkbMirroredSkeletonInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3334658639u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbMirroredSkeletonInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("mirrorAxis", &self.m_mirrorAxis)?;
            serializer.serialize_array_meta_field("bonePairMap", &self.m_bonePairMap)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_array_field("bonePairMap", &self.m_bonePairMap)?;
            serializer.end()
        }
    }
};
