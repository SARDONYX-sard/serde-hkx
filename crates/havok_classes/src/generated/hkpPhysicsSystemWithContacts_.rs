use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPhysicsSystemWithContacts`
/// -         version: `0`
/// -       signature: `0xd0fd4bbe`
/// -          size:  80(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPhysicsSystemWithContacts<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpPhysicsSystem<'a>,
    /// # C++ Info
    /// -          name: `contacts`(ctype: `hkArray<hkpSerializedAgentNnEntry*>`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_contacts: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpPhysicsSystemWithContacts<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPhysicsSystemWithContacts"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd0fd4bbe)
        }
    }
    impl<'a> _serde::Serialize for hkpPhysicsSystemWithContacts<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd0fd4bbe)));
            let mut serializer = __serializer
                .serialize_struct("hkpPhysicsSystemWithContacts", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("rigidBodies", &self.parent.m_rigidBodies)?;
            serializer
                .serialize_array_meta_field("constraints", &self.parent.m_constraints)?;
            serializer.serialize_array_meta_field("actions", &self.parent.m_actions)?;
            serializer.serialize_array_meta_field("phantoms", &self.parent.m_phantoms)?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("active", &self.parent.m_active)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_meta_field("contacts", &self.m_contacts)?;
            serializer.serialize_array_field("rigidBodies", &self.parent.m_rigidBodies)?;
            serializer.serialize_array_field("constraints", &self.parent.m_constraints)?;
            serializer.serialize_array_field("actions", &self.parent.m_actions)?;
            serializer.serialize_array_field("phantoms", &self.parent.m_phantoms)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("contacts", &self.m_contacts)?;
            serializer.end()
        }
    }
};
