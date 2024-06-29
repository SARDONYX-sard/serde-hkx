use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorInfo`
/// -         version: `1`
/// -       signature: `0xf7645395`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorInfo<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `data`(ctype: `struct hkbBehaviorGraphData*`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_data: Pointer,
    /// # C++ Info
    /// -          name: `idToNamePairs`(ctype: `hkArray<struct hkbBehaviorInfoIdToNamePair>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_idToNamePairs: Vec<hkbBehaviorInfoIdToNamePair<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbBehaviorInfo<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbBehaviorInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4150547349u32)
        }
    }
    impl<'a> __serde::Serialize for hkbBehaviorInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer
                .serialize_array_meta_field("idToNamePairs", &self.m_idToNamePairs)?;
            serializer.serialize_array_field("idToNamePairs", &self.m_idToNamePairs)?;
            serializer.end()
        }
    }
};
