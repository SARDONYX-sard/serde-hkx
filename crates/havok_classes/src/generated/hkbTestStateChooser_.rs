use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbTestStateChooser`
/// -         version: `0`
/// -       signature: `0xc0fcc436`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbTestStateChooser<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbStateChooser,
    /// # C++ Info
    /// -          name: `int`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_int: i32,
    /// # C++ Info
    /// -          name: `real`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_real: f32,
    /// # C++ Info
    /// -          name: `string`(ctype: `hkStringPtr`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_string: StringPtr<'a>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbTestStateChooser<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbTestStateChooser"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3237790774u32)
        }
    }
    impl<'a> __serde::Serialize for hkbTestStateChooser<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3237790774u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbTestStateChooser", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("int", &self.m_int)?;
            serializer.serialize_field("real", &self.m_real)?;
            serializer.serialize_stringptr_meta_field("string", &self.m_string)?;
            serializer.serialize_stringptr_field("string", &self.m_string)?;
            serializer.end()
        }
    }
};
