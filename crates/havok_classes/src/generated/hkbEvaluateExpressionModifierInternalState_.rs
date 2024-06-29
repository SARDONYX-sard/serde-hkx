use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEvaluateExpressionModifierInternalState`
/// -         version: `0`
/// -       signature: `0xb414d58e`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEvaluateExpressionModifierInternalState {
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
    /// -          name: `internalExpressionsData`(ctype: `hkArray<struct hkbEvaluateExpressionModifierInternalExpressionData>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_internalExpressionsData: Vec<
        hkbEvaluateExpressionModifierInternalExpressionData,
    >,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbEvaluateExpressionModifierInternalState {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbEvaluateExpressionModifierInternalState"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3021264270u32)
        }
    }
    impl __serde::Serialize for hkbEvaluateExpressionModifierInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbEvaluateExpressionModifierInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "internalExpressionsData",
                    &self.m_internalExpressionsData,
                )?;
            serializer
                .serialize_array_field(
                    "internalExpressionsData",
                    &self.m_internalExpressionsData,
                )?;
            serializer.end()
        }
    }
};
