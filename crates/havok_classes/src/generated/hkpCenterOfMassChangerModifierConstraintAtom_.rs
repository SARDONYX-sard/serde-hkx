use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCenterOfMassChangerModifierConstraintAtom`
/// -         version: `0`
/// -       signature: `0x1d7dbdd2`
/// -          size:  64(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCenterOfMassChangerModifierConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpModifierConstraintAtom,
    /// # C++ Info
    /// -          name: `displacementA`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_displacementA: Vector4,
    /// # C++ Info
    /// -          name: `displacementB`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_displacementB: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCenterOfMassChangerModifierConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpCenterOfMassChangerModifierConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(494779858u32)
        }
    }
    impl __serde::Serialize for hkpCenterOfMassChangerModifierConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpCenterOfMassChangerModifierConstraintAtom",
                    class_meta,
                )?;
            serializer.serialize_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer
                .serialize_field("modifierAtomSize", &self.parent.m_modifierAtomSize)?;
            serializer.serialize_field("childSize", &self.parent.m_childSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("child", &self.parent.m_child)?;
            serializer.serialize_field("pad", &self.parent.m_pad.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("displacementA", &self.m_displacementA)?;
            serializer.serialize_field("displacementB", &self.m_displacementB)?;
            serializer.end()
        }
    }
};
