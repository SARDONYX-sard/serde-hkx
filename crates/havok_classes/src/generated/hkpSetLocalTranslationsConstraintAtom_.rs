use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSetLocalTranslationsConstraintAtom`
/// -         version: `0`
/// -       signature: `0x5cbfcf4a`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSetLocalTranslationsConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// -          name: `translationA`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_translationA: Vector4,
    /// # C++ Info
    /// -          name: `translationB`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_translationB: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSetLocalTranslationsConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSetLocalTranslationsConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1556074314u32)
        }
    }
    impl __serde::Serialize for hkpSetLocalTranslationsConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSetLocalTranslationsConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("translationA", &self.m_translationA)?;
            serializer.serialize_field("translationB", &self.m_translationB)?;
            serializer.end()
        }
    }
};
