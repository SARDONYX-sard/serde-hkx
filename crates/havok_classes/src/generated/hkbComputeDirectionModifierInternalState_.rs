use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbComputeDirectionModifierInternalState`
/// -         version: `0`
/// -       signature: `0x6ac054d7`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbComputeDirectionModifierInternalState {
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
    /// -          name: `pointOut`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pointOut: Vector4,
    /// # C++ Info
    /// -          name: `groundAngleOut`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_groundAngleOut: f32,
    /// # C++ Info
    /// -          name: `upAngleOut`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_upAngleOut: f32,
    /// # C++ Info
    /// -          name: `computedOutput`(ctype: `hkBool`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_computedOutput: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbComputeDirectionModifierInternalState {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbComputeDirectionModifierInternalState"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1790989527u32)
        }
    }
    impl __serde::Serialize for hkbComputeDirectionModifierInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbComputeDirectionModifierInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("pointOut", &self.m_pointOut)?;
            serializer.serialize_field("groundAngleOut", &self.m_groundAngleOut)?;
            serializer.serialize_field("upAngleOut", &self.m_upAngleOut)?;
            serializer.serialize_field("computedOutput", &self.m_computedOutput)?;
            serializer.pad_field([0u8; 7usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
