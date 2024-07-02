use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpDisplayBindingDataPhysicsSystem`
/// -         version: `1`
/// -       signature: `0xc8ae86a7`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpDisplayBindingDataPhysicsSystem {
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
    /// -          name: `bindings`(ctype: `hkArray<hkpDisplayBindingDataRigidBody*>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bindings: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `system`(ctype: `struct hkpPhysicsSystem*`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_system: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpDisplayBindingDataPhysicsSystem {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpDisplayBindingDataPhysicsSystem"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3366880935u32)
        }
    }
    impl _serde::Serialize for hkpDisplayBindingDataPhysicsSystem {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3366880935u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpDisplayBindingDataPhysicsSystem", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("bindings", &self.m_bindings)?;
            serializer.serialize_field("system", &self.m_system)?;
            serializer.serialize_array_field("bindings", &self.m_bindings)?;
            serializer.end()
        }
    }
};
