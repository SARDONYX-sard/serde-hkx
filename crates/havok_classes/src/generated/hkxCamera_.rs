use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxCamera`
/// -         version: `1`
/// -       signature: `0xe3597b02`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxCamera {
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
    /// -          name: `from`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_from: Vector4,
    /// # C++ Info
    /// -          name: `focus`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_focus: Vector4,
    /// # C++ Info
    /// -          name: `up`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_up: Vector4,
    /// # C++ Info
    /// -          name: `fov`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fov: f32,
    /// # C++ Info
    /// -          name: `far`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_far: f32,
    /// # C++ Info
    /// -          name: `near`(ctype: `hkReal`)
    /// -        offset:  72(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_near: f32,
    /// # C++ Info
    /// -          name: `leftHanded`(ctype: `hkBool`)
    /// -        offset:  76(x86)/ 76(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_leftHanded: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxCamera {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxCamera"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3814292226u32)
        }
    }
    impl __serde::Serialize for hkxCamera {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3814292226u32)));
            let mut serializer = __serializer.serialize_struct("hkxCamera", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("from", &self.m_from)?;
            serializer.serialize_field("focus", &self.m_focus)?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer.serialize_field("fov", &self.m_fov)?;
            serializer.serialize_field("far", &self.m_far)?;
            serializer.serialize_field("near", &self.m_near)?;
            serializer.serialize_field("leftHanded", &self.m_leftHanded)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
