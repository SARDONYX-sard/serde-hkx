use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpDisplayBindingDataRigidBody`
/// -         version: `2`
/// -       signature: `0xfe16e2a3`
/// -          size:  80(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpDisplayBindingDataRigidBody {
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
    /// -          name: `rigidBody`(ctype: `struct hkpRigidBody*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rigidBody: Pointer,
    /// # C++ Info
    /// -          name: `displayObjectPtr`(ctype: `struct hkReferencedObject*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_displayObjectPtr: Pointer,
    /// # C++ Info
    /// -          name: `rigidBodyFromDisplayObjectTransform`(ctype: `hkMatrix4`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_rigidBodyFromDisplayObjectTransform: Matrix4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpDisplayBindingDataRigidBody {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpDisplayBindingDataRigidBody"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xfe16e2a3)
        }
    }
    impl _serde::Serialize for hkpDisplayBindingDataRigidBody {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xfe16e2a3)));
            let mut serializer = __serializer
                .serialize_struct("hkpDisplayBindingDataRigidBody", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("rigidBody", &self.m_rigidBody)?;
            serializer.serialize_field("displayObjectPtr", &self.m_displayObjectPtr)?;
            serializer
                .serialize_field(
                    "rigidBodyFromDisplayObjectTransform",
                    &self.m_rigidBodyFromDisplayObjectTransform,
                )?;
            serializer.end()
        }
    }
};
