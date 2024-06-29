use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpGravityGun`
/// -         version: `0`
/// -       signature: `0x5e2754cd`
/// -          size:  96(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpGravityGun<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpFirstPersonGun<'a>,
    /// # C++ Info
    /// -          name: `grabbedBodies`(ctype: `hkArray<void*>`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_grabbedBodies: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `maxNumObjectsPicked`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxNumObjectsPicked: i32,
    /// # C++ Info
    /// -          name: `maxMassOfObjectPicked`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxMassOfObjectPicked: f32,
    /// # C++ Info
    /// -          name: `maxDistOfObjectPicked`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxDistOfObjectPicked: f32,
    /// # C++ Info
    /// -          name: `impulseAppliedWhenObjectNotPicked`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_impulseAppliedWhenObjectNotPicked: f32,
    /// # C++ Info
    /// -          name: `throwVelocity`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_throwVelocity: f32,
    /// # C++ Info
    /// -          name: `capturedObjectPosition`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_capturedObjectPosition: Vector4,
    /// # C++ Info
    /// -          name: `capturedObjectsOffset`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_capturedObjectsOffset: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpGravityGun<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpGravityGun"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1579635917u32)
        }
    }
    impl<'a> __serde::Serialize for hkpGravityGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpGravityGun", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("keyboardKey", &self.parent.m_keyboardKey)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_array_meta_field("listeners", &self.parent.m_listeners)?;
            serializer.skip_array_meta_field("grabbedBodies", &self.m_grabbedBodies)?;
            serializer
                .serialize_field("maxNumObjectsPicked", &self.m_maxNumObjectsPicked)?;
            serializer
                .serialize_field(
                    "maxMassOfObjectPicked",
                    &self.m_maxMassOfObjectPicked,
                )?;
            serializer
                .serialize_field(
                    "maxDistOfObjectPicked",
                    &self.m_maxDistOfObjectPicked,
                )?;
            serializer
                .serialize_field(
                    "impulseAppliedWhenObjectNotPicked",
                    &self.m_impulseAppliedWhenObjectNotPicked,
                )?;
            serializer.serialize_field("throwVelocity", &self.m_throwVelocity)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "capturedObjectPosition",
                    &self.m_capturedObjectPosition,
                )?;
            serializer
                .serialize_field(
                    "capturedObjectsOffset",
                    &self.m_capturedObjectsOffset,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("listeners", &self.parent.m_listeners)?;
            serializer.serialize_array_field("grabbedBodies", &self.m_grabbedBodies)?;
            serializer.end()
        }
    }
};
