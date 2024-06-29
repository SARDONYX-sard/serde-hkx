use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpExtendedMeshShapeSubpart`
/// -         version: `2`
/// -       signature: `0xf4608207`
/// -          size:  20(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpExtendedMeshShapeSubpart {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum SubpartType`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: SubpartType,
    /// # C++ Info
    /// -          name: `materialIndexStridingType`(ctype: `enum MaterialIndexStridingType`)
    /// -        offset:   1(x86)/  1(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_materialIndexStridingType: MaterialIndexStridingType,
    /// # C++ Info
    /// -          name: `materialStriding`(ctype: `hkInt16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialStriding: i16,
    /// # C++ Info
    /// -          name: `materialIndexBase`(ctype: `void*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialIndexBase: Pointer,
    /// # C++ Info
    /// -          name: `materialIndexStriding`(ctype: `hkUint16`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_materialIndexStriding: u16,
    /// # C++ Info
    /// -          name: `numMaterials`(ctype: `hkUint16`)
    /// -        offset:  10(x86)/ 18(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_numMaterials: u16,
    /// # C++ Info
    /// -          name: `materialBase`(ctype: `void*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialBase: Pointer,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpExtendedMeshShapeSubpart {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpExtendedMeshShapeSubpart"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4099965447u32)
        }
    }
    impl __serde::Serialize for hkpExtendedMeshShapeSubpart {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpExtendedMeshShapeSubpart", class_meta)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer
                .serialize_field(
                    "materialIndexStridingType",
                    &self.m_materialIndexStridingType,
                )?;
            serializer.skip_field("materialStriding", &self.m_materialStriding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialIndexBase", &self.m_materialIndexBase)?;
            serializer
                .serialize_field(
                    "materialIndexStriding",
                    &self.m_materialIndexStriding,
                )?;
            serializer.serialize_field("numMaterials", &self.m_numMaterials)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialBase", &self.m_materialBase)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.end()
        }
    }
};
