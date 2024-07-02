use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpModifierConstraintAtom`
/// -         version: `0`
/// -       signature: `0xb13fef1f`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpModifierConstraintAtom {
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
    /// -          name: `modifierAtomSize`(ctype: `hkUint16`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_modifierAtomSize: u16,
    /// # C++ Info
    /// -          name: `childSize`(ctype: `hkUint16`)
    /// -        offset:  18(x86)/ 18(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_childSize: u16,
    /// # C++ Info
    /// -          name: `child`(ctype: `struct hkpConstraintAtom*`)
    /// -        offset:  20(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_child: Pointer,
    /// # C++ Info
    /// -          name: `pad`(ctype: `hkUint32[2]`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_pad: [u32; 2usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpModifierConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpModifierConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2973757215u32)
        }
    }
    impl __serde::Serialize for hkpModifierConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2973757215u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpModifierConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("modifierAtomSize", &self.m_modifierAtomSize)?;
            serializer.serialize_field("childSize", &self.m_childSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("child", &self.m_child)?;
            serializer.serialize_field("pad", &self.m_pad.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
