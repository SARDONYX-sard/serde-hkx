use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbDampingModifierInternalState`
/// -         version: `0`
/// -       signature: `0x508d3b36`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbDampingModifierInternalState {
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
    /// -          name: `dampedVector`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_dampedVector: Vector4,
    /// # C++ Info
    /// -          name: `vecErrorSum`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vecErrorSum: Vector4,
    /// # C++ Info
    /// -          name: `vecPreviousError`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vecPreviousError: Vector4,
    /// # C++ Info
    /// -          name: `dampedValue`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dampedValue: f32,
    /// # C++ Info
    /// -          name: `errorSum`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_errorSum: f32,
    /// # C++ Info
    /// -          name: `previousError`(ctype: `hkReal`)
    /// -        offset:  72(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_previousError: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbDampingModifierInternalState {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbDampingModifierInternalState"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1351433014u32)
        }
    }
    impl __serde::Serialize for hkbDampingModifierInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbDampingModifierInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("dampedVector", &self.m_dampedVector)?;
            serializer.serialize_field("vecErrorSum", &self.m_vecErrorSum)?;
            serializer.serialize_field("vecPreviousError", &self.m_vecPreviousError)?;
            serializer.serialize_field("dampedValue", &self.m_dampedValue)?;
            serializer.serialize_field("errorSum", &self.m_errorSum)?;
            serializer.serialize_field("previousError", &self.m_previousError)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
