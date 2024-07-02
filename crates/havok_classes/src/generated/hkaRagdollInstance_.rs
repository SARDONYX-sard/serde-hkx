use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaRagdollInstance`
/// -         version: `0`
/// -       signature: `0x154948e8`
/// -          size:  48(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaRagdollInstance {
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
    /// -          name: `boneToRigidBodyMap`(ctype: `hkArray<hkInt32>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_boneToRigidBodyMap: Vec<i32>,
    /// # C++ Info
    /// -          name: `skeleton`(ctype: `struct hkaSkeleton*`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_skeleton: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaRagdollInstance {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaRagdollInstance"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(357124328u32)
        }
    }
    impl _serde::Serialize for hkaRagdollInstance {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(357124328u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaRagdollInstance", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("rigidBodies", &self.m_rigidBodies)?;
            serializer.serialize_array_meta_field("constraints", &self.m_constraints)?;
            serializer
                .serialize_array_meta_field(
                    "boneToRigidBodyMap",
                    &self.m_boneToRigidBodyMap,
                )?;
            serializer.serialize_field("skeleton", &self.m_skeleton)?;
            serializer.serialize_array_field("rigidBodies", &self.m_rigidBodies)?;
            serializer.serialize_array_field("constraints", &self.m_constraints)?;
            serializer
                .serialize_array_field(
                    "boneToRigidBodyMap",
                    &self.m_boneToRigidBodyMap,
                )?;
            serializer.end()
        }
    }
};
