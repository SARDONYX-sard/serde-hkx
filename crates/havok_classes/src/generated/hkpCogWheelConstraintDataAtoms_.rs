use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCogWheelConstraintDataAtoms`
/// -         version: `0`
/// -       signature: `0xf855ba44`
/// -          size: 160(x86)/160(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCogWheelConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `transforms`(ctype: `struct hkpSetLocalTransformsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size: 144(x86)/144(x86_64)
    ///
    pub m_transforms: hkpSetLocalTransformsConstraintAtom,
    /// # C++ Info
    /// -          name: `cogWheels`(ctype: `struct hkpCogWheelConstraintAtom`)
    /// -        offset: 144(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_cogWheels: hkpCogWheelConstraintAtom,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCogWheelConstraintDataAtoms {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpCogWheelConstraintDataAtoms"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4166367812u32)
        }
    }
    impl __serde::Serialize for hkpCogWheelConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpCogWheelConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer.serialize_field("cogWheels", &self.m_cogWheels)?;
            serializer.end()
        }
    }
};