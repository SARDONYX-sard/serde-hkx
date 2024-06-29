use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSetLocalRotationsConstraintAtom`
/// -         version: `0`
/// -       signature: `0xf81db8e`
/// -          size: 112(x86)/112(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSetLocalRotationsConstraintAtom {
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
    /// -          name: `rotationA`(ctype: `hkRotation`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_rotationA: Rotation,
    /// # C++ Info
    /// -          name: `rotationB`(ctype: `hkRotation`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_rotationB: Rotation,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSetLocalRotationsConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSetLocalRotationsConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(260168590u32)
        }
    }
    impl __serde::Serialize for hkpSetLocalRotationsConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSetLocalRotationsConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("rotationA", &self.m_rotationA)?;
            serializer.serialize_field("rotationB", &self.m_rotationB)?;
            serializer.end()
        }
    }
};
