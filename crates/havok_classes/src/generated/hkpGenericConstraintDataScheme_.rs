use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpGenericConstraintDataScheme`
/// -         version: `0`
/// -       signature: `0x11fd6f6c`
/// -          size:  64(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpGenericConstraintDataScheme {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `info`(ctype: `struct hkpGenericConstraintDataSchemeConstraintInfo`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_info: hkpGenericConstraintDataSchemeConstraintInfo,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkArray<hkVector4>`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_data: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `commands`(ctype: `hkArray<hkInt32>`)
    /// -        offset:  28(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_commands: Vec<i32>,
    /// # C++ Info
    /// -          name: `modifiers`(ctype: `hkArray<void*>`)
    /// -        offset:  40(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_modifiers: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `motors`(ctype: `hkArray<hkpConstraintMotor*>`)
    /// -        offset:  52(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_motors: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpGenericConstraintDataScheme {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpGenericConstraintDataScheme"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(301821804u32)
        }
    }
    impl __serde::Serialize for hkpGenericConstraintDataScheme {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(301821804u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpGenericConstraintDataScheme", class_meta)?;
            serializer.skip_field("info", &self.m_info)?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_array_meta_field("commands", &self.m_commands)?;
            serializer.skip_array_meta_field("modifiers", &self.m_modifiers)?;
            serializer.serialize_array_meta_field("motors", &self.m_motors)?;
            serializer.serialize_array_field("data", &self.m_data)?;
            serializer.serialize_array_field("commands", &self.m_commands)?;
            serializer.serialize_array_field("modifiers", &self.m_modifiers)?;
            serializer.serialize_array_field("motors", &self.m_motors)?;
            serializer.end()
        }
    }
};
