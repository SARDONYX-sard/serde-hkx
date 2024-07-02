use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxEdgeSelectionChannel`
/// -         version: `1`
/// -       signature: `0x9ad32a5e`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxEdgeSelectionChannel {
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
    /// -          name: `selectedEdges`(ctype: `hkArray<hkInt32>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_selectedEdges: Vec<i32>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxEdgeSelectionChannel {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxEdgeSelectionChannel"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2597530206u32)
        }
    }
    impl __serde::Serialize for hkxEdgeSelectionChannel {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2597530206u32)));
            let mut serializer = __serializer
                .serialize_struct("hkxEdgeSelectionChannel", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("selectedEdges", &self.m_selectedEdges)?;
            serializer.serialize_array_field("selectedEdges", &self.m_selectedEdges)?;
            serializer.end()
        }
    }
};
