use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTriSampledHeightFieldBvTreeShape`
/// -         version: `0`
/// -       signature: `0x58e1e585`
/// -          size:  48(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTriSampledHeightFieldBvTreeShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpBvTreeShape,
    /// # C++ Info
    /// -          name: `childContainer`(ctype: `struct hkpSingleShapeContainer`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_childContainer: hkpSingleShapeContainer,
    /// # C++ Info
    /// -          name: `childSize`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_childSize: i32,
    /// # C++ Info
    /// -          name: `wantAabbRejectionTest`(ctype: `hkBool`)
    /// -        offset:  32(x86)/ 60(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_wantAabbRejectionTest: bool,
    /// # C++ Info
    /// -          name: `padding`(ctype: `hkUint8[12]`)
    /// -        offset:  33(x86)/ 61(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_padding: [u8; 12usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpTriSampledHeightFieldBvTreeShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTriSampledHeightFieldBvTreeShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x58e1e585)
        }
    }
    impl _serde::Serialize for hkpTriSampledHeightFieldBvTreeShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x58e1e585)));
            let mut serializer = __serializer
                .serialize_struct("hkpTriSampledHeightFieldBvTreeShape", class_meta)?;
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
            serializer.serialize_field("bvTreeType", &self.parent.m_bvTreeType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("childContainer", &self.m_childContainer)?;
            serializer.skip_field("childSize", &self.m_childSize)?;
            serializer
                .serialize_field(
                    "wantAabbRejectionTest",
                    &self.m_wantAabbRejectionTest,
                )?;
            serializer.serialize_field("padding", &self.m_padding.as_slice())?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
