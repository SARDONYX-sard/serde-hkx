use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbComputeRotationToTargetModifierInternalState`
/// -         version: `0`
/// -       signature: `0x71cd1eb0`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbComputeRotationToTargetModifierInternalState {
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
    /// -          name: `rotationOut`(ctype: `hkQuaternion`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotationOut: Quaternion,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbComputeRotationToTargetModifierInternalState {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbComputeRotationToTargetModifierInternalState"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1909268144u32)
        }
    }
    impl __serde::Serialize for hkbComputeRotationToTargetModifierInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbComputeRotationToTargetModifierInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("rotationOut", &self.m_rotationOut)?;
            serializer.end()
        }
    }
};
