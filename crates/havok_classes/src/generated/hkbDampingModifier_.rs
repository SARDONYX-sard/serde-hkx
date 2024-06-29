use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbDampingModifier`
/// -         version: `1`
/// -       signature: `0x9a040f03`
/// -          size: 160(x86)/192(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbDampingModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `kP`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_kP: f32,
    /// # C++ Info
    /// -          name: `kI`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_kI: f32,
    /// # C++ Info
    /// -          name: `kD`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_kD: f32,
    /// # C++ Info
    /// -          name: `enableScalarDamping`(ctype: `hkBool`)
    /// -        offset:  56(x86)/ 92(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableScalarDamping: bool,
    /// # C++ Info
    /// -          name: `enableVectorDamping`(ctype: `hkBool`)
    /// -        offset:  57(x86)/ 93(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableVectorDamping: bool,
    /// # C++ Info
    /// -          name: `rawValue`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_rawValue: f32,
    /// # C++ Info
    /// -          name: `dampedValue`(ctype: `hkReal`)
    /// -        offset:  64(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dampedValue: f32,
    /// # C++ Info
    /// -          name: `rawVector`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rawVector: Vector4,
    /// # C++ Info
    /// -          name: `dampedVector`(ctype: `hkVector4`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_dampedVector: Vector4,
    /// # C++ Info
    /// -          name: `vecErrorSum`(ctype: `hkVector4`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vecErrorSum: Vector4,
    /// # C++ Info
    /// -          name: `vecPreviousError`(ctype: `hkVector4`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vecPreviousError: Vector4,
    /// # C++ Info
    /// -          name: `errorSum`(ctype: `hkReal`)
    /// -        offset: 144(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_errorSum: f32,
    /// # C++ Info
    /// -          name: `previousError`(ctype: `hkReal`)
    /// -        offset: 148(x86)/180(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_previousError: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbDampingModifier<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbDampingModifier"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2583957251u32)
        }
    }
    impl<'a> __serde::Serialize for hkbDampingModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbDampingModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("kP", &self.m_kP)?;
            serializer.serialize_field("kI", &self.m_kI)?;
            serializer.serialize_field("kD", &self.m_kD)?;
            serializer
                .serialize_field("enableScalarDamping", &self.m_enableScalarDamping)?;
            serializer
                .serialize_field("enableVectorDamping", &self.m_enableVectorDamping)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("rawValue", &self.m_rawValue)?;
            serializer.serialize_field("dampedValue", &self.m_dampedValue)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("rawVector", &self.m_rawVector)?;
            serializer.serialize_field("dampedVector", &self.m_dampedVector)?;
            serializer.serialize_field("vecErrorSum", &self.m_vecErrorSum)?;
            serializer.serialize_field("vecPreviousError", &self.m_vecPreviousError)?;
            serializer.serialize_field("errorSum", &self.m_errorSum)?;
            serializer.serialize_field("previousError", &self.m_previousError)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
