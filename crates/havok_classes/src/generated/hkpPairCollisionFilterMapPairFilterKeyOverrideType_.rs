use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPairCollisionFilterMapPairFilterKeyOverrideType`
/// -         version: `0`
/// -       signature: `0x36195969`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPairCollisionFilterMapPairFilterKeyOverrideType {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `elem`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_elem: Pointer,
    /// # C++ Info
    /// -          name: `numElems`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numElems: i32,
    /// # C++ Info
    /// -          name: `hashMod`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_hashMod: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpPairCollisionFilterMapPairFilterKeyOverrideType {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPairCollisionFilterMapPairFilterKeyOverrideType"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(907630953u32)
        }
    }
    impl __serde::Serialize for hkpPairCollisionFilterMapPairFilterKeyOverrideType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(907630953u32)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpPairCollisionFilterMapPairFilterKeyOverrideType",
                    class_meta,
                )?;
            serializer.skip_field("elem", &self.m_elem)?;
            serializer.serialize_field("numElems", &self.m_numElems)?;
            serializer.serialize_field("hashMod", &self.m_hashMod)?;
            serializer.end()
        }
    }
};
