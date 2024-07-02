use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpWheelConstraintData`
/// -         version: `0`
/// -       signature: `0xb4c46671`
/// -          size: 352(x86)/368(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpWheelConstraintData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintData,
    /// # C++ Info
    /// -          name: `atoms`(ctype: `struct hkpWheelConstraintDataAtoms`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size: 304(x86)/304(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_atoms: hkpWheelConstraintDataAtoms,
    /// # C++ Info
    /// -          name: `initialAxleInB`(ctype: `hkVector4`)
    /// -        offset: 320(x86)/336(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_initialAxleInB: Vector4,
    /// # C++ Info
    /// -          name: `initialSteeringAxisInB`(ctype: `hkVector4`)
    /// -        offset: 336(x86)/352(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_initialSteeringAxisInB: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpWheelConstraintData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpWheelConstraintData"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3032770161u32)
        }
    }
    impl __serde::Serialize for hkpWheelConstraintData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3032770161u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpWheelConstraintData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.serialize_field("initialAxleInB", &self.m_initialAxleInB)?;
            serializer
                .serialize_field(
                    "initialSteeringAxisInB",
                    &self.m_initialSteeringAxisInB,
                )?;
            serializer.end()
        }
    }
};
