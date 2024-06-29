use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPoweredChainData`
/// -         version: `0`
/// -       signature: `0x38aeafc3`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPoweredChainData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintChainData,
    /// # C++ Info
    /// -          name: `atoms`(ctype: `struct hkpBridgeAtoms`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_atoms: hkpBridgeAtoms,
    /// # C++ Info
    /// -          name: `infos`(ctype: `hkArray<struct hkpPoweredChainDataConstraintInfo>`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_infos: Vec<hkpPoweredChainDataConstraintInfo>,
    /// # C++ Info
    /// -          name: `tau`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_tau: f32,
    /// # C++ Info
    /// -          name: `damping`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_damping: f32,
    /// # C++ Info
    /// -          name: `cfmLinAdd`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cfmLinAdd: f32,
    /// # C++ Info
    /// -          name: `cfmLinMul`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cfmLinMul: f32,
    /// # C++ Info
    /// -          name: `cfmAngAdd`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cfmAngAdd: f32,
    /// # C++ Info
    /// -          name: `cfmAngMul`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cfmAngMul: f32,
    /// # C++ Info
    /// -          name: `maxErrorDistance`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxErrorDistance: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpPoweredChainData {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpPoweredChainData"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(950972355u32)
        }
    }
    impl __serde::Serialize for hkpPoweredChainData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpPoweredChainData", class_meta)?;
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
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.serialize_array_meta_field("infos", &self.m_infos)?;
            serializer.serialize_field("tau", &self.m_tau)?;
            serializer.serialize_field("damping", &self.m_damping)?;
            serializer.serialize_field("cfmLinAdd", &self.m_cfmLinAdd)?;
            serializer.serialize_field("cfmLinMul", &self.m_cfmLinMul)?;
            serializer.serialize_field("cfmAngAdd", &self.m_cfmAngAdd)?;
            serializer.serialize_field("cfmAngMul", &self.m_cfmAngMul)?;
            serializer.serialize_field("maxErrorDistance", &self.m_maxErrorDistance)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("infos", &self.m_infos)?;
            serializer.end()
        }
    }
};