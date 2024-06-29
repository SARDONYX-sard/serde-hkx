use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMotorAction`
/// -         version: `0`
/// -       signature: `0x8ff131d9`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMotorAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpUnaryAction<'a>,
    /// # C++ Info
    /// -          name: `axis`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_axis: Vector4,
    /// # C++ Info
    /// -          name: `spinRate`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_spinRate: f32,
    /// # C++ Info
    /// -          name: `gain`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_gain: f32,
    /// # C++ Info
    /// -          name: `active`(ctype: `hkBool`)
    /// -        offset:  56(x86)/ 88(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_active: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpMotorAction<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpMotorAction"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2414948825u32)
        }
    }
    impl<'a> __serde::Serialize for hkpMotorAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpMotorAction", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.skip_field("island", &self.parent.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_field("entity", &self.parent.m_entity)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("axis", &self.m_axis)?;
            serializer.serialize_field("spinRate", &self.m_spinRate)?;
            serializer.serialize_field("gain", &self.m_gain)?;
            serializer.serialize_field("active", &self.m_active)?;
            serializer.pad_field([0u8; 7usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
