use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConstraintChainInstanceAction`
/// -         version: `0`
/// -       signature: `0xc3971189`
/// -          size:  28(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintChainInstanceAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpAction<'a>,
    /// # C++ Info
    /// -          name: `constraintInstance`(ctype: `struct hkpConstraintChainInstance*`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `NOT_OWNED`
    ///
    pub m_constraintInstance: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpConstraintChainInstanceAction<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConstraintChainInstanceAction"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3281457545u32)
        }
    }
    impl<'a> __serde::Serialize for hkpConstraintChainInstanceAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3281457545u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintChainInstanceAction", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.m_world)?;
            serializer.skip_field("island", &self.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer
                .serialize_field("constraintInstance", &self.m_constraintInstance)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.end()
        }
    }
};
