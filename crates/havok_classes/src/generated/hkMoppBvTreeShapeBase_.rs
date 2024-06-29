use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMoppBvTreeShapeBase`
/// -         version: `0`
/// -       signature: `0x7c338c66`
/// -          size:  48(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMoppBvTreeShapeBase {
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
    /// -          name: `code`(ctype: `struct hkpMoppCode*`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_code: Pointer,
    /// # C++ Info
    /// -          name: `moppData`(ctype: `void*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_moppData: Pointer,
    /// # C++ Info
    /// -          name: `moppDataSize`(ctype: `hkUint32`)
    /// -        offset:  28(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_moppDataSize: u32,
    /// # C++ Info
    /// -          name: `codeInfoCopy`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_codeInfoCopy: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkMoppBvTreeShapeBase {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMoppBvTreeShapeBase"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2083753062u32)
        }
    }
    impl __serde::Serialize for hkMoppBvTreeShapeBase {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMoppBvTreeShapeBase", class_meta)?;
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
            serializer.serialize_field("code", &self.m_code)?;
            serializer.skip_field("moppData", &self.m_moppData)?;
            serializer.skip_field("moppDataSize", &self.m_moppDataSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("codeInfoCopy", &self.m_codeInfoCopy)?;
            serializer.end()
        }
    }
};
