use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpHingeLimitsData`
/// -         version: `0`
/// -       signature: `0xbd46760a`
/// -          size: 160(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpHingeLimitsData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintData,
    /// # C++ Info
    /// -          name: `atoms`(ctype: `struct hkpHingeLimitsDataAtoms`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size: 144(x86)/144(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_atoms: hkpHingeLimitsDataAtoms,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpHingeLimitsData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpHingeLimitsData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xbd46760a)
        }
    }
    impl _serde::Serialize for hkpHingeLimitsData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xbd46760a)));
            let mut serializer = __serializer
                .serialize_struct("hkpHingeLimitsData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.end()
        }
    }
};
