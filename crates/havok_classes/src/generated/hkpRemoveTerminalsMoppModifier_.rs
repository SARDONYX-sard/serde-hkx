use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpRemoveTerminalsMoppModifier`
/// -         version: `0`
/// -       signature: `0x91367f03`
/// -          size:  28(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRemoveTerminalsMoppModifier {
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
    /// -          name: `removeInfo`(ctype: `hkArray<hkUint32>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_removeInfo: Vec<u32>,
    /// # C++ Info
    /// -          name: `tempShapesToRemove`(ctype: `void*`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_tempShapesToRemove: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpRemoveTerminalsMoppModifier {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRemoveTerminalsMoppModifier"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2436267779u32)
        }
    }
    impl __serde::Serialize for hkpRemoveTerminalsMoppModifier {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2436267779u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpRemoveTerminalsMoppModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_array_meta_field("removeInfo", &self.m_removeInfo)?;
            serializer.skip_field("tempShapesToRemove", &self.m_tempShapesToRemove)?;
            serializer.serialize_array_field("removeInfo", &self.m_removeInfo)?;
            serializer.end()
        }
    }
};
