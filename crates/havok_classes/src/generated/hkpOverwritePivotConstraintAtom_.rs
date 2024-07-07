use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpOverwritePivotConstraintAtom`
/// -         version: `0`
/// -       signature: `0x1f11b467`
/// -          size:   4(x86)/  4(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpOverwritePivotConstraintAtom {
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
    /// -          name: `copyToPivotBFromPivotA`(ctype: `hkUint8`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_copyToPivotBFromPivotA: u8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpOverwritePivotConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpOverwritePivotConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x1f11b467)
        }
    }
    impl _serde::Serialize for hkpOverwritePivotConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x1f11b467)));
            let mut serializer = __serializer
                .serialize_struct("hkpOverwritePivotConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer
                .serialize_field(
                    "copyToPivotBFromPivotA",
                    &self.m_copyToPivotBFromPivotA,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
