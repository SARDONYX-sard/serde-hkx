use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConstraintCollisionFilter`
/// -         version: `0`
/// -       signature: `0xc3b577b1`
/// -          size:  68(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintCollisionFilter {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpPairCollisionFilter,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConstraintCollisionFilter {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConstraintCollisionFilter"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc3b577b1)
        }
    }
    impl _serde::Serialize for hkpConstraintCollisionFilter {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc3b577b1)));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintCollisionFilter", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 16usize].as_slice(), [0u8; 32usize].as_slice())?;
            serializer
                .serialize_field("prepad", &self.parent.parent.m_prepad.as_slice())?;
            serializer.serialize_field("type", &self.parent.parent.m_type)?;
            serializer
                .serialize_field("postpad", &self.parent.parent.m_postpad.as_slice())?;
            serializer.skip_field("disabledPairs", &self.parent.m_disabledPairs)?;
            serializer.serialize_field("childFilter", &self.parent.m_childFilter)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
