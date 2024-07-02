use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxAnimatedMatrix`
/// -         version: `1`
/// -       signature: `0x5838e337`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxAnimatedMatrix {
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
    /// -          name: `matrices`(ctype: `hkArray<hkMatrix4>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_matrices: Vec<Matrix4>,
    /// # C++ Info
    /// -          name: `hint`(ctype: `enum Hint`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hint: Hint,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxAnimatedMatrix {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxAnimatedMatrix"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1480123191u32)
        }
    }
    impl _serde::Serialize for hkxAnimatedMatrix {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1480123191u32)));
            let mut serializer = __serializer
                .serialize_struct("hkxAnimatedMatrix", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("matrices", &self.m_matrices)?;
            serializer.serialize_field("hint", &self.m_hint)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_field("matrices", &self.m_matrices)?;
            serializer.end()
        }
    }
};
