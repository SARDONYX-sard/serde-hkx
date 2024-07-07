use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSetLocalRotationsConstraintAtom`
/// -         version: `0`
/// -       signature: `0xf81db8e`
/// -          size: 112(x86)/112(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSetLocalRotationsConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// -          name: `rotationA`(ctype: `hkRotation`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_rotationA: Rotation,
    /// # C++ Info
    /// -          name: `rotationB`(ctype: `hkRotation`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_rotationB: Rotation,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSetLocalRotationsConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSetLocalRotationsConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf81db8e)
        }
    }
    impl _serde::Serialize for hkpSetLocalRotationsConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf81db8e)));
            let mut serializer = __serializer
                .serialize_struct("hkpSetLocalRotationsConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("rotationA", &self.m_rotationA)?;
            serializer.serialize_field("rotationB", &self.m_rotationB)?;
            serializer.end()
        }
    }
};
