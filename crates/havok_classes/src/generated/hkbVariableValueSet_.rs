use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbVariableValueSet`
/// -         version: `0`
/// -       signature: `0x27812d8d`
/// -          size:  44(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbVariableValueSet {
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
    /// -          name: `wordVariableValues`(ctype: `hkArray<struct hkbVariableValue>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wordVariableValues: Vec<hkbVariableValue>,
    /// # C++ Info
    /// -          name: `quadVariableValues`(ctype: `hkArray<hkVector4>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_quadVariableValues: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `variantVariableValues`(ctype: `hkArray<hkReferencedObject*>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_variantVariableValues: Vec<Pointer>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbVariableValueSet {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbVariableValueSet"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(662777229u32)
        }
    }
    impl __serde::Serialize for hkbVariableValueSet {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(662777229u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbVariableValueSet", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "wordVariableValues",
                    &self.m_wordVariableValues,
                )?;
            serializer
                .serialize_array_meta_field(
                    "quadVariableValues",
                    &self.m_quadVariableValues,
                )?;
            serializer
                .serialize_array_meta_field(
                    "variantVariableValues",
                    &self.m_variantVariableValues,
                )?;
            serializer
                .serialize_array_field(
                    "wordVariableValues",
                    &self.m_wordVariableValues,
                )?;
            serializer
                .serialize_array_field(
                    "quadVariableValues",
                    &self.m_quadVariableValues,
                )?;
            serializer
                .serialize_array_field(
                    "variantVariableValues",
                    &self.m_variantVariableValues,
                )?;
            serializer.end()
        }
    }
};
