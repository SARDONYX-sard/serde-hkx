use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSimpleContactConstraintAtom`
/// -         version: `0`
/// -       signature: `0x920df11a`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSimpleContactConstraintAtom {
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
    /// -          name: `sizeOfAllAtoms`(ctype: `hkUint16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_sizeOfAllAtoms: u16,
    /// # C++ Info
    /// -          name: `numContactPoints`(ctype: `hkUint16`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_numContactPoints: u16,
    /// # C++ Info
    /// -          name: `numReservedContactPoints`(ctype: `hkUint16`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_numReservedContactPoints: u16,
    /// # C++ Info
    /// -          name: `numUserDatasForBodyA`(ctype: `hkUint8`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numUserDatasForBodyA: u8,
    /// # C++ Info
    /// -          name: `numUserDatasForBodyB`(ctype: `hkUint8`)
    /// -        offset:   9(x86)/  9(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numUserDatasForBodyB: u8,
    /// # C++ Info
    /// -          name: `contactPointPropertiesStriding`(ctype: `hkUint8`)
    /// -        offset:  10(x86)/ 10(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_contactPointPropertiesStriding: u8,
    /// # C++ Info
    /// -          name: `maxNumContactPoints`(ctype: `hkUint16`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_maxNumContactPoints: u16,
    /// # C++ Info
    /// -          name: `info`(ctype: `struct hkpSimpleContactConstraintDataInfo`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_info: hkpSimpleContactConstraintDataInfo,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSimpleContactConstraintAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSimpleContactConstraintAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2450387226u32)
        }
    }
    impl __serde::Serialize for hkpSimpleContactConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSimpleContactConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("sizeOfAllAtoms", &self.m_sizeOfAllAtoms)?;
            serializer.serialize_field("numContactPoints", &self.m_numContactPoints)?;
            serializer
                .serialize_field(
                    "numReservedContactPoints",
                    &self.m_numReservedContactPoints,
                )?;
            serializer
                .serialize_field("numUserDatasForBodyA", &self.m_numUserDatasForBodyA)?;
            serializer
                .serialize_field("numUserDatasForBodyB", &self.m_numUserDatasForBodyB)?;
            serializer
                .serialize_field(
                    "contactPointPropertiesStriding",
                    &self.m_contactPointPropertiesStriding,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field("maxNumContactPoints", &self.m_maxNumContactPoints)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("info", &self.m_info)?;
            serializer.end()
        }
    }
};
