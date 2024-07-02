use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSIStateManagerModifierBSiStateData`
/// -         version: `0`
/// -       signature: `0x6b8a15fc`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSIStateManagerModifierBSiStateData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `pStateMachine`(ctype: `struct hkbGenerator*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_pStateMachine: Pointer,
    /// # C++ Info
    /// -          name: `StateID`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_StateID: i32,
    /// # C++ Info
    /// -          name: `iStateToSetAs`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_iStateToSetAs: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for BSIStateManagerModifierBSiStateData {
        #[inline]
        fn name(&self) -> &'static str {
            "BSIStateManagerModifierBSiStateData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1804211708u32)
        }
    }
    impl _serde::Serialize for BSIStateManagerModifierBSiStateData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1804211708u32)));
            let mut serializer = __serializer
                .serialize_struct("BSIStateManagerModifierBSiStateData", class_meta)?;
            serializer.serialize_field("pStateMachine", &self.m_pStateMachine)?;
            serializer.serialize_field("StateID", &self.m_StateID)?;
            serializer.serialize_field("iStateToSetAs", &self.m_iStateToSetAs)?;
            serializer.end()
        }
    }
};
