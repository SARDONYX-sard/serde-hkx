use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMoppBvTreeShape`
/// -         version: `0`
/// -       signature: `0x90b29d39`
/// -          size:  64(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMoppBvTreeShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkMoppBvTreeShapeBase,
    /// # C++ Info
    /// -          name: `child`(ctype: `struct hkpSingleShapeContainer`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_child: hkpSingleShapeContainer,
    /// # C++ Info
    /// -          name: `childSize`(ctype: `hkInt32`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_childSize: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpMoppBvTreeShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMoppBvTreeShape"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2427624761u32)
        }
    }
    impl __serde::Serialize for hkpMoppBvTreeShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2427624761u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpMoppBvTreeShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("bvTreeType", &self.parent.parent.m_bvTreeType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("code", &self.parent.m_code)?;
            serializer.skip_field("moppData", &self.parent.m_moppData)?;
            serializer.skip_field("moppDataSize", &self.parent.m_moppDataSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("codeInfoCopy", &self.parent.m_codeInfoCopy)?;
            serializer.serialize_field("child", &self.m_child)?;
            serializer.skip_field("childSize", &self.m_childSize)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
