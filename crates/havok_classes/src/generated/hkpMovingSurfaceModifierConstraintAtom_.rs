use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMovingSurfaceModifierConstraintAtom`
/// -         version: `0`
/// -       signature: `0x79ab517d`
/// -          size:  48(x86)/ 64(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMovingSurfaceModifierConstraintAtom {
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
    /// -          name: `velocity`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_velocity: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMovingSurfaceModifierConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMovingSurfaceModifierConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2041270653u32)
        }
    }
    impl _serde::Serialize for hkpMovingSurfaceModifierConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2041270653u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpMovingSurfaceModifierConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer
                .serialize_field("modifierAtomSize", &self.parent.m_modifierAtomSize)?;
            serializer.serialize_field("childSize", &self.parent.m_childSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("child", &self.parent.m_child)?;
            serializer.serialize_field("pad", &self.parent.m_pad.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("velocity", &self.m_velocity)?;
            serializer.end()
        }
    }
};
