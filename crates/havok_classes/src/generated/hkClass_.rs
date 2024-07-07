use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkClass`
/// -         version: `0`
/// -       signature: `0x75585ef6`
/// -          size:  48(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkClass<'a> {
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
    /// -          name: `parent`(ctype: `struct hkClass*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_parent: Pointer,
    /// # C++ Info
    /// -          name: `objectSize`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_objectSize: i32,
    /// # C++ Info
    /// -          name: `numImplementedInterfaces`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numImplementedInterfaces: i32,
    /// # C++ Info
    /// -          name: `declaredEnums`(ctype: `hkSimpleArray<struct hkClassEnum>`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   8(x86)/ 12(x86_64)
    ///
    pub m_declaredEnums: Vec<hkClassEnum<'a>>,
    /// # C++ Info
    /// -          name: `declaredMembers`(ctype: `hkSimpleArray<struct hkClassMember>`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   8(x86)/ 12(x86_64)
    ///
    pub m_declaredMembers: Vec<hkClassMember<'a>>,
    /// # C++ Info
    /// -          name: `defaults`(ctype: `void*`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_defaults: Pointer,
    /// # C++ Info
    /// -          name: `attributes`(ctype: `struct hkCustomAttributes*`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_attributes: Pointer,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags FlagValues`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_flags: FlagValues,
    /// # C++ Info
    /// -          name: `describedVersion`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_describedVersion: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkClass<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkClass"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x75585ef6)
        }
    }
    impl<'a> _serde::Serialize for hkClass<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x75585ef6)));
            let mut serializer = __serializer.serialize_struct("hkClass", class_meta)?;
            serializer.serialize_cstring_meta_field("name", &self.m_name)?;
            serializer.serialize_field("parent", &self.m_parent)?;
            serializer.serialize_field("objectSize", &self.m_objectSize)?;
            serializer
                .serialize_field(
                    "numImplementedInterfaces",
                    &self.m_numImplementedInterfaces,
                )?;
            serializer.serialize_field("declaredEnums", &self.m_declaredEnums)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("declaredMembers", &self.m_declaredMembers)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("defaults", &self.m_defaults)?;
            serializer.skip_field("attributes", &self.m_attributes)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("describedVersion", &self.m_describedVersion)?;
            serializer.serialize_cstring_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    FlagValues : u32 { #[doc = "0"] const FLAGS_NONE = 0u32; #[doc = "1"] const
    FLAGS_NOT_SERIALIZABLE = 1u32; }
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
                    Self::FLAGS_NOT_SERIALIZABLE => {
                        __serializer
                            .serialize_field(
                                "FLAGS_NOT_SERIALIZABLE",
                                &Self::FLAGS_NOT_SERIALIZABLE,
                            )
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for FlagValues {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FlagValues>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FlagValues;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct FlagValues(flags)",
                    )
                }
                #[inline]
                fn visit_uint32<__E>(
                    self,
                    __value: u32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    Ok(FlagValues::from_bits_retain(__value as _))
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match <FlagValues as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint32,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FlagValues>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
