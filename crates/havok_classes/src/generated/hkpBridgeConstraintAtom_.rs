use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpBridgeConstraintAtom`
/// -         version: `0`
/// -       signature: `0x87a4f31b`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBridgeConstraintAtom {
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
    /// -          name: `buildJacobianFunc`(ctype: `void*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_buildJacobianFunc: Pointer,
    /// # C++ Info
    /// -          name: `constraintData`(ctype: `struct hkpConstraintData*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `NOT_OWNED`
    ///
    pub m_constraintData: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpBridgeConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBridgeConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x87a4f31b)
        }
    }
    impl _serde::Serialize for hkpBridgeConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x87a4f31b)));
            let mut serializer = __serializer
                .serialize_struct("hkpBridgeConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.skip_field("buildJacobianFunc", &self.m_buildJacobianFunc)?;
            serializer.serialize_field("constraintData", &self.m_constraintData)?;
            serializer.end()
        }
    }
};
