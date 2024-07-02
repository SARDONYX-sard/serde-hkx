use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSingleShapeContainer`
/// -         version: `0`
/// -       signature: `0x73aa1d38`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSingleShapeContainer {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShapeContainer,
    /// # C++ Info
    /// -          name: `childShape`(ctype: `struct hkpShape*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_childShape: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSingleShapeContainer {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSingleShapeContainer"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1940528440u32)
        }
    }
    impl _serde::Serialize for hkpSingleShapeContainer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1940528440u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpSingleShapeContainer", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("childShape", &self.m_childShape)?;
            serializer.end()
        }
    }
};
