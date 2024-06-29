use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPhysicsSystem`
/// -         version: `0`
/// -       signature: `0xff724c17`
/// -          size:  68(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPhysicsSystem<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `rigidBodies`(ctype: `hkArray<hkpRigidBody*>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rigidBodies: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `constraints`(ctype: `hkArray<hkpConstraintInstance*>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_constraints: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `actions`(ctype: `hkArray<hkpAction*>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_actions: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `phantoms`(ctype: `hkArray<hkpPhantom*>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_phantoms: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:  60(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
    /// # C++ Info
    /// -          name: `active`(ctype: `hkBool`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_active: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpPhysicsSystem<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpPhysicsSystem"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4285680663u32)
        }
    }
    impl<'a> __serde::Serialize for hkpPhysicsSystem<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpPhysicsSystem", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("rigidBodies", &self.m_rigidBodies)?;
            serializer.serialize_array_meta_field("constraints", &self.m_constraints)?;
            serializer.serialize_array_meta_field("actions", &self.m_actions)?;
            serializer.serialize_array_meta_field("phantoms", &self.m_phantoms)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.serialize_field("active", &self.m_active)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_field("rigidBodies", &self.m_rigidBodies)?;
            serializer.serialize_array_field("constraints", &self.m_constraints)?;
            serializer.serialize_array_field("actions", &self.m_actions)?;
            serializer.serialize_array_field("phantoms", &self.m_phantoms)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
