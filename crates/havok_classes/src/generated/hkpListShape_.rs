use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpListShape`
/// -         version: `0`
/// -       signature: `0xa1937cbd`
/// -          size: 112(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpListShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShapeCollection,
    /// # C++ Info
    /// -          name: `childInfo`(ctype: `hkArray<struct hkpListShapeChildInfo>`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childInfo: Vec<hkpListShapeChildInfo>,
    /// # C++ Info
    /// -          name: `flags`(ctype: `hkUint16`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_flags: u16,
    /// # C++ Info
    /// -          name: `numDisabledChildren`(ctype: `hkUint16`)
    /// -        offset:  38(x86)/ 66(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_numDisabledChildren: u16,
    /// # C++ Info
    /// -          name: `aabbHalfExtents`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_aabbHalfExtents: Vector4,
    /// # C++ Info
    /// -          name: `aabbCenter`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_aabbCenter: Vector4,
    /// # C++ Info
    /// -          name: `enabledChildren`(ctype: `hkUint32[8]`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_enabledChildren: [u32; 8usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpListShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpListShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2710797501u32)
        }
    }
    impl _serde::Serialize for hkpListShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2710797501u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpListShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("disableWelding", &self.parent.m_disableWelding)?;
            serializer.serialize_field("collectionType", &self.parent.m_collectionType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_array_meta_field("childInfo", &self.m_childInfo)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer
                .serialize_field("numDisabledChildren", &self.m_numDisabledChildren)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_field("aabbHalfExtents", &self.m_aabbHalfExtents)?;
            serializer.serialize_field("aabbCenter", &self.m_aabbCenter)?;
            serializer
                .serialize_field("enabledChildren", &self.m_enabledChildren.as_slice())?;
            serializer.serialize_array_field("childInfo", &self.m_childInfo)?;
            serializer.end()
        }
    }
};
