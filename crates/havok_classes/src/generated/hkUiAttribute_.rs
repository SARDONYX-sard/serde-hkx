use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkUiAttribute`
/// -         version: `2`
/// -       signature: `0xeb6e96e3`
/// -          size:  20(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkUiAttribute<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `visible`(ctype: `hkBool`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_visible: bool,
    /// # C++ Info
    /// -          name: `hideInModeler`(ctype: `enum HideInModeler`)
    /// -        offset:   1(x86)/  1(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hideInModeler: HideInModeler,
    /// # C++ Info
    /// -          name: `label`(ctype: `char*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_label: CString<'a>,
    /// # C++ Info
    /// -          name: `group`(ctype: `char*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_group: CString<'a>,
    /// # C++ Info
    /// -          name: `hideBaseClassMembers`(ctype: `char*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_hideBaseClassMembers: CString<'a>,
    /// # C++ Info
    /// -          name: `endGroup`(ctype: `hkBool`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_endGroup: bool,
    /// # C++ Info
    /// -          name: `endGroup2`(ctype: `hkBool`)
    /// -        offset:  17(x86)/ 33(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_endGroup2: bool,
    /// # C++ Info
    /// -          name: `advanced`(ctype: `hkBool`)
    /// -        offset:  18(x86)/ 34(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_advanced: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkUiAttribute<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkUiAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3949893347u32)
        }
    }
    impl<'a> __serde::Serialize for hkUiAttribute<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkUiAttribute", class_meta)?;
            serializer.serialize_field("visible", &self.m_visible)?;
            serializer.serialize_field("hideInModeler", &self.m_hideInModeler)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_cstring_meta_field("label", &self.m_label)?;
            serializer.serialize_cstring_meta_field("group", &self.m_group)?;
            serializer
                .serialize_cstring_meta_field(
                    "hideBaseClassMembers",
                    &self.m_hideBaseClassMembers,
                )?;
            serializer.serialize_field("endGroup", &self.m_endGroup)?;
            serializer.serialize_field("endGroup2", &self.m_endGroup2)?;
            serializer.serialize_field("advanced", &self.m_advanced)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_cstring_field("label", &self.m_label)?;
            serializer.serialize_cstring_field("group", &self.m_group)?;
            serializer
                .serialize_cstring_field(
                    "hideBaseClassMembers",
                    &self.m_hideBaseClassMembers,
                )?;
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
pub enum HideInModeler {
    #[default]
    NONE = 0isize,
    MAX = 1isize,
    MAYA = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for HideInModeler {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NONE => __serializer.serialize_field("NONE", &0u64),
                Self::MAX => __serializer.serialize_field("MAX", &1u64),
                Self::MAYA => __serializer.serialize_field("MAYA", &2u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum HideInModeler to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
