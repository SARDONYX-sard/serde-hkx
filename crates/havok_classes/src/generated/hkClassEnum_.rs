use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkClassEnum`
/// -         version: `0`
/// -       signature: `0x8a3609cf`
/// -          size:  20(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkClassEnum<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `name`(ctype: `char*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: CString<'a>,
    /// # C++ Info
    /// -          name: `items`(ctype: `hkSimpleArray<struct hkClassEnumItem>`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   8(x86)/ 12(x86_64)
    ///
    pub m_items: Vec<hkClassEnumItem<'a>>,
    /// # C++ Info
    /// -          name: `attributes`(ctype: `struct hkCustomAttributes*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_attributes: Pointer,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags FlagValues`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_flags: FlagValues,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkClassEnum<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkClassEnum"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2318797263u32)
        }
    }
    impl<'a> __serde::Serialize for hkClassEnum<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkClassEnum", class_meta)?;
            serializer.serialize_cstring_meta_field("name", &self.m_name)?;
            serializer.serialize_field("items", &self.m_items)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("attributes", &self.m_attributes)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_cstring_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)] pub
    struct FlagValues : u32 { #[doc = "0"] const FLAGS_NONE = 0u32; }
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for FlagValues {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::FLAGS_NONE => {
                        __serializer.serialize_field("FLAGS_NONE", &Self::FLAGS_NONE)
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
            __serializer.end()
        }
    }
};
