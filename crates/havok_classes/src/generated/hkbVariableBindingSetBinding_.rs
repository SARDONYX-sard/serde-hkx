use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbVariableBindingSetBinding`
/// -         version: `1`
/// -       signature: `0x4d592f72`
/// -          size:  32(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbVariableBindingSetBinding<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `memberPath`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_memberPath: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `memberClass`(ctype: `void*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_memberClass: Pointer,
    /// # C++ Info
    /// -          name: `offsetInObjectPlusOne`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_offsetInObjectPlusOne: i32,
    /// # C++ Info
    /// -          name: `offsetInArrayPlusOne`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_offsetInArrayPlusOne: i32,
    /// # C++ Info
    /// -          name: `rootVariableIndex`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_rootVariableIndex: i32,
    /// # C++ Info
    /// -          name: `variableIndex`(ctype: `hkInt32`)
    /// -        offset:  20(x86)/ 28(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_variableIndex: i32,
    /// # C++ Info
    /// -          name: `bitIndex`(ctype: `hkInt8`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bitIndex: i8,
    /// # C++ Info
    /// -          name: `bindingType`(ctype: `enum BindingType`)
    /// -        offset:  25(x86)/ 33(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bindingType: BindingType,
    /// # C++ Info
    /// -          name: `memberType`(ctype: `enum unknown`)
    /// -        offset:  26(x86)/ 34(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_memberType: u8,
    /// # C++ Info
    /// -          name: `variableType`(ctype: `hkInt8`)
    /// -        offset:  27(x86)/ 35(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_variableType: i8,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags unknown`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_flags: i8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbVariableBindingSetBinding<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbVariableBindingSetBinding"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1297690482u32)
        }
    }
    impl<'a> __serde::Serialize for hkbVariableBindingSetBinding<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbVariableBindingSetBinding", class_meta)?;
            serializer.serialize_stringptr_meta_field("memberPath", &self.m_memberPath)?;
            serializer.skip_field("memberClass", &self.m_memberClass)?;
            serializer
                .skip_field("offsetInObjectPlusOne", &self.m_offsetInObjectPlusOne)?;
            serializer.skip_field("offsetInArrayPlusOne", &self.m_offsetInArrayPlusOne)?;
            serializer.skip_field("rootVariableIndex", &self.m_rootVariableIndex)?;
            serializer.serialize_field("variableIndex", &self.m_variableIndex)?;
            serializer.serialize_field("bitIndex", &self.m_bitIndex)?;
            serializer.serialize_field("bindingType", &self.m_bindingType)?;
            serializer.skip_field("memberType", &self.m_memberType)?;
            serializer.skip_field("variableType", &self.m_variableType)?;
            serializer.skip_field("flags", &self.m_flags)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_stringptr_field("memberPath", &self.m_memberPath)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum BindingType {
    #[default]
    BINDING_TYPE_VARIABLE = 0isize,
    BINDING_TYPE_CHARACTER_PROPERTY = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BindingType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BINDING_TYPE_VARIABLE => {
                    __serializer.serialize_field("BINDING_TYPE_VARIABLE", &0u64)
                }
                Self::BINDING_TYPE_CHARACTER_PROPERTY => {
                    __serializer
                        .serialize_field("BINDING_TYPE_CHARACTER_PROPERTY", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BindingType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
