use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConstraintChainInstance`
/// -         version: `0`
/// -       signature: `0x7a490753`
/// -          size:  72(x86)/136(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintChainInstance<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintInstance<'a>,
    /// # C++ Info
    /// -          name: `chainedEntities`(ctype: `hkArray<hkpEntity*>`)
    /// -        offset:  56(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_chainedEntities: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `action`(ctype: `struct hkpConstraintChainInstanceAction*`)
    /// -        offset:  68(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_action: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpConstraintChainInstance<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpConstraintChainInstance"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2051606355u32)
        }
    }
    impl<'a> __serde::Serialize for hkpConstraintChainInstance<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintChainInstance", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("owner", &self.parent.m_owner)?;
            serializer.serialize_field("data", &self.parent.m_data)?;
            serializer
                .serialize_field(
                    "constraintModifiers",
                    &self.parent.m_constraintModifiers,
                )?;
            serializer.serialize_field("entities", &self.parent.m_entities.as_slice())?;
            serializer.serialize_field("priority", &self.parent.m_priority)?;
            serializer.serialize_field("wantRuntime", &self.parent.m_wantRuntime)?;
            serializer
                .serialize_field(
                    "destructionRemapInfo",
                    &self.parent.m_destructionRemapInfo,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.skip_field("listeners", &self.parent.m_listeners)?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.skip_field("internal", &self.parent.m_internal)?;
            serializer.skip_field("uid", &self.parent.m_uid)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("chainedEntities", &self.m_chainedEntities)?;
            serializer.serialize_field("action", &self.m_action)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer
                .serialize_array_field("chainedEntities", &self.m_chainedEntities)?;
            serializer.end()
        }
    }
};
