use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSetLocalTransformsConstraintAtom`
/// -         version: `0`
/// -       signature: `0x6e2a5198`
/// -          size: 144(x86)/144(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSetLocalTransformsConstraintAtom {
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
    /// -          name: `transformA`(ctype: `hkTransform`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_transformA: Transform,
    /// # C++ Info
    /// -          name: `transformB`(ctype: `hkTransform`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_transformB: Transform,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSetLocalTransformsConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSetLocalTransformsConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6e2a5198)
        }
    }
    impl _serde::Serialize for hkpSetLocalTransformsConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6e2a5198)));
            let mut serializer = __serializer
                .serialize_struct("hkpSetLocalTransformsConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("transformA", &self.m_transformA)?;
            serializer.serialize_field("transformB", &self.m_transformB)?;
            serializer.end()
        }
    }
};
