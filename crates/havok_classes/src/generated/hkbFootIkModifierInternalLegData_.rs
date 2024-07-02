use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkModifierInternalLegData`
/// -         version: `1`
/// -       signature: `0xe5ca3677`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkModifierInternalLegData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `groundPosition`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_groundPosition: Vector4,
    /// # C++ Info
    /// -          name: `footIkSolver`(ctype: `void*`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_footIkSolver: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbFootIkModifierInternalLegData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkModifierInternalLegData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3855234679u32)
        }
    }
    impl _serde::Serialize for hkbFootIkModifierInternalLegData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3855234679u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkModifierInternalLegData", class_meta)?;
            serializer.serialize_field("groundPosition", &self.m_groundPosition)?;
            serializer.skip_field("footIkSolver", &self.m_footIkSolver)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
