use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTypedBroadPhaseHandle`
/// -         version: `0`
/// -       signature: `0xf4b0f799`
/// -          size:  12(x86)/ 12(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTypedBroadPhaseHandle {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpBroadPhaseHandle,
    /// # C++ Info
    /// -          name: `type`(ctype: `hkInt8`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: i8,
    /// # C++ Info
    /// -          name: `ownerOffset`(ctype: `hkInt8`)
    /// -        offset:   5(x86)/  5(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_ownerOffset: i8,
    /// # C++ Info
    /// -          name: `objectQualityType`(ctype: `hkInt8`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_objectQualityType: i8,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpTypedBroadPhaseHandle {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTypedBroadPhaseHandle"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(4105238425u32)
        }
    }
    impl _serde::Serialize for hkpTypedBroadPhaseHandle {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(4105238425u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpTypedBroadPhaseHandle", class_meta)?;
            serializer.skip_field("id", &self.parent.m_id)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.skip_field("ownerOffset", &self.m_ownerOffset)?;
            serializer.serialize_field("objectQualityType", &self.m_objectQualityType)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.end()
        }
    }
};
